//! Shrinker — lossless image compression for everyone.
//!
//! This crate exposes the compression pipeline used by the `shrinker` CLI.
//! Every image is re-encoded with a lossless codec and the smallest of the
//! two (original or re-encoded) is written to disk.

pub mod batch;
pub mod cli;
pub mod compress;
pub mod encode;
pub mod format;
pub mod io;
pub mod png;
pub mod progress;
pub mod types;
pub mod util;
pub mod webp;
