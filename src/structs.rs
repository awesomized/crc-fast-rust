// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

#![allow(dead_code)]

use crate::traits::{CrcCalculator, CrcWidth};
use crate::{arch, generate, CrcAlgorithm, CrcParams};

/// CRC-32 width implementation
#[derive(Clone, Copy)]
pub struct Width32;

impl CrcWidth for Width32 {
    const WIDTH: u32 = 32;
    type Value = u32;
}

/// CRC-64 width implementation
#[derive(Clone, Copy)]
pub struct Width64;

impl CrcWidth for Width64 {
    const WIDTH: u32 = 64;
    type Value = u64;
}

/// CRC State wrapper to manage the SIMD operations and reflection mode
#[derive(Debug, Clone, Copy)]
pub struct CrcState<T> {
    pub value: T,
    pub reflected: bool,
}

pub(crate) struct Calculator {}

impl CrcCalculator for Calculator {
    #[inline(always)]
    fn calculate(state: u64, data: &[u8], params: CrcParams) -> u64 {
        unsafe { arch::update(state, data, params) }
    }
}

impl CrcParams {
    /// Creates custom CRC parameters for a given set of Rocksoft CRC parameters.
    ///
    /// Does not support mis-matched refin/refout parameters, so both must be true or both false.
    ///
    /// Rocksoft parameters for lots of variants: https://reveng.sourceforge.io/crc-catalogue/all.htm
    pub fn new(
        name: &'static str,
        width: u8,
        poly: u64,
        init: u64,
        reflected: bool,
        xorout: u64,
        check: u64,
    ) -> Self {
        let keys = generate::keys(width, poly, reflected);

        let algorithm = match width {
            32 => CrcAlgorithm::Crc32Custom,
            64 => CrcAlgorithm::Crc64Custom,
            _ => panic!("Unsupported width: {}", width),
        };

        Self {
            algorithm,
            name,
            width,
            poly,
            init,
            refin: reflected,
            refout: reflected,
            xorout,
            check,
            keys,
        }
    }
}
