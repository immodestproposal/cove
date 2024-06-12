//! This module provides implementations of the Bitwise trait

use crate::casts::{AssumedLossless, Bitwise, Cast};
use crate::errors::{LosslessCastError, LossyCastError};

use core::num::{
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
};

// -- NonZeroPrimitive -- //
/// Helper trait for identifying the primitive type associated with a `NonZero*`
trait NonZeroPrimitive {
    type Primitive;
}

macro_rules! nonzero_primitive {
    ($($nonzero:ty as $primitive:ty),*) => {
        $(
            impl NonZeroPrimitive for $nonzero {
                type Primitive = $primitive;
            }
        )*
    }
}

nonzero_primitive!(
    NonZeroU8  as u8,  NonZeroU16  as u16,  NonZeroU32   as u32, 
    NonZeroU64 as u64, NonZeroU128 as u128, NonZeroUsize as usize,
    NonZeroI8  as i8,  NonZeroI16  as i16,  NonZeroI32   as i32, 
    NonZeroI64 as i64, NonZeroI128 as i128, NonZeroIsize as isize
);

// -- Bitwise -- //
macro_rules! bitwise {
    // The actual implementations for primitive -> primitive
    (primitive primitive $from:ty => {$($to:ty),+}) => {
        $(
            impl Bitwise<$to> for Result<$to, LossyCastError<$from, $to>> {
                #[inline]
                fn bitwise(self) -> $to {
                    // Extract the original value from before the cast. The basic idea is that if 
                    // the cast was successful (i.e. lossless), it can be losslessly cast back to 
                    // its original value. If not, the value is available in the error itself.
                    let original = match self {
                        Ok(value) => value.cast::<$from>().assumed_lossless(),
                        Err(error) => error.from
                    };
                    
                    // Convert by bytes
                    <$to>::from_ne_bytes(original.to_ne_bytes())
                }
            }
        
            impl Bitwise<$to> for Result<$to, LosslessCastError<$from, $to>> {
                #[inline]
                fn bitwise(self) -> $to {
                    // Extract the original value from before the cast. The basic idea is that the 
                    // cast had to have been successful (i.e. lossless) and therefore can be 
                    // losslessly cast back to its original value. We can unwrap safely since 
                    // LosslessCastError cannot be instantiated.
                    let original = unsafe {self.unwrap_unchecked()}
                        .cast::<$from>()
                        .assumed_lossless();
                    
                    // Convert by bytes
                    <$to>::from_ne_bytes(original.to_ne_bytes())
                }
            } 
        )*
    };
    
    // The actual implementations for nonzero -> primitive
    (nonzero primitive $from:ty => {$($to:ty),+}) => {
        $(
            impl Bitwise<$to> for Result<$to, LossyCastError<$from, $to>> {
                #[inline]
                fn bitwise(self) -> $to {
                    // Extract the original value from before the cast, but as a primitive. The 
                    // basic idea is that if the cast was successful (i.e. lossless), it can be 
                    // losslessly cast back to its original value (in primitive form). If not, the 
                    // value is available in the error itself.
                    let original_primitive = match self {
                        Ok(value) => value
                            .cast::<<$from as NonZeroPrimitive>::Primitive>()
                            .assumed_lossless(),
                        Err(error) => error.from.get()
                    };
                    
                    // Convert by bytes
                    <$to>::from_ne_bytes(original_primitive.to_ne_bytes())
                }
            }
        
            impl Bitwise<$to> for Result<$to, LosslessCastError<$from, $to>> {
                #[inline]
                fn bitwise(self) -> $to {
                    // Extract the original value from before the cast, but as a primitive. The 
                    // basic idea is that the cast had to have been successful (i.e. lossless) and 
                    // therefore can be losslessly cast back to its original value (in primitive 
                    // form). We can unwrap safely since LosslessCastError cannot be instantiated.
                    let original_primitive = unsafe {self.unwrap_unchecked()}
                        .cast::<<$from as NonZeroPrimitive>::Primitive>()
                        .assumed_lossless();
                    
                    // Convert by bytes
                    <$to>::from_ne_bytes(original_primitive.to_ne_bytes())
                }
            } 
        )*
    };
    
    // The actual implementations for nonzero -> nonzero
    (nonzero nonzero $from:ty => {$($to:ty),+}) => {
        $(
            impl Bitwise<$to> for Result<$to, LossyCastError<$from, $to>> {
                #[inline]
                fn bitwise(self) -> $to {
                    // Extract the original value from before the cast, but as a primitive. The 
                    // basic idea is that if the cast was successful (i.e. lossless), it can be 
                    // losslessly cast back to its original value (in primitive form). If not, the 
                    // value is available in the error itself.
                    let original_primitive = match self {
                        Ok(value) => value
                            .cast::<<$from as NonZeroPrimitive>::Primitive>()
                            .assumed_lossless(),
                        Err(error) => error.from.get()
                    };
                    
                    // Convert by bytes; it is safe to use new_unchecked because the original was
                    // a NonZero* and thus couldn't have been zero-valued.
                    let bytes = original_primitive.to_ne_bytes();
                    let primitive = <$to as NonZeroPrimitive>::Primitive::from_ne_bytes(bytes);
                    unsafe {<$to>::new_unchecked(primitive)}
                }
            }
        
            impl Bitwise<$to> for Result<$to, LosslessCastError<$from, $to>> {
                #[inline]
                fn bitwise(self) -> $to {
                    // Extract the original value from before the cast, but as a primitive. The 
                    // basic idea is that the cast had to have been successful (i.e. lossless) and 
                    // therefore can be losslessly cast back to its original value (in primitive 
                    // form). We can unwrap safely since LosslessCastError cannot be instantiated.
                    let original_primitive = unsafe {self.unwrap_unchecked()}
                        .cast::<<$from as NonZeroPrimitive>::Primitive>()
                        .assumed_lossless();
                    
                    // Convert by bytes; it is safe to use new_unchecked because the original was
                    // a NonZero* and thus couldn't have been zero-valued.
                    let bytes = original_primitive.to_ne_bytes();
                    let primitive = <$to as NonZeroPrimitive>::Primitive::from_ne_bytes(bytes);
                    unsafe {<$to>::new_unchecked(primitive)}
                }
            } 
        )*
    };
    
    // Iteratively generate implementations for primitive -> primitive
    (primitive primitive $first:ty, $($from:ty),+ => $to:tt) => {
        bitwise!(primitive primitive $first => $to);
        $(bitwise!(primitive primitive $from => $to);)*
    };
    
    // Iteratively generate implementations for nonzero -> primitive
    (nonzero primitive $first:ty, $($from:ty),+ => $to:tt) => {
        bitwise!(nonzero primitive $first => $to);
        $(bitwise!(nonzero primitive $from => $to);)*
    };
    
    // Iteratively generate implementations for nonzero -> nonzero
    (nonzero nonzero $first:ty, $($from:ty),+ => $to:tt) => {
        bitwise!(nonzero nonzero $first => $to);
        $(bitwise!(nonzero nonzero $from => $to);)*
    };
    
    // Generate implementations in n-squared fashion for primitive -> primitive
    (primitive $($ty:ty),*) => {
        bitwise!(primitive primitive $($ty),* => {$($ty),*});
    };
    
    // Generate implementations in n-squared fashion for nonzero -> nonzero
    (nonzero $($ty:ty),*) => {
        bitwise!(nonzero nonzero $($ty),* => {$($ty),*});
    }
}

// -- Platform-independent -- //
bitwise!(primitive u8, i8);
bitwise!(primitive u16, i16);
bitwise!(primitive u32, i32, f32);
bitwise!(primitive u64, i64, f64);
bitwise!(primitive u128, i128);

bitwise!(nonzero primitive NonZeroU8, NonZeroI8 => {u8, i8});
bitwise!(nonzero primitive NonZeroU16, NonZeroI16 => {u16, i16});
bitwise!(nonzero primitive NonZeroU32, NonZeroI32 => {u32, i32, f32});
bitwise!(nonzero primitive NonZeroU64, NonZeroI64 => {u64, i64, f64});
bitwise!(nonzero primitive NonZeroU128, NonZeroI128 => {u128, i128});

bitwise!(nonzero NonZeroU8, NonZeroI8);
bitwise!(nonzero NonZeroU16, NonZeroI16);
bitwise!(nonzero NonZeroU32, NonZeroI32);
bitwise!(nonzero NonZeroU64, NonZeroI64);
bitwise!(nonzero NonZeroU128, NonZeroI128);
bitwise!(nonzero NonZeroUsize, NonZeroIsize);

// -- Platform-dependent -- //
#[cfg(target_pointer_width = "16")]
#[allow(clippy::wildcard_imports)]
mod platform_dependent {
    use super::*;

    bitwise!(primitive primitive u16, i16 => {usize, isize});
    bitwise!(primitive primitive usize, isize => {u16, i16});

    bitwise!(nonzero primitive NonZeroUsize, NonZeroIsize => {u16, i16});

    bitwise!(nonzero nonzero NonZeroU16, NonZeroI16 => {NonZeroUsize, NonZeroIsize});
    bitwise!(nonzero nonzero NonZeroUsize, NonZeroIsize => {NonZeroU16, NonZeroI16});
}

#[cfg(target_pointer_width = "32")]
#[allow(clippy::wildcard_imports)]
mod platform_dependent {
    use super::*;

    bitwise!(primitive primitive u32, i32 => {usize, isize});
    bitwise!(primitive primitive usize, isize => {u32, i32});

    bitwise!(nonzero primitive NonZeroUsize, NonZeroIsize => {u32, i32});

    bitwise!(nonzero nonzero NonZeroU32, NonZeroI32 => {NonZeroUsize, NonZeroIsize});
    bitwise!(nonzero nonzero NonZeroUsize, NonZeroIsize => {NonZeroU32, NonZeroI32});
}

#[cfg(target_pointer_width = "64")]
#[allow(clippy::wildcard_imports)]
mod platform_dependent {
    use super::*;

    bitwise!(primitive primitive u64, i64 => {usize, isize});
    bitwise!(primitive primitive usize, isize => {u64, i64});
    
    bitwise!(nonzero primitive NonZeroUsize, NonZeroIsize => {u64, i64});
    
    bitwise!(nonzero nonzero NonZeroU64, NonZeroI64 => {NonZeroUsize, NonZeroIsize});
    bitwise!(nonzero nonzero NonZeroUsize, NonZeroIsize => {NonZeroU64, NonZeroI64});
}

#[cfg(target_pointer_width = "128")]
#[allow(clippy::wildcard_imports)]
mod platform_dependent {
    use super::*;

    bitwise!(primitive primitive u128, i128 => {usize, isize});
    bitwise!(primitive primitive usize, isize => {u128, i128});

    bitwise!(nonzero primitive NonZeroUsize, NonZeroIsize => {u128, i128});

    bitwise!(nonzero nonzero NonZeroU128, NonZeroI128 => {NonZeroUsize, NonZeroIsize});
    bitwise!(nonzero nonzero NonZeroUsize, NonZeroIsize => {NonZeroU128, NonZeroI128});
}