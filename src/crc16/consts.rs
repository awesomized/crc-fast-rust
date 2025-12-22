// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

#![allow(dead_code)]

use crate::consts::{
    NAME_CRC16_ARC, NAME_CRC16_CDMA2000, NAME_CRC16_CMS, NAME_CRC16_DDS_110, NAME_CRC16_DECT_R,
    NAME_CRC16_DECT_X, NAME_CRC16_DNP, NAME_CRC16_EN_13757, NAME_CRC16_GENIBUS, NAME_CRC16_GSM,
    NAME_CRC16_IBM_3740, NAME_CRC16_IBM_SDLC, NAME_CRC16_ISO_IEC_14443_3_A, NAME_CRC16_KERMIT,
    NAME_CRC16_LJ1200, NAME_CRC16_M17, NAME_CRC16_MAXIM_DOW, NAME_CRC16_MCRF4XX, NAME_CRC16_MODBUS,
    NAME_CRC16_NRSC_5, NAME_CRC16_OPENSAFETY_A, NAME_CRC16_OPENSAFETY_B, NAME_CRC16_PROFIBUS,
    NAME_CRC16_RIELLO, NAME_CRC16_SPI_FUJITSU, NAME_CRC16_T10_DIF, NAME_CRC16_TELEDISK,
    NAME_CRC16_TMS37157, NAME_CRC16_UMTS, NAME_CRC16_USB, NAME_CRC16_XMODEM,
};
use crate::CrcAlgorithm;
use crate::CrcParams;
use crc::{
    CRC_16_ARC, CRC_16_CDMA2000, CRC_16_CMS, CRC_16_DDS_110, CRC_16_DECT_R, CRC_16_DECT_X,
    CRC_16_DNP, CRC_16_EN_13757, CRC_16_GENIBUS, CRC_16_GSM, CRC_16_IBM_3740, CRC_16_IBM_SDLC,
    CRC_16_ISO_IEC_14443_3_A, CRC_16_KERMIT, CRC_16_LJ1200, CRC_16_M17, CRC_16_MAXIM_DOW,
    CRC_16_MCRF4XX, CRC_16_MODBUS, CRC_16_NRSC_5, CRC_16_OPENSAFETY_A, CRC_16_OPENSAFETY_B,
    CRC_16_PROFIBUS, CRC_16_RIELLO, CRC_16_SPI_FUJITSU, CRC_16_T10_DIF, CRC_16_TELEDISK,
    CRC_16_TMS37157, CRC_16_UMTS, CRC_16_USB, CRC_16_XMODEM,
};

// width=16 poly=0x8005 init=0x0000 refin=true refout=true xorout=0x0000 check=0xbb3d residue=0x0000 name="CRC-16/ARC"
pub const CRC16_ARC: CrcParams = CrcParams {
    name: NAME_CRC16_ARC,
    algorithm: CrcAlgorithm::Crc16Arc,
    width: 16,
    poly: CRC_16_ARC.poly as u64,
    init: CRC_16_ARC.init as u64,
    refin: CRC_16_ARC.refin,
    refout: CRC_16_ARC.refout,
    xorout: CRC_16_ARC.xorout as u64,
    check: CRC_16_ARC.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0xc867 init=0xffff refin=false refout=false xorout=0x0000 check=0x4c06 residue=0x0000 name="CRC-16/CDMA2000"
pub const CRC16_CDMA2000: CrcParams = CrcParams {
    name: NAME_CRC16_CDMA2000,
    algorithm: CrcAlgorithm::Crc16Cdma2000,
    width: 16,
    poly: CRC_16_CDMA2000.poly as u64,
    init: CRC_16_CDMA2000.init as u64,
    refin: CRC_16_CDMA2000.refin,
    refout: CRC_16_CDMA2000.refout,
    xorout: CRC_16_CDMA2000.xorout as u64,
    check: CRC_16_CDMA2000.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x8005 init=0xffff refin=false refout=false xorout=0x0000 check=0xaee7 residue=0x0000 name="CRC-16/CMS"
pub const CRC16_CMS: CrcParams = CrcParams {
    name: NAME_CRC16_CMS,
    algorithm: CrcAlgorithm::Crc16Cms,
    width: 16,
    poly: CRC_16_CMS.poly as u64,
    init: CRC_16_CMS.init as u64,
    refin: CRC_16_CMS.refin,
    refout: CRC_16_CMS.refout,
    xorout: CRC_16_CMS.xorout as u64,
    check: CRC_16_CMS.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x8005 init=0x800d refin=false refout=false xorout=0x0000 check=0x9ecf residue=0x0000 name="CRC-16/DDS-110"
pub const CRC16_DDS_110: CrcParams = CrcParams {
    name: NAME_CRC16_DDS_110,
    algorithm: CrcAlgorithm::Crc16Dds110,
    width: 16,
    poly: CRC_16_DDS_110.poly as u64,
    init: CRC_16_DDS_110.init as u64,
    refin: CRC_16_DDS_110.refin,
    refout: CRC_16_DDS_110.refout,
    xorout: CRC_16_DDS_110.xorout as u64,
    check: CRC_16_DDS_110.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x0589 init=0x0000 refin=false refout=false xorout=0x0001 check=0x007e residue=0x0589 name="CRC-16/DECT-R"
pub const CRC16_DECT_R: CrcParams = CrcParams {
    name: NAME_CRC16_DECT_R,
    algorithm: CrcAlgorithm::Crc16DectR,
    width: 16,
    poly: CRC_16_DECT_R.poly as u64,
    init: CRC_16_DECT_R.init as u64,
    refin: CRC_16_DECT_R.refin,
    refout: CRC_16_DECT_R.refout,
    xorout: CRC_16_DECT_R.xorout as u64,
    check: CRC_16_DECT_R.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x0589 init=0x0000 refin=false refout=false xorout=0x0000 check=0x007f residue=0x0000 name="CRC-16/DECT-X"
pub const CRC16_DECT_X: CrcParams = CrcParams {
    name: NAME_CRC16_DECT_X,
    algorithm: CrcAlgorithm::Crc16DectX,
    width: 16,
    poly: CRC_16_DECT_X.poly as u64,
    init: CRC_16_DECT_X.init as u64,
    refin: CRC_16_DECT_X.refin,
    refout: CRC_16_DECT_X.refout,
    xorout: CRC_16_DECT_X.xorout as u64,
    check: CRC_16_DECT_X.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x3d65 init=0x0000 refin=true refout=true xorout=0xffff check=0xea82 residue=0x66c5 name="CRC-16/DNP"
pub const CRC16_DNP: CrcParams = CrcParams {
    name: NAME_CRC16_DNP,
    algorithm: CrcAlgorithm::Crc16Dnp,
    width: 16,
    poly: CRC_16_DNP.poly as u64,
    init: CRC_16_DNP.init as u64,
    refin: CRC_16_DNP.refin,
    refout: CRC_16_DNP.refout,
    xorout: CRC_16_DNP.xorout as u64,
    check: CRC_16_DNP.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x3d65 init=0x0000 refin=false refout=false xorout=0xffff check=0xc2b7 residue=0xa366 name="CRC-16/EN-13757"
pub const CRC16_EN_13757: CrcParams = CrcParams {
    name: NAME_CRC16_EN_13757,
    algorithm: CrcAlgorithm::Crc16En13757,
    width: 16,
    poly: CRC_16_EN_13757.poly as u64,
    init: CRC_16_EN_13757.init as u64,
    refin: CRC_16_EN_13757.refin,
    refout: CRC_16_EN_13757.refout,
    xorout: CRC_16_EN_13757.xorout as u64,
    check: CRC_16_EN_13757.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x1021 init=0xffff refin=false refout=false xorout=0xffff check=0xd64e residue=0x1d0f name="CRC-16/GENIBUS"
pub const CRC16_GENIBUS: CrcParams = CrcParams {
    name: NAME_CRC16_GENIBUS,
    algorithm: CrcAlgorithm::Crc16Genibus,
    width: 16,
    poly: CRC_16_GENIBUS.poly as u64,
    init: CRC_16_GENIBUS.init as u64,
    refin: CRC_16_GENIBUS.refin,
    refout: CRC_16_GENIBUS.refout,
    xorout: CRC_16_GENIBUS.xorout as u64,
    check: CRC_16_GENIBUS.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x1021 init=0x0000 refin=false refout=false xorout=0xffff check=0xce3c residue=0x1d0f name="CRC-16/GSM"
pub const CRC16_GSM: CrcParams = CrcParams {
    name: NAME_CRC16_GSM,
    algorithm: CrcAlgorithm::Crc16Gsm,
    width: 16,
    poly: CRC_16_GSM.poly as u64,
    init: CRC_16_GSM.init as u64,
    refin: CRC_16_GSM.refin,
    refout: CRC_16_GSM.refout,
    xorout: CRC_16_GSM.xorout as u64,
    check: CRC_16_GSM.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0xa001 init=0x0000 refin=true refout=true xorout=0x0000 check=0x29b1 residue=0x0000 name="CRC-16/IBM-3740"
pub const CRC16_IBM_3740: CrcParams = CrcParams {
    name: NAME_CRC16_IBM_3740,
    algorithm: CrcAlgorithm::Crc16Ibm3740,
    width: 16,
    poly: CRC_16_IBM_3740.poly as u64,
    init: CRC_16_IBM_3740.init as u64,
    refin: CRC_16_IBM_3740.refin,
    refout: CRC_16_IBM_3740.refout,
    xorout: CRC_16_IBM_3740.xorout as u64,
    check: CRC_16_IBM_3740.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x1021 init=0xffff refin=true refout=true xorout=0xffff check=0x906e residue=0xf0b8 name="CRC-16/IBM-SDLC"
pub const CRC16_IBM_SDLC: CrcParams = CrcParams {
    name: NAME_CRC16_IBM_SDLC,
    algorithm: CrcAlgorithm::Crc16IbmSdlc,
    width: 16,
    poly: CRC_16_IBM_SDLC.poly as u64,
    init: CRC_16_IBM_SDLC.init as u64,
    refin: CRC_16_IBM_SDLC.refin,
    refout: CRC_16_IBM_SDLC.refout,
    xorout: CRC_16_IBM_SDLC.xorout as u64,
    check: CRC_16_IBM_SDLC.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1021_REVERSE),
};

// width=16 poly=0x1021 init=0xc6c6 refin=true refout=true xorout=0x0000 check=0xbf05 residue=0x0000 name="CRC-16/ISO-IEC-14443-3-A"
pub const CRC16_ISO_IEC_14443_3_A: CrcParams = CrcParams {
    name: NAME_CRC16_ISO_IEC_14443_3_A,
    algorithm: CrcAlgorithm::Crc16IsoIec144433A,
    width: 16,
    poly: CRC_16_ISO_IEC_14443_3_A.poly as u64,
    init: CRC_16_ISO_IEC_14443_3_A.init as u64,
    refin: CRC_16_ISO_IEC_14443_3_A.refin,
    refout: CRC_16_ISO_IEC_14443_3_A.refout,
    xorout: CRC_16_ISO_IEC_14443_3_A.xorout as u64,
    check: CRC_16_ISO_IEC_14443_3_A.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x1021 init=0x0000 refin=true refout=true xorout=0x0000 check=0x2189 residue=0x0000 name="CRC-16/KERMIT"
pub const CRC16_KERMIT: CrcParams = CrcParams {
    name: NAME_CRC16_KERMIT,
    algorithm: CrcAlgorithm::Crc16Kermit,
    width: 16,
    poly: CRC_16_KERMIT.poly as u64,
    init: CRC_16_KERMIT.init as u64,
    refin: CRC_16_KERMIT.refin,
    refout: CRC_16_KERMIT.refout,
    xorout: CRC_16_KERMIT.xorout as u64,
    check: CRC_16_KERMIT.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x6f63 init=0x0000 refin=false refout=false xorout=0x0000 check=0xbdf4 residue=0x0000 name="CRC-16/LJ1200"
pub const CRC16_LJ1200: CrcParams = CrcParams {
    name: NAME_CRC16_LJ1200,
    algorithm: CrcAlgorithm::Crc16Lj1200,
    width: 16,
    poly: CRC_16_LJ1200.poly as u64,
    init: CRC_16_LJ1200.init as u64,
    refin: CRC_16_LJ1200.refin,
    refout: CRC_16_LJ1200.refout,
    xorout: CRC_16_LJ1200.xorout as u64,
    check: CRC_16_LJ1200.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x5935 init=0xffff refin=false refout=false xorout=0x0000 check=0x772b residue=0x0000 name="CRC-16/M17"
pub const CRC16_M17: CrcParams = CrcParams {
    name: NAME_CRC16_M17,
    algorithm: CrcAlgorithm::Crc16M17,
    width: 16,
    poly: CRC_16_M17.poly as u64,
    init: CRC_16_M17.init as u64,
    refin: CRC_16_M17.refin,
    refout: CRC_16_M17.refout,
    xorout: CRC_16_M17.xorout as u64,
    check: CRC_16_M17.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x8005 init=0x0000 refin=true refout=true xorout=0xffff check=0x44c2 residue=0xb001 name="CRC-16/MAXIM-DOW"
pub const CRC16_MAXIM_DOW: CrcParams = CrcParams {
    name: NAME_CRC16_MAXIM_DOW,
    algorithm: CrcAlgorithm::Crc16MaximDow,
    width: 16,
    poly: CRC_16_MAXIM_DOW.poly as u64,
    init: CRC_16_MAXIM_DOW.init as u64,
    refin: CRC_16_MAXIM_DOW.refin,
    refout: CRC_16_MAXIM_DOW.refout,
    xorout: CRC_16_MAXIM_DOW.xorout as u64,
    check: CRC_16_MAXIM_DOW.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x1021 init=0xffff refin=true refout=true xorout=0x0000 check=0x6f91 residue=0x0000 name="CRC-16/MCRF4XX"
pub const CRC16_MCRF4XX: CrcParams = CrcParams {
    name: NAME_CRC16_MCRF4XX,
    algorithm: CrcAlgorithm::Crc16Mcrf4xx,
    width: 16,
    poly: CRC_16_MCRF4XX.poly as u64,
    init: CRC_16_MCRF4XX.init as u64,
    refin: CRC_16_MCRF4XX.refin,
    refout: CRC_16_MCRF4XX.refout,
    xorout: CRC_16_MCRF4XX.xorout as u64,
    check: CRC_16_MCRF4XX.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1021_REVERSE),
};

// width=16 poly=0x8005 init=0xffff refin=true refout=true xorout=0x0000 check=0x4b37 residue=0x0000 name="CRC-16/MODBUS"
pub const CRC16_MODBUS: CrcParams = CrcParams {
    name: NAME_CRC16_MODBUS,
    algorithm: CrcAlgorithm::Crc16Modbus,
    width: 16,
    poly: CRC_16_MODBUS.poly as u64,
    init: CRC_16_MODBUS.init as u64,
    refin: CRC_16_MODBUS.refin,
    refout: CRC_16_MODBUS.refout,
    xorout: CRC_16_MODBUS.xorout as u64,
    check: CRC_16_MODBUS.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x080b init=0xffff refin=true refout=true xorout=0x0000 check=0xa066 residue=0x0000 name="CRC-16/NRSC-5"
pub const CRC16_NRSC_5: CrcParams = CrcParams {
    name: NAME_CRC16_NRSC_5,
    algorithm: CrcAlgorithm::Crc16Nrsc5,
    width: 16,
    poly: CRC_16_NRSC_5.poly as u64,
    init: CRC_16_NRSC_5.init as u64,
    refin: CRC_16_NRSC_5.refin,
    refout: CRC_16_NRSC_5.refout,
    xorout: CRC_16_NRSC_5.xorout as u64,
    check: CRC_16_NRSC_5.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x5935 init=0x0000 refin=false refout=false xorout=0x0000 check=0x5d38 residue=0x0000 name="CRC-16/OPENSAFETY-A"
pub const CRC16_OPENSAFETY_A: CrcParams = CrcParams {
    name: NAME_CRC16_OPENSAFETY_A,
    algorithm: CrcAlgorithm::Crc16OpensafetyA,
    width: 16,
    poly: CRC_16_OPENSAFETY_A.poly as u64,
    init: CRC_16_OPENSAFETY_A.init as u64,
    refin: CRC_16_OPENSAFETY_A.refin,
    refout: CRC_16_OPENSAFETY_A.refout,
    xorout: CRC_16_OPENSAFETY_A.xorout as u64,
    check: CRC_16_OPENSAFETY_A.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x755b init=0x0000 refin=false refout=false xorout=0x0000 check=0x20fe residue=0x0000 name="CRC-16/OPENSAFETY-B"
pub const CRC16_OPENSAFETY_B: CrcParams = CrcParams {
    name: NAME_CRC16_OPENSAFETY_B,
    algorithm: CrcAlgorithm::Crc16OpensafetyB,
    width: 16,
    poly: CRC_16_OPENSAFETY_B.poly as u64,
    init: CRC_16_OPENSAFETY_B.init as u64,
    refin: CRC_16_OPENSAFETY_B.refin,
    refout: CRC_16_OPENSAFETY_B.refout,
    xorout: CRC_16_OPENSAFETY_B.xorout as u64,
    check: CRC_16_OPENSAFETY_B.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x1dcf init=0xffff refin=false refout=false xorout=0xffff check=0xa819 residue=0xe394 name="CRC-16/PROFIBUS"
pub const CRC16_PROFIBUS: CrcParams = CrcParams {
    name: NAME_CRC16_PROFIBUS,
    algorithm: CrcAlgorithm::Crc16Profibus,
    width: 16,
    poly: CRC_16_PROFIBUS.poly as u64,
    init: CRC_16_PROFIBUS.init as u64,
    refin: CRC_16_PROFIBUS.refin,
    refout: CRC_16_PROFIBUS.refout,
    xorout: CRC_16_PROFIBUS.xorout as u64,
    check: CRC_16_PROFIBUS.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x1021 init=0xb2aa refin=true refout=true xorout=0x0000 check=0x63d0 residue=0x0000 name="CRC-16/RIELLO"
pub const CRC16_RIELLO: CrcParams = CrcParams {
    name: NAME_CRC16_RIELLO,
    algorithm: CrcAlgorithm::Crc16Riello,
    width: 16,
    poly: CRC_16_RIELLO.poly as u64,
    init: CRC_16_RIELLO.init as u64,
    refin: CRC_16_RIELLO.refin,
    refout: CRC_16_RIELLO.refout,
    xorout: CRC_16_RIELLO.xorout as u64,
    check: CRC_16_RIELLO.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1021_REVERSE),
};

// width=16 poly=0x1021 init=0x1d0f refin=false refout=false xorout=0x0000 check=0xe5cc residue=0x0000 name="CRC-16/SPI-FUJITSU"
pub const CRC16_SPI_FUJITSU: CrcParams = CrcParams {
    name: NAME_CRC16_SPI_FUJITSU,
    algorithm: CrcAlgorithm::Crc16SpiFujitsu,
    width: 16,
    poly: CRC_16_SPI_FUJITSU.poly as u64,
    init: CRC_16_SPI_FUJITSU.init as u64,
    refin: CRC_16_SPI_FUJITSU.refin,
    refout: CRC_16_SPI_FUJITSU.refout,
    xorout: CRC_16_SPI_FUJITSU.xorout as u64,
    check: CRC_16_SPI_FUJITSU.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1021_REVERSE),
};

// width=16 poly=0x8bb7 init=0x0000 refin=false refout=false xorout=0x0000 check=0xd0db residue=0x0000 name="CRC-16/T10-DIF"
pub const CRC16_T10_DIF: CrcParams = CrcParams {
    name: NAME_CRC16_T10_DIF,
    algorithm: CrcAlgorithm::Crc16T10Dif,
    width: 16,
    poly: CRC_16_T10_DIF.poly as u64,
    init: CRC_16_T10_DIF.init as u64,
    refin: CRC_16_T10_DIF.refin,
    refout: CRC_16_T10_DIF.refout,
    xorout: CRC_16_T10_DIF.xorout as u64,
    check: CRC_16_T10_DIF.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_8BB7_FORWARD),
};

// width=16 poly=0xa097 init=0x0000 refin=false refout=false xorout=0x0000 check=0x0fb3 residue=0x0000 name="CRC-16/TELEDISK"
pub const CRC16_TELEDISK: CrcParams = CrcParams {
    name: NAME_CRC16_TELEDISK,
    algorithm: CrcAlgorithm::Crc16Teledisk,
    width: 16,
    poly: CRC_16_TELEDISK.poly as u64,
    init: CRC_16_TELEDISK.init as u64,
    refin: CRC_16_TELEDISK.refin,
    refout: CRC_16_TELEDISK.refout,
    xorout: CRC_16_TELEDISK.xorout as u64,
    check: CRC_16_TELEDISK.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x1021 init=0x89ec refin=true refout=true xorout=0x0000 check=0x26b1 residue=0x0000 name="CRC-16/TMS37157"
pub const CRC16_TMS37157: CrcParams = CrcParams {
    name: NAME_CRC16_TMS37157,
    algorithm: CrcAlgorithm::Crc16Tms37157,
    width: 16,
    poly: CRC_16_TMS37157.poly as u64,
    init: CRC_16_TMS37157.init as u64,
    refin: CRC_16_TMS37157.refin,
    refout: CRC_16_TMS37157.refout,
    xorout: CRC_16_TMS37157.xorout as u64,
    check: CRC_16_TMS37157.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1021_REVERSE),
};

// width=16 poly=0x8005 init=0x0000 refin=false refout=false xorout=0x0000 check=0xfee8 residue=0x0000 name="CRC-16/UMTS"
pub const CRC16_UMTS: CrcParams = CrcParams {
    name: NAME_CRC16_UMTS,
    algorithm: CrcAlgorithm::Crc16Umts,
    width: 16,
    poly: CRC_16_UMTS.poly as u64,
    init: CRC_16_UMTS.init as u64,
    refin: CRC_16_UMTS.refin,
    refout: CRC_16_UMTS.refout,
    xorout: CRC_16_UMTS.xorout as u64,
    check: CRC_16_UMTS.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x8005 init=0xffff refin=true refout=true xorout=0xffff check=0xb4c8 residue=0xb001 name="CRC-16/USB"
pub const CRC16_USB: CrcParams = CrcParams {
    name: NAME_CRC16_USB,
    algorithm: CrcAlgorithm::Crc16Usb,
    width: 16,
    poly: CRC_16_USB.poly as u64,
    init: CRC_16_USB.init as u64,
    refin: CRC_16_USB.refin,
    refout: CRC_16_USB.refout,
    xorout: CRC_16_USB.xorout as u64,
    check: CRC_16_USB.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

// width=16 poly=0x1021 init=0x0000 refin=false refout=false xorout=0x0000 check=0x31c3 residue=0x0000 name="CRC-16/XMODEM"
pub const CRC16_XMODEM: CrcParams = CrcParams {
    name: NAME_CRC16_XMODEM,
    algorithm: CrcAlgorithm::Crc16Xmodem,
    width: 16,
    poly: CRC_16_XMODEM.poly as u64,
    init: CRC_16_XMODEM.init as u64,
    refin: CRC_16_XMODEM.refin,
    refout: CRC_16_XMODEM.refout,
    xorout: CRC_16_XMODEM.xorout as u64,
    check: CRC_16_XMODEM.check as u64,
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_PLACEHOLDER), // placeholder
};

pub const KEYS_PLACEHOLDER: [u64; 23] = [0; 23];

pub const KEYS_8BB7_FORWARD: [u64; 23] = [
    0x0000000000000000,
    0x2d56000000000000, // 2^(32* 3) mod Q << 32
    0x06df000000000000, // 2^(32* 5) mod Q << 32
    0x9d9d000000000000, // 2^(32*31) mod Q << 32
    0x7cf5000000000000, // 2^(32*33) mod Q << 32
    0x2d56000000000000, // 2^(32* 3) mod Q << 32
    0x1368000000000000, // 2^(32* 2) mod Q << 32
    0x00000001f65a57f8, // floor(2^64/Q)
    0x000000018bb70000, // Q
    0xceae000000000000, // 2^(32*27) mod Q << 32
    0xbfd6000000000000, // 2^(32*29) mod Q << 32
    0x1e16000000000000, // 2^(32*23) mod Q << 32
    0x713c000000000000, // 2^(32*25) mod Q << 32
    0xf7f9000000000000, // 2^(32*19) mod Q << 32
    0x80a6000000000000, // 2^(32*21) mod Q << 32
    0x044c000000000000, // 2^(32*15) mod Q << 32
    0xe658000000000000, // 2^(32*17) mod Q << 32
    0xad18000000000000, // 2^(32*11) mod Q << 32
    0xa497000000000000, // 2^(32*13) mod Q << 32
    0x6ee3000000000000, // 2^(32* 7) mod Q << 32
    0xe7b5000000000000, // 2^(32* 9) mod Q << 32
    0xdccf000000000000, // 2^(32*63) mod Q << 32 (256-byte folding)
    0x4b0b000000000000, // 2^(32*65) mod Q << 32 (256-byte folding)
];

pub const KEYS_1021_REVERSE: [u64; 23] = [
    0x0000000000000000,
    0x00000000000189ae, // (2^(32* 3) mod P(x))' << 1
    0x0000000000008e10, // (2^(32* 5) mod P(x))' << 1
    0x00000000000160be, // (2^(32*31) mod P(x))' << 1
    0x000000000001bed8, // (2^(32*33) mod P(x))' << 1
    0x00000000000189ae, // (2^(32* 3) mod P(x))' << 1
    0x00000000000114aa, // (2^(32* 2) mod P(x))' << 1
    0x000000011c581911, // (floor(2^64/P(x)))'
    0x0000000000010811, // (P(x))'
    0x000000000001ce5e, // (2^(32*27) mod P(x))' << 1
    0x000000000001c584, // (2^(32*29) mod P(x))' << 1
    0x000000000001db50, // (2^(32*23) mod P(x))' << 1
    0x000000000000b8f2, // (2^(32*25) mod P(x))' << 1
    0x0000000000000842, // (2^(32*19) mod P(x))' << 1
    0x000000000000b072, // (2^(32*21) mod P(x))' << 1
    0x0000000000014ff2, // (2^(32*15) mod P(x))' << 1
    0x0000000000019a3c, // (2^(32*17) mod P(x))' << 1
    0x0000000000000e3a, // (2^(32*11) mod P(x))' << 1
    0x0000000000004d7a, // (2^(32*13) mod P(x))' << 1
    0x0000000000005b44, // (2^(32* 7) mod P(x))' << 1
    0x0000000000007762, // (2^(32* 9) mod P(x))' << 1
    0x0000000000019208, // (2^(32*63) mod P(x))' << 1 (256-byte folding)
    0x0000000000002df8, // (2^(32*65) mod P(x))' << 1 (256-byte folding)
];
