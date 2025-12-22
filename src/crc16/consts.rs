// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

#![allow(dead_code)]

use crate::consts::{NAME_CRC16_IBM_SDLC, NAME_CRC16_T10_DIF};
use crate::CrcAlgorithm;
use crate::CrcParams;
use crc::{CRC_16_IBM_SDLC, CRC_16_T10_DIF};

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
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_8BB7_FORWARD),
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
    keys: crate::CrcKeysStorage::from_keys_fold_256(KEYS_1021_REVERSE),
};

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
    0x0000000000000000, // TODO: calculate correct value for 256-byte folding (AVX512)
    0x0000000000000000, // TODO: calculate correct value for 256-byte folding (AVX512)
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
    0x0000000000000000, // TODO: calculate correct value for 256-byte folding (AVX512)
    0x0000000000000000, // TODO: calculate correct value for 256-byte folding (AVX512)
];
