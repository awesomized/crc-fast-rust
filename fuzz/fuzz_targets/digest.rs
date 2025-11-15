#![no_main]

use crc_fast::{CrcAlgorithm::Crc32IsoHdlc, Digest};
use libfuzzer_sys::fuzz_target;

const ALGORITHMS: &[CrcAlgorithm] = &[
    Crc32Autosar, // reflected
    Crc32IsoHdlc, // reflected, fusion (aarch64 only)
    Crc32Iscsi,   // reflected, fusion (x86 & aarch64)
    Crc32Bzip2,   // forward
    Crc64Ecma182, // forward
    Crc64Nvme,    // reflected
];

fuzz_target!(|data: &[u8]| {
    let mut digest = Digest::new(Crc32IsoHdlc);
    digest.update(data);
    digest.finalize();
});
