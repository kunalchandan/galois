use std::ops::{Add, AddAssign,
               Sub, SubAssign,
               Mul, MulAssign,
               Div, DivAssign,
               BitOr, BitOrAssign,
               BitAnd, BitAndAssign,
               BitXor, BitXorAssign,
               Not, Neg};

struct GF2 {
    value: bool
}

impl Add<GF2> for GF2 {
    type Output = GF2;
    fn add(self, _rhs: GF2) -> GF2 {
        GF2 {
            value: self.value ^ _rhs.value
        }
    }
}
impl AddAssign<GF2> for GF2 {
    fn add_assign(&mut self, rhs: GF2) {
        self.value ^= rhs.value
    }
}


impl Sub<GF2> for GF2 {
    type Output = GF2;
    fn sub(self, _rhs: GF2) -> GF2 {
        GF2 {
            value: self.value ^ _rhs.value
        }
    }
}
impl SubAssign<GF2> for GF2 {
    fn sub_assign(&mut self, rhs: GF2) {
        self.value ^= rhs.value
    }
}


impl Mul<GF2> for GF2 {
    type Output = GF2;
    fn mul(self, _rhs: GF2) -> GF2 {
        GF2 {
            value: self.value & _rhs.value
        }
    }
}
impl MulAssign<GF2> for GF2 {
    fn mul_assign(&mut self, rhs: GF2) {
        self.value &= rhs.value
    }
}


impl Div<GF2> for GF2 {
    type Output = GF2;
    fn div(self, _rhs: GF2) -> GF2 {
        GF2 {
            value: self.value & _rhs.value
        }
    }
}
impl DivAssign<GF2> for GF2 {
    fn div_assign(&mut self, rhs: GF2) {
        self.value &= rhs.value
    }
}

impl BitXor<GF2> for GF2 {
    type Output = GF2;
    fn bitxor(self, _rhs: GF2) -> GF2 {
        GF2 {
            value: self.value ^ _rhs.value
        }
    }
}
impl BitXorAssign<GF2> for GF2 {
    fn bitxor_assign(&mut self, rhs: GF2) {
        self.value ^= rhs.value
    }
}


impl BitAnd<GF2> for GF2 {
    type Output = GF2;
    fn bitand(self, _rhs: GF2) -> GF2 {
        GF2 {
            value: self.value & _rhs.value
        }
    }
}
impl BitAndAssign<GF2> for GF2 {
    fn bitand_assign(&mut self, rhs: GF2) {
        self.value &= rhs.value
    }
}


impl BitOr<GF2> for GF2 {
    type Output = GF2;
    fn bitor(self, _rhs: GF2) -> GF2 {
        GF2 {
            value: self.value | _rhs.value
        }
    }
}
impl BitOrAssign<GF2> for GF2 {
    fn bitor_assign(&mut self, rhs: GF2) {
        self.value |= rhs.value;
    }
}

impl Neg for GF2 {
    type Output = GF2;
    fn neg(self) -> GF2 {
        GF2 {
            value: !self.value
        }
    }
}
impl Not for GF2 {
    type Output = GF2;
    fn not(self) -> GF2 {
        GF2 {
            value: !self.value
        }
    }
}