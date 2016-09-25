// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Interfaces to the operating system provided random number
//! generators.

use Rng;
use libtww::system::tww::random_u32;

/// A random number generator that retrieves randomness straight from
/// the operating system. Platform sources:
///
/// - Unix-like systems (Linux, Android, Mac OSX): read directly from
///   `/dev/urandom`, or from `getrandom(2)` system call if available.
/// - Windows: calls `CryptGenRandom`, using the default cryptographic
///   service provider with the `PROV_RSA_FULL` type.
/// - iOS: calls SecRandomCopyBytes as /dev/(u)random is sandboxed.
/// - PNaCl: calls into the `nacl-irt-random-0.1` IRT interface.
///
/// This does not block.
#[derive(Copy, Clone)]
pub struct OsRng;

impl OsRng {
    /// Create a new `OsRng`.
    pub fn new() -> OsRng {
        OsRng
    }
}

impl Rng for OsRng {
    fn next_u32(&mut self) -> u32 {
        random_u32()
    }

    fn next_u64(&mut self) -> u64 {
        ((random_u32() as u64) << 32) | random_u32() as u64
    }

    fn fill_bytes(&mut self, v: &mut [u8]) {
        for chunk in v.chunks_mut(4) {
            let rand = random_u32();
            for (i, value) in chunk.iter_mut().enumerate() {
                *value = (rand >> (8 * i)) as u8;
            }
        }
    }
}
