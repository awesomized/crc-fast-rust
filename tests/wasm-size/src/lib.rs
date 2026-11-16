// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! Calculates CRC-64/NVME through each of the entry points which the README promises only link
//! the lookup table of the algorithm they're called with, so that CI can check that they do.

#![no_std]

use crc_fast::{checksum, crc64_nvme, CrcAlgorithm, Digest};

/// # Safety
///
/// `data` must point to `len` readable bytes.
#[no_mangle]
pub unsafe extern "C" fn helper(data: *const u8, len: usize) -> u64 {
    // SAFETY: the caller guarantees that `data` points to `len` readable bytes
    let data = unsafe { core::slice::from_raw_parts(data, len) };

    crc64_nvme(data)
}

/// # Safety
///
/// `data` must point to `len` readable bytes.
#[no_mangle]
pub unsafe extern "C" fn one_shot(data: *const u8, len: usize) -> u64 {
    // SAFETY: the caller guarantees that `data` points to `len` readable bytes
    let data = unsafe { core::slice::from_raw_parts(data, len) };

    checksum(CrcAlgorithm::Crc64Nvme, data)
}

/// # Safety
///
/// `data` must point to `len` readable bytes.
#[no_mangle]
pub unsafe extern "C" fn streaming(data: *const u8, len: usize) -> u64 {
    // SAFETY: the caller guarantees that `data` points to `len` readable bytes
    let data = unsafe { core::slice::from_raw_parts(data, len) };

    let mut digest = Digest::new(CrcAlgorithm::Crc64Nvme);
    digest.update(data);
    digest.finalize()
}
