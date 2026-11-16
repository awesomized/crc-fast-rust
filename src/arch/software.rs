// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module contains a software fallback for unsupported architectures.

use crate::CrcAlgorithm;
use crate::CrcParams;

// ============================================================================
// Native Table Generation Functions
// ============================================================================

/// Computes a single CRC-16 value for table generation.
const fn crc16_single(poly: u16, reflect: bool, mut value: u16) -> u16 {
    if reflect {
        let mut i = 0;
        while i < 8 {
            value = (value >> 1) ^ ((value & 1) * poly);
            i += 1;
        }
    } else {
        value <<= 8;
        let mut i = 0;
        while i < 8 {
            value = (value << 1) ^ (((value >> 15) & 1) * poly);
            i += 1;
        }
    }
    value
}

/// Computes a single CRC-32 value for table generation.
const fn crc32_single(poly: u32, reflect: bool, mut value: u32) -> u32 {
    if reflect {
        let mut i = 0;
        while i < 8 {
            value = (value >> 1) ^ ((value & 1) * poly);
            i += 1;
        }
    } else {
        value <<= 24;
        let mut i = 0;
        while i < 8 {
            value = (value << 1) ^ (((value >> 31) & 1) * poly);
            i += 1;
        }
    }
    value
}

/// Computes a single CRC-64 value for table generation.
const fn crc64_single(poly: u64, reflect: bool, mut value: u64) -> u64 {
    if reflect {
        let mut i = 0;
        while i < 8 {
            value = (value >> 1) ^ ((value & 1) * poly);
            i += 1;
        }
    } else {
        value <<= 56;
        let mut i = 0;
        while i < 8 {
            value = (value << 1) ^ (((value >> 63) & 1) * poly);
            i += 1;
        }
    }
    value
}

/// Fills a 16-lane lookup table for CRC-16 calculations in place.
///
/// This is the in-place form of [`generate_table_u16`], used both at compile time and, with the
/// `runtime-tables` feature, to build tables at first use without a large stack temporary.
pub(crate) const fn fill_table_u16(
    table: &mut [[u16; 256]; 16],
    width: u8,
    poly: u16,
    reflect: bool,
) {
    let poly = if reflect {
        let poly = poly.reverse_bits();
        poly >> (16u8 - width)
    } else {
        poly << (16u8 - width)
    };

    // Generate first table (lane 0) directly
    let mut i = 0;
    while i < 256 {
        table[0][i] = crc16_single(poly, reflect, i as u16);
        i += 1;
    }

    // Generate subsequent lanes based on lane 0
    let mut i = 0;
    while i < 256 {
        let mut e = 1;
        while e < 16 {
            let one_lower = table[e - 1][i];
            if reflect {
                table[e][i] = (one_lower >> 8) ^ table[0][(one_lower & 0xFF) as usize];
            } else {
                table[e][i] = (one_lower << 8) ^ table[0][((one_lower >> 8) & 0xFF) as usize];
            }
            e += 1;
        }
        i += 1;
    }
}

/// Generates a 16-lane lookup table for CRC-16 calculations.
///
/// This function creates a table compatible with the `crc` crate's `Table<16>` format,
/// enabling processing of 16 bytes at a time for improved performance.
pub const fn generate_table_u16(width: u8, poly: u16, reflect: bool) -> [[u16; 256]; 16] {
    let mut table = [[0u16; 256]; 16];
    fill_table_u16(&mut table, width, poly, reflect);
    table
}

/// Fills a 16-lane lookup table for CRC-32 calculations in place.
///
/// This is the in-place form of [`generate_table_u32`], used both at compile time and, with the
/// `runtime-tables` feature, to build tables at first use without a large stack temporary.
pub(crate) const fn fill_table_u32(
    table: &mut [[u32; 256]; 16],
    width: u8,
    poly: u32,
    reflect: bool,
) {
    let poly = if reflect {
        let poly = poly.reverse_bits();
        poly >> (32u8 - width)
    } else {
        poly << (32u8 - width)
    };

    // Generate first table (lane 0) directly
    let mut i = 0;
    while i < 256 {
        table[0][i] = crc32_single(poly, reflect, i as u32);
        i += 1;
    }

    // Generate subsequent lanes based on lane 0
    let mut i = 0;
    while i < 256 {
        let mut e = 1;
        while e < 16 {
            let one_lower = table[e - 1][i];
            if reflect {
                table[e][i] = (one_lower >> 8) ^ table[0][(one_lower & 0xFF) as usize];
            } else {
                table[e][i] = (one_lower << 8) ^ table[0][((one_lower >> 24) & 0xFF) as usize];
            }
            e += 1;
        }
        i += 1;
    }
}

/// Generates a 16-lane lookup table for CRC-32 calculations.
///
/// This function creates a table compatible with the `crc` crate's `Table<16>` format,
/// enabling processing of 16 bytes at a time for improved performance.
pub const fn generate_table_u32(width: u8, poly: u32, reflect: bool) -> [[u32; 256]; 16] {
    let mut table = [[0u32; 256]; 16];
    fill_table_u32(&mut table, width, poly, reflect);
    table
}

/// Fills a 16-lane lookup table for CRC-64 calculations in place.
///
/// This is the in-place form of [`generate_table_u64`], used both at compile time and, with the
/// `runtime-tables` feature, to build tables at first use without a large stack temporary.
pub(crate) const fn fill_table_u64(
    table: &mut [[u64; 256]; 16],
    width: u8,
    poly: u64,
    reflect: bool,
) {
    let poly = if reflect {
        let poly = poly.reverse_bits();
        poly >> (64u8 - width)
    } else {
        poly << (64u8 - width)
    };

    // Generate first table (lane 0) directly
    let mut i = 0;
    while i < 256 {
        table[0][i] = crc64_single(poly, reflect, i as u64);
        i += 1;
    }

    // Generate subsequent lanes based on lane 0
    let mut i = 0;
    while i < 256 {
        let mut e = 1;
        while e < 16 {
            let one_lower = table[e - 1][i];
            if reflect {
                table[e][i] = (one_lower >> 8) ^ table[0][(one_lower & 0xFF) as usize];
            } else {
                table[e][i] = (one_lower << 8) ^ table[0][((one_lower >> 56) & 0xFF) as usize];
            }
            e += 1;
        }
        i += 1;
    }
}

/// Generates a 16-lane lookup table for CRC-64 calculations.
///
/// This function creates a table compatible with the `crc` crate's `Table<16>` format,
/// enabling processing of 16 bytes at a time for improved performance.
pub const fn generate_table_u64(width: u8, poly: u64, reflect: bool) -> [[u64; 256]; 16] {
    let mut table = [[0u64; 256]; 16];
    fill_table_u64(&mut table, width, poly, reflect);
    table
}

// ============================================================================
// Caching for custom CRC algorithms
// ============================================================================

#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
use std::sync::{Mutex, OnceLock};

#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
use hashbrown::HashMap;
#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
use spin::{Mutex, Once};

// Cache key types for custom algorithms
#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type Crc16Key = (u16, u16, bool, bool, u16, u16);
#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type Crc32Key = (u32, u32, bool, bool, u32, u32);
#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type Crc64Key = (u64, u64, bool, bool, u64, u64);

// Cache value types for custom algorithms - stores the generated table
#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type Crc16CacheValue = &'static [[u16; 256]; 16];
#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type Crc32CacheValue = &'static [[u32; 256]; 16];
#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type Crc64CacheValue = &'static [[u64; 256]; 16];

// Global caches for custom algorithms (std version)
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
static CUSTOM_CRC16_CACHE: OnceLock<Mutex<HashMap<Crc16Key, Crc16CacheValue>>> = OnceLock::new();
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
static CUSTOM_CRC32_CACHE: OnceLock<Mutex<HashMap<Crc32Key, Crc32CacheValue>>> = OnceLock::new();
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
static CUSTOM_CRC64_CACHE: OnceLock<Mutex<HashMap<Crc64Key, Crc64CacheValue>>> = OnceLock::new();

// Global caches for custom algorithms (no_std + cache version)
#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
static CUSTOM_CRC16_CACHE: Once<Mutex<HashMap<Crc16Key, Crc16CacheValue>>> = Once::new();
#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
static CUSTOM_CRC32_CACHE: Once<Mutex<HashMap<Crc32Key, Crc32CacheValue>>> = Once::new();
#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
static CUSTOM_CRC64_CACHE: Once<Mutex<HashMap<Crc64Key, Crc64CacheValue>>> = Once::new();

// ============================================================================
// Lookup table access
// ============================================================================

/// Lazily generated lookup tables, used instead of the precomputed ones when the `runtime-tables`
/// feature is enabled.
///
/// Requires compare-and-swap support, so targets without it keep using the precomputed tables.
#[cfg(all(feature = "runtime-tables", target_has_atomic = "8"))]
mod lazy {
    use core::cell::UnsafeCell;
    use core::sync::atomic::{AtomicU8, Ordering};

    const EMPTY: u8 = 0;
    const BUSY: u8 = 1;
    const READY: u8 = 2;

    /// A 16-lane lookup table which is generated on first use.
    ///
    /// The table lives in zero-initialized static memory, so it adds nothing to the size of the
    /// compiled artifact. Initialization is guarded by a small spin lock: the first caller fills
    /// the table in place while concurrent callers wait, after which the table is immutable.
    pub(crate) struct LazyTable<T: 'static> {
        state: AtomicU8,
        table: UnsafeCell<[[T; 256]; 16]>,
    }

    // SAFETY: the table is only written by the single thread which moved `state` from EMPTY to
    // BUSY, and shared references are only handed out once `state` is READY, after which the table
    // is never written again.
    unsafe impl<T: Sync> Sync for LazyTable<T> {}

    macro_rules! lazy_table_new {
        ($($ty:ty),*) => {$(
            impl LazyTable<$ty> {
                pub(crate) const fn new() -> Self {
                    Self {
                        state: AtomicU8::new(EMPTY),
                        table: UnsafeCell::new([[0; 256]; 16]),
                    }
                }
            }
        )*};
    }

    lazy_table_new!(u16, u32, u64);

    impl<T> LazyTable<T> {
        /// Returns the table, generating it with `fill` if this is the first use.
        #[inline(always)]
        pub(crate) fn get_or_init(
            &'static self,
            fill: fn(&mut [[T; 256]; 16]),
        ) -> &'static [[T; 256]; 16] {
            if self.state.load(Ordering::Acquire) != READY {
                self.init(fill);
            }

            // SAFETY: `state` is READY, so the table is fully initialized and no longer mutated.
            unsafe { &*self.table.get() }
        }

        #[cold]
        #[inline(never)]
        fn init(&self, fill: fn(&mut [[T; 256]; 16])) {
            match self
                .state
                .compare_exchange(EMPTY, BUSY, Ordering::Acquire, Ordering::Acquire)
            {
                Ok(_) => {
                    // SAFETY: we hold the BUSY state, so we're the only thread with access.
                    fill(unsafe { &mut *self.table.get() });
                    self.state.store(READY, Ordering::Release);
                }
                Err(_) => {
                    // another thread is generating (or just finished generating) the table
                    while self.state.load(Ordering::Acquire) != READY {
                        core::hint::spin_loop();
                    }
                }
            }
        }
    }
}

/// Accessors for the lookup table of each predefined CRC algorithm.
///
/// Each algorithm gets its own accessor so that only the tables for the algorithms actually used
/// end up in the compiled artifact, rather than all of them.
///
/// A lookup table only depends on the width, polynomial, and reflection of an algorithm, so
/// algorithms which share those also share a single table.
#[allow(non_snake_case)]
pub(crate) mod table {
    macro_rules! define_tables {
        ($ty:ty, $fill:ident, $consts:ident:) => {};

        // an algorithm with its own lookup table
        ($ty:ty, $fill:ident, $consts:ident: $params:ident => $table:ident, $($rest:tt)*) => {
            #[inline(always)]
            pub(crate) fn $params() -> &'static [[$ty; 256]; 16] {
                #[cfg(not(all(feature = "runtime-tables", target_has_atomic = "8")))]
                {
                    &crate::tables::$consts::$table
                }

                #[cfg(all(feature = "runtime-tables", target_has_atomic = "8"))]
                {
                    use crate::$consts::consts::$params as PARAMS;

                    static TABLE: super::lazy::LazyTable<$ty> =
                        super::lazy::LazyTable::<$ty>::new();

                    TABLE.get_or_init(|table| {
                        super::$fill(table, PARAMS.width, PARAMS.poly as $ty, PARAMS.refin)
                    })
                }
            }

            define_tables!($ty, $fill, $consts: $($rest)*);
        };

        // an algorithm which shares the lookup table of another algorithm
        ($ty:ty, $fill:ident, $consts:ident: $params:ident => same as $other:ident, $($rest:tt)*) => {
            #[inline(always)]
            pub(crate) fn $params() -> &'static [[$ty; 256]; 16] {
                $other()
            }

            define_tables!($ty, $fill, $consts: $($rest)*);
        };
    }

    define_tables!(u16, fill_table_u16, crc16:
        CRC16_ARC => CRC16_ARC_TABLE,
        CRC16_CDMA2000 => CRC16_CDMA2000_TABLE,
        CRC16_CMS => CRC16_CMS_TABLE,
        CRC16_DDS_110 => same as CRC16_CMS,
        CRC16_DECT_R => CRC16_DECT_R_TABLE,
        CRC16_DECT_X => same as CRC16_DECT_R,
        CRC16_DNP => CRC16_DNP_TABLE,
        CRC16_EN_13757 => CRC16_EN_13757_TABLE,
        CRC16_GENIBUS => CRC16_GENIBUS_TABLE,
        CRC16_GSM => same as CRC16_GENIBUS,
        CRC16_IBM_3740 => same as CRC16_GENIBUS,
        CRC16_IBM_SDLC => CRC16_IBM_SDLC_TABLE,
        CRC16_ISO_IEC_14443_3_A => same as CRC16_IBM_SDLC,
        CRC16_KERMIT => same as CRC16_IBM_SDLC,
        CRC16_LJ1200 => CRC16_LJ1200_TABLE,
        CRC16_M17 => CRC16_M17_TABLE,
        CRC16_MAXIM_DOW => same as CRC16_ARC,
        CRC16_MCRF4XX => same as CRC16_IBM_SDLC,
        CRC16_MODBUS => same as CRC16_ARC,
        CRC16_NRSC_5 => CRC16_NRSC_5_TABLE,
        CRC16_OPENSAFETY_A => same as CRC16_M17,
        CRC16_OPENSAFETY_B => CRC16_OPENSAFETY_B_TABLE,
        CRC16_PROFIBUS => CRC16_PROFIBUS_TABLE,
        CRC16_RIELLO => same as CRC16_IBM_SDLC,
        CRC16_SPI_FUJITSU => same as CRC16_GENIBUS,
        CRC16_T10_DIF => CRC16_T10_DIF_TABLE,
        CRC16_TELEDISK => CRC16_TELEDISK_TABLE,
        CRC16_TMS37157 => same as CRC16_IBM_SDLC,
        CRC16_UMTS => same as CRC16_CMS,
        CRC16_USB => same as CRC16_ARC,
        CRC16_XMODEM => same as CRC16_GENIBUS,
    );

    define_tables!(u32, fill_table_u32, crc32:
        CRC32_AIXM => CRC32_AIXM_TABLE,
        CRC32_AUTOSAR => CRC32_AUTOSAR_TABLE,
        CRC32_BASE91_D => CRC32_BASE91_D_TABLE,
        CRC32_BZIP2 => CRC32_BZIP2_TABLE,
        CRC32_CD_ROM_EDC => CRC32_CD_ROM_EDC_TABLE,
        CRC32_CKSUM => same as CRC32_BZIP2,
        CRC32_ISCSI => CRC32_ISCSI_TABLE,
        CRC32_ISO_HDLC => CRC32_ISO_HDLC_TABLE,
        CRC32_JAMCRC => same as CRC32_ISO_HDLC,
        CRC32_MEF => CRC32_MEF_TABLE,
        CRC32_MPEG_2 => same as CRC32_BZIP2,
        CRC32_XFER => CRC32_XFER_TABLE,
    );

    define_tables!(u64, fill_table_u64, crc64:
        CRC64_ECMA_182 => CRC64_ECMA_182_TABLE,
        CRC64_GO_ISO => CRC64_GO_ISO_TABLE,
        CRC64_MS => CRC64_MS_TABLE,
        CRC64_NVME => CRC64_NVME_TABLE,
        CRC64_REDIS => CRC64_REDIS_TABLE,
        CRC64_WE => same as CRC64_ECMA_182,
        CRC64_XZ => CRC64_XZ_TABLE,
    );
}

/// Calculator functions dedicated to a single predefined CRC algorithm.
///
/// Unlike [`update`], which selects the lookup table at runtime and therefore references every
/// table, each of these only references its own table. Targets which always use the software
/// implementation (such as WebAssembly) dispatch to these so that unused tables can be dropped.
#[allow(dead_code)] // only dispatched to on targets without SIMD acceleration
pub(crate) mod calc {
    use super::{native_update_u16, native_update_u32, native_update_u64, table};
    use crate::CrcParams;

    macro_rules! define_calculators {
        ($ty:ty, $kernel:ident, $consts:ident, $all:ident: $($name:ident => $params:ident,)*) => {
            $(
                pub(crate) fn $name(state: u64, data: &[u8], _params: &CrcParams) -> u64 {
                    $kernel(
                        state as $ty,
                        table::$params(),
                        crate::$consts::consts::$params.refin,
                        data,
                    ) as u64
                }
            )*

            /// Every dedicated calculator of this width, with the parameters it implements.
            #[cfg(test)]
            pub(crate) const $all: &[(crate::CalculatorFn, CrcParams)] = &[
                $(($name, crate::$consts::consts::$params),)*
            ];
        };
    }

    define_calculators!(u16, native_update_u16, crc16, ALL_CRC16:
        crc16_arc => CRC16_ARC,
        crc16_cdma2000 => CRC16_CDMA2000,
        crc16_cms => CRC16_CMS,
        crc16_dds_110 => CRC16_DDS_110,
        crc16_dect_r => CRC16_DECT_R,
        crc16_dect_x => CRC16_DECT_X,
        crc16_dnp => CRC16_DNP,
        crc16_en_13757 => CRC16_EN_13757,
        crc16_genibus => CRC16_GENIBUS,
        crc16_gsm => CRC16_GSM,
        crc16_ibm_3740 => CRC16_IBM_3740,
        crc16_ibm_sdlc => CRC16_IBM_SDLC,
        crc16_iso_iec_14443_3_a => CRC16_ISO_IEC_14443_3_A,
        crc16_kermit => CRC16_KERMIT,
        crc16_lj1200 => CRC16_LJ1200,
        crc16_m17 => CRC16_M17,
        crc16_maxim_dow => CRC16_MAXIM_DOW,
        crc16_mcrf4xx => CRC16_MCRF4XX,
        crc16_modbus => CRC16_MODBUS,
        crc16_nrsc_5 => CRC16_NRSC_5,
        crc16_opensafety_a => CRC16_OPENSAFETY_A,
        crc16_opensafety_b => CRC16_OPENSAFETY_B,
        crc16_profibus => CRC16_PROFIBUS,
        crc16_riello => CRC16_RIELLO,
        crc16_spi_fujitsu => CRC16_SPI_FUJITSU,
        crc16_t10_dif => CRC16_T10_DIF,
        crc16_teledisk => CRC16_TELEDISK,
        crc16_tms37157 => CRC16_TMS37157,
        crc16_umts => CRC16_UMTS,
        crc16_usb => CRC16_USB,
        crc16_xmodem => CRC16_XMODEM,
    );

    define_calculators!(u32, native_update_u32, crc32, ALL_CRC32:
        crc32_aixm => CRC32_AIXM,
        crc32_autosar => CRC32_AUTOSAR,
        crc32_base91_d => CRC32_BASE91_D,
        crc32_bzip2 => CRC32_BZIP2,
        crc32_cd_rom_edc => CRC32_CD_ROM_EDC,
        crc32_cksum => CRC32_CKSUM,
        crc32_iscsi => CRC32_ISCSI,
        crc32_iso_hdlc => CRC32_ISO_HDLC,
        crc32_jamcrc => CRC32_JAMCRC,
        crc32_mef => CRC32_MEF,
        crc32_mpeg_2 => CRC32_MPEG_2,
        crc32_xfer => CRC32_XFER,
    );

    define_calculators!(u64, native_update_u64, crc64, ALL_CRC64:
        crc64_ecma_182 => CRC64_ECMA_182,
        crc64_go_iso => CRC64_GO_ISO,
        crc64_ms => CRC64_MS,
        crc64_nvme => CRC64_NVME,
        crc64_redis => CRC64_REDIS,
        crc64_we => CRC64_WE,
        crc64_xz => CRC64_XZ,
    );
}

// ============================================================================
// Main dispatch function
// ============================================================================

#[allow(unused)]
#[allow(deprecated)]
pub(crate) fn update(state: u64, data: &[u8], params: &CrcParams) -> u64 {
    match params.width {
        16 => update_crc16(state as u16, data, params) as u64,
        32 => update_crc32(state as u32, data, params) as u64,
        64 => update_crc64(state, data, params),
        _ => panic!("Unsupported CRC width: {}", params.width),
    }
}

// ============================================================================
// CRC-16 dispatch
// ============================================================================

fn update_crc16(state: u16, data: &[u8], params: &CrcParams) -> u16 {
    let (table, refin) = match params.algorithm {
        CrcAlgorithm::Crc16Arc => (table::CRC16_ARC(), true),
        CrcAlgorithm::Crc16Cdma2000 => (table::CRC16_CDMA2000(), false),
        CrcAlgorithm::Crc16Cms => (table::CRC16_CMS(), false),
        CrcAlgorithm::Crc16Dds110 => (table::CRC16_DDS_110(), false),
        CrcAlgorithm::Crc16DectR => (table::CRC16_DECT_R(), false),
        CrcAlgorithm::Crc16DectX => (table::CRC16_DECT_X(), false),
        CrcAlgorithm::Crc16Dnp => (table::CRC16_DNP(), true),
        CrcAlgorithm::Crc16En13757 => (table::CRC16_EN_13757(), false),
        CrcAlgorithm::Crc16Genibus => (table::CRC16_GENIBUS(), false),
        CrcAlgorithm::Crc16Gsm => (table::CRC16_GSM(), false),
        CrcAlgorithm::Crc16Ibm3740 => (table::CRC16_IBM_3740(), false),
        CrcAlgorithm::Crc16IbmSdlc => (table::CRC16_IBM_SDLC(), true),
        CrcAlgorithm::Crc16IsoIec144433A => (table::CRC16_ISO_IEC_14443_3_A(), true),
        CrcAlgorithm::Crc16Kermit => (table::CRC16_KERMIT(), true),
        CrcAlgorithm::Crc16Lj1200 => (table::CRC16_LJ1200(), false),
        CrcAlgorithm::Crc16M17 => (table::CRC16_M17(), false),
        CrcAlgorithm::Crc16MaximDow => (table::CRC16_MAXIM_DOW(), true),
        CrcAlgorithm::Crc16Mcrf4xx => (table::CRC16_MCRF4XX(), true),
        CrcAlgorithm::Crc16Modbus => (table::CRC16_MODBUS(), true),
        CrcAlgorithm::Crc16Nrsc5 => (table::CRC16_NRSC_5(), true),
        CrcAlgorithm::Crc16OpensafetyA => (table::CRC16_OPENSAFETY_A(), false),
        CrcAlgorithm::Crc16OpensafetyB => (table::CRC16_OPENSAFETY_B(), false),
        CrcAlgorithm::Crc16Profibus => (table::CRC16_PROFIBUS(), false),
        CrcAlgorithm::Crc16Riello => (table::CRC16_RIELLO(), true),
        CrcAlgorithm::Crc16SpiFujitsu => (table::CRC16_SPI_FUJITSU(), false),
        CrcAlgorithm::Crc16T10Dif => (table::CRC16_T10_DIF(), false),
        CrcAlgorithm::Crc16Teledisk => (table::CRC16_TELEDISK(), false),
        CrcAlgorithm::Crc16Tms37157 => (table::CRC16_TMS37157(), true),
        CrcAlgorithm::Crc16Umts => (table::CRC16_UMTS(), false),
        CrcAlgorithm::Crc16Usb => (table::CRC16_USB(), true),
        CrcAlgorithm::Crc16Xmodem => (table::CRC16_XMODEM(), false),
        CrcAlgorithm::CrcCustom => {
            return update_crc16_custom(state, data, params);
        }
        _ => panic!("Invalid algorithm for u16 CRC"),
    };

    native_update_u16(state, table, refin, data)
}

#[cfg(feature = "alloc")]
fn update_crc16_custom(state: u16, data: &[u8], params: &CrcParams) -> u16 {
    extern crate alloc;
    use alloc::boxed::Box;

    let refin = params.refin;

    #[cfg(any(feature = "std", feature = "cache"))]
    let table: &'static [[u16; 256]; 16] = {
        let key: Crc16Key = (
            params.poly as u16,
            params.init as u16,
            params.refin,
            params.refout,
            params.xorout as u16,
            params.check as u16,
        );

        #[cfg(feature = "std")]
        {
            let cache = CUSTOM_CRC16_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
            let mut cache_guard = cache.lock().unwrap();

            cache_guard.entry(key).or_insert_with(|| {
                let table = generate_table_u16(params.width, params.poly as u16, refin);
                Box::leak(Box::new(table))
            })
        }

        #[cfg(all(not(feature = "std"), feature = "cache"))]
        {
            let cache = CUSTOM_CRC16_CACHE.call_once(|| Mutex::new(HashMap::new()));
            let mut cache_guard = cache.lock();

            cache_guard.entry(key).or_insert_with(|| {
                let table = generate_table_u16(params.width, params.poly as u16, refin);
                Box::leak(Box::new(table))
            })
        }
    };

    #[cfg(not(any(feature = "std", feature = "cache")))]
    let table: &'static [[u16; 256]; 16] = {
        let table = generate_table_u16(params.width, params.poly as u16, refin);
        Box::leak(Box::new(table))
    };

    native_update_u16(state, table, refin, data)
}

#[cfg(not(feature = "alloc"))]
fn update_crc16_custom(_state: u16, _data: &[u8], _params: &CrcParams) -> u16 {
    panic!("Custom CRC parameters require the 'alloc' feature")
}

// ============================================================================
// CRC-32 dispatch
// ============================================================================

fn update_crc32(state: u32, data: &[u8], params: &CrcParams) -> u32 {
    let (table, refin) = match params.algorithm {
        CrcAlgorithm::Crc32Aixm => (table::CRC32_AIXM(), false),
        CrcAlgorithm::Crc32Autosar => (table::CRC32_AUTOSAR(), true),
        CrcAlgorithm::Crc32Base91D => (table::CRC32_BASE91_D(), true),
        CrcAlgorithm::Crc32Bzip2 => (table::CRC32_BZIP2(), false),
        CrcAlgorithm::Crc32CdRomEdc => (table::CRC32_CD_ROM_EDC(), true),
        CrcAlgorithm::Crc32Cksum => (table::CRC32_CKSUM(), false),
        CrcAlgorithm::Crc32Iscsi => (table::CRC32_ISCSI(), true),
        CrcAlgorithm::Crc32IsoHdlc => (table::CRC32_ISO_HDLC(), true),
        CrcAlgorithm::Crc32Jamcrc => (table::CRC32_JAMCRC(), true),
        CrcAlgorithm::Crc32Mef => (table::CRC32_MEF(), true),
        CrcAlgorithm::Crc32Mpeg2 => (table::CRC32_MPEG_2(), false),
        CrcAlgorithm::Crc32Xfer => (table::CRC32_XFER(), false),
        #[allow(deprecated)]
        CrcAlgorithm::Crc32Custom | CrcAlgorithm::CrcCustom => {
            return update_crc32_custom(state, data, params);
        }
        _ => panic!("Invalid algorithm for u32 CRC"),
    };

    native_update_u32(state, table, refin, data)
}

#[cfg(feature = "alloc")]
fn update_crc32_custom(state: u32, data: &[u8], params: &CrcParams) -> u32 {
    extern crate alloc;
    use alloc::boxed::Box;

    let refin = params.refin;

    #[cfg(any(feature = "std", feature = "cache"))]
    let table: &'static [[u32; 256]; 16] = {
        let key: Crc32Key = (
            params.poly as u32,
            params.init as u32,
            params.refin,
            params.refout,
            params.xorout as u32,
            params.check as u32,
        );

        #[cfg(feature = "std")]
        {
            let cache = CUSTOM_CRC32_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
            let mut cache_guard = cache.lock().unwrap();

            cache_guard.entry(key).or_insert_with(|| {
                let table = generate_table_u32(params.width, params.poly as u32, refin);
                Box::leak(Box::new(table))
            })
        }

        #[cfg(all(not(feature = "std"), feature = "cache"))]
        {
            let cache = CUSTOM_CRC32_CACHE.call_once(|| Mutex::new(HashMap::new()));
            let mut cache_guard = cache.lock();

            cache_guard.entry(key).or_insert_with(|| {
                let table = generate_table_u32(params.width, params.poly as u32, refin);
                Box::leak(Box::new(table))
            })
        }
    };

    #[cfg(not(any(feature = "std", feature = "cache")))]
    let table: &'static [[u32; 256]; 16] = {
        let table = generate_table_u32(params.width, params.poly as u32, refin);
        Box::leak(Box::new(table))
    };

    native_update_u32(state, table, refin, data)
}

#[cfg(not(feature = "alloc"))]
fn update_crc32_custom(_state: u32, _data: &[u8], _params: &CrcParams) -> u32 {
    panic!("Custom CRC parameters require the 'alloc' feature")
}

// ============================================================================
// CRC-64 dispatch
// ============================================================================

fn update_crc64(state: u64, data: &[u8], params: &CrcParams) -> u64 {
    let (table, refin) = match params.algorithm {
        CrcAlgorithm::Crc64Ecma182 => (table::CRC64_ECMA_182(), false),
        CrcAlgorithm::Crc64GoIso => (table::CRC64_GO_ISO(), true),
        CrcAlgorithm::Crc64Ms => (table::CRC64_MS(), true),
        CrcAlgorithm::Crc64Nvme => (table::CRC64_NVME(), true),
        CrcAlgorithm::Crc64Redis => (table::CRC64_REDIS(), true),
        CrcAlgorithm::Crc64We => (table::CRC64_WE(), false),
        CrcAlgorithm::Crc64Xz => (table::CRC64_XZ(), true),
        #[allow(deprecated)]
        CrcAlgorithm::Crc64Custom | CrcAlgorithm::CrcCustom => {
            return update_crc64_custom(state, data, params);
        }
        _ => panic!("Invalid algorithm for u64 CRC"),
    };

    native_update_u64(state, table, refin, data)
}

#[cfg(feature = "alloc")]
fn update_crc64_custom(state: u64, data: &[u8], params: &CrcParams) -> u64 {
    extern crate alloc;
    use alloc::boxed::Box;

    let refin = params.refin;

    #[cfg(any(feature = "std", feature = "cache"))]
    let table: &'static [[u64; 256]; 16] = {
        let key: Crc64Key = (
            params.poly,
            params.init,
            params.refin,
            params.refout,
            params.xorout,
            params.check,
        );

        #[cfg(feature = "std")]
        {
            let cache = CUSTOM_CRC64_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
            let mut cache_guard = cache.lock().unwrap();

            cache_guard.entry(key).or_insert_with(|| {
                let table = generate_table_u64(params.width, params.poly, refin);
                Box::leak(Box::new(table))
            })
        }

        #[cfg(all(not(feature = "std"), feature = "cache"))]
        {
            let cache = CUSTOM_CRC64_CACHE.call_once(|| Mutex::new(HashMap::new()));
            let mut cache_guard = cache.lock();

            cache_guard.entry(key).or_insert_with(|| {
                let table = generate_table_u64(params.width, params.poly, refin);
                Box::leak(Box::new(table))
            })
        }
    };

    #[cfg(not(any(feature = "std", feature = "cache")))]
    let table: &'static [[u64; 256]; 16] = {
        let table = generate_table_u64(params.width, params.poly, refin);
        Box::leak(Box::new(table))
    };

    native_update_u64(state, table, refin, data)
}

#[cfg(not(feature = "alloc"))]
fn update_crc64_custom(_state: u64, _data: &[u8], _params: &CrcParams) -> u64 {
    panic!("Custom CRC parameters require the 'alloc' feature")
}

// ============================================================================
// Native CRC Update Functions (Table<16> equivalent)
// ============================================================================

/// Defines a CRC update function using 16-lane lookup tables.
///
/// Processes 16 bytes at a time (slice-by-16), then handles the remaining bytes with single-byte
/// lookups.
///
/// Each 16-byte block needs one lookup per byte. Only the leading bytes of a block depend on the
/// current state, so the lookups are split in two groups:
///
/// - the lookups for the trailing 8 bytes, which don't depend on the state and can therefore be
///   computed ahead of time by the CPU, and
/// - the lookups for the leading 8 bytes, which are loaded as a single word, XORed with the state,
///   and combined as a balanced tree to keep the loop-carried dependency chain short.
///
/// Blocks are iterated through `chunks_exact`, which keeps the hot loop free of bounds checks.
/// That's both faster and considerably smaller, which matters for targets like WebAssembly.
macro_rules! define_native_update {
    ($(#[$doc:meta])* $name:ident, $ty:ty) => {
        $(#[$doc])*
        #[allow(dead_code)]
        pub(crate) fn $name(
            mut crc: $ty,
            table: &[[$ty; 256]; 16],
            reflect: bool,
            bytes: &[u8],
        ) -> $ty {
            // extracts the byte at bit offset `$shift` of `$word`, as a table index
            macro_rules! byte {
                ($word:expr, $shift:expr) => {
                    (($word >> $shift) as u8) as usize
                };
            }

            // the lookups for the trailing 8 bytes of a block, which don't depend on the state
            macro_rules! trailing {
                ($chunk:expr) => {
                    table[7][$chunk[8] as usize]
                        ^ table[6][$chunk[9] as usize]
                        ^ table[5][$chunk[10] as usize]
                        ^ table[4][$chunk[11] as usize]
                        ^ table[3][$chunk[12] as usize]
                        ^ table[2][$chunk[13] as usize]
                        ^ table[1][$chunk[14] as usize]
                        ^ table[0][$chunk[15] as usize]
                };
            }

            // loads the leading 8 bytes of a block
            macro_rules! leading {
                ($chunk:expr) => {
                    [
                        $chunk[0], $chunk[1], $chunk[2], $chunk[3], $chunk[4], $chunk[5],
                        $chunk[6], $chunk[7],
                    ]
                };
            }

            const TOP_BYTE: u32 = <$ty>::BITS - 8;

            let mut chunks = bytes.chunks_exact(16);

            if reflect {
                // Process 16 bytes at a time
                for chunk in &mut chunks {
                    // the state is XORed into the leading bytes of the block, starting with its
                    // least significant byte
                    let word = u64::from_le_bytes(leading!(chunk)) ^ (crc as u64);

                    crc = trailing!(chunk)
                        ^ ((table[15][byte!(word, 0)] ^ table[14][byte!(word, 8)])
                            ^ (table[13][byte!(word, 16)] ^ table[12][byte!(word, 24)]))
                        ^ ((table[11][byte!(word, 32)] ^ table[10][byte!(word, 40)])
                            ^ (table[9][byte!(word, 48)] ^ table[8][byte!(word, 56)]));
                }

                // Process remaining bytes one at a time
                for &byte in chunks.remainder() {
                    crc = table[0][(crc as u8 ^ byte) as usize] ^ (crc >> 8);
                }
            } else {
                // Process 16 bytes at a time
                for chunk in &mut chunks {
                    // the state is XORed into the leading bytes of the block, starting with its
                    // most significant byte
                    let word = u64::from_be_bytes(leading!(chunk))
                        ^ ((crc as u64) << (u64::BITS - <$ty>::BITS));

                    crc = trailing!(chunk)
                        ^ ((table[15][byte!(word, 56)] ^ table[14][byte!(word, 48)])
                            ^ (table[13][byte!(word, 40)] ^ table[12][byte!(word, 32)]))
                        ^ ((table[11][byte!(word, 24)] ^ table[10][byte!(word, 16)])
                            ^ (table[9][byte!(word, 8)] ^ table[8][byte!(word, 0)]));
                }

                // Process remaining bytes one at a time
                for &byte in chunks.remainder() {
                    crc = table[0][((crc >> TOP_BYTE) as u8 ^ byte) as usize] ^ (crc << 8);
                }
            }

            crc
        }
    };
}

define_native_update!(
    /// Native CRC-16 update function using 16-lane lookup tables.
    native_update_u16,
    u16
);

define_native_update!(
    /// Native CRC-32 update function using 16-lane lookup tables.
    native_update_u32,
    u32
);

define_native_update!(
    /// Native CRC-64 update function using 16-lane lookup tables.
    native_update_u64,
    u64
);

// ============================================================================
// Tests which exercise the software implementation directly
// ============================================================================

/// Unlike the tests which go through the public API, and therefore only reach this module on
/// targets without SIMD acceleration, these call the software implementation directly so that
/// it's covered on every target.
#[cfg(test)]
mod direct_tests {
    use super::*;
    use crate::test::consts::TEST_ALL_CONFIGS;
    use crate::test::enums::AnyCrcTestConfig;
    use rand::{rng, Rng};

    /// Lengths which cover the single-byte tail, the 16-byte fast path, and both combined.
    fn test_lengths() -> impl Iterator<Item = usize> {
        (0..=70).chain([127, 128, 129, 255, 256, 257, 1000, 1023, 1024, 1025, 4099])
    }

    fn random_data(len: usize) -> Vec<u8> {
        let mut data = vec![0u8; len];
        rng().fill(&mut data[..]);
        data
    }

    fn reference_config(params: &CrcParams) -> &'static AnyCrcTestConfig {
        TEST_ALL_CONFIGS
            .iter()
            .find(|config| config.get_algorithm() == params.algorithm)
            .unwrap_or_else(|| panic!("no test config for {}", params.name))
    }

    #[test]
    #[cfg_attr(miri, ignore)] // table-driven, no unsafe; covered by the faster tests below
    fn test_generic_update_matches_reference() {
        for config in TEST_ALL_CONFIGS {
            for len in test_lengths() {
                let data = random_data(len);

                let actual = update(config.get_init_algorithm(), &data, config.get_params())
                    ^ config.get_xorout();

                assert_eq!(
                    actual,
                    config.checksum_with_reference(&data),
                    "generic software mismatch for {} with length {}",
                    config.get_name(),
                    len
                );
            }
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)] // table-driven, no unsafe; covered by the faster tests below
    fn test_dedicated_calculators_match_reference() {
        let calculators = calc::ALL_CRC16
            .iter()
            .chain(calc::ALL_CRC32)
            .chain(calc::ALL_CRC64);

        let mut count = 0;

        for (calculator, params) in calculators {
            let config = reference_config(params);

            for len in test_lengths() {
                let data = random_data(len);

                let actual = calculator(params.init_algorithm, &data, params) ^ params.xorout;

                assert_eq!(
                    actual,
                    config.checksum_with_reference(&data),
                    "dedicated software mismatch for {} with length {}",
                    params.name,
                    len
                );
            }

            count += 1;
        }

        // every predefined algorithm must have a dedicated calculator
        assert_eq!(count, TEST_ALL_CONFIGS.len());
    }

    #[test]
    fn test_check_values() {
        let calculators = calc::ALL_CRC16
            .iter()
            .chain(calc::ALL_CRC32)
            .chain(calc::ALL_CRC64);

        for (calculator, params) in calculators {
            let data = crate::test::consts::TEST_CHECK_STRING;

            let dedicated = calculator(params.init_algorithm, data, params) ^ params.xorout;
            let generic = update(params.init_algorithm, data, params) ^ params.xorout;

            assert_eq!(dedicated, params.check, "dedicated, {}", params.name);
            assert_eq!(generic, params.check, "generic, {}", params.name);
        }
    }

    /// Streaming in arbitrary pieces must match a single pass, since the 16-byte fast path and the
    /// single-byte tail carry the state between them differently.
    #[test]
    fn test_split_updates_match_single_pass() {
        let data = random_data(257);

        for config in TEST_ALL_CONFIGS {
            let params = config.get_params();
            let expected = update(params.init_algorithm, &data, params);

            for split in [0, 1, 7, 15, 16, 17, 31, 32, 33, 128, 255, 256, 257] {
                let (left, right) = data.split_at(split);
                let state = update(params.init_algorithm, left, params);

                assert_eq!(
                    update(state, right, params),
                    expected,
                    "split mismatch for {} at {}",
                    params.name,
                    split
                );
            }
        }
    }

    /// The first use of a lazily generated table from many threads at once must hand every thread
    /// the complete table.
    #[test]
    #[cfg(all(feature = "runtime-tables", target_has_atomic = "8"))]
    fn test_lazy_table_concurrent_first_use() {
        use crate::crc64::consts::CRC64_NVME;
        use std::sync::{Arc, Barrier};

        static TABLE: lazy::LazyTable<u64> = lazy::LazyTable::<u64>::new();

        let expected = Arc::new(generate_table_u64(
            CRC64_NVME.width,
            CRC64_NVME.poly,
            CRC64_NVME.refin,
        ));

        let threads = 8;
        let barrier = Arc::new(Barrier::new(threads));

        let handles: Vec<_> = (0..threads)
            .map(|_| {
                let barrier = Arc::clone(&barrier);
                let expected = Arc::clone(&expected);

                std::thread::spawn(move || {
                    barrier.wait();

                    let table = TABLE.get_or_init(|table| {
                        fill_table_u64(table, CRC64_NVME.width, CRC64_NVME.poly, CRC64_NVME.refin)
                    });

                    assert!(table == &*expected);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

// ============================================================================
// Property Tests for Native Implementation
// ============================================================================

#[cfg(test)]
mod property_tests {
    use crate::test::consts::{
        RUST_CRC16_ARC, RUST_CRC16_IBM_SDLC, RUST_CRC16_T10_DIF, RUST_CRC32_BZIP2,
        RUST_CRC32_ISCSI, RUST_CRC32_ISO_HDLC, RUST_CRC64_ECMA_182, RUST_CRC64_NVME, RUST_CRC64_XZ,
        TEST_CHECK_STRING,
    };
    use crate::test::miri_compatible_proptest_config;
    use crate::{checksum, checksum_with_params, CrcAlgorithm, CrcParams, Digest};
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(miri_compatible_proptest_config())]

        /// Feature: remove-crc-runtime-dependency, Property 1: Native Implementation Correctness
        /// *For any* supported CRC algorithm and *for any* byte sequence, the native implementation
        /// SHALL produce the same checksum as the `crc` crate reference implementation.
        /// **Validates: Requirements 2.2, 2.3, 4.2, 4.3, 6.2, 6.5**
        #[test]
        fn prop_native_crc16_reflected_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            // Test CRC-16/IBM-SDLC (reflected)
            let our_result = checksum(CrcAlgorithm::Crc16IbmSdlc, &data);
            let reference_result = RUST_CRC16_IBM_SDLC.checksum(&data) as u64;
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-16/IBM-SDLC native mismatch for {} bytes: our=0x{:04X}, ref=0x{:04X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: remove-crc-runtime-dependency, Property 1: Native Implementation Correctness
        /// *For any* supported CRC algorithm and *for any* byte sequence, the native implementation
        /// SHALL produce the same checksum as the `crc` crate reference implementation.
        /// **Validates: Requirements 2.2, 2.3, 4.2, 4.3, 6.2, 6.5**
        #[test]
        fn prop_native_crc16_forward_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            // Test CRC-16/T10-DIF (forward/non-reflected)
            let our_result = checksum(CrcAlgorithm::Crc16T10Dif, &data);
            let reference_result = RUST_CRC16_T10_DIF.checksum(&data) as u64;
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-16/T10-DIF native mismatch for {} bytes: our=0x{:04X}, ref=0x{:04X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: remove-crc-runtime-dependency, Property 1: Native Implementation Correctness
        /// *For any* supported CRC algorithm and *for any* byte sequence, the native implementation
        /// SHALL produce the same checksum as the `crc` crate reference implementation.
        /// **Validates: Requirements 2.2, 2.3, 4.2, 4.3, 6.2, 6.5**
        #[test]
        fn prop_native_crc32_reflected_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            // Test CRC-32/ISO-HDLC (reflected)
            let our_result = checksum(CrcAlgorithm::Crc32IsoHdlc, &data);
            let reference_result = RUST_CRC32_ISO_HDLC.checksum(&data) as u64;
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-32/ISO-HDLC native mismatch for {} bytes: our=0x{:08X}, ref=0x{:08X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: remove-crc-runtime-dependency, Property 1: Native Implementation Correctness
        /// *For any* supported CRC algorithm and *for any* byte sequence, the native implementation
        /// SHALL produce the same checksum as the `crc` crate reference implementation.
        /// **Validates: Requirements 2.2, 2.3, 4.2, 4.3, 6.2, 6.5**
        #[test]
        fn prop_native_crc32_forward_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            // Test CRC-32/BZIP2 (forward/non-reflected)
            let our_result = checksum(CrcAlgorithm::Crc32Bzip2, &data);
            let reference_result = RUST_CRC32_BZIP2.checksum(&data) as u64;
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-32/BZIP2 native mismatch for {} bytes: our=0x{:08X}, ref=0x{:08X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: remove-crc-runtime-dependency, Property 1: Native Implementation Correctness
        /// *For any* supported CRC algorithm and *for any* byte sequence, the native implementation
        /// SHALL produce the same checksum as the `crc` crate reference implementation.
        /// **Validates: Requirements 2.2, 2.3, 4.2, 4.3, 6.2, 6.5**
        #[test]
        fn prop_native_crc64_reflected_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            // Test CRC-64/XZ (reflected)
            let our_result = checksum(CrcAlgorithm::Crc64Xz, &data);
            let reference_result = RUST_CRC64_XZ.checksum(&data);
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-64/XZ native mismatch for {} bytes: our=0x{:016X}, ref=0x{:016X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: remove-crc-runtime-dependency, Property 1: Native Implementation Correctness
        /// *For any* supported CRC algorithm and *for any* byte sequence, the native implementation
        /// SHALL produce the same checksum as the `crc` crate reference implementation.
        /// **Validates: Requirements 2.2, 2.3, 4.2, 4.3, 6.2, 6.5**
        #[test]
        fn prop_native_crc64_forward_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            // Test CRC-64/ECMA-182 (forward/non-reflected)
            let our_result = checksum(CrcAlgorithm::Crc64Ecma182, &data);
            let reference_result = RUST_CRC64_ECMA_182.checksum(&data);
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-64/ECMA-182 native mismatch for {} bytes: our=0x{:016X}, ref=0x{:016X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: remove-crc-runtime-dependency, Property 3: Incremental Update Equivalence
        /// *For any* CRC algorithm and *for any* way of partitioning input data into chunks,
        /// computing the CRC incrementally via `Digest::update()` calls SHALL produce the same
        /// result as computing it in a single `checksum()` call.
        /// **Validates: Requirements 6.3, 6.5**
        #[test]
        fn prop_incremental_update_crc32_equivalence(
            data in proptest::collection::vec(any::<u8>(), 0..1024),
            split_point in 0usize..=1024usize
        ) {
            let split_point = split_point.min(data.len());
            let (part1, part2) = data.split_at(split_point);

            // Compute incrementally
            let mut digest = Digest::new(CrcAlgorithm::Crc32IsoHdlc);
            digest.update(part1);
            digest.update(part2);
            let incremental_result = digest.finalize();

            // Compute in one shot
            let single_result = checksum(CrcAlgorithm::Crc32IsoHdlc, &data);

            prop_assert_eq!(
                incremental_result, single_result,
                "CRC-32/ISO-HDLC incremental mismatch: incremental=0x{:08X}, single=0x{:08X}, split_point={}",
                incremental_result, single_result, split_point
            );
        }

        /// Feature: remove-crc-runtime-dependency, Property 3: Incremental Update Equivalence
        /// *For any* CRC algorithm and *for any* way of partitioning input data into chunks,
        /// computing the CRC incrementally via `Digest::update()` calls SHALL produce the same
        /// result as computing it in a single `checksum()` call.
        /// **Validates: Requirements 6.3, 6.5**
        #[test]
        fn prop_incremental_update_crc64_equivalence(
            data in proptest::collection::vec(any::<u8>(), 0..1024),
            split_point in 0usize..=1024usize
        ) {
            let split_point = split_point.min(data.len());
            let (part1, part2) = data.split_at(split_point);

            // Compute incrementally
            let mut digest = Digest::new(CrcAlgorithm::Crc64Nvme);
            digest.update(part1);
            digest.update(part2);
            let incremental_result = digest.finalize();

            // Compute in one shot
            let single_result = checksum(CrcAlgorithm::Crc64Nvme, &data);

            prop_assert_eq!(
                incremental_result, single_result,
                "CRC-64/NVME incremental mismatch: incremental=0x{:016X}, single=0x{:016X}, split_point={}",
                incremental_result, single_result, split_point
            );
        }

        /// Feature: remove-crc-runtime-dependency, Property 4: Custom Parameters Correctness
        /// *For any* valid custom CRC parameters (width 16, 32, or 64) and *for any* byte sequence,
        /// the native implementation SHALL produce the same checksum as the `crc` crate when
        /// configured with equivalent parameters.
        /// **Validates: Requirements 5.1, 6.4**
        #[test]
        fn prop_custom_params_crc16_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            // Custom CRC-16 parameters equivalent to CRC-16/ARC
            let custom_params = CrcParams::new(
                "CRC-16/CUSTOM-ARC",
                16,
                0x8005,  // poly
                0x0000,  // init
                true,    // refin
                0x0000,  // xorout
                0xBB3D,  // check
            );
            let our_result = checksum_with_params(custom_params, &data);
            let reference_result = RUST_CRC16_ARC.checksum(&data) as u64;
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-16 custom params mismatch for {} bytes: our=0x{:04X}, ref=0x{:04X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: remove-crc-runtime-dependency, Property 4: Custom Parameters Correctness
        /// *For any* valid custom CRC parameters (width 16, 32, or 64) and *for any* byte sequence,
        /// the native implementation SHALL produce the same checksum as the `crc` crate when
        /// configured with equivalent parameters.
        /// **Validates: Requirements 5.1, 6.4**
        #[test]
        fn prop_custom_params_crc32_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            // Custom CRC-32 parameters equivalent to CRC-32/ISCSI
            let custom_params = CrcParams::new(
                "CRC-32/CUSTOM-ISCSI",
                32,
                0x1EDC6F41,  // poly
                0xFFFFFFFF,  // init
                true,        // refin
                0xFFFFFFFF,  // xorout
                0xE3069283,  // check
            );
            let our_result = checksum_with_params(custom_params, &data);
            let reference_result = RUST_CRC32_ISCSI.checksum(&data) as u64;
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-32 custom params mismatch for {} bytes: our=0x{:08X}, ref=0x{:08X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: remove-crc-runtime-dependency, Property 4: Custom Parameters Correctness
        /// *For any* valid custom CRC parameters (width 16, 32, or 64) and *for any* byte sequence,
        /// the native implementation SHALL produce the same checksum as the `crc` crate when
        /// configured with equivalent parameters.
        /// **Validates: Requirements 5.1, 6.4**
        #[test]
        fn prop_custom_params_crc64_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            // Custom CRC-64 parameters equivalent to CRC-64/NVME
            let custom_params = CrcParams::new(
                "CRC-64/CUSTOM-NVME",
                64,
                0xAD93D23594C93659,  // poly
                0xFFFFFFFFFFFFFFFF,  // init
                true,                 // refin
                0xFFFFFFFFFFFFFFFF,  // xorout
                0xAE8B14860A799888,  // check
            );
            let our_result = checksum_with_params(custom_params, &data);
            let reference_result = RUST_CRC64_NVME.checksum(&data);
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-64 custom params mismatch for {} bytes: our=0x{:016X}, ref=0x{:016X}",
                data.len(), our_result, reference_result
            );
        }
    }

    /// Feature: remove-crc-runtime-dependency, Property 2: Check Value Verification
    /// *For any* known CRC algorithm constant, computing the CRC of the byte sequence
    /// `b"123456789"` SHALL produce the documented check value from the CRC catalogue specification.
    /// **Validates: Requirements 3.2, 3.3, 3.4, 3.5**
    #[test]
    fn test_check_values_crc16() {
        use crate::crc16::consts::*;

        let test_cases: &[(&str, CrcParams, u64)] = &[
            ("CRC-16/ARC", CRC16_ARC, 0xBB3D),
            ("CRC-16/CDMA2000", CRC16_CDMA2000, 0x4C06),
            ("CRC-16/CMS", CRC16_CMS, 0xAEE7),
            ("CRC-16/DDS-110", CRC16_DDS_110, 0x9ECF),
            ("CRC-16/DECT-R", CRC16_DECT_R, 0x007E),
            ("CRC-16/DECT-X", CRC16_DECT_X, 0x007F),
            ("CRC-16/DNP", CRC16_DNP, 0xEA82),
            ("CRC-16/EN-13757", CRC16_EN_13757, 0xC2B7),
            ("CRC-16/GENIBUS", CRC16_GENIBUS, 0xD64E),
            ("CRC-16/GSM", CRC16_GSM, 0xCE3C),
            ("CRC-16/IBM-3740", CRC16_IBM_3740, 0x29B1),
            ("CRC-16/IBM-SDLC", CRC16_IBM_SDLC, 0x906E),
            ("CRC-16/ISO-IEC-14443-3-A", CRC16_ISO_IEC_14443_3_A, 0xBF05),
            ("CRC-16/KERMIT", CRC16_KERMIT, 0x2189),
            ("CRC-16/LJ1200", CRC16_LJ1200, 0xBDF4),
            ("CRC-16/M17", CRC16_M17, 0x772B),
            ("CRC-16/MAXIM-DOW", CRC16_MAXIM_DOW, 0x44C2),
            ("CRC-16/MCRF4XX", CRC16_MCRF4XX, 0x6F91),
            ("CRC-16/MODBUS", CRC16_MODBUS, 0x4B37),
            ("CRC-16/NRSC-5", CRC16_NRSC_5, 0xA066),
            ("CRC-16/OPENSAFETY-A", CRC16_OPENSAFETY_A, 0x5D38),
            ("CRC-16/OPENSAFETY-B", CRC16_OPENSAFETY_B, 0x20FE),
            ("CRC-16/PROFIBUS", CRC16_PROFIBUS, 0xA819),
            ("CRC-16/RIELLO", CRC16_RIELLO, 0x63D0),
            ("CRC-16/SPI-FUJITSU", CRC16_SPI_FUJITSU, 0xE5CC),
            ("CRC-16/T10-DIF", CRC16_T10_DIF, 0xD0DB),
            ("CRC-16/TELEDISK", CRC16_TELEDISK, 0x0FB3),
            ("CRC-16/TMS37157", CRC16_TMS37157, 0x26B1),
            ("CRC-16/UMTS", CRC16_UMTS, 0xFEE8),
            ("CRC-16/USB", CRC16_USB, 0xB4C8),
            ("CRC-16/XMODEM", CRC16_XMODEM, 0x31C3),
        ];

        for (name, params, expected_check) in test_cases {
            let result = checksum_with_params(*params, TEST_CHECK_STRING);
            assert_eq!(
                result, *expected_check,
                "{} check value mismatch: got 0x{:04X}, expected 0x{:04X}",
                name, result, expected_check
            );
        }
    }

    /// Feature: remove-crc-runtime-dependency, Property 2: Check Value Verification
    /// *For any* known CRC algorithm constant, computing the CRC of the byte sequence
    /// `b"123456789"` SHALL produce the documented check value from the CRC catalogue specification.
    /// **Validates: Requirements 3.2, 3.3, 3.4, 3.5**
    #[test]
    fn test_check_values_crc32() {
        use crate::crc32::consts::*;

        let test_cases: &[(&str, CrcParams, u64)] = &[
            ("CRC-32/AIXM", CRC32_AIXM, 0x3010BF7F),
            ("CRC-32/AUTOSAR", CRC32_AUTOSAR, 0x1697D06A),
            ("CRC-32/BASE91-D", CRC32_BASE91_D, 0x87315576),
            ("CRC-32/BZIP2", CRC32_BZIP2, 0xFC891918),
            ("CRC-32/CD-ROM-EDC", CRC32_CD_ROM_EDC, 0x6EC2EDC4),
            ("CRC-32/CKSUM", CRC32_CKSUM, 0x765E7680),
            ("CRC-32/ISCSI", CRC32_ISCSI, 0xE3069283),
            ("CRC-32/ISO-HDLC", CRC32_ISO_HDLC, 0xCBF43926),
            ("CRC-32/JAMCRC", CRC32_JAMCRC, 0x340BC6D9),
            ("CRC-32/MEF", CRC32_MEF, 0xD2C22F51),
            ("CRC-32/MPEG-2", CRC32_MPEG_2, 0x0376E6E7),
            ("CRC-32/XFER", CRC32_XFER, 0xBD0BE338),
        ];

        for (name, params, expected_check) in test_cases {
            let result = checksum_with_params(*params, TEST_CHECK_STRING);
            assert_eq!(
                result, *expected_check,
                "{} check value mismatch: got 0x{:08X}, expected 0x{:08X}",
                name, result, expected_check
            );
        }
    }

    /// Feature: remove-crc-runtime-dependency, Property 2: Check Value Verification
    /// *For any* known CRC algorithm constant, computing the CRC of the byte sequence
    /// `b"123456789"` SHALL produce the documented check value from the CRC catalogue specification.
    /// **Validates: Requirements 3.2, 3.3, 3.4, 3.5**
    #[test]
    fn test_check_values_crc64() {
        use crate::crc64::consts::*;

        let test_cases: &[(&str, CrcParams, u64)] = &[
            ("CRC-64/ECMA-182", CRC64_ECMA_182, 0x6C40DF5F0B497347),
            ("CRC-64/GO-ISO", CRC64_GO_ISO, 0xB90956C775A41001),
            ("CRC-64/MS", CRC64_MS, 0x75D4B74F024ECEEA),
            ("CRC-64/NVME", CRC64_NVME, 0xAE8B14860A799888),
            ("CRC-64/REDIS", CRC64_REDIS, 0xE9C6D914C4B8D9CA),
            ("CRC-64/WE", CRC64_WE, 0x62EC59E3F1A4F00A),
            ("CRC-64/XZ", CRC64_XZ, 0x995DC9BBDF1939FA),
        ];

        for (name, params, expected_check) in test_cases {
            let result = checksum_with_params(*params, TEST_CHECK_STRING);
            assert_eq!(
                result, *expected_check,
                "{} check value mismatch: got 0x{:016X}, expected 0x{:016X}",
                name, result, expected_check
            );
        }
    }
}
