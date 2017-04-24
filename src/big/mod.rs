#[macro_use]
pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use std::cmp::Ordering;
use std::fmt;
use big::wrappers::*;

impl Ord for BIG {
    fn cmp(&self, other: &BIG) -> Ordering {
        let r = unsafe { BIG_comp(self, other) };
        if r > 0 {
            return Ordering::Greater;
        }
        if r < 0 {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

impl Eq for BIG {}

impl PartialOrd for BIG {
    fn partial_cmp(&self, other: &BIG) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BIG {
    fn eq(&self, other: &BIG) -> bool {
        self.val == other.val
    }
}

impl Copy for BIG { }

impl Clone for BIG {
    fn clone(&self) -> BIG {
        BIG {
            val: self.val
        }
    }
}

impl BIG {
    pub fn new_int(x:isize) -> BIG {
        let mut s= BIG::default();
        s.val[0]=x as chunk;
        return s;
    }

    pub fn new_copy(y:&BIG) -> BIG {
        let mut s= BIG::default();
        for i in 0..NLEN {
            s.val[i]=y.val[i];
        }
        return s;
    }

    pub fn iszilch(&self) -> bool {
        for i in 0 ..NLEN {
            if self.val[i]!=0 {
                return false;
            }
        }
        return true;
    }

    pub fn parity(&self) -> isize {
        return (self.val[0]%2) as isize;
    }

    pub fn powmod(&mut self, e: &mut BIG,m: &BIG) -> BIG {
        self.norm();
        e.norm();
        let mut a=BIG::new_int(1);
        let mut z=BIG::new_copy(e);
        let mut s=BIG::new_copy(self);
        loop {
            let bt=z.parity();
            z.fshr(1);
            if bt==1 {
                a = BIG::modmul(&a, &s, m);
            }
            if z.iszilch() {break}
            s = BIG::modsqr(&s, m);
        }
        return a;
    }

    pub fn nbits(a: &BIG) -> i32 {
        let mut ret;
        unsafe {
            ret = BIG_nbits(a) as i32;
        }
        return ret;
    }

    pub fn reduce(&mut self) {
        let p = Modulus;
        BIG::rmod(self, &p);
    }

    pub fn norm(&mut self) -> chunk {
        let mut ret;
        unsafe {
            ret = BIG_norm(self) as chunk;
        }
        return ret;
    }

    pub fn fshr(&mut self, k: i32) -> i32 {
        let mut ret;
        unsafe {
            ret = BIG_fshr(self, k as c_int) as i32;
        }
        return ret;
    }

    pub fn fshl(&mut self, k: i32) -> i32 {
        let mut ret;
        unsafe {
            ret = BIG_fshl(self, k as c_int) as i32;
        }
        return ret;
    }

    pub fn copy(d: &mut BIG, s: &BIG) {
        unsafe {
            BIG_copy(d, s);
        }
    }

    pub fn shr(a: &mut BIG, k: i32) {
        unsafe {
            BIG_shr(a, k as c_int);
        }
    }

    pub fn rcopy(b: &mut BIG, a: &BIG) {
        unsafe {
            BIG_copy(b, a);
        }
    }

    pub fn comp(a: &BIG, b: &BIG) -> i32 {
        let mut ret;
        unsafe {
            ret = BIG_comp(a, b) as i32;
        }
        return ret;
    }

    pub fn modsqr(a: &BIG, b: &BIG) -> BIG {
        let mut r: BIG = BIG::default();
        unsafe {
            BIG_modsqr(&mut r, a, b);
        }
        return r;
    }

    pub fn add(a: &BIG, b: &BIG) -> BIG {
        let mut r = BIG::default();
        unsafe {
            BIG_add(&mut r, a, b);
        }
        return r;
    }

    pub fn rmod(b: &mut BIG, c: &BIG) {
        unsafe {
            BIG_mod(b, c);
        }
    }

    pub fn sqr(c: &BIG) -> DBIG {
        let mut r: DBIG = DBIG::default();
        unsafe {
            BIG_sqr(&mut r, c);
        }
        return r;
    }

    pub fn sqrm(&mut self) {
        let r: DBIG = BIG::sqr(self);
        for i in 0..NLEN {
            self.val[i] = r.val[i];
        }
    }

    pub fn one(&mut self) {
        self.val[0]=1;
        for i in 1 ..NLEN {
            self.val[i]=0;
        }
    }

    pub fn modmul(a: &BIG, b: &BIG, m: &BIG) -> BIG {
        let mut r: BIG = BIG::default();
        unsafe {
            BIG_modmul(&mut r, a, b, m);
        }
        return r;
    }

    pub fn mul(a: &BIG, b: &BIG) -> DBIG {
        let mut r: DBIG = DBIG::default();
        unsafe {
            BIG_mul(&mut r, a, b);
        }
        return r;
    }

    pub fn mulm(a: &BIG, b: &BIG) -> BIG {
        let mut ret: BIG = BIG::default();
        let mut r: DBIG = DBIG::default();
        unsafe {
            BIG_mul(&mut r, a, b);
        }
        for i in 0..NLEN {
            ret.val[i] = r.val[i];
        }
        return ret;
    }

    pub fn imul(a: &BIG, b: i32) -> BIG {
        let mut r: BIG = BIG::default();
        unsafe {
            BIG_imul(&mut r, a, b as c_int);
        }
        return r;
    }

    pub fn modneg(r: &mut BIG, a: &mut BIG, m: &BIG) {
        unsafe {
            BIG_modneg(r, a, m);
        }
    }

    pub fn excess(a: &BIG) -> chunk {
        return (a.val[NLEN-1] & OMASK) >> (MBITS % BASEBITS);
    }

    fn logb2(w: u32) -> usize {
        let mut v=w;
        v |= v >> 1;
        v |= v >> 2;
        v |= v >> 4;
        v |= v >> 8;
        v |= v >> 16;

        v = v - ((v >> 1) & 0x55555555);
        v = (v & 0x33333333) + ((v >> 2) & 0x33333333);
        let r= ((   ((v + (v >> 4)) & 0xF0F0F0F)   * 0x1010101) >> 24) as usize;
        return r+1;
    }

    pub fn neg(&mut self) {
        let mut p = Modulus;
        self.norm();
        let sb = BIG::logb2(BIG::excess(&self) as u32);
        BIG::fshl(&mut p, sb as i32);
        self.rsub(&p);
        if BIG::excess(&self)>=FEXCESS {
            self.reduce();
        }
    }

    pub fn rsub(&mut self, x:&BIG) {
        for i in 0 ..NLEN {
            self.val[i]=x.val[i]-self.val[i]
        }
    }

    pub fn toBytes(b: &mut [u8], a: &BIG) {
        unsafe {
            BIG_toBytes(&mut b[0], a);
        }
    }

    pub fn fromBytes(b: &[u8]) -> BIG {
        let mut ret: BIG = BIG::default();
        unsafe {
            BIG_fromBytes(&mut ret, b.as_ptr());
        }
        return ret;
    }
}

impl fmt::Display for BIG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BIG: [{}]", big_to_hex(self))
    }
}

impl fmt::Debug for BIG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BIG: [{}]", big_to_hex(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes() {
        let mut bytes: [u8; MODBYTES] = [0; MODBYTES];
        let mut outbytes: [u8; MODBYTES] = [0; MODBYTES];
        bytes[0] = 0xFF;
        let a: BIG = BIG::fromBytes(&bytes[..]);
        BIG::toBytes(&mut outbytes[..], &a);
        assert_eq!(bytes, outbytes);
    }
}
