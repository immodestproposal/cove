//! These tests cover the `Closest` trait for primitives to primitives. This is a hard one to test 
//! since it is primarily concerned with not allowing compilation, which isn't something we can
//! easily test in integration tests. We will consequently use doctests to cover the compilation
//! failure cases, and check the success cases here.

use crate::util::IsNaN;
use cove::prelude::*;

macro_rules! success {
    ($name:ident as $source:ty => $($target:ty),*) => {
        #[test]
        #[allow(clippy::float_cmp)]
        fn $name () {
            // Initialization: allocate space for the test buffers and determine the initial seed
            let mut random = crate::util::random_seed();

            // Perform the tests
            for _ in 0 .. crate::util::settings::FAST_ITERATIONS {
                // Generate the test value and next random number
                let (buffer, next_random) = crate::util::random_bytes(random);
                let value = <$source>::from_ne_bytes(buffer);
                random = next_random;

                // Validate that the cast is lossless and yields the same value as the `as` keyword 
                // for each target type
                $(
                    {
                        let casted = value.cast::<$target>();
                        assert!(casted.is_ok());
                        
                        let lhs = casted.lossless();
                        let rhs = value as $target;
                        assert!(lhs == rhs || (lhs.is_nan() && rhs.is_nan()));
                    }
                )*
            }
        }
    }
}

success!(random_u8   as u8   => u8, u16, u32, u64, u128, i16, i32, i64, i128, f32, f64);
success!(random_u16  as u16  =>     u16, u32, u64, u128,      i32, i64, i128, f32, f64);
success!(random_u32  as u32  =>          u32, u64, u128,           i64, i128,      f64);
success!(random_u64  as u64  =>               u64, u128,                i128          );
success!(random_u128 as u128 =>                    u128                               );

success!(random_i8   as i8   => i8, i16, i32, i64, i128, f32, f64);
success!(random_i16  as i16  =>     i16, i32, i64, i128, f32, f64);
success!(random_i32  as i32  =>          i32, i64, i128,      f64);
success!(random_i64  as i64  =>               i64, i128          );
success!(random_i128 as i128 =>                    i128          );

success!(random_usize as usize  => usize);
success!(random_isize as isize  => isize);

#[cfg(target_pointer_width = "16")]
mod platform_dependent {
    use super::*;

    success!(random_usize as usize  => u16, u32, u64, u128, i32, i64, i128, f32, f64);
    success!(random_isize as isize  => i16, i32, i64, i128, f32, f64);

    success!(random_usize_from_u8  as u8  => usize);
    success!(random_usize_from_u16 as u16 => usize);

    success!(random_isize_from_u8  as u8  => isize);

    success!(random_isize_from_i8  as i8  => isize);
    success!(random_isize_from_i16 as i16 => isize);
}

#[cfg(target_pointer_width = "32")]
mod platform_dependent {
    use super::*;

    success!(random_usize as usize  => u32, u64, u128, i64, i128, f64);
    success!(random_isize as isize  => i32, i64, i128, f64);

    success!(random_usize_from_u8  as u8  => usize);
    success!(random_usize_from_u16 as u16 => usize);
    success!(random_usize_from_u32 as u32 => usize);

    success!(random_isize_from_u8  as u8  => isize);
    success!(random_isize_from_u16 as u16 => isize);

    success!(random_isize_from_i8  as i8  => isize);
    success!(random_isize_from_i16 as i16 => isize);
    success!(random_isize_from_i32 as i32 => isize);
}

#[cfg(target_pointer_width = "64")]
mod platform_dependent {
    use super::*;

    success!(random_usize as usize  => u64, u128, i128);
    success!(random_isize as isize  => i64, i128);
    
    success!(random_usize_from_u8  as u8  => usize);
    success!(random_usize_from_u16 as u16 => usize);
    success!(random_usize_from_u32 as u32 => usize);
    success!(random_usize_from_u64 as u64 => usize);

    success!(random_isize_from_u8  as u8  => isize);
    success!(random_isize_from_u16 as u16 => isize);
    success!(random_isize_from_u32 as u32 => isize);

    success!(random_isize_from_i8  as i8  => isize);
    success!(random_isize_from_i16 as i16 => isize);
    success!(random_isize_from_i32 as i32 => isize);
    success!(random_isize_from_i64 as i64 => isize);
}

#[cfg(target_pointer_width = "128")]
mod platform_dependent {
    use super::*;

    success!(random_usize as usize  => u128);
    success!(random_isize as isize  => i128);

    success!(random_usize_from_u8   as u8   => usize);
    success!(random_usize_from_u16  as u16  => usize);
    success!(random_usize_from_u32  as u32  => usize);
    success!(random_usize_from_u64  as u64  => usize);
    success!(random_usize_from_u128 as u128 => usize);

    success!(random_isize_from_u8   as u8   => isize);
    success!(random_isize_from_u16  as u16  => isize);
    success!(random_isize_from_u32  as u32  => isize);
    success!(random_isize_from_u64  as u64  => isize);

    success!(random_isize_from_i8   as i8   => isize);
    success!(random_isize_from_i16  as i16  => isize);
    success!(random_isize_from_i32  as i32  => isize);
    success!(random_isize_from_i64  as i64  => isize);
    success!(random_isize_from_i128 as i128 => isize);
}