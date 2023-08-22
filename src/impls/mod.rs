mod blanket;
mod nonzero;
mod primitives;

/// Marker trait to activate the blanket implementation for casts which are guaranteed lossless
trait LosslessCast {}