//! Shrinker — lossless image compression for everyone.
//!
//! This crate exposes the compression pipeline used by the `shrinker` CLI.
//! Every image is re-encoded with a lossless codec and the smallest of the
//! two (original or re-encoded) is written to disk.
