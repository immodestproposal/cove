use cove::{AssumedLossless, Cast, Closest, Lossless, Lossy, Saturated};

use core::num::{
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
};

#[test]
fn nonzero_closest() {
    // Narrowing: NonZero -> NonZero
    assert_eq!(NonZeroU16::new(3).unwrap().cast::<NonZeroU8>().closest().get(), 3u8);
    assert_eq!(NonZeroU16::new(261).unwrap().cast::<NonZeroU8>().closest().get(), 255u8);

    // Narrowing: NonZero -> primitive
    assert_eq!(NonZeroU16::new(3).unwrap().cast::<u8>().closest(), 3u8);
    assert_eq!(NonZeroU16::new(261).unwrap().cast::<u8>().closest(), 255u8);

    // Narrowing: primitive -> NonZero
    assert_eq!(3u16.cast::<NonZeroU8>().closest(), NonZeroU8::new(3).unwrap());
    assert_eq!(261u16.cast::<NonZeroU8>().closest(), NonZeroU8::new(255).unwrap());
    assert_eq!(0u16.cast::<NonZeroU8>().closest(), NonZeroU8::new(1).unwrap());
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