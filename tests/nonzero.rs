use cove::prelude::*;

use core::num::{
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
};

#[test]
fn nonzero_assumed_lossless() {
    // Narrowing, Sign Change: NonZero -> NonZero
    assert_eq!(NonZeroI32::new(7).unwrap().cast::<NonZeroU16>().assumed_lossless().get(), 7u16);
}

#[test]
fn nonzero_estimated() {
    // Narrowing: NonZero -> NonZero
    assert_eq!(NonZeroU16::new(3).unwrap().cast::<NonZeroU8>().estimated().get(), 3u8);
    assert_eq!(NonZeroU16::new(261).unwrap().cast::<NonZeroU8>().estimated().get(), 255u8);

    // Narrowing: NonZero -> primitive
    assert_eq!(NonZeroU16::new(3).unwrap().cast::<u8>().estimated(), 3u8);
    assert_eq!(NonZeroU16::new(261).unwrap().cast::<u8>().estimated(), 255u8);

    // Narrowing: primitive -> NonZero
    assert_eq!(3u16.cast::<NonZeroU8>().estimated(), NonZeroU8::new(3).unwrap());
    assert_eq!(261u16.cast::<NonZeroU8>().estimated(), NonZeroU8::new(255).unwrap());
    assert_eq!(0u16.cast::<NonZeroU8>().estimated(), NonZeroU8::new(1).unwrap());

    // Floating point
    assert_eq!(8.0f32.cast::<NonZeroI32>().estimated(), NonZeroI32::new(8).unwrap());
}

#[test]
fn nonzero_lossy() {
    // Narrowing: NonZero -> NonZero
    assert_eq!(NonZeroU16::new(3).unwrap().cast::<NonZeroU8>().lossy().get(), 3u8);
    assert_eq!(NonZeroU16::new(261).unwrap().cast::<NonZeroU8>().lossy().get(), 5u8);

    // Narrowing: NonZero -> primitive
    assert_eq!(NonZeroU16::new(3).unwrap().cast::<u8>().lossy(), 3u8);
    assert_eq!(NonZeroU16::new(261).unwrap().cast::<u8>().lossy(), 5u8);
}

#[test]
fn nonzero_saturated() {
    // Narrowing: NonZero -> NonZero
    assert_eq!(NonZeroU16::new(3).unwrap().cast::<NonZeroU8>().saturated().get(), 3u8);
    assert_eq!(NonZeroU16::new(261).unwrap().cast::<NonZeroU8>().saturated().get(), 255u8);

    // Narrowing: NonZero -> primitive
    assert_eq!(NonZeroU16::new(3).unwrap().cast::<u8>().saturated(), 3u8);
    assert_eq!(NonZeroU16::new(261).unwrap().cast::<u8>().saturated(), 255u8);
}