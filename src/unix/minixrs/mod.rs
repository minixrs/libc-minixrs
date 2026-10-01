//! minixrs, aarch64: definitions for musl-minixrs (musl 1.2.6).
//!
//! musl-minixrs leaves musl's aarch64 type layouts and constants untouched,
//! so everything here is the Linux-musl-aarch64 definition, copied from
//! `src/unix/linux_like/`. Nothing here is derived by hand from a header.
//!
//! Every `pub type`, `pub struct` and `pub const` in this module is checked
//! against the C headers by `verify/check-libc-abi.sh` in the minixrs tooling
//! repo. Adding an item here without adding it to that manifest fails the gate.

use crate::prelude::*;
