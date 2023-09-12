//! Provides for easy importation of the extension traits commonly required to use cove
//!
//! For most uses of cove, the only imports needed are:
//! ```
//! use cove::prelude::*;
//! ```
//!
//! While it is possible to selectively import required objects, that can be needlessly verbose.

pub use crate::casts::{AssumedLossless, Cast, Closest, Lossless, Lossy};