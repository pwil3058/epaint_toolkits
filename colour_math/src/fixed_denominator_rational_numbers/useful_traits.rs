// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::fmt::Debug;

use num_traits::{NumCast, PrimInt, Unsigned, Zero};

pub trait UnsignedCheckedCasts: Sized + Copy + PrimInt {
    fn bits() -> u32;
    fn to_usize(self) -> usize;
    fn to_u8(self) -> u8;
    fn to_u16(self) -> u16;
    fn to_u32(self) -> u32;
    fn to_u64(self) -> u64;
    fn to_u128(self) -> u128;
    fn to_f32(self) -> f32;
    fn to_f64(self) -> f64;
    fn fm_usize(arg: usize) -> Self;
    fn fm_u8(arg: u8) -> Self;
    fn fm_u16(arg: u16) -> Self;
    fn fm_u32(arg: u32) -> Self;
    fn fm_u64(arg: u64) -> Self;
    fn fm_u128(arg: u128) -> Self;
    fn fm_f32(arg: f32) -> Self;
    fn fm_f64(arg: f64) -> Self;
}

macro_rules! implement_unsigned_conversions (
    ($core: ty) => {
        impl UnsignedCheckedCasts for $core {
            fn bits() -> u32 {
                Self::BITS
            }

            fn to_usize(self) -> usize {
                if usize::BITS >= Self::bits() {
                    self as usize
                } else {
                    <usize as NumCast>::from(self).expect("value out of usize range")
                }
            }

            fn to_u8(self) -> u8 {
                if u8::BITS >= Self::bits() {
                    self as u8
                } else {
                    <u8 as NumCast>::from(self).expect("value ({self}) out of u8 range")
                }
            }

            fn to_u16(self) -> u16 {
                if u16::BITS >= Self::bits() {
                    self as u16
                } else {
                    <u16 as NumCast>::from(self).expect("value out of u16 range")
                }
            }

            fn to_u32(self) -> u32 {
                if u32::BITS >= Self::bits() {
                    self as u32
                } else {
                    <u32 as NumCast>::from(self).expect("value ({self}) out of u32 range")
                }
            }

            fn to_u64(self) -> u64 {
                if u64::BITS >= Self::bits() {
                    self as u64
                } else {
                    <u64 as NumCast>::from(self).expect("value out of u64 range")
                }
            }

            fn to_u128(self) -> u128 {
                self as u128
            }

            fn to_f32(self) -> f32 {
                self as f32
            }

            fn to_f64(self) -> f64 {
                self as f64
            }

            fn fm_usize(arg: usize) -> Self {
                if usize::BITS <= Self::bits() {
                    arg as Self
                } else {
                    <Self as NumCast>::from(arg).expect("value out of range")
                }
            }

            fn fm_u8(arg: u8) -> Self {
                if u8::BITS <= Self::bits() {
                    arg as Self
                } else {
                    <Self as NumCast>::from(arg).expect("value out of range")
                }
            }

            fn fm_u16(arg: u16) -> Self {
                if u16::BITS <= Self::bits() {
                    arg as Self
                } else {
                    <Self as NumCast>::from(arg).expect("value out of range")
                }
            }

            fn fm_u32(arg: u32) -> Self {
                if u32::BITS <= Self::bits() {
                    arg as Self
                } else {
                    <Self as NumCast>::from(arg).expect("value out of range")
                }
            }

            fn fm_u64(arg: u64) -> Self {
                if u64::BITS <= Self::bits() {
                    arg as Self
                } else {
                    <Self as NumCast>::from(arg).expect("value out of range")
                }
            }

            fn fm_u128(arg: u128) -> Self {
                <Self as NumCast>::from(arg).expect("value out of range")
            }

            fn fm_f32(arg: f32) -> Self {
                <Self as NumCast>::from(arg).expect("value out of range")
            }

            fn fm_f64(arg: f64) -> Self {
                <Self as NumCast>::from(arg).expect("value out of range")
            }
        }
    }
);

implement_unsigned_conversions!(u8);
implement_unsigned_conversions!(u16);
implement_unsigned_conversions!(u32);
implement_unsigned_conversions!(u64);
implement_unsigned_conversions!(u128);

pub trait FromAsProp<T: Sized> {
    fn from_as_prop(value: T) -> Self;
}

pub trait IntoAsProp<T: Sized> {
    fn into_as_prop(self) -> T;
}

impl<T: FromAsProp<U>, U> IntoAsProp<T> for U {
    fn into_as_prop(self) -> T {
        T::from_as_prop(self)
    }
}

pub trait AsProportion: Sized + Copy + PartialEq + PartialOrd + Debug {
    const ZERO: Self;
    const ALMOST_ZERO: Self;
    const ONE: Self;
    const ALMOST_ONE: Self;
    const ONE_QUARTER: Self;
    const HALF: Self;
    const THREE_QUARTERS: Self;

    fn is_valid_proportion(&self) -> bool {
        *self >= Self::ZERO && *self <= Self::ONE
    }
}

impl AsProportion for f64 {
    const ONE: Self = 1.0;
    const ALMOST_ONE: Self = 1.0 - 1.0e-20;
    const ZERO: Self = 0.0;
    const ALMOST_ZERO: Self = 1.0e-20;
    const ONE_QUARTER: Self = 0.25;
    const HALF: Self = 0.5;
    const THREE_QUARTERS: Self = 0.75;
}

macro_rules! impl_as_proportion {
    ($unsigned:ty) => {
        impl AsProportion for $unsigned {
            const ZERO: Self = 0;
            const ALMOST_ZERO: Self = 1;
            const ONE: Self = Self::MAX;
            const ALMOST_ONE: Self = Self::MAX - 1;
            const ONE_QUARTER: Self = Self::MAX / 4;
            const HALF: Self = Self::MAX / 2;
            // make sure quarters add up to one
            const THREE_QUARTERS: Self = Self::MAX - Self::MAX / 4;
        }
    };
}

impl_as_proportion!(u8);
impl_as_proportion!(u16);
impl_as_proportion!(u32);
impl_as_proportion!(u64);

pub trait UnsignedInteger: Unsigned + UnsignedCheckedCasts + Zero {
    fn non_zero(self) -> bool {
        !self.is_zero()
    }
}

impl UnsignedInteger for u8 {}
impl UnsignedInteger for u16 {}
impl UnsignedInteger for u32 {}
impl UnsignedInteger for u64 {}

pub trait IntoFromRawI128 {
    /// Raw from an i128 value (will panic if value is greater than the maximum value of Self)
    fn from_i128(value: i128) -> Self;
    /// Into a raw u128 value
    fn into_i128(self) -> i128;
}

impl<S: SignedInteger> IntoFromRawI128 for S {
    fn from_i128(value: i128) -> Self {
        value
            .try_into()
            .map_err(|_| "whatever")
            .expect("Value is greater than the maximum value of u8")
    }

    fn into_i128(self) -> i128 {
        self.into()
    }
}

pub trait Signed: Copy + Debug {}
pub trait SignedInteger: Signed + Into<i128> + TryFrom<i128> {}

impl Signed for i8 {}
impl Signed for i16 {}
impl Signed for i32 {}
impl Signed for i64 {}
impl Signed for i128 {}
impl SignedInteger for i8 {}
impl SignedInteger for i16 {}
impl SignedInteger for i32 {}
impl SignedInteger for i64 {}

#[cfg(test)]
pub mod trait_tests {
    use super::*;

    #[test]
    fn test_casts() {
        assert_eq!(u8::fm_f32(10.0), 10u8);
    }
}
