//! These tests cover the `Lossy` trait for primitives to primitives

use crate::util::IsNaN;
use cove::prelude::*;

macro_rules! random {
    ($name:ident as $source:ty => $($target:ty),+) => {
        #[test]
        #[allow(
            clippy::float_cmp, clippy::useless_transmute, 
            clippy::transmute_int_to_float, clippy::transmute_float_to_int
        )]
        fn $name () {
            // Initialization: allocate space for the test buffers and determine the initial seed
            let mut random = crate::util::random_seed();

            // Perform the tests
            for _ in 0 .. crate::util::settings::FAST_ITERATIONS {
                // Generate the test value and next random number
                let (buffer, next_random) = crate::util::random_bytes(random);
                let value = <$source>::from_ne_bytes(buffer);
                random = next_random;

                // Validate that the cast yields the same value as transmute for each type
                $(
                    {
                        let lhs = value.cast::<$target>().bitwise();
                        let rhs: $target = unsafe {core::mem::transmute(value)};
                        assert!(lhs == rhs || (lhs.is_nan() && rhs.is_nan()));
                    }
                )*
            }
        }
    };
}

random!(random_u8   as u8   => u8,   i8);
random!(random_u16  as u16  => u16,  i16);
random!(random_u32  as u32  => u32,  i32, f32);
random!(random_u64  as u64  => u64,  i64, f64);
random!(random_u128 as u128 => u128, i128);
random!(random_i8   as i8   => u8,   i8);
random!(random_i16  as i16  => u16,  i16);
random!(random_i32  as i32  => u32,  i32, f32);
random!(random_i64  as i64  => u64,  i64, f64);
random!(random_i128 as i128 => u128, i128);
random!(random_f32  as f32  => u32,  i32, f32);
random!(random_f64  as f64  => u64,  i64, f64);
random!(random_usize as usize => usize, isize);
random!(random_isize as isize => usize, isize);

#[cfg(target_pointer_width = "16")]
mod platform_dependent {
    use super::*;

    random!(random_usize_16 as usize => u16, i16);
    random!(random_isize_16 as isize => u16, i16);

    random!(random_u16_to_size_16 as u16 => usize, isize);
    random!(random_i16_to_size_16 as i16 => usize, isize);
}

#[cfg(target_pointer_width = "32")]
mod platform_dependent {
    use super::*;

    random!(random_usize_32 as usize => u32, i32, f32);
    random!(random_isize_32 as isize => u32, i32, f32);

    random!(random_u32_to_size_32 as u32 => usize, isize);
    random!(random_i32_to_size_32 as i32 => usize, isize);
    random!(random_f32_to_size_32 as f32 => usize, isize);
}

#[cfg(target_pointer_width = "64")]
mod platform_dependent {
    use super::*;
    
    random!(random_usize_64 as usize => u64, i64, f64);
    random!(random_isize_64 as isize => u64, i64, f64);
    
    random!(random_u64_to_size_64 as u64 => usize, isize);
    random!(random_i64_to_size_64 as i64 => usize, isize);
    random!(random_f64_to_size_64 as f64 => usize, isize);
}

#[cfg(target_pointer_width = "128")]
mod platform_dependent {
    use super::*;

    random!(random_usize_128 as usize => u128, i128);
    random!(random_isize_128 as isize => u128, i128);

    random!(random_u128_to_size_128 as u128 => usize, isize);
    random!(random_i128_to_size_128 as i128 => usize, isize);
}