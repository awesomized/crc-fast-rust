// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! A minimal WebAssembly module which only calculates CRC-64/NVME.
//!
//! The module doesn't use an allocator: callers place the input in the module's linear memory
//! themselves (growing it if necessary) and pass its location.

#![no_std]

use crc_fast::{CrcAlgorithm, Digest};

/// Calculates the CRC-64/NVME checksum of the `len` bytes at `data`, continuing from `crc`.
///
/// Pass `0` as `crc` to start a new checksum, or the result of a previous call to keep going,
/// which makes it possible to checksum a stream piece by piece:
///
/// ```text
/// crc64_nvme(crc64_nvme(0, "1234"), "56789") == crc64_nvme(0, "123456789")
/// ```
///
/// # Safety
///
/// `data` must point to `len` readable bytes.
#[no_mangle]
pub unsafe extern "C" fn crc64_nvme(crc: u64, data: *const u8, len: usize) -> u64 {
    // a finalized checksum is the internal state XORed with all ones, so undo that to resume
    let mut digest = Digest::new_with_init_state(CrcAlgorithm::Crc64Nvme, !crc);

    // SAFETY: the caller guarantees that `data` points to `len` readable bytes
    digest.update(unsafe { core::slice::from_raw_parts(data, len) });

    digest.finalize()
}
