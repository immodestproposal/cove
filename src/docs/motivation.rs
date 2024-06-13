
//! ## Motivation
//! Given the existence of [`From`]/[`Into`]/[`TryFrom`]/[`TryInto`] and the `as` keyword, it is
//! natural to ask what value additional numeric casting functionality could provide. The
//! motivation is simple: the existing mechanisms, while perfectly serviceable, are
//! general-purpose tools which do not align precisely to the use cases for numeric casts. This
//! creates an opportunity for improvements; though each improvement is minor, in codebases rife
//! with casts they can collectively have an outsized effect.
//!
//! ### [`From`]/[`Into`]
//! The [`From`]/[`Into`] traits are implemented for numeric casts which are guaranteed to be
//! lossless on all supported platforms based on their types alone. This is a strong guarantee,
//! and if these traits fit your use case you should think hard before picking anything else,
//! including cove's casts. However, such a strong guarantee naturally comes with a limited scope;
//! for the many use cases which do not conform, other casting mechanisms are required.
//!
//! ### [`TryFrom`]/[`TryInto`]
//! The [`TryFrom`]/[`TryInto`] traits are provided for numeric casts which might be lossy, to
//! allow for testing of this lossiness at runtime. This covers many of the use cases unaddressed
//! by [`From`]/[`Into`], but not all. For example:
//!
//! * Some conversions which might be desired are not provided, such as from floating points to
//!     integers
//! * If the cast is lossy but you want to use whatever it produces anyway, [`TryFrom`]/[`TryInto`]
//!     can't help
//! * If the cast is lossy but you want as close as it can get, [`TryFrom`]/[`TryInto`] can't help
//! * If the cast is lossy and you want good error messages, [`TryFrom`]/[`TryInto`]'s errors tend
//!     to disappoint
//! * If you know a cast is lossless, you are stuck with suboptimal options:
//!     * Risk the unsafeness of [`unwrap_unchecked`](Result::unwrap_unchecked)
//!     * Absorb the performance cost of [`unwrap`](Result::unwrap)
//!     * Absorb the performance cost and polluted interface implied by returning a [`Result`]
//!
//! ### `as` Keyword
//! The use cases not covered by [`From`]/[`Into`]/[`TryFrom`]/[`TryInto`] are generally left to the
//! `as` keyword. This is unfortunately a fairly blunt instrument which requires paying careful
//! attention to the semantics of numeric casts to ensure correct use. For this reason, usage of
//! `as` for numeric casts often triggers complaints from linters, such as when using clippy in
//! pedantic mode.
//!
//! Since `as` is not a trait it is quite difficult to use it in generic contexts. Moreover, due to
//! being overloaded for other type casts it can be more challenging to search its usages for
//! possible sources of numeric cast bugs. In general it is a good idea to avoid the `as` keyword
//! for numeric casts, at least in the presence of better options. This crate aims to provide those
//! better options.