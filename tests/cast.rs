//! These tests cover the base Cast trait for primitives

use cove::prelude::*;

#[test]
#[allow(clippy::cast_possible_truncation, clippy::float_cmp)]
fn narrowing_casts_lossless() {
    // Unsigned to unsigned
    assert_eq!(98usize.cast::<u8>().unwrap(), 98u8);
    assert_eq!(62000u64.cast::<u32>().unwrap(), 62000u32);

    // Unsigned to signed
    assert_eq!(12345u32.cast::<i16>().unwrap(), 12345i16);
    assert_eq!(33usize.cast::<i8>().unwrap(), 33i8);

    // Signed to signed
    assert_eq!((-82i16).cast::<i8>().unwrap(), -82i8);
    assert_eq!(7000i128.cast::<i64>().unwrap(), 7000i64);

    // Signed to unsigned
    assert_eq!(13i16.cast::<u8>().unwrap(), 13u8);
    assert_eq!(8123i64.cast::<u32>().unwrap(), 8123u32);

    // Float to float
    assert_eq!((-3.5f64).cast::<f32>().unwrap(), -3.5f64 as f32);
    assert_eq!(80.0f64.cast::<f32>().unwrap(), 80.0f64 as f32);
}

#[test]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::float_cmp)]
fn narrowing_casts_lossy() {
    // Unsigned to unsigned
    assert_eq!(70000u32.cast::<u16>().unwrap_err().to, 4464u16);
    assert_eq!(401u128.cast::<u8>().unwrap_err().to, 145u8);

    // Unsigned to signed
    assert_eq!(1_000_000_000_000u64.cast::<i32>().unwrap_err().to, 1_000_000_000_000u64 as i32);
    assert_eq!(320usize.cast::<i8>().unwrap_err().to, 320usize as i8);

    // Signed to signed
    assert_eq!((-973i128).cast::<i8>().unwrap_err().to, -973i128 as i8);
    assert_eq!(82010i64.cast::<i16>().unwrap_err().to, 82010i64 as i16);

    // Signed to unsigned
    assert_eq!((-90i32).cast::<u16>().unwrap_err().to, -90i32 as u16);
    assert_eq!(800i16.cast::<u8>().unwrap_err().to, 800i16 as u8);

    // Float to float
    assert_eq!(f64::MIN.cast::<f32>().unwrap_err().to, f64::MIN as f32);
    assert_eq!(f64::MAX.cast::<f32>().unwrap_err().to, f64::MAX as f32);
}