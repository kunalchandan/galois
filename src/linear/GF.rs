use num::integer::Integer;
// use crate::linear::scalar::*;

// Numbers in GF(N) are always integers.

pub struct GF<N: Integer> {
    val: u32
}

impl ToString for GF<N> {
    fn to_string(&self) -> String {
        String(self.val)
    }
}