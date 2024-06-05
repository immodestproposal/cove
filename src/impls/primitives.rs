//! This module provides built-in implementation of the casting traits for primitive types

#![allow(clippy::wildcard_imports)]

use crate::casts::{Cast, Closest};
use crate::errors::LossyCastError;
use crate::base::CastImpl;
use super::LosslessCast;

macro_rules! cast {
    ($($num:ty),+) => {
        $(
            impl Cast for $num {}
        )*
    };

    (integer $from:ty => $($to:ty),+) => {
        $(
            impl CastImpl<$to> for $from {
                type Error = LossyCastError<Self, $to>;

                #[inline]
                fn cast_impl(self) -> Result<$to, Self::Error> {
                    self.try_into().map_err(|_| LossyCastError {
                        from: self,
                        to: self as $to
                    })
                }
            }

            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // Cast failed; if this is less than 0 use the target's MIN, otherwise
                    // use its MAX. Note that the branch will be optimized away for unsigned source 
                    // types, at least in release builds.
                    #[allow(unused_comparisons)]
                    match self.from < 0 {
                        true => <$to>::MIN,
                        false => <$to>::MAX
                    }
                }
            }
        )*
    };

    (int_to_float $from:ty => $($to:ty),+) => {
        $(
            impl CastImpl<$to> for $from {
                type Error = LossyCastError<Self, $to>;

                #[inline]
                fn cast_impl(self) -> Result<$to, Self::Error> {
                    // This implementation leverages the float_to_int cast to do a checked round
                    // cast: int -> float -> int, where the float -> int portion is checked. If
                    // the float -> int portion is lossless AND yields the starting value, the 
                    // int -> float portion must also have been lossless; otherwise, it must have
                    // been lossy.
                    let value = self as $to;
                    match CastImpl::<$from>::cast_impl(value) {
                        Ok(this) if this == self => Ok(value),
                        _ => Err(LossyCastError {
                            from: self,
                            to: value
                        })
                    }
                }
            }

            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // For int-to-float the raw cast is the closest except in the case of overflow
                    match self.to {
                        <$to>::INFINITY => <$to>::MAX,
                        <$to>::NEG_INFINITY => <$to>::MIN,
                        _ => self.to
                    }
                }
            }
        )*
    };

    (float_to_int $from:ty as $int:ty => $(($to:ty, $max:expr)),+) => {
        $(
            impl CastImpl<$to> for $from {
                type Error = LossyCastError<Self, $to>;

                #[inline]
                fn cast_impl(self) -> Result<$to, Self::Error> {
                    // Compute bit count constants for this floating point type
                    const TOTAL_BITS: u32 = core::mem::size_of::<$from>() as u32 * 8;
                    const SIGN_BITS: u32 = 1;
                    const MANTISSA_BITS: u32 = <$from>::MANTISSA_DIGITS - 1;
                    const EXPONENT_BITS: u32 = TOTAL_BITS - MANTISSA_BITS - SIGN_BITS;

                    // Compute mask constants for this floating point type
                    const MANTISSA_MASK: $int = <$int>::MAX >> (TOTAL_BITS - MANTISSA_BITS);
                    const EXPONENT_MASK: $int = <$int>::MAX >> (TOTAL_BITS - EXPONENT_BITS);
                    const EXPONENT_BIAS: $int = EXPONENT_MASK >> 1;

                    // Extract the exponent from the raw bits
                    let bits = self.to_bits();
                    let exponent = (bits >> MANTISSA_BITS) & EXPONENT_MASK;

                    // Check the exponent to determine whether the float is an int
                    let is_int = match exponent {
                        // A zero exponent implies a subnormal: either a fraction or the value 0
                        0 => (bits & MANTISSA_MASK) == 0,

                        // A max exponent indicates infinity or NaN; in all cases, not an integer
                        EXPONENT_MASK => false,

                        // Not a special case exponent, so adjust for the bias
                        exponent => match exponent.checked_sub(EXPONENT_BIAS) {
                            // A negative exponent implies a fraction
                            None => false,

                            // Positive exponent; adjust the mantissa for the exponent and determine
                            // whether there any remaining bits to make this a fraction
                            Some(exponent) => {
                                let mask = MANTISSA_MASK.checked_shr(exponent as u32).unwrap_or(0);
                                (bits & (mask)) == 0
                            }
                        }
                    };

                    // If the float is an int we also need to check that it is in the target type's
                    // range. All integer MIN values can be casted losslessly to floats, but this
                    // is not true for all MAXs; therefore, we accept the max castable as a macro
                    // argument (in float form).
                    match is_int && self >= <$to>::MIN as $from && self <= $max {
                        true => Ok(self as $to),
                        false => Err(LossyCastError {
                            from: self,
                            to: self as $to
                        })
                    }
                }
            }

            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // For float-to-int we must first round the number, then use the raw cast. If we
                    // don't round first the default will round towards zero.
                    #[cfg(feature = "std")] {
                        // We have access to std so use the built-in round() function (which uses a
                        // compiler intrinsic in its turn)
                        self.from.round() as $to
                    }

                    #[cfg(not(feature = "std"))] {
                        // We lack access to std so we must implement our own slower round()
                        match self.from.is_sign_positive() {
                            true => (self.from + 0.5) as $to,
                            false => (self.from - 0.5) as $to
                        }
                    }
                }
            }
        )*
    };

    (float_to_self $($float:ty),+) => {
        $(
            impl CastImpl<$float> for $float {
                type Error = LossyCastError<Self, Self>;

                #[inline]
                fn cast_impl(self) -> Result<Self, Self::Error> {
                    match self.is_nan() {
                        false => Ok(self),
                        true => Err(LossyCastError {
                            from: self,
                            to: self
                        })
                    }
                }
            }

            impl Closest<$float> for LossyCastError<$float, $float> {
                #[inline]
                fn closest(self) -> $float {
                    self.from
                }
            }
        )*
    };

    (int_to_float $first:ty, $($from:ty),+ => $to:ty) => {
        cast!(int_to_float $first => $to);
        $(cast!(int_to_float $from => $to));*;
    };

    (float_to_int $first:ty, $($from:ty),+ => $to:ty) => {
        cast!(float_to_int $first => $to);
        $(cast!(float_to_int $from => $to));*;
    };

    (lossless $from:ty => $($to:ty),+) => {
        $(impl LosslessCast for LossyCastError<$from, $to> {})*
    };

    (lossless $first:ty, $($from:ty),+ => $to:ty) => {
        cast!(lossless $first => $to);
        $(cast!(lossless $from => $to));*;
    };
}

// -- Macro-Generated Bulk Implementations: Portable -- //
cast!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

cast!(integer u8 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast!(integer u16 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast!(integer u32 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f64);
cast!(integer u64 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
cast!(integer u128 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
cast!(integer usize => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

cast!(integer i8 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast!(integer i16 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast!(integer i32 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f64);
cast!(integer i64 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
cast!(integer i128 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
cast!(integer isize => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

cast!(int_to_float u32, u64, u128, usize, i32, i64, i128, isize => f32);
cast!(int_to_float u64, u128, usize, i64, i128, isize => f64);

// Supply precomputed max values for each integer type
cast!(
    float_to_int f32 as u32 => 
    (u8, 255_f32), // Same as MAX
    (u16, 65_535_f32), // Same as MAX
    (u32, 4_294_967_040_f32), 
    (u64, 18_446_742_974_197_923_840_f32), 
    (u128, f32::INFINITY), 
    (i8, 127_f32), // Same as MAX
    (i16, 32_767_f32), // Same as MAX
    (i32, 2_147_483_520_f32),
    (i64, 9_223_371_487_098_961_920_f32),
    (i128, 170_141_173_319_264_429_905_852_091_742_258_462_720_f32)
);

// Supply precomputed max values for each integer type
cast!(
    float_to_int f64 as u64 => 
    (u8, 255_f64), // Same as MAX
    (u16, 65_535_f64), // Same as MAX
    (u32, 4_294_967_295_f64), // Same as MAX
    (u64, 18_446_744_073_709_549_568_f64),
    (u128, 340_282_366_920_938_425_684_442_744_474_606_501_888_f64),
    (i8, 127_f64), // Same as MAX
    (i16, 32_767_f64), // Same as MAX
    (i32, 2_147_483_647_f64), // Same as MAX
    (i64, 9_223_372_036_854_774_784_f64),
    (i128, 170_141_183_460_469_212_842_221_372_237_303_250_944_f64)
);

cast!(float_to_self f32, f64);

cast!(lossless u8 => u8, u16, u32, u64, u128, i16, i32, i64, i128, f32, f64);
cast!(lossless u16 => u16, u32, u64, u128, i32, i64, i128, f32, f64);
cast!(lossless u32 => u32, u64, u128, i64, i128, f64);
cast!(lossless u64 => u64, u128, i128);
cast!(lossless u128 => u128);
cast!(lossless usize => usize);

cast!(lossless i8 => i8, i16, i32, i64, i128, f32, f64);
cast!(lossless i16 => i16, i32, i64, i128, f32, f64);
cast!(lossless i32 => i32, i64, i128, f64);
cast!(lossless i64 => i64, i128);
cast!(lossless i128 => i128);
cast!(lossless isize => isize);

cast!(lossless f32 => f32, f64);
cast!(lossless f64 => f64);

// -- Macro-Generated Bulk Implementations: Non-Portable -- //
#[cfg(target_pointer_width = "16")]
mod platform_dependent {
    use super::*;

    cast!(lossless usize => u16, u32, u64, u128, i32, i64, i128, f32, f64);
    cast!(lossless isize => i16, i32, i64, i128, f32, f64);

    cast!(lossless u8, u16 => usize);
    cast!(lossless u8, i8, i16 => isize);

    cast!(float_to_int f32 as u32 => (usize, 65_535_f32));
    cast!(float_to_int f32 as u32 => (isize, 32_767_f32));

    cast!(float_to_int f64 as u64 => (usize, 65_535_f64));
    cast!(float_to_int f64 as u64 => (isize, 32_767_f64));
}

#[cfg(target_pointer_width = "32")]
mod platform_dependent {
    use super::*;

    cast!(lossless usize => u32, u64, u128, i64, i128, f64);
    cast!(lossless isize => i32, i64, i128, f64);

    cast!(lossless u8, u16, u32 => usize);
    cast!(lossless u8, u16, i8, i16, i32 => isize);

    cast!(float_to_int f32 as u32 => (usize, 4_294_967_040_f32));
    cast!(float_to_int f32 as u32 => (isize, 2_147_483_520_f32));

    cast!(float_to_int f64 as u64 => (usize, 4_294_967_295_f64));
    cast!(float_to_int f64 as u64 => (isize, 2_147_483_647_f64));
}

#[cfg(target_pointer_width = "64")]
mod platform_dependent {
    use super::*;

    cast!(lossless usize => u64, u128, i128);
    cast!(lossless isize => i64, i128);

    cast!(lossless u8, u16, u32, u64 => usize);
    cast!(lossless u8, u16, u32, i8, i16, i32, i64 => isize);

    cast!(float_to_int f32 as u32 => (usize, 18_446_742_974_197_923_840_f32));
    cast!(float_to_int f32 as u32 => (isize, 9_223_371_487_098_961_920_f32));

    cast!(float_to_int f64 as u64 => (usize, 18_446_744_073_709_549_568_f64));
    cast!(float_to_int f64 as u64 => (isize, 9_223_372_036_854_774_784_f64));
}

// -- Manual Implementations -- //
impl CastImpl<f64> for f32 {
    type Error = LossyCastError<Self, f64>;
    
    #[inline]
    fn cast_impl(self) -> Result<f64, Self::Error> {
        match self.is_nan() {
            false => Ok(f64::from(self)),
            true => Err(LossyCastError {
                from: self,
                to: f64::from(self)
            })
        }
    }    
}

impl Closest<f64> for LossyCastError<f32, f64> {
    #[inline]
    fn closest(self) -> f64 {
        self.to
    }
}

impl CastImpl<f32> for f64 {
    type Error = LossyCastError<Self, f32>;

    #[inline]
    fn cast_impl(self) -> Result<f32, Self::Error> {
        // Perform the cast
        #[allow(clippy::cast_possible_truncation)]
        let value = self as f32;
        
        // Cast back and compare; this works since f32 -> f64 is lossless and the default behaviors
        // of equality testing are what we want, despite usually being inadvisable for floats. This
        // test handles the NaN case naturally.
        #[allow(clippy::float_cmp)]
        match f64::from(value) == self {
            true => Ok(value),
            false => Err(LossyCastError {
                from: self,
                to: value
            })
        }
    }
}

impl Closest<f32> for LossyCastError<f64, f32> {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn closest(self) -> f32 {
        // Handle the specific cases of finite overflow turning into infinity
        match self.to {
            f32::INFINITY if self.from != f64::INFINITY => f32::MAX,
            f32::NEG_INFINITY if self.from != f64::NEG_INFINITY => f32::MIN,
            _ => self.to
        }
    }
}