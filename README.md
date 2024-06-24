# cove
**C**asts **O**f **V**arying **E**legance

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]
![Maintenace][maintenance-badge]

[maintenance-badge]: https://img.shields.io/badge/status-actively%2D-developed-blue
[crates-badge]: https://img.shields.io/crates/v/cove.svg
[crates-url]: https://crates.io/crates/cove
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/immodestproposal/cove/blob/master/LICENSE
[actions-badge]: https://github.com/immodestproposal/cove/actions/workflows/CI.yml/badge.svg
[actions-url]: https://github.com/immodestproposal/cove/actions?query=workflow%3ACI+branch%3Amaster

Provides a collection of extension traits to improve the safety and maintainability of numerical casts.
Cove's primary goals are:
* **clarity**: the programmer's intention for a cast is clear from the name
* **correctness**: suspicious casts via `as` can be reduced or eliminated altogether
* **performance**: in release builds, cove's casts generally compile down to the same
assembly as manual implementations
* **independence**: no required dependencies and the only optional dependency is `std`

## Quick Usage
```rust
use cove::prelude::*;
use core::num::{NonZeroI8, NonZeroI32, NonZeroI64, NonZeroU16, NonZeroU64};

// Check whether a cast is lossy at runtime
assert_eq!(8i16.cast::<u8>()?, 8u8);
assert!(0u128.cast::<NonZeroI8>().is_err());

// Of course, turbofish disambiguation is unnecessary if the compiler can deduce the type:
fn foo(x: u8) -> u8 {x}
assert_eq!(foo(2i16.cast()?), 2u8);

// If the cast ends up being lossy, you can usually still use the lossy value if you like:
assert_eq!(9.2f64.cast::<usize>().unwrap_err().to, 9usize);

// ...or more concisely:
assert_eq!(9.2f64.cast::<usize>().lossy(), 9usize);

// Perhaps you don't mind if the cast is lossy, but you'd like to get as close as possible:
assert_eq!(300u32.cast::<u8>().closest(), 255u8);
assert_eq!((-7isize).cast::<u16>().closest(), 0u16);
assert_eq!(-4.6f32.cast::<i16>().closest(), -5i16);
assert_eq!(-0.0f64.cast::<NonZeroI32>().closest(), NonZeroI32::new(-1).unwrap());

// If you are supremely confident a cast is lossless you can always use unwrap_unchecked:
assert_eq!(unsafe {90u32.cast::<u8>().unwrap_unchecked()}, 90);

// ...but if the unsafeness makes you uncomfortable you might prefer cove's assumed_lossless,
// which will use a debug_assert instead of unsafe (and just risk lossiness in release builds):
assert_eq!(90u32.cast::<u8>().assumed_lossless(), 90);

// If desired, you can instead preserve bits (rather than mathematical value) across a cast:
assert_eq!(NonZeroI64::new(-1).unwrap().cast::<u64>().bitwise(), u64::MAX);
assert_eq!(10f32.cast::<u32>().bitwise(), 1_092_616_192u32);

// If the types guarantee a lossless cast, you can of course always use `From`/`Into`:
assert_eq!(NonZeroU64::from(NonZeroU16::new(12).unwrap()), NonZeroU64::new(12).unwrap());

// ...but what if those traits aren't provided because the cast could be lossy on some other
// platform? If you don't mind losing portability, try out cove's `lossless`. This will only
// compile on platforms where usize is at least 64 bits:
assert_eq!(31u64.cast::<usize>().lossless(), 31usize);
```

## Features
Cove supports one feature, `std`, which is included in the default features. Enabling this
feature (or rather, failing to disable it) enables support for the Rust standard library.
If this is disabled, cove depends only on the Rust core library.
Enabling `std` causes cove's error types to implement [`std::error::Error`]; otherwise they do
not, as at the time of writing [`core::error::Error`] is unstable. In addition, some cast
implementations are controlled by this feature, as the rust standard library allows for
optimizations via intrinsics not available in stable [`core`].

## Links
* Read about how to use cove's [`casts`](https://docs.rs/cove/latest/cove/casts/index.html)
* Read about generic [`bounds`](https://docs.rs/cove/latest/cove/bounds/index.html) for cove's casts
* Read about [`extending`](https://docs.rs/cove/latest/cove/base/index.html) cove's casts to new types
* Read about the [`motivation`](https://docs.rs/cove/latest/cove/docs/motivation/index.html) behind cove
* Read about [`performance`](https://docs.rs/cove/latest/cove/docs/performance/index.html) considerations when using cove
* Read about [`testing`](https://docs.rs/cove/latest/cove/docs/testing/index.html) considerations with cove