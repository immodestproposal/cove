//! These tests cover the `assumed_lossless` trait; they only work for std since they rely on
//! `std::panic::catch_unwind`. In practice, however, that does a pretty good job of testing the
//! core case, too.

use crate::util::IsNaN;
use cove::prelude::*;
use cove::bounds::CastTo;
use core::fmt::Debug;

#[cfg(feature = "std")]
#[test]
fn from_nan() {
    validate::<i8>(f32::NAN);
}

macro_rules! random {
    ($name:ident as $source:ty => $($target:ty),+) => {
        #[cfg(feature = "std")]
        #[test]
        #[allow(clippy::float_cmp)]
        fn $name () {
            // Initialization: allocate space for the test buffers and determine the initial seed
            let mut random = crate::util::random_seed();

            // Perform the tests
            for _ in 0 .. crate::util::settings::SLOW_ITERATIONS {
                // Generate the test value and next random number
                let (buffer, next_random) = crate::util::random_bytes(random);
                let value = <$source>::from_ne_bytes(buffer);
                random = next_random;

                // Perform validation
                $(validate::<$target>(value);)*
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

// Helper method to perform cast validation; this implementation assumes Cast itself isn't buggy, 
// but that is tested elsewhere.
#[cfg(feature = "std")]
fn validate<TO: Copy + Debug + PartialEq + IsNaN>(from: impl Copy + CastTo<TO> + IsNaN) {
    // Perform the cast
    let casted = from.cast::<TO>();
    match casted {
        // Cast was lossless; assert that the value matches assumed_lossless()
        Ok(value) => {
            assert!(value == casted.assumed_lossless() || (value.is_nan() && from.is_nan()));
        }

        // Cast was lossless
        Err(_error) => {
            // Use assumed_lossless() and validate that there is a panic
            #[cfg(debug_assertions)] {
                assert!(
                    std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| casted.assumed_lossless())
                    ).is_err()
                );
            }

            // Use assumed_lossless() and validate that there is no panic
            #[cfg(not(debug_assertions))] {
                let _ = casted.assumed_lossless();
            }
        }
    }
}