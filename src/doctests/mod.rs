//! This module exists to run doctests as integration tests, entirely to leverage the fact that
//! doctests can check for compilation failure while normal integration tests cannot. It has no
//! other bearing on Cove's interface or implementation.
mod bitwise;
mod lossless;