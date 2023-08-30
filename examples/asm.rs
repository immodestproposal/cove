use cove::{AssumedLossless, Cast, Lossless, Lossy};

#[inline(never)]
#[allow(clippy::cast_possible_truncation)]
fn run_u32_to_u8(value: u32) {
    std::hint::black_box(value as u8);
    std::hint::black_box(value.cast::<u8>().lossy());
    std::hint::black_box(value.cast::<u8>().assumed_lossless());
}

#[inline(never)]
#[allow(clippy::cast_lossless)]
fn run_u8_to_u32(value: u8) {
    std::hint::black_box(value as u32);
    std::hint::black_box(value.cast::<u32>().lossy());
    std::hint::black_box(value.cast::<u32>().assumed_lossless());
    std::hint::black_box(value.cast::<u32>().lossless());
}

fn main() {
    run_u32_to_u8(12);
    run_u32_to_u8(300);
    run_u8_to_u32(12);
    run_u8_to_u32(20);
}