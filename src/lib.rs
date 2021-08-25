//!
//! ```rust
//! # use simd_euclidean::*;
//! # use rand::*;
//! for &i in [16, 32, 64, 128].into_iter() {
//!   // Dispatch to F32x4 or F32x8 (above 64 elements)
//!     let mut rng = rand::thread_rng();
//!     let a = (0..i).map(|_| rng.gen::<f32>()).collect::<Vec<f32>>();
//!     let b = (0..i).map(|_| rng.gen::<f32>()).collect::<Vec<f32>>();

//!     let v = Vectorized::distance(&a, &b);
//!     let n = Naive::distance(&a, &b);
//!     assert!((n-v).abs() < 0.00001);
//! }
//! ```

#[macro_use]
mod macros;

mod f32x4;
mod f32x8;

mod f64x2;
mod f64x4;

pub use f32x4::F32x4;
pub use f32x8::F32x8;
pub use f64x2::F64x2;
pub use f64x4::F64x4;

pub trait Naive {
    type Output;
    type Ty;

    fn squared_distance(self, other: Self) -> Self::Output;
    fn distance(self, other: Self) -> Self::Output;
}

pub trait Vectorized {
    type Output;
    fn squared_distance(self, other: Self) -> Self::Output;
    fn distance(self, other: Self) -> Self::Output;
}

impl_naive!(f64, f64);
impl_naive!(f32, f32);

/// Calculate the euclidean distance between two slices of equal length
///
/// # Panics
///
/// Will panic if the lengths of the slices are not equal
pub fn scalar_euclidean<T: Naive>(a: T, b: T) -> T::Output {
    Naive::distance(a, b)
}

/// SIMD-capable calculation of the euclidean distance between two slices
/// of equal length
///
/// ```rust
/// # use simd_euclidean::*;
/// let distance = vector_euclidean(&[0.1, 0.2, 0.3, 0.4f32] as &[f32], &[0.4, 0.3, 0.2, 0.1f32]);
/// ```
/// # Panics
///
/// Will panic if the lengths of the slices are not equal
pub fn vector_euclidean<T: Vectorized>(a: T, b: T) -> T::Output {
    Vectorized::distance(a, b)
}

impl Vectorized for &[f32] {
    type Output = f32;
    fn squared_distance(self, other: Self) -> Self::Output {
        if self.len() >= 64 {
            F32x8::squared_distance(self, other)
        } else {
            F32x4::squared_distance(self, other)
        }
    }

    fn distance(self, other: Self) -> Self::Output {
        Vectorized::squared_distance(self, other).sqrt()
    }
}

impl Vectorized for &Vec<f32> {
    type Output = f32;
    fn squared_distance(self, other: Self) -> Self::Output {
        if self.len() >= 64 {
            F32x8::squared_distance(self, other)
        } else {
            F32x4::squared_distance(self, other)
        }
    }

    fn distance(self, other: Self) -> Self::Output {
        Vectorized::squared_distance(self, other).sqrt()
    }
}

impl Vectorized for &[f64] {
    type Output = f64;
    fn squared_distance(self, other: Self) -> Self::Output {
        if self.len() >= 16 {
            F64x4::squared_distance(self, other)
        } else {
            F64x2::squared_distance(self, other)
        }
    }

    fn distance(self, other: Self) -> Self::Output {
        Vectorized::squared_distance(self, other).sqrt()
    }
}

impl Vectorized for &Vec<f64> {
    type Output = f64;
    fn squared_distance(self, other: Self) -> Self::Output {
        if self.len() >= 16 {
            F64x4::squared_distance(self, other)
        } else {
            F64x2::squared_distance(self, other)
        }
    }

    fn distance(self, other: Self) -> Self::Output {
        Vectorized::squared_distance(self, other).sqrt()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub const XS: [f32; 72] = [
        6.1125, 10.795, 20.0, 0.0, 10.55, 10.63, 20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.26, 10.73, 0.0,
        0.0, 20.0, 0.0, 10.4975, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 20.0, 0.0, 20.0, 20.0, 20.0,
        0.0, 0.0, 0.0, 0.0, 10.475, 6.0905, 20.0, 0.0, 20.0, 20.0, 0.0, 10.5375, 10.54, 10.575,
        0.0, 0.0, 0.0, 10.76, 10.755, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 20.0,
        20.0, 0.0, 20.0, 0.0, 0.0, 20.0, 0.0, 0.0, 0.0, 20.0,
    ];
    pub const YS: [f32; 72] = [
        6.0905, 20.0, 0.0, 20.0, 20.0, 0.0, 10.5375, 10.54, 10.575, 0.0, 0.0, 0.0, 10.76, 10.755,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 20.0, 20.0, 0.0, 20.0, 0.0, 0.0,
        20.0, 0.0, 0.0, 0.0, 20.0, 6.1125, 10.795, 20.0, 0.0, 10.55, 10.63, 20.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 10.26, 10.73, 0.0, 0.0, 20.0, 0.0, 10.4975, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        20.0, 0.0, 20.0, 20.0, 20.0, 0.0, 0.0, 0.0, 0.0, 10.475,
    ];

    #[test]
    fn verify() {
        for i in 0..XS.len() {
            let x = &XS[..i];
            let y = &YS[..i];
            let res = scalar_euclidean(x, y);
            assert!(
                (Vectorized::distance(x, y) - res).abs() < 0.0001,
                "iter {}, {} != {}",
                i,
                Vectorized::distance(x, y),
                res
            );
            assert!(
                (F32x8::distance(x, y) - res).abs() < 0.0001,
                "iter {}, {} != {}",
                i,
                F32x8::distance(x, y),
                res
            );
            assert!(
                (F32x4::distance(x, y) - res).abs() < 0.0001,
                "iter {}, {} != {}",
                i,
                F32x4::distance(x, y),
                res
            );
        }
    }

    #[test]
    fn verify_random() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let input_sizes = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

        for &i in input_sizes.iter() {
            let len = i + rng.gen_range(0, 16) as usize;
            let mut a = Vec::with_capacity(len);
            let mut b = Vec::with_capacity(len);

            for _ in 0..len {
                a.push(rng.gen::<f32>());
                b.push(rng.gen::<f32>());
            }

            let diff = (vector_euclidean(&a, &b) - scalar_euclidean(&a, &b)).abs();
            assert!(diff <= 0.0001, "diff = {}, len = {}", diff, i);
        }
    }

    #[test]
    fn smoke_mul() {
        let a = F32x4::from_slice(&[1.0, 2.0, 3.0, 4.0]);
        let b = F32x4::from_slice(&[4.0, 3.0, 2.0, 1.0]);
        let c = a * b;
        assert_eq!(c.horizontal_add(), 4.0 + 6.0 + 6.0 + 4.0);
    }

    #[test]
    fn smoke_mul_assign() {
        let mut a = F32x4::from_slice(&[1.0, 2.0, 3.0, 4.0]);
        let b = F32x4::from_slice(&[4.0, 3.0, 2.0, 1.0]);
        a *= b;
        assert_eq!(a.horizontal_add(), 4.0 + 6.0 + 6.0 + 4.0);
    }

    #[test]
    fn smoke_add() {
        let a = F32x4::from_slice(&[1.0, 2.0, 3.0, 4.0]);
        let b = F32x4::from_slice(&[4.0, 3.0, 2.0, 1.0]);
        let c = a + b;
        assert_eq!(c, F32x4::new(5.0, 5.0, 5.0, 5.0));
    }

    #[test]
    fn smoke_sub() {
        let a = F32x4::from_slice(&[1.0, 2.0, 3.0, 4.0]);
        let b = F32x4::from_slice(&[4.0, 3.0, 2.0, 1.0]);
        let c = a - b;
        assert_eq!(c, F32x4::new(-3.0, -1.0, 1.0, 3.0));
    }
}
