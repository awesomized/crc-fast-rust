// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides AVX2 and VPCLMULQDQ-specific implementations of the ArchOps trait.
//!
//! It performs folding using 4 x YMM registers of 256-bits each.
//!
//! VPCLMULQDQ is available without AVX-512 on AMD Zen 3, on Intel hybrid client parts from Alder
//! Lake onward (where AVX-512 is fused off), and on Intel E-core server parts such as Sierra
//! Forest. On those CPUs it carries the same instruction throughput as 128-bit PCLMULQDQ while
//! operating on twice the data per instruction.

#![cfg(target_arch = "x86_64")]

use crate::arch::x86::sse::X86SsePclmulqdqOps;
use crate::enums::Reflector;
use crate::structs::CrcState;
use crate::traits::{ArchOps, EnhancedCrcWidth};
use core::arch::x86_64::*;
use core::ops::BitXor;

/// Implements the ArchOps trait using 256-bit AVX2 and VPCLMULQDQ instructions.
/// Delegates to X86SsePclmulqdqOps for standard 128-bit operations
#[derive(Debug, Copy, Clone)]
pub struct X86_64Avx2VpclmulqdqOps(X86SsePclmulqdqOps);

impl Default for X86_64Avx2VpclmulqdqOps {
    fn default() -> Self {
        Self::new()
    }
}

impl X86_64Avx2VpclmulqdqOps {
    #[inline(always)]
    pub fn new() -> Self {
        Self(X86SsePclmulqdqOps)
    }
}

// Wrapper for __m256i to make it easier to work with
#[derive(Debug, Copy, Clone)]
struct Simd256(__m256i);

impl Simd256 {
    #[inline]
    #[target_feature(enable = "avx")]
    unsafe fn new(x3: u64, x2: u64, x1: u64, x0: u64) -> Self {
        Self(_mm256_set_epi64x(
            x3 as i64, x2 as i64, x1 as i64, x0 as i64,
        ))
    }

    /// Fold this register forward by the coefficient and XOR in the newly loaded data.
    #[inline]
    #[target_feature(enable = "avx2,vpclmulqdq")]
    unsafe fn fold_32(&self, coeff: &Self, new_data: &Self) -> Self {
        // AVX2 has no ternary-logic instruction, so the XOR3 that the AVX-512 path folds into
        // `_mm512_ternarylogic_epi64` is spelled here as two XORs.
        let low = _mm256_clmulepi64_epi128(self.0, coeff.0, 0);
        let high = _mm256_clmulepi64_epi128(self.0, coeff.0, 17);

        Self(_mm256_xor_si256(_mm256_xor_si256(low, high), new_data.0))
    }

    #[inline]
    #[target_feature(enable = "avx")]
    unsafe fn load_from_ptr(ptr: *const u8) -> Self {
        Self(_mm256_loadu_si256(ptr as *const __m256i))
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn to_128i_extract<const INDEX: i32>(self) -> __m128i {
        _mm256_extracti128_si256(self.0, INDEX)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn xor(&self, other: &Self) -> Self {
        Self(_mm256_xor_si256(self.0, other.0))
    }
}

impl X86_64Avx2VpclmulqdqOps {
    /// Process aligned blocks using VPCLMULQDQ with 4 x 256-bit registers
    #[inline]
    #[target_feature(enable = "avx2,vpclmulqdq")]
    unsafe fn process_blocks<W: EnhancedCrcWidth>(
        &self,
        state: &mut CrcState<<X86_64Avx2VpclmulqdqOps as ArchOps>::Vector>,
        first: &[__m128i; 8],
        rest: &[[__m128i; 8]],
        keys: &[u64; 23],
        reflected: bool,
    ) -> W::Value
    where
        W::Value: Copy + BitXor<Output = W::Value>,
    {
        let state_u64s = self.extract_u64s(state.value);

        let positioned_state = if reflected {
            Simd256::new(0, 0, 0, state_u64s[0])
        } else {
            Simd256::new(state_u64s[1], 0, 0, 0)
        };

        let reflector = create_reflector256(reflected);

        // 4 x 256-bit registers hold the 128 bytes of `first`.
        let first_ptr = first.as_ptr() as *const u8;

        let mut x = [
            reflect_bytes256(&reflector, Simd256::load_from_ptr(first_ptr)),
            reflect_bytes256(&reflector, Simd256::load_from_ptr(first_ptr.add(32))),
            reflect_bytes256(&reflector, Simd256::load_from_ptr(first_ptr.add(64))),
            reflect_bytes256(&reflector, Simd256::load_from_ptr(first_ptr.add(96))),
        ];

        x[0] = positioned_state.xor(&x[0]);

        // Each iteration consumes one 128-byte block, so the registers fold across a 128-byte
        // distance.
        let coeff = self.create_avx2_128byte_coefficient(keys, reflected);

        for block in rest {
            let block_ptr = block.as_ptr() as *const u8;

            x[0] = x[0].fold_32(
                &coeff,
                &reflect_bytes256(&reflector, Simd256::load_from_ptr(block_ptr)),
            );
            x[1] = x[1].fold_32(
                &coeff,
                &reflect_bytes256(&reflector, Simd256::load_from_ptr(block_ptr.add(32))),
            );
            x[2] = x[2].fold_32(
                &coeff,
                &reflect_bytes256(&reflector, Simd256::load_from_ptr(block_ptr.add(64))),
            );
            x[3] = x[3].fold_32(
                &coeff,
                &reflect_bytes256(&reflector, Simd256::load_from_ptr(block_ptr.add(96))),
            );
        }

        let folded = self.fold_from_4x256_to_1x128(x, keys, reflected);

        W::perform_final_reduction(folded, reflected, keys, self)
    }

    /// Create a folding coefficient for 128-byte folding distances
    #[inline(always)]
    unsafe fn create_avx2_128byte_coefficient(&self, keys: &[u64; 23], reflected: bool) -> Simd256 {
        let (k1, k2) = if reflected {
            (keys[3], keys[4])
        } else {
            (keys[4], keys[3])
        };

        // Replicate the coefficient pair
        Simd256::new(k1, k2, k1, k2)
    }

    /// Fold from 4 x 256-bit to 1 x 128-bit
    #[inline(always)]
    unsafe fn fold_from_4x256_to_1x128(
        &self,
        x: [Simd256; 4],
        keys: &[u64; 23],
        reflected: bool,
    ) -> __m128i {
        // Create the fold coefficients for different distances
        let fold_coefficients = [
            self.create_vector_from_u64_pair(keys[10], keys[9], reflected), // 112 bytes
            self.create_vector_from_u64_pair(keys[12], keys[11], reflected), // 96 bytes
            self.create_vector_from_u64_pair(keys[14], keys[13], reflected), // 80 bytes
            self.create_vector_from_u64_pair(keys[16], keys[15], reflected), // 64 bytes
            self.create_vector_from_u64_pair(keys[18], keys[17], reflected), // 48 bytes
            self.create_vector_from_u64_pair(keys[20], keys[19], reflected), // 32 bytes
            self.create_vector_from_u64_pair(keys[2], keys[1], reflected),  // 16 bytes
        ];

        // Extract the 8 x 128-bit vectors from the 4 x 256-bit vectors, oldest first. Reflection
        // reverses byte order within each register, so the non-reflected path takes the lanes in
        // the opposite order.
        let v128 = if reflected {
            [
                x[0].to_128i_extract::<0>(),
                x[0].to_128i_extract::<1>(),
                x[1].to_128i_extract::<0>(),
                x[1].to_128i_extract::<1>(),
                x[2].to_128i_extract::<0>(),
                x[2].to_128i_extract::<1>(),
                x[3].to_128i_extract::<0>(),
                x[3].to_128i_extract::<1>(),
            ]
        } else {
            [
                x[0].to_128i_extract::<1>(),
                x[0].to_128i_extract::<0>(),
                x[1].to_128i_extract::<1>(),
                x[1].to_128i_extract::<0>(),
                x[2].to_128i_extract::<1>(),
                x[2].to_128i_extract::<0>(),
                x[3].to_128i_extract::<1>(),
                x[3].to_128i_extract::<0>(),
            ]
        };

        // Fold the 8 xmm registers to 1 xmm register
        let mut res = v128[7];

        for (i, &coeff) in fold_coefficients.iter().enumerate() {
            let folded_h = self.carryless_mul_00(v128[i], coeff);
            let folded_l = self.carryless_mul_11(v128[i], coeff);
            res = self.xor3_vectors(folded_h, folded_l, res);
        }

        res
    }
}

// 256-bit version of the Reflector
#[derive(Clone, Copy)]
enum Reflector256 {
    NoReflector,
    ForwardReflector { smask: Simd256 },
}

// Function to create the appropriate reflector based on CRC parameters
#[inline(always)]
unsafe fn create_reflector256(reflected: bool) -> Reflector256 {
    if reflected {
        Reflector256::NoReflector
    } else {
        // Load shuffle mask
        let smask = Simd256::new(
            0x08090a0b0c0d0e0f,
            0x0001020304050607,
            0x08090a0b0c0d0e0f,
            0x0001020304050607,
        );
        Reflector256::ForwardReflector { smask }
    }
}

// Function to apply reflection to a 256-bit vector
#[inline(always)]
unsafe fn reflect_bytes256(reflector: &Reflector256, data: Simd256) -> Simd256 {
    match reflector {
        Reflector256::NoReflector => data,
        Reflector256::ForwardReflector { smask } => shuffle_bytes256(data, *smask),
    }
}

// Implement a 256-bit byte shuffle function
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn shuffle_bytes256(data: Simd256, mask: Simd256) -> Simd256 {
    // _mm256_shuffle_epi8 works within each 128-bit lane, so the quadword lanes are reversed
    // afterwards to complete the byte reversal across the full register. 0x1b maps
    // (0,1,2,3) -> (3,2,1,0).
    Simd256(_mm256_permute4x64_epi64::<0x1b>(_mm256_shuffle_epi8(
        data.0, mask.0,
    )))
}

// Delegate all ArchOps methods to the inner X86SsePclmulqdqOps instance
impl ArchOps for X86_64Avx2VpclmulqdqOps {
    type Vector = __m128i;

    #[inline(always)]
    unsafe fn process_enhanced_simd_blocks<W: EnhancedCrcWidth>(
        &self,
        state: &mut CrcState<Self::Vector>,
        first: &[Self::Vector; 8],
        rest: &[[Self::Vector; 8]],
        _reflector: &Reflector<Self::Vector>,
        keys: &[u64; 23],
    ) -> bool
    where
        Self::Vector: Copy,
    {
        // Update the state with the result
        *state = W::create_state(
            self.process_blocks::<W>(state, first, rest, keys, state.reflected),
            state.reflected,
            self,
        );

        // Return true to indicate we handled it
        true
    }

    // Delegate all other methods to X86SsePclmulqdqOps
    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn create_vector_from_u64_pair(
        &self,
        high: u64,
        low: u64,
        reflected: bool,
    ) -> Self::Vector {
        self.0.create_vector_from_u64_pair(high, low, reflected)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn create_vector_from_u64_pair_non_reflected(
        &self,
        high: u64,
        low: u64,
    ) -> Self::Vector {
        self.0.create_vector_from_u64_pair_non_reflected(high, low)
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    unsafe fn create_vector_from_u64(&self, value: u64, high: bool) -> Self::Vector {
        self.0.create_vector_from_u64(value, high)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn extract_u64s(&self, vector: Self::Vector) -> [u64; 2] {
        self.0.extract_u64s(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn extract_poly64s(&self, vector: Self::Vector) -> [u64; 2] {
        self.0.extract_poly64s(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn xor_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.xor_vectors(a, b)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn load_bytes(&self, ptr: *const u8) -> Self::Vector {
        self.0.load_bytes(ptr)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn load_aligned(&self, ptr: *const [u64; 2]) -> Self::Vector {
        self.0.load_aligned(ptr)
    }

    #[inline]
    #[target_feature(enable = "ssse3")]
    unsafe fn shuffle_bytes(&self, data: Self::Vector, mask: Self::Vector) -> Self::Vector {
        self.0.shuffle_bytes(data, mask)
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    unsafe fn blend_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        mask: Self::Vector,
    ) -> Self::Vector {
        self.0.blend_vectors(a, b, mask)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_left_8(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_8(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn set_all_bytes(&self, value: u8) -> Self::Vector {
        self.0.set_all_bytes(value)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn create_compare_mask(&self, vector: Self::Vector) -> Self::Vector {
        self.0.create_compare_mask(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn and_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.and_vectors(a, b)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_32(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_32(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_left_32(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_32(vector)
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    unsafe fn create_vector_from_u32(&self, value: u32, high: bool) -> Self::Vector {
        self.0.create_vector_from_u32(value, high)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_left_4(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_4(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_4(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_4(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_8(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_8(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_5(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_5(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_6(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_6(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_7(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_7(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_right_12(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_12(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn shift_left_12(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_12(vector)
    }

    #[inline]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn carryless_mul_00(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_00(a, b)
    }

    #[inline]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn carryless_mul_01(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_01(a, b)
    }

    #[inline]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn carryless_mul_10(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_10(a, b)
    }

    #[inline]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn carryless_mul_11(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_11(a, b)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn xor3_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        c: Self::Vector,
    ) -> Self::Vector {
        // AVX2 has no ternary-logic instruction, so this delegates to the SSE pair of XORs.
        self.0.xor3_vectors(a, b, c)
    }
}
