//! This module provides a function to combine CRCs of two sequences of bytes.
//!
//! It is based on the work of Mark Adler and is designed to be used with
//! different CRC algorithms.
/*
  Derived from this excellent answer by Mark Adler on StackOverflow:
  https://stackoverflow.com/questions/29915764/generic-crc-8-16-32-64-combine-implementation/29928573#29928573
*/

/* crccomb.c -- generalized combination of CRCs
 * Copyright (C) 2015 Mark Adler
 * Version 1.1  29 Apr 2015  Mark Adler
 */

/*
 This software is provided 'as-is', without any express or implied
 warranty.  In no event will the author be held liable for any damages
 arising from the use of this software.

 Permission is granted to anyone to use this software for any purpose,
 including commercial applications, and to alter it and redistribute it
 freely, subject to the following restrictions:

 1. The origin of this software must not be misrepresented; you must not
    claim that you wrote the original software. If you use this software
    in a product, an acknowledgment in the product documentation would be
    appreciated but is not required.
 2. Altered source versions must be plainly marked as such, and must not be
    misrepresented as being the original software.
 3. This notice may not be removed or altered from any source distribution.

 Mark Adler
 madler@alumni.caltech.edu
*/

/*
  zlib provides a fast operation to combine the CRCs of two sequences of bytes
  into a single CRC, which is the CRC of the two sequences concatenated.  That
  operation requires only the two CRC's and the length of the second sequence.
  The routine in zlib only works on the particular CRC-32 used by zlib.  The
  code provided here generalizes that operation to apply to a wide range of
  CRCs.  The CRC is specified in a series of #defines, based on the
  parameterization found in Ross William's excellent CRC tutorial here:

     http://www.ross.net/crc/download/crc_v3.txt

  A comprehensive catalogue of known CRCs, their parameters, check values, and
  references can be found here:

     http://reveng.sourceforge.net/crc-catalogue/all.htm
*/

use crate::CrcParams;

/// Keys for combining, one per power-of-two byte length: `x^(8 * 2^i) mod P(x)` for i in `0..64`,
/// in normal (non-reflected) form, plus the Barrett constant used to reduce a product.
///
/// Appending `len2` zero bytes multiplies a CRC by `x^(8 * len2)` in `GF(2)[x]/P(x)`, so
///
/// ```text
/// crc(A || B) = crc(A) * x^(8 * len(B)) + crc(B)
/// ```
///
/// and `x^(8 * len2)` is the product of the keys that `len2`'s set bits select. Combining is
/// therefore one field multiply per set bit, and a single multiply when `len2` is a power of two.
///
/// The same kind of constant as the crate's folding keys -- `x^n mod P(x)` -- but for the
/// exponents combining needs rather than the ones the folding kernel needs.
#[derive(Debug)]
pub struct CombineKeys {
    /// `x^(8 * 2^i) mod P(x)`.
    keys: [u64; 64],
    /// `floor(x^(2 * width) / P(x))`, low `width` bits; the implicit `x^width` is folded into the
    /// reduction rather than stored.
    barrett_mu: u64,
}

impl CombineKeys {
    /// Build the table for a `width`-bit polynomial given in normal form.
    ///
    /// Each key is the square of the one below it, starting from `x^8` -- one zero byte -- which
    /// needs no reduction because every supported width exceeds 8.
    pub const fn new(poly: u64, width: u8) -> Self {
        let mut keys = [0u64; 64];
        keys[0] = 1u64 << 8;

        let mut i = 1;
        while i < 64 {
            keys[i] = mul_mod_poly(keys[i - 1], keys[i - 1], poly, width);
            i += 1;
        }

        Self {
            keys,
            barrett_mu: barrett_mu(poly, width),
        }
    }
}

/// `floor(x^(2 * width) / P(x))`, truncated to `width` bits.
///
/// Bitwise long division of `x^(2 * width)` by `P(x) = x^width + poly`. The quotient has
/// `width + 1` bits and its leading term is always `x^width`, so only the low `width` are kept
/// and the reduction folds the implicit term back in.
const fn barrett_mu(poly: u64, width: u8) -> u64 {
    let mask = if width == 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    };

    let mut remainder = 0u64;
    let mut quotient = 0u64;

    // Walk the dividend from its leading term down. Only bit `2 * width` is set.
    let mut pos = 2 * width as u32 + 1;
    while pos > 0 {
        pos -= 1;
        let bit = (pos == 2 * width as u32) as u64;
        // The bit shifted past `width - 1` lands in position `width`, which means P divides in
        // here and the quotient takes a one.
        let carry = (remainder >> (width - 1)) & 1;
        remainder = (((remainder << 1) | bit) & mask) ^ (poly & 0u64.wrapping_sub(carry));
        quotient = (quotient << 1) | carry;
    }

    quotient & mask
}

/// Carry-less multiply modulo `P(x)`, for a `width`-bit polynomial in normal form.
///
/// Russian-peasant: `b` is consumed a bit at a time while `a` is doubled, reducing by `poly`
/// whenever doubling overflows the field.
const fn mul_mod_poly(mut a: u64, mut b: u64, poly: u64, width: u8) -> u64 {
    let msb = 1u64 << (width - 1);
    let mask = if width == 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    };

    // Branch-free: both conditions depend on data a predictor cannot learn, inside a dependency
    // chain.
    let mut product = 0u64;
    let mut n = 0;
    while n < width {
        product ^= a & 0u64.wrapping_sub(b & 1);
        b >>= 1;
        let overflow = 0u64.wrapping_sub((a & msb) >> (width - 1));
        a = ((a << 1) & mask) ^ (poly & overflow);
        n += 1;
    }

    product
}

/// Carry-less multiply of two 64-bit polynomials, returning the 128-bit product as
/// `(high, low)`.
///
/// # Safety
/// Caller must have established that `pclmulqdq` is available.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq", enable = "sse2")]
unsafe fn clmul64(a: u64, b: u64) -> (u64, u64) {
    use core::arch::x86_64::{_mm_clmulepi64_si128, _mm_extract_epi64, _mm_set_epi64x};

    let product = {
        _mm_clmulepi64_si128(
            _mm_set_epi64x(0, a as i64),
            _mm_set_epi64x(0, b as i64),
            0x00,
        )
    };
    let low = _mm_extract_epi64::<0>(product) as u64;
    let high = _mm_extract_epi64::<1>(product) as u64;
    (high, low)
}

/// `a * b mod P(x)` via carry-less multiply and Barrett reduction.
///
/// Three multiplies: the product, the Barrett quotient estimate, and folding that estimate back.
/// `P(x)` and the Barrett constant both have an implicit `x^width` that does not fit a `u64`, so
/// each is applied as a separate exclusive-or of the operand.
///
/// # Safety
/// Caller must have established that `pclmulqdq` is available.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq", enable = "sse2")]
unsafe fn mul_mod_poly_clmul(a: u64, b: u64, poly: u64, mu: u64, width: u8) -> u64 {
    let mask = if width == 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    };

    // Full product, then split at the field width.
    let (product_high, product_low) = clmul64(a, b);
    let (upper, lower) = if width == 64 {
        (product_high, product_low)
    } else {
        (
            (product_low >> width) | (product_high << (64 - width)),
            product_low & mask,
        )
    };

    // q = high(upper * mu), where mu = x^width + barrett_mu, so the implicit term contributes
    // `upper` itself.
    let (mu_high, mu_low) = clmul64(upper, mu);
    let quotient = (if width == 64 {
        mu_high
    } else {
        (mu_low >> width) | (mu_high << (64 - width))
    }) ^ upper;

    // r = low(product) ^ low(q * P). The implicit x^width of P contributes only above the
    // window, so only `poly` matters here.
    let (_, fold_low) = clmul64(quotient, poly);
    (lower ^ fold_low) & mask
}

/// Whether the carry-less multiply path is usable on this machine.
///
/// Defers to the crate's cached feature detection rather than probing separately: every x86 tier
/// it can select is a PCLMULQDQ one, so anything but the software fallback has the instruction.
#[cfg(target_arch = "x86_64")]
fn has_clmul() -> bool {
    !matches!(
        crate::feature_detection::get_arch_ops(),
        crate::feature_detection::ArchOpsInstance::SoftwareFallback
    )
}

/// Reflect a `width`-bit value, converting between the reflected and normal domains.
const fn reflect_value(value: u64, width: u8) -> u64 {
    value.reverse_bits() >> (64 - width as u32)
}

impl CombineKeys {
    /// `a * b mod P(x)`, using the carry-less multiply when the machine has one.
    #[inline]
    fn mul_mod(&self, a: u64, b: u64, params: &CrcParams) -> u64 {
        #[cfg(target_arch = "x86_64")]
        if has_clmul() {
            // SAFETY: guarded by the feature check above.
            return unsafe { mul_mod_poly_clmul(a, b, params.poly, self.barrett_mu, params.width) };
        }

        mul_mod_poly(a, b, params.poly, params.width)
    }

    /// Multiply `crc1` by `x^(8 * len2)` using the keys `len2`'s set bits select, then add `crc2`.
    ///
    /// The keys are in normal form, so a reflected CRC is converted in and out once per combine
    /// rather than once per multiply.
    fn combine(&self, crc1: u64, crc2: u64, len2: u64, params: &CrcParams) -> u64 {
        let normalized = crc1 ^ params.init_algorithm ^ params.xorout;
        let mut acc = if params.refin {
            reflect_value(normalized, params.width)
        } else {
            normalized
        };

        let mut len = len2;
        let mut i = 0;
        while len > 0 {
            if len & 1 == 1 {
                acc = self.mul_mod(acc, self.keys[i], params);
            }
            len >>= 1;
            i += 1;
        }

        let shifted = if params.refin {
            reflect_value(acc, params.width)
        } else {
            acc
        };

        shifted ^ crc2
    }
}

/* Combine the CRCs of two successive sequences, where crc1 is the CRC of the
first sequence of bytes, crc2 is the CRC of the immediately following
sequence of bytes, and len2 is the length of the second sequence.  The CRC
of the combined sequence is returned. */
pub fn checksums(crc1: u64, crc2: u64, len2: u64, params: &CrcParams) -> u64 {
    match params.combine_keys {
        Some(keys) => keys.combine(crc1, crc2, len2, params),
        // A custom polynomial has no const table, so build one. Sixty-four squarings, once per
        // combine.
        None => CombineKeys::new(params.poly, params.width).combine(crc1, crc2, len2, params),
    }
}
#[cfg(test)]
mod combine_keys_tests {
    use super::*;

    // The GF(2) zeros-operator implementation, retained as the independent reference the
    // combine-key path is checked against. Derived from Mark Adler's crccomb.c; see the notice at
    // the top of this file.

    /* Multiply the GF(2) vector vec by the GF(2) matrix mat, returning the
    resulting vector.  The vector is stored as bits in a crc_t.  The matrix is
    similarly stored with each column as a crc_t, where the number of columns is
    at least enough to cover the position of the most significant 1 bit in the
    vector (so a dimension parameter is not needed). */
    fn gf2_matrix_times(mat: &[u64; 64], mut vec: u64) -> u64 {
        let mut sum = 0;
        let mut idx = 0;
        while vec > 0 {
            if vec & 1 == 1 {
                sum ^= mat[idx];
            }
            vec >>= 1;
            idx += 1;
        }

        sum
    }

    /* Multiply the matrix mat by itself, returning the result in square.  WIDTH is
    the dimension of the matrices, i.e., the number of bits in each crc_t
    (rows), and the number of crc_t's (columns). */
    fn gf2_matrix_square(square: &mut [u64; 64], mat: &[u64; 64]) {
        for n in 0..64 {
            square[n] = gf2_matrix_times(mat, mat[n]);
        }
    }

    /// Construct the operator for one zero bit.
    fn one_bit_operator(params: &CrcParams) -> [u64; 64] {
        let mut odd = [0u64; 64];
        let mut col: u64;

        if params.refin && params.refout {
            // use the reflected POLY
            odd[0] = reflect_poly(params.poly, params.width as u32);
            col = 1;
            for n in 1..params.width {
                odd[n as usize] = col;
                col <<= 1;
            }
        } else if !params.refin && !params.refout {
            col = 2;
            for n in 0..params.width - 1 {
                odd[n as usize] = col;
                col <<= 1;
            }
            // Put poly at the last valid index (width-1)
            odd[(params.width - 1) as usize] = params.poly;
        } else {
            panic!("Unsupported CRC configuration");
        }

        odd
    }

    /// Combine by building and applying a GF(2) zeros operator.
    ///
    /// Test-only: the independent reference the combine-key path is checked against.
    fn checksums_via_matrix(mut crc1: u64, crc2: u64, mut len2: u64, params: &CrcParams) -> u64 {
        let mut even = [0u64; 64]; /* even-power-of-two zeros operator */

        /* exclusive-or the result with len2 zeros applied to the CRC of an empty
        sequence */
        crc1 ^= params.init_algorithm ^ params.xorout;

        /* construct the operator for one zero bit and put in odd[] */
        let mut odd = one_bit_operator(params);

        /* put operator for two zero bits in even */
        gf2_matrix_square(&mut even, &odd);

        /* put operator for four zero bits in odd */
        gf2_matrix_square(&mut odd, &even);

        /* apply len2 zeros to crc1 (first square will put the operator for one
        zero byte, eight zero bits, in even) */
        loop {
            /* apply zeros operator for this bit of len2 */
            gf2_matrix_square(&mut even, &odd);
            if len2 & 1 == 1 {
                crc1 = gf2_matrix_times(&even, crc1);
            }
            len2 >>= 1;

            /* if no more bits set, then done */
            if len2 == 0 {
                break;
            }

            /* another iteration of the loop with odd and even swapped */
            gf2_matrix_square(&mut odd, &even);
            if len2 & 1 == 1 {
                crc1 = gf2_matrix_times(&odd, crc1);
            }
            len2 >>= 1;

            /* if no more bits set, then done */
            if len2 == 0 {
                break;
            }
        }

        /* return combined crc */
        crc1 ^= crc2;

        crc1
    }

    fn reflect_poly(poly: u64, width: u32) -> u64 {
        assert!(width <= 64, "Width must be <= 64 bits");

        // First reverse all bits
        let reversed = bit_reverse(poly);

        // Shift right to get the significant bits in the correct position
        // For a 32-bit poly, we need to shift right by (64 - 32) = 32 bits
        let shifted = reversed >> (64 - width);

        // Create mask for the target width
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };

        // Apply mask to ensure we only keep the bits we want
        shifted & mask
    }

    fn bit_reverse(mut forward: u64) -> u64 {
        let mut reversed = 0;

        for _ in 0..64 {
            reversed <<= 1;
            reversed |= forward & 1;
            forward >>= 1;
        }

        reversed
    }

    /// Barrett constants, cross-checked against polynomial long division done independently.
    #[test]
    fn barrett_constants_match_long_division() {
        assert_eq!(
            barrett_mu(0xad93d235_94c93659, 64),
            0xddf3eeb2_98be6fc8,
            "CRC-64/NVME"
        );
        assert_eq!(barrett_mu(0x04C11DB7, 32), 0x04d101df, "CRC-32/ISO-HDLC");
        assert_eq!(barrett_mu(0x8005, 16), 0xfffb, "CRC-16/ARC");
    }

    use crate::{checksum_with_params, get_calculator_params, CrcAlgorithm};

    const ALGORITHMS: &[CrcAlgorithm] = &[
        CrcAlgorithm::Crc32IsoHdlc,
        CrcAlgorithm::Crc32Iscsi,
        CrcAlgorithm::Crc32Bzip2,
        CrcAlgorithm::Crc64Nvme,
        CrcAlgorithm::Crc64Xz,
        CrcAlgorithm::Crc64Redis,
    ];

    /// The combine-key path must agree with the matrix reference, for every algorithm and a
    /// spread of lengths: powers of two, values either side of them, and lengths with many set
    /// bits.
    #[test]
    fn combine_keys_match_the_matrix_reference() {
        let lengths: Vec<u64> = (0..24)
            .flat_map(|i: u32| {
                let p = 1u64 << i;
                [p.saturating_sub(1), p, p + 1]
            })
            .chain([0, 5, 12345, 0xFFFF_FFFF, u32::MAX as u64 + 1])
            .collect();

        for &algorithm in ALGORITHMS {
            let params = get_calculator_params(algorithm).1;
            let crc1 = checksum_with_params(params, b"the first sequence");
            let crc2 = checksum_with_params(params, b"and the second one");

            for &len2 in &lengths {
                assert_eq!(
                    checksums(crc1, crc2, len2, &params),
                    checksums_via_matrix(crc1, crc2, len2, &params),
                    "{algorithm:?} disagreed at len2={len2}",
                );
            }
        }
    }

    /// Every bit of a `u64` length must select the right key, including the high bits no
    /// plausible payload reaches. A table indexed one entry short, or a loop that stops early,
    /// would only show up here.
    #[test]
    fn combine_keys_cover_every_length_bit() {
        for &algorithm in ALGORITHMS {
            let params = get_calculator_params(algorithm).1;
            let crc1 = checksum_with_params(params, b"the first sequence");
            let crc2 = checksum_with_params(params, b"and the second one");

            let lengths =
                (0..64)
                    .map(|i| 1u64 << i)
                    .chain([u64::MAX, u64::MAX - 1, (1u64 << 63) | 1]);

            for len2 in lengths {
                assert_eq!(
                    checksums(crc1, crc2, len2, &params),
                    checksums_via_matrix(crc1, crc2, len2, &params),
                    "{algorithm:?} disagreed at len2={len2:#x}",
                );
            }
        }
    }

    /// The combined CRC must equal a CRC over the concatenation. The reference agreeing is
    /// necessary but not sufficient: both paths derive from the same polynomial, so they could be
    /// wrong the same way.
    #[test]
    fn combine_keys_match_a_flat_checksum_of_the_concatenation() {
        let whole: Vec<u8> = (0..4096u32).map(|i| (i % 251) as u8).collect();

        for &algorithm in ALGORITHMS {
            let params = get_calculator_params(algorithm).1;

            for split in [0usize, 1, 7, 64, 255, 1024, 4096] {
                let (first, second) = whole.split_at(split);
                assert_eq!(
                    checksums(
                        checksum_with_params(params, first),
                        checksum_with_params(params, second),
                        second.len() as u64,
                        &params,
                    ),
                    checksum_with_params(params, &whole),
                    "{algorithm:?} disagreed splitting 4096 bytes at {split}",
                );
            }
        }
    }

    /// A custom polynomial has no const table and builds one per call. It must agree with the
    /// built-in algorithm that names the same polynomial, which is what shows the two ways of
    /// obtaining the keys produce the same ones.
    #[test]
    fn a_custom_polynomial_combines_like_its_built_in_twin() {
        let params = get_calculator_params(CrcAlgorithm::Crc64Nvme).1;
        let custom = crate::CrcParams::new(
            "custom",
            params.width,
            params.poly,
            params.init,
            params.refin,
            params.xorout,
            params.check,
        );
        assert!(
            custom.combine_keys.is_none(),
            "a custom polynomial carries no table"
        );

        for len2 in [0u64, 1, 4096, 1 << 20, 1_060_921] {
            assert_eq!(
                checksums(0xdead_beef, 0x9e37_79b9, len2, &custom),
                checksums(0xdead_beef, 0x9e37_79b9, len2, &params),
                "custom and built-in disagreed at len2={len2}",
            );
        }
    }

    /// A second combine on the same polynomial takes the cache-hit path. It must return what the
    /// first one did, which is what catches a table stored under the wrong key.
    #[test]
    fn combine_keys_are_stable_across_polynomials() {
        for _ in 0..3 {
            for &algorithm in ALGORITHMS {
                let params = get_calculator_params(algorithm).1;
                assert_eq!(
                    checksums(0xdead_beef, 0x9e37_79b9, 1 << 20, &params),
                    checksums_via_matrix(0xdead_beef, 0x9e37_79b9, 1 << 20, &params),
                    "{algorithm:?} disagreed on a repeat combine",
                );
            }
        }
    }
}
