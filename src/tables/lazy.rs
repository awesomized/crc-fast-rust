use std::hint::black_box;
use std::sync::LazyLock;

use crate::arch::software::{generate_table_u16, generate_table_u32, generate_table_u64};

macro_rules! define_tables {
    ($module:ident, $word:ty, $generate:ident;
     $($name:ident => $algorithm:path),+;
     $($alias:ident => $target:ident),* $(,)?) => {
        $(
            pub static $name: LazyLock<Box<[[$word; 256]; 16]>> = LazyLock::new(|| {
                let algorithm = $algorithm;
                // Keep optimization from replacing runtime generation with embedded table bytes.
                Box::new($generate(algorithm.width, black_box(algorithm.poly), algorithm.refin))
            });
        )+
        $(pub use self::$target as $alias;)*

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn generated_tables_match_prebuilt() {
                $(assert_eq!(
                    LazyLock::force(&$name).as_ref(),
                    &crate::tables::prebuilt::$module::$name,
                    stringify!($name),
                );)+
                $(assert_eq!(
                    LazyLock::force(&$alias).as_ref(),
                    &crate::tables::prebuilt::$module::$alias,
                    stringify!($alias),
                );)*
            }
        }
    };
}

pub mod crc16 {
    use super::*;
    use crate::crc16::consts as algorithms;

    define_tables!(crc16, u16, generate_table_u16;
        CRC16_ARC_TABLE => algorithms::CRC_16_ARC,
        CRC16_CDMA2000_TABLE => algorithms::CRC_16_CDMA2000,
        CRC16_CMS_TABLE => algorithms::CRC_16_CMS,
        CRC16_DECT_R_TABLE => algorithms::CRC_16_DECT_R,
        CRC16_DNP_TABLE => algorithms::CRC_16_DNP,
        CRC16_EN_13757_TABLE => algorithms::CRC_16_EN_13757,
        CRC16_GENIBUS_TABLE => algorithms::CRC_16_GENIBUS,
        CRC16_IBM_SDLC_TABLE => algorithms::CRC_16_IBM_SDLC,
        CRC16_LJ1200_TABLE => algorithms::CRC_16_LJ1200,
        CRC16_M17_TABLE => algorithms::CRC_16_M17,
        CRC16_NRSC_5_TABLE => algorithms::CRC_16_NRSC_5,
        CRC16_OPENSAFETY_B_TABLE => algorithms::CRC_16_OPENSAFETY_B,
        CRC16_PROFIBUS_TABLE => algorithms::CRC_16_PROFIBUS,
        CRC16_T10_DIF_TABLE => algorithms::CRC_16_T10_DIF,
        CRC16_TELEDISK_TABLE => algorithms::CRC_16_TELEDISK;
        CRC16_DDS_110_TABLE => CRC16_CMS_TABLE,
        CRC16_DECT_X_TABLE => CRC16_DECT_R_TABLE,
        CRC16_GSM_TABLE => CRC16_GENIBUS_TABLE,
        CRC16_IBM_3740_TABLE => CRC16_GENIBUS_TABLE,
        CRC16_ISO_IEC_14443_3_A_TABLE => CRC16_IBM_SDLC_TABLE,
        CRC16_KERMIT_TABLE => CRC16_IBM_SDLC_TABLE,
        CRC16_MAXIM_DOW_TABLE => CRC16_ARC_TABLE,
        CRC16_MCRF4XX_TABLE => CRC16_IBM_SDLC_TABLE,
        CRC16_MODBUS_TABLE => CRC16_ARC_TABLE,
        CRC16_OPENSAFETY_A_TABLE => CRC16_M17_TABLE,
        CRC16_RIELLO_TABLE => CRC16_IBM_SDLC_TABLE,
        CRC16_SPI_FUJITSU_TABLE => CRC16_GENIBUS_TABLE,
        CRC16_TMS37157_TABLE => CRC16_IBM_SDLC_TABLE,
        CRC16_UMTS_TABLE => CRC16_CMS_TABLE,
        CRC16_USB_TABLE => CRC16_ARC_TABLE,
        CRC16_XMODEM_TABLE => CRC16_GENIBUS_TABLE,
    );
}

pub mod crc32 {
    use super::*;
    use crate::crc32::consts as algorithms;

    define_tables!(crc32, u32, generate_table_u32;
        CRC32_AIXM_TABLE => algorithms::CRC_32_AIXM,
        CRC32_AUTOSAR_TABLE => algorithms::CRC_32_AUTOSAR,
        CRC32_BASE91_D_TABLE => algorithms::CRC_32_BASE91_D,
        CRC32_BZIP2_TABLE => algorithms::CRC_32_BZIP2,
        CRC32_CD_ROM_EDC_TABLE => algorithms::CRC_32_CD_ROM_EDC,
        CRC32_ISCSI_TABLE => algorithms::CRC_32_ISCSI,
        CRC32_ISO_HDLC_TABLE => algorithms::CRC_32_ISO_HDLC,
        CRC32_MEF_TABLE => algorithms::CRC_32_MEF,
        CRC32_XFER_TABLE => algorithms::CRC_32_XFER;
        CRC32_CKSUM_TABLE => CRC32_BZIP2_TABLE,
        CRC32_JAMCRC_TABLE => CRC32_ISO_HDLC_TABLE,
        CRC32_MPEG_2_TABLE => CRC32_BZIP2_TABLE,
    );
}

pub mod crc64 {
    use super::*;
    use crate::crc64::consts as algorithms;

    define_tables!(crc64, u64, generate_table_u64;
        CRC64_ECMA_182_TABLE => algorithms::CRC_64_ECMA_182,
        CRC64_GO_ISO_TABLE => algorithms::CRC_64_GO_ISO,
        CRC64_MS_TABLE => algorithms::CRC_64_MS,
        CRC64_NVME_TABLE => algorithms::CRC_64_NVME,
        CRC64_REDIS_TABLE => algorithms::CRC_64_REDIS,
        CRC64_XZ_TABLE => algorithms::CRC_64_XZ;
        CRC64_WE_TABLE => CRC64_ECMA_182_TABLE,
    );
}
