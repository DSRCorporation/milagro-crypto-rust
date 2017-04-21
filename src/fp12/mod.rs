pub mod wrappers;

extern crate libc;
use self::libc::{c_int};

use std::fmt;
use big::wrappers::BIG;
use randapi::wrappers::octet;
use fp12::wrappers::*;

impl FP12 {
    pub fn mul(w: &mut FP12, y: &FP12) {
        unsafe {
            FP12_mul(w, y);
        }
    }

    pub fn pow(r: &mut FP12, a: &FP12, b: &BIG) {
        unsafe {
            FP12_pow(r, a, b);
        }
    }

    pub fn inv(w: &mut FP12, x: &FP12) {
        unsafe {
            FP12_inv(w, x);
        }
    }

    pub fn toOctet(W: &mut octet, g: &FP12) {
        unsafe {
            FP12_toOctet(W, g);
        }
    }

    pub fn fromOctet(W: *const octet) -> FP12 {
        let mut ret: FP12 = FP12::default();
        unsafe {
            FP12_fromOctet(&mut ret, W);
        }
        return ret;
    }
}

impl fmt::Display for FP12 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FP12: [ {}, {}, {} ]", self.a, self.b, self.c)
    }
}

impl fmt::Debug for FP12 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FP12: [ {}, {}, {} ]", self.a, self.b, self.c)
    }
}