//! Cached zero-byte operators for the reflected CRC64-NVMe polynomial.
//!
//! The 64 matrices occupy 32 KiB, initialize on the first nontrivial combine,
//! and are shared by checksum, Digest, and FFI callers. No heap allocation is
//! needed. no_std builds opt in with the existing `cache` feature.

use super::gf2_matrix_times;
use crate::crc64::consts::CRC64_NVME;

#[cfg(feature = "std")]
static SHIFTS: std::sync::LazyLock<[[u64; 64]; 64]> = std::sync::LazyLock::new(make_shifts);

#[cfg(not(feature = "std"))]
static SHIFTS: spin::Once<[[u64; 64]; 64]> = spin::Once::new();

// crc1 has already been adjusted for the CRC's initial state and final XOR.
pub(super) fn checksums(mut crc1: u64, crc2: u64, mut len2: u64) -> u64 {
    if crc1 == 0 || len2 == 0 {
        return crc1 ^ crc2;
    }

    #[cfg(feature = "std")]
    let shifts = &*SHIFTS;
    #[cfg(not(feature = "std"))]
    let shifts = SHIFTS.call_once(make_shifts);

    while len2 != 0 {
        crc1 = gf2_matrix_times(&shifts[len2.trailing_zeros() as usize], crc1);
        len2 &= len2 - 1;
    }
    crc1 ^ crc2
}

fn make_shifts() -> [[u64; 64]; 64] {
    // Each column is the raw CRC of one zero byte starting with a basis bit.
    let mut matrix = core::array::from_fn(|bit| {
        let mut crc = 1u64 << bit;
        for _ in 0..8 {
            crc = (crc >> 1) ^ ((crc & 1) * CRC64_NVME.poly.reverse_bits());
        }
        crc
    });

    // Squaring advances from 2^(n-1) zero bytes to 2^n zero bytes. All 64
    // operators are needed because the public combine API accepts u64 lengths.
    core::array::from_fn(|bit| {
        if bit != 0 {
            matrix = core::array::from_fn(|column| gf2_matrix_times(&matrix, matrix[column]));
        }
        matrix
    })
}
