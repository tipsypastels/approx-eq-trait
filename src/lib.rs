#![doc = include_str!("../README.md")]
#![no_std]

// Based on the "Relative epsilon method" defined here:
// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/.

mod sealed {
    pub trait Sealed {}

    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// A trait for floating point equality.
pub trait ApproxEq: sealed::Sealed {
    fn approx_eq(self, other: Self) -> bool;
}

impl ApproxEq for f32 {
    fn approx_eq(self, other: Self) -> bool {
        let diff = (self - other).abs();
        let a = self.abs();
        let b = other.abs();
        let largest = f32::max(a, b);
        diff <= largest * f32::EPSILON
    }
}

impl ApproxEq for f64 {
    fn approx_eq(self, other: Self) -> bool {
        let diff = (self - other).abs();
        let a = self.abs();
        let b = other.abs();
        let largest = f64::max(a, b);
        diff <= largest * f64::EPSILON
    }
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        assert!(
            $crate::ApproxEq::approx_eq(a, b),
            "assertion `{a}.approx_eq({b})` failed"
        )
    }};
}

#[macro_export]
macro_rules! assert_approx_eq_slice {
    ($a:expr, $b:expr) => {
        let a: &[_] = ::core::convert::AsRef::as_ref(&$a);
        let b: &[_] = ::core::convert::AsRef::as_ref(&$b);
        assert_eq!(
            a.len(),
            b.len(),
            "assertion `{a:?}.len() == {b:?}.len()` failed"
        );

        for (&a, &b) in a.iter().zip(b) {
            $crate::assert_approx_eq!(a, b);
        }
    };
}
