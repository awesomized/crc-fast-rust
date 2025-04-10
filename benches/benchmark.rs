// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

use crc_fast::checksum;
use crc_fast::CrcAlgorithm;
use criterion::*;
use rand::{rng, RngCore};

pub const SIZES: &[(&str, i32); 2] = &[
    ("1 MiB", 1024 * 1024),
    //("512 KiB", 512 * 1024),
    //("256 KiB", 256 * 1024),
    //("128 KiB", 128 * 1024),
    //("64 KiB", 64 * 1024),
    //("16 KiB", 16 * 1024),
    //("8 KiB", 8 * 1024),
    //("4 KiB", 4 * 1024),
    //("2 KiB", 4 * 1024),
    ("1 KiB", 1024),
    //("768 bytes", 768),
    //("512 bytes", 512),
    //("256 bytes", 256),
    //("255 bytes", 255),
    //("128 bytes", 128),
    //("127 bytes", 127),
    //("64 bytes", 64),
    //("32 bytes", 32),
    //("16 bytes", 16),
    //("1 bytes", 1),
];

// these are the most important algorithms in popular use, with forward/reflected coverage
pub const CRC32_ALGORITHMS: &[CrcAlgorithm; 4] = &[
    CrcAlgorithm::Crc32Autosar, // reflected, internal
    CrcAlgorithm::Crc32Iscsi,   // reflected, custom
    CrcAlgorithm::Crc32IsoHdlc, // reflected, custom
    CrcAlgorithm::Crc32Bzip2,   // forward, internal
];

// these are the most important algorithms in popular use, with forward/reflected coverage
pub const CRC64_ALGORITHMS: &[CrcAlgorithm; 2] = &[
    CrcAlgorithm::Crc64Ecma182, // forward
    CrcAlgorithm::Crc64Nvme,    // reflected
];

#[inline(always)]
fn random_data(size: i32) -> Vec<u8> {
    let mut rng = rng();
    let mut buf = vec![0u8; size as usize];
    rng.fill_bytes(&mut buf);

    buf
}

#[inline(always)]
fn bench_crc32(c: &mut Criterion) {
    let mut group = c.benchmark_group("CRC-32");

    println!(
        "CRC-32/ISCSI implementation {}",
        crc_fast::get_calculator_target(CrcAlgorithm::Crc32Iscsi)
    );
    println!(
        "CRC-32/ISO-HDLC implementation {}",
        crc_fast::get_calculator_target(CrcAlgorithm::Crc32IsoHdlc)
    );

    for (size_name, size) in SIZES {
        let buf = random_data(*size);

        let (part1, rest) = buf.split_at(buf.len() / 4);
        let (part2, rest) = rest.split_at(rest.len() / 3);
        let (part3, part4) = rest.split_at(rest.len() / 2);

        for algorithm in CRC32_ALGORITHMS {
            let algorithm_name = algorithm.to_string();
            let mut algorithm_name_parts = algorithm_name.split('/');
            let _ = algorithm_name_parts.next();
            let alg_suffix = algorithm_name_parts.next();

            group.throughput(Throughput::Bytes(*size as u64));

            let bench_name = [alg_suffix.unwrap(), "(checksum)"].join(" ");

            group.bench_function(BenchmarkId::new(bench_name, size_name), |b| {
                b.iter(|| black_box(checksum(*algorithm, &buf)))
            });

            let bench_name = [algorithm_name.clone(), "(4-part digest)".parse().unwrap()].join(" ");

            group.bench_function(BenchmarkId::new(bench_name, size_name), |b| {
                b.iter(|| {
                    black_box({
                        let mut digest = crc_fast::Digest::new(*algorithm);
                        digest.update(&part1);
                        digest.update(&part2);
                        digest.update(&part3);
                        digest.update(&part4);
                        digest.finalize()
                    })
                })
            });
        }
    }
}

#[inline(always)]
fn bench_crc64(c: &mut Criterion) {
    let mut group = c.benchmark_group("CRC-64");

    for (size_name, size) in SIZES {
        let buf = random_data(*size);

        let (part1, rest) = buf.split_at(buf.len() / 4);
        let (part2, rest) = rest.split_at(rest.len() / 3);
        let (part3, part4) = rest.split_at(rest.len() / 2);

        for algorithm in CRC64_ALGORITHMS {
            let algorithm_name = algorithm.to_string();
            let mut algorithm_name_parts = algorithm_name.split('/');
            let _ = algorithm_name_parts.next();
            let alg_suffix = algorithm_name_parts.next();

            group.throughput(Throughput::Bytes(*size as u64));

            let bench_name = [alg_suffix.unwrap(), "(checksum)"].join(" ");

            group.bench_function(BenchmarkId::new(bench_name, size_name), |b| {
                b.iter(|| black_box(checksum(*algorithm, &buf)))
            });

            let bench_name = [algorithm_name.clone(), "(4-part digest)".parse().unwrap()].join(" ");

            group.bench_function(BenchmarkId::new(bench_name, size_name), |b| {
                b.iter(|| {
                    black_box({
                        let mut digest = crc_fast::Digest::new(*algorithm);
                        digest.update(&part1);
                        digest.update(&part2);
                        digest.update(&part3);
                        digest.update(&part4);
                        digest.finalize()
                    })
                })
            });
        }
    }
}

criterion_group!(benches, bench_crc64, bench_crc32);

criterion_main!(benches);
