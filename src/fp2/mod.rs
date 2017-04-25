#![allow(non_snake_case)]

pub mod wrappers;

use std::fmt;
use big::wrappers::*;
use fp2::wrappers::*;

impl FP2 {
    pub fn from_BIGs(w: &mut FP2, x: &BIG, y: &BIG) {
        unsafe {
            FP2_from_BIGs(w, x, y);
        }
    }
}

impl Copy for FP2 { }

impl Clone for FP2 {
    fn clone(&self) -> FP2 {
        FP2 {
            a: self.a,
            b: self.b
        }
    }
}

impl fmt::Display for FP2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FP2: [ {}, {} ]", self.a, self.b)
    }
}

impl fmt::Debug for FP2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FP2: [ {}, {} ]", self.a, self.b)
    }
}
