//! Run in a fresh process to measure the first combine, including lazy setup.
//! Example: cargo run --locked --release --example combine_first_call -- 16384

use crc_fast::{checksum_combine, CrcAlgorithm};
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let len: u64 = std::env::args()
        .nth(1)
        .expect("second length in bytes")
        .parse()
        .expect("u64 length");
    let first = black_box(0xae8b_1486_0a79_9888);
    let second = black_box(0x9f21_c83a_dbc1_cd7d);
    let algorithm = black_box(CrcAlgorithm::Crc64Nvme);
    let start = Instant::now();
    let combined = checksum_combine(algorithm, first, second, black_box(len));
    let elapsed = start.elapsed();
    black_box(combined);
    println!("{}", elapsed.as_nanos());
}
