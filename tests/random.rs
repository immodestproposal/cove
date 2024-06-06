mod util;

use cove::bounds::CastTo;
use cove::prelude::*;
use core::fmt::{Display, Write};
use util::FixedString;

// Helper macro for checking a cast
macro_rules! check_cast {
    ($from_buffer:expr, $to_buffer:expr, $value:expr; $from:ty => $($to:ty),*) => {
        $(check_cast::<$from, $to>($value, $from_buffer, $to_buffer);)*
    };

    ($from_buffer:expr, $to_buffer:expr, $value:expr; $from:ty => all primitives) => {
        check_cast!(
            $from_buffer, $to_buffer, $value;
            $from => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
        );
    }
}

#[test]

fn random_f32() {
    random(|value| {
        // Build the f32 from the transmuted u32
        (f32::from_bits(value), util::random_next(value))
    });
}

#[test]

fn random_f64() {
    random(|high| {
        // Build the f64 from a transmuted u64 created from high and low u32s
        let low = util::random_next(high);
        (f64::from_bits((u64::from(high) << 32) | u64::from(low)), util::random_next(low))
    });
}

#[test]

fn random_u64() {
    random(|high| {
        // Build the u64 from high and low u32s
        let low = util::random_next(high);
        ((u64::from(high) << 32) | u64::from(low), util::random_next(low))
    });
}

fn random<
    T: Copy + Display +
    CastTo<i8>    + CastTo<u8>    +
    CastTo<i16>   + CastTo<u16>   +
    CastTo<i32>   + CastTo<u32>   +
    CastTo<i64>   + CastTo<u64>   +
    CastTo<i128>  + CastTo<u128>  +
    CastTo<isize> + CastTo<usize> +
    CastTo<f32>   + CastTo<f64>
>(callback: impl Fn(u32) -> (T, u32)) {
    // Initialization: allocate space for the test buffers and determine the initial seed
    let mut from_buffer = TestString::new();
    let mut to_buffer = TestString::new();
    let mut random = util::random_seed();

    // Perform the tests
    for _ in 0 .. ITERATIONS {
        // Generate the test value and the next random number via the callback
        let (value, next_random) = callback(random);
        random = next_random;

        // Check casting the test value to each primitive type
        check_cast!(
            &mut from_buffer, &mut to_buffer, value;
            T => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
        );
    }
}

const ITERATIONS: usize = 1000;

/// Convenience alias for strings used in the tests
type TestString = FixedString<5192>;

/// Performs and checks the actual cast
fn check_cast<FROM: Copy + Display + CastTo<TO>, TO: Copy + Display>
(from: FROM, from_buffer: &mut TestString, to_buffer: &mut TestString) {
    let result = from.cast::<TO>();
    let to = result.lossy();

    // Determines whether two numbers are equal through a bit of an unorthodox method:
    // comparing their formatted view. This allows us to not depend on casting, which is after all
    // what we are testing. Start by formatting the values into the buffers.
    from_buffer.clear();
    write!(from_buffer, "{from:.1024}").unwrap();

    to_buffer.clear();
    write!(to_buffer, "{to:.1024}").unwrap();

    // Normalize the strings and compare
    let from_text = normalize(from_buffer.as_str());
    let to_text = normalize(to_buffer.as_str());
    let are_equal = from_text == to_text;

    #[allow(clippy::match_bool)]
    match result.is_ok() {
        true if are_equal => {},
        true => panic!(
            "Called lossy cast lossless [{from_text}_{} -> {to_text}_{}]",
            core::any::type_name::<FROM>(),
            core::any::type_name::<TO>()
        ),
        false if are_equal => panic!(
            "Called lossless cast lossy [{from_text}_{} -> {to_text}_{}]",
            core::any::type_name::<FROM>(),
            core::any::type_name::<TO>()
        ),
        false => {}
    }
}

/// "Normalize" the string representation of a number; this means the following:
/// * -0 is reduced to just 0
/// * Trailing zeroes after a decimal point are removed; if that is all the digits after the
/// decimal point, the decimal point is also removed.
fn normalize(value: &str) -> &str {
    // If there a decimal point, trim trailing zeros and possibly the decimal point as well
    #[allow(clippy::match_bool)]
    let value = match value.find('.').is_some() {
        true => value.trim_end_matches('0').trim_end_matches('.'),
        false => value
    };

    // Special case fix-ups
    match value {
        "-0" => "0",
        _ => value
    }
}

