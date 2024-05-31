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
                    // If the int's type can entirely fit within the float's mantissa digits,
                    // we can safely just cast with values known at compile time.
                    if <$from>::BITS <= (<$to>::MANTISSA_DIGITS) {
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
                    
                    println!("[MATT] I-to-F: [Self: {self}][Value: {value}][MaxMantissa: {}", (2 as $from).saturating_pow(<$to>::MANTISSA_DIGITS));

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

    (float_to_int $from:ty as $int:ty => $($to:ty),+) => {
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
                    // range; for this it is sufficient to compare with the target type's MIN and
                    // MAX casted as the float. The only MIN/MAX that doesn't convert to floating
                    // point losslessly is u128::MAX as f32, and this becomes positive infinity
                    // which still makes our check correct.
                    match is_int && self >= <$to>::MIN as $from && self <= <$to>::MAX as $from {
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
cast!(float_to_int f32 as u32 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
cast!(float_to_int f64 as u64 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

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
impl CastImpl<f64> for f32 {
    type Error = LossyCastError<Self, f64>;
    
    #[inline]
    fn cast_impl(self) -> Result<f64, Self::Error> {
        Ok(f64::from(self))
    }    
}

impl Closest<f64> for f32 {
    #[inline]
    fn closest(self) -> f64 {
        self.into()
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
        // of equality testing are what we want, despite usually being inadvisable for floats.
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

impl Closest<f32> for f64 {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn closest(self) -> f32 {
        self as f32
    }
}