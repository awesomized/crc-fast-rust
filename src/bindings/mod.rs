// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides bindings to the C implementations of CRC32 algorithms.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

use crate::structs::CrcParams;
use std::ffi::CStr;
use std::os::raw::c_char;

#[cfg(not(rust_implementation_only))]
mod crc32_iscsi;
#[cfg(not(rust_implementation_only))]
mod crc32_iso_hdlc;

// note that the initial state needs to be reversed
#[cfg(not(rust_implementation_only))]
#[inline(always)]
pub(crate) fn crc32_iso_hdlc(state: u64, data: &[u8], params: CrcParams) -> u64 {
    unsafe {
        // TODO: Examine the C implementation and see why we have to invert the state...
        crc32_iso_hdlc::crc32_iso_hdlc_impl(
            !state as u32,
            data.as_ptr() as *const c_char,
            data.len(),
        ) as u64
            ^ params.xorout
    }
}

// note that the initial state needs to be reversed
#[cfg(not(rust_implementation_only))]
#[inline(always)]
pub(crate) fn crc32_iscsi(state: u64, data: &[u8], params: CrcParams) -> u64 {
    unsafe {
        // TODO: Examine the C implementation and see why we have to invert the state...
        crc32_iscsi::crc32_iscsi_impl(!state as u32, data.as_ptr() as *const c_char, data.len())
            as u64
            ^ params.xorout
    }
}

#[cfg(not(rust_implementation_only))]
#[allow(unused)]
pub unsafe fn get_iso_hdlc_target() -> String {
    convert_to_string(crc32_iso_hdlc::get_iso_hdlc_target())
}

#[cfg(not(rust_implementation_only))]
#[allow(unused)]
pub unsafe fn get_iscsi_target() -> String {
    convert_to_string(crc32_iscsi::get_iscsi_target())
}

fn convert_to_string(ptr: *const c_char) -> String {
    unsafe {
        // First ensure the pointer isn't null
        assert!(!ptr.is_null());

        // Convert to CStr - this handles finding the null terminator
        let c_str = CStr::from_ptr(ptr);

        // Convert to a regular string, handling any invalid UTF-8
        c_str.to_string_lossy().into_owned()
    }
}
