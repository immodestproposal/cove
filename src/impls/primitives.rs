//! This module provides built-in implementation of the casting traits for primitive types

#![allow(clippy::wildcard_imports)]

use crate::casts::{Cast, Closest};
use crate::errors::LossyCastError;
use crate::base::CastImpl;
use super::LosslessCast;
use core::num::FpCategory;

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
                    // If the int's type can entirely fit within the float's mantissa digits,
                    // we can safely just cast with values known at compile time.
                    if <$from>::BITS <= <$to>::MANTISSA_DIGITS {
                        return Ok(self as $to)
                    }

                    // Compute the absolute value; we use abs_diff() because it is defined on both
                    // signed and unsigned integers and works for minimum signed numbers.
                    let value = self.abs_diff(0);

                    // If the int's value fits within the float's mantissa digits, we can safely
                    // just cast. The formula for the max int should be 2^MANTISSA_DIGITS - 1, but
                    // we use 2^MANTISSA_DIGITS because otherwise we would have to handle the
                    // overflowing case specially. Since 2^MANTISSA_DIGITS is an in-range power of 2
                    // it should work anyway by the later check.
                    if value as $from <= (2 as $from).saturating_pow(<$to>::MANTISSA_DIGITS) {
                        // This check involves casting back to $from; if $from was unsigned this is
                        // a no-op -- that is, the value is unchanged. If $from was signed this is
                        // also a no-op unless self was specifically $from::MIN, in which case it
                        // restores the value to $from::MIN. Since that will be negative this check
                        // will be trivially true in that case; this is acceptable because the
                        // absolute value of $from::MIN is always an in-range power of two, so it
                        // would pass the next check anyway.
                        return Ok(self as $to)
                    }

                    // If the int is a power of 2 and in range we can safely just cast. For the
                    // range test it is sufficient to just cast $to::MAX to $from because either it
                    // is larger than $from::MAX in which case the bounds check is unnecessary and
                    // harmless, or else it is smaller than $from::MAX and thus will round toward
                    // zero, which is what we want. Note also that only u128::MAX is in danger of
                    // failing the self <= MAX_FLOAT_AS_INT check, as the only power of two which
                    // exceeds any floating point max.
                    if value.is_power_of_two() && self <= <$to>::MAX as $from {
                        return Ok(self as $to)
                    }

                    // All the lossless cases have been covered, so this cast is lossy
                    Err(LossyCastError {
                        from: self,
                        to: self as $to
                    })
                }
            }

            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // For int-to-float the raw cast is the closest
                    self.to
                }
            }
        )*
    };

    (float_to_int $from:ty => $($to:ty),+) => {
        $(
            impl CastImpl<$to> for $from {
                type Error = LossyCastError<Self, $to>;

                #[inline]
                fn cast_impl(self) -> Result<$to, Self::Error> {
                    // TODO: the logic is:
                    // Must be finite
                    // Must not contain a fractional part
                    // Must be less than or equal to $to::MAX
                    // If all these conditions are met, the cast is lossless; otherwise, lossy
                    // NOTE: consult crate libm, fns trunc, truncf for implementation ideas
                    
                    // Only finite floats can be integers, not NaNs and infinities
                    if !self.is_finite() {
                        return Err(LossyCastError {
                            from: self,
                            to: self as $to
                        })
                    }
                    
                    // Extract the fractional part
                    let has_fract = {
                        // If std is enabled, just use fract() to extract the fractional part
                        #[cfg(feature = "std")] {
                            self.fract().classify() != FpCategory::Zero
                        }
                        
                        // No std, so we will need to manually check for a fractional part
                        #[cfg(not(feature = "std"))] {
                            // Identifies the number of bits in the mantissa; 23 for f32, 52 for f64
                            const MANTISSA_BITS = $from::MANTISSA_DIGITS - 1;
                            
                            // Identifies the bitmask for the exponent (after shifting); this is
                            // 0xFF (8 bits) for f32 and 0x7FF (11 bits) for f64
                            const MAX_EXP_MASK = ($from::MAX_EXP << 1) - 1;
                            
                            let value = self.to_bits();
                            let exponent = (value >> MANTISSA_BITS & MAX_EXP_MASK);
                        }
                    };
                    
                    // 

                    // Check for a fractional part; if std is enabled, use fract()
                    #[cfg(feature = "std")] {
                        if self.fract().classify() != FpCategory::Zero {
                            // There is a fraction, so this cannot cast cleanly to an int
                            return Err(LossyCastERror {
                                from: self,
                                to: self as $to
                            })
                        }
                    }

                    // No std, so we will need to manually check for a fractional part
                    #[cfg(not(feature = "std"))] {
                        // Determine which is the least-significant 1 bit in the mantissa
                        let value = self.to_bits();
                        let trailing_zeros = value.trailing_zeros();
                        if trailing_zeros < $from::MANTISSA_DIGITS - 1
                        && trailing_zeros
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

    (int_to_float $first:ty, $($from:ty),+ => $to:ty) => {
        cast!(int_to_float $first => $to);
        $(cast!(int_to_float $from => $to));*;
    };

    (float_to_int $first:ty, $($from:ty),+ => $to:ty) => {
        cast!(float_to_int $first => $to);
        $(cast!(float_to_int $from => $to));*;
    };

    (lossless $from:ty => $($to:ty),+) => {
        $(impl LosslessCast for Result<$to, LossyCastError<$from, $to>> {})*
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
cast!(float_to_int f32 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast!(float_to_int f64 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

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
}

#[cfg(target_pointer_width = "32")]
mod platform_dependent {
    use super::*;

    cast!(lossless usize => u32, u64, u128, i64, i128, f64);
    cast!(lossless isize => i32, i64, i128, f64);

    cast!(lossless u8, u16, u32 => usize);
    cast!(lossless u8, u16, i8, i16, i32 => isize);
}

#[cfg(target_pointer_width = "64")]
mod platform_dependent {
    use super::*;

    cast!(lossless usize => u64, u128, i128);
    cast!(lossless isize => i64, i128);

    cast!(lossless u8, u16, u32, u64 => usize);
    cast!(lossless u8, u16, u32, i8, i16, i32, i64 => isize);
}

// -- Manual Implementations -- //
impl Closest<f64> for f32 {
    #[inline]
    fn closest(self) -> f64 {
        self.into()
    }
}

impl Closest<f32> for f64 {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn closest(self) -> f32 {
        self as f32
    }
}
