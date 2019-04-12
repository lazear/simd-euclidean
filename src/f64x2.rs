use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

define_ty!(F64x2, f64, f64);
impl_minimal!(F64x2, f64, 2, x0, x1);

impl F64x2 {
    pub fn from_slice(slice: &[f64]) -> F64x2 {
        assert!(slice.len() >= Self::lanes());
        unsafe { F64x2(*slice.get_unchecked(0), *slice.get_unchecked(1)) }
    }

    pub fn horizontal_add(self) -> f64 {
        self.0 + self.1
    }
}

impl_op2!(Mul, mul, F64x2, *);
impl_op2!(assn MulAssign, mul_assign, F64x2, *=);
impl_op2!(Div, div, F64x2, /);
impl_op2!(assn DivAssign, div_assign, F64x2, /=);
impl_op2!(Add, add, F64x2, +);
impl_op2!(assn AddAssign, add_assign, F64x2, +=);
impl_op2!(Sub, sub, F64x2, -);
impl_op2!(assn SubAssign, sub_assign, F64x2, -=);

impl_euclidean!(F64x2, f64);
