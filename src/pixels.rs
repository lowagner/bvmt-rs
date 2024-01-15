#![allow(dead_code)]

use crate::color::*;
use crate::gpu::Synced;

pub struct Pixels {
    pub(crate) synced: Synced,
    // Invariant: this array has the correct size.
    pub(crate) array: Vec<Vec<Color>>,
}
