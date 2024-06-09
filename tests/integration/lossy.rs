//! These tests cover the `Lossy` trait for primitives to primitives

use cove::prelude::*;

macro_rules! random {
    ($name:ident as $source:ty => $($target:ty),+) => {
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

                // Validate that the cast yields the same value as the `as` keyword for each target type
                $(
                    {
                        let lhs = value.cast::<$target>().lossy();
                        let rhs = value as $target;
                        assert!(lhs == rhs || (lhs.is_nan() && rhs.is_nan()));
                    }
                )*
            }
        }
    };

    ($($name:ident as $source:ty),*) => {
        $(
            random!(
                $name as $source =>
                u8, u16, u32, u64, u128, usize,
                i8, i16, i32, i64, i128, isize,
                f32, f64
            );
        )*
    }
}

random!(
    random_u8    as u8,    random_i8    as i8,
    random_u16   as u16,   random_i16   as i16,
    random_u32   as u32,   random_i32   as i32,
    random_u64   as u64,   random_i64   as i64,
    random_u128  as u128,  random_i128  as i128,
    random_usize as usize, random_isize as isize,
    random_f32   as f32,   random_f64   as f64
);

// Helper trait to identify whether a number is NaN
trait IsNaN {
    fn is_nan(&self) -> bool {
        false
    }
}

impl IsNaN for f32 {
    fn is_nan(&self) -> bool {
        f32::is_nan(*self)
    }
}

impl IsNaN for f64 {
    fn is_nan(&self) -> bool {
        f64::is_nan(*self)
    }
}

macro_rules! mark_is_nan {
    ($($int:ty),*) => {
        $(impl IsNaN for $int {})*
    }
}

mark_is_nan!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);