use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

define_ty!(F32x8, f32, f32, f32, f32, f32, f32, f32, f32);
impl_minimal!(F32x8, f32, 8, x0, x1, x2, x3, x4, x5, x6, x7);

impl F32x8 {
    pub fn from_slice(slice: &[f32]) -> Self {
        assert!(slice.len() >= Self::lanes());
        unsafe {
            F32x8(
                *slice.get_unchecked(0),
                *slice.get_unchecked(1),
                *slice.get_unchecked(2),
                *slice.get_unchecked(3),
                *slice.get_unchecked(4),
                *slice.get_unchecked(5),
                *slice.get_unchecked(6),
                *slice.get_unchecked(7),
            )
        }
    }

    pub fn horizontal_add(self) -> f32 {
        self.0 + self.1 + self.2 + self.3 + self.4 + self.5 + self.6 + self.7
    }
}

impl_op8!(Mul, mul, F32x8, *);
impl_op8!(assn MulAssign, mul_assign, F32x8, *=);
impl_op8!(Div, div, F32x8, /);
impl_op8!(assn DivAssign, div_assign, F32x8, /=);
impl_op8!(Add, add, F32x8, +);
impl_op8!(assn AddAssign, add_assign, F32x8, +=);
impl_op8!(Sub, sub, F32x8, -);
impl_op8!(assn SubAssign, sub_assign, F32x8, -=);
impl_euclidean!(F32x8, f32);
