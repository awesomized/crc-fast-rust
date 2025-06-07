`crc-fast`
===========

[![Test status](https://github.com/awesomized/crc-fast-rust/workflows/Tests/badge.svg)](https://github.com/awesomized/crc-fast-rust/actions?query=workflow%3ATests)
[![Latest Version](https://img.shields.io/crates/v/crc-fast.svg)](https://crates.io/crates/crc-fast)
[![Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/crc-fast)

Fast, hardware-accelerated CRC calculation for 
[all known CRC-32 and CRC-64 variants](https://reveng.sourceforge.io/crc-catalogue/all.htm) using SIMD intrinsics,
which can exceed _100GiB/s_ for `CRC-32`, and _50GiB/s_ for `CRC-64`, on modern systems. 

Supports acceleration on `aarch64`, `x86_64`, and `x86` architectures, plus has a safe non-accelerated software 
fallback for other architectures.

The [crc crate](https://crates.io/crates/crc) is ~0.5GiB/s by default, so this is 
[up to >200X faster](#tldr-just-tell-me-how-to-turn-it-up-to-11-), and even the most conservative baseline settings 
are >27X.

This is unique, not just because of the performance, but also because I couldn't find a single generic SIMD-accelerated 
implementation (in any language) which worked for _all_ known variants, using the 
[Rocksoft model](http://www.ross.net/crc/download/crc_v3.txt), especially the "non-reflected" variants.

So I wrote one.

## Other languages

Supplies a [C/C++ compatible shared library](#cc-compatible-shared-library) for use with other non-`Rust` languages.

## Changes

See [CHANGELOG](CHANGELOG.md).

## Build & Install

`cargo build` will obviously build the library, including
the [C-compatible shared library](#c-compatible-shared-library). There are fine-tuning [feature flags](Cargo.toml) 
available, should they be necessary for your deployment and [acceleration](#acceleration-targets) targets.

A _very_ basic [Makefile](Makefile) is supplied which supports `make install` to install the shared library and header file to
the local system. Specifying the `DESTDIR` environment variable will allow you to customize the install location.

```
DESTDIR=/my/custom/path make install
```

You'll need to adjust if you want to optimize with [feature flags](Cargo.toml).

## Usage

Add `crc-fast = { version = "1.1", features = ["optimize_crc32_auto"] }` to your `Cargo.toml` dependencies, which will
enable every available optimization for the `stable` toolchain. Adjust as necessary for your desired 
[acceleration targets](#acceleration-targets).

### Digest

Implements the [digest::DynDigest](https://docs.rs/digest/latest/digest/trait.DynDigest.html)
trait for easier integration with existing Rust code.

Creates a `Digest` which can be updated over time, for stream processing, intermittent workloads, etc, enabling 
finalizing the checksum once processing is complete.

 ```rust
 use crc_fast::{Digest, CrcAlgorithm::Crc32IsoHdlc};

let mut digest = Digest::new(Crc32IsoHdlc);
digest.update(b"1234");
digest.update(b"56789");
let checksum = digest.finalize();

assert_eq!(checksum, 0xcbf43926);
 ```

### Digest Write

Implements the [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) trait for
easier integration with existing Rust code.

 ```rust
use std::env;
use std::fs::File;
use crc_fast::{Digest, CrcAlgorithm::Crc32IsoHdlc};

// for example/test purposes only, use your own file path
let binding = env::current_dir().expect("missing working dir").join("crc-check.txt");
let file_on_disk = binding.to_str().unwrap();

// actual usage
let mut digest = Digest::new(Crc32IsoHdlc);
let mut file = File::open(file_on_disk).unwrap();
std::io::copy(&mut file, &mut digest).unwrap();
let checksum = digest.finalize();

assert_eq!(checksum, 0xcbf43926);
 ```

### checksum

Checksums a string.

```rust
 use crc_fast::{checksum, CrcAlgorithm::Crc32IsoHdlc};

let checksum = checksum(Crc32IsoHdlc, b"123456789");

assert_eq!(checksum, 0xcbf43926);
 ```

### checksum_combine

Combines checksums from two different sources, which can be useful for distributed or multithreaded workloads, etc.

```rust
 use crc_fast::{checksum, checksum_combine, CrcAlgorithm::Crc32IsoHdlc};

let checksum_1 = checksum(Crc32IsoHdlc, b"1234");
let checksum_2 = checksum(Crc32IsoHdlc, b"56789");
let checksum = checksum_combine(Crc32IsoHdlc, checksum_1, checksum_2, 5);

assert_eq!(checksum, 0xcbf43926);
 ```

### checksum_file

Checksums a file, which will chunk through the file optimally, limiting RAM usage and maximizing throughput. Chunk size
is optional.

```rust
 use crc_fast::{checksum_file, CrcAlgorithm::Crc32IsoHdlc};

// for example/test purposes only, use your own file path
let binding = env::current_dir().expect("missing working dir").join("crc-check.txt");
let file_on_disk = binding.to_str().unwrap();

let checksum = checksum_file(Crc32IsoHdlc, file_on_disk, None);

assert_eq!(checksum.unwrap(), 0xcbf43926);
 ```

## C/C++ compatible shared library

`cargo build` will produce a shared library target (`.so` on Linux, `.dll` on Windows, `.dylib` on macOS, etc) and an 
auto-generated [libcrc_fast.h](libcrc_fast.h) header file for use in non-Rust projects, such as through 
[FFI](https://en.wikipedia.org/wiki/Foreign_function_interface).

There is a [crc-fast PHP extension](https://github.com/awesomized/crc-fast-php-ext) using it, for example.

## Background

This implementation is based on Intel's 
[Fast CRC Computation for Generic Polynomials Using PCLMULQDQ Instruction](https://web.archive.org/web/20131224125630/https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/fast-crc-computation-generic-polynomials-pclmulqdq-paper.pdf)
white paper, though it folds 8-at-a-time, like other modern implementations, rather than the 4-at-a-time as in Intel's 
paper.

This library works on `aarch64`, `x86_64`, and `x86` architectures, and is hardware-accelerated and optimized for each
architecture.

Inspired by [`crc32fast`](https://crates.io/crates/crc32fast),
[`crc64fast`](https://crates.io/crates/crc64fast),
and [`crc64fast-nvme`](https://crates.io/crates/crc64fast-nvme), each of which only accelerates a single, different CRC
variant, and all of them were "reflected" variants. 

In contrast, this library accelerates _every known variant_ (and should accelerate any future variants without changes),
including all the "non-reflected" variants.

## Important CRC variants

While there are [many variants](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iso-hdlc), three
stand out as being the most important and widely used (all of which are "reflected"):

### [CRC-32/ISCSI](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iscsi)

Many, but not all, implementations simply call this `crc32c` and it's probably the 2nd most popular and widely used, 
after `CRC-32/ISO-HDLC`. It's used in `iSCSI`, `ext4`, `btrfs`, etc.

### [CRC-32/ISO-HDLC](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iso-hdlc)

Many, but not all, implementations simply call this `crc32` and it may be the most popular and widely used. It's used in
`Ethernet`, `PKZIP`, `xz`, etc.

### [CRC-64/NVME](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-nvme)

`CRC-64/NVME` comes from the [NVM Express® NVM Command Set Specification](https://nvmexpress.org/wp-content/uploads/NVM-Express-NVM-Command-Set-Specification-1.0d-2023.12.28-Ratified.pdf)
(Revision 1.0d, December 2023),
is [AWS S3's recommended checksum option](https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html)
(as `CRC64-NVME`), and has also been implemented in the 
[Linux kernel](https://github.com/torvalds/linux/blob/786c8248dbd33a5a7a07f7c6e55a7bfc68d2ca48/lib/crc64.c#L66-L73)
(where it's called `CRC-64/Rocksoft`).

Note that the `Check` value in the `NVMe` spec uses incorrect endianness (see `Section 5.2.1.3.4, Figure 120, page 83`)
but all known public & private implementations agree on the correct value, which this library produces.

# Acceleration targets

This library has baseline support for accelerating all known `CRC-32` and `CRC-64` variants on `aarch64`, `x86_64`, and 
`x86` internally in pure `Rust`. It's extremely fast (up to dozens of GiB/s). This is the default if no feature flags are
specified. 

With feature flags, it can be even faster. 😎

### tl;dr: Just tell me how to turn it up to 11! 🤘

For modern `x86_64` systems (requires `nightly` toolchain) which further accelerates _all_ variants, especially
`CRC-32/ISCSI` and `CRC-32/ISO-HDLC`: 
```
rustup toolchain install nightly
cargo +nightly build --release --features=optimize_crc32_auto,vpclmulqdq
```

For `aarch64`, and older `x86_64` / `x86`, systems (no `nightly` required) which further accelerates `CRC-32/ISCSI` and 
`CRC-32/ISO-HDLC`:
```
cargo build --release --features=optimize_crc32_auto
```

At [Awesome](https://awesome.co/), we use these 👆 at large scale in production at [Flickr](https://flickr.com/) and 
[SmugMug](https://www.smugmug.com/).

### CRC-32/ISO-HDLC and CRC-32/ISCSI optimization

By using the `optimize_crc32_auto` feature flag, the library will use 
[fast-crc32](https://github.com/corsix/fast-crc32/) instead to accelerate _only_ `CRC-32/ISO-HDLC` and/or `CRC-32/ISCSI` 
using a [fusion](https://www.corsix.org/content/fast-crc32c-4k) of hardware `crc32(c)` support and `PCLMULQDQ`. 

`fast-crc32` does not accelerate any other `CRC-32` variants, or any `CRC-64` variants, since none of the others have 
native hardware-acceleration support in any CPUs which would enable `fusion`.

`fast-crc32` will use `VPCLMULQDQ` if available, without requiring the need for `nightly` builds (since it's an external
C implementation).

There's an [arch-check](src/bin/arch-check.rs) binary which will explain the selected target architecture.

```
// test it works on your system (patches welcome!)
cargo test --features=optimize_crc32_auto

// examine the chosen acceleration targets
cargo run --features=optimize_crc32_auto arch-check

// build for release
cargo build --features=optimize_crc32_auto --release
```

There are additional [feature flags](Cargo.toml) to force certain implementations for fine-tuning, benchmarking, etc.

### Experimental VPCLMULQDQ support in Rust

This library also supports [VPCLMULQDQ](https://en.wikichip.org/wiki/x86/vpclmulqdq) for accelerating all 
`CRC-32` and `CRC-64` variants on modern `x86_64` platforms which support it when using `nightly` builds and the 
`vpclmulqdq` feature flag. 

Typical performance boosts are ~2X, and they apply to CPUs beginning with Intel 
[Ice Lake](https://en.wikipedia.org/wiki/Ice_Lake_%28microprocessor%29) (Sep 2019) and AMD 
[Zen4](https://en.wikipedia.org/wiki/Zen_4) (Sep 2022).

```
rustup toolchain install nightly
cargo +nightly build --release --features=vpclmulqdq
```

There's a [tracking issue](https://github.com/rust-lang/rust/issues/111137) for when these features might land on 
`stable`, which looks like [very soon](https://github.com/rust-lang/rust/issues/111137#issuecomment-2787196977), at
which point this library will adopt it as a default.

## Performance

Modern systems can exceed 100GiB/s for calculating `CRC-32/ISCSI` and `CRC-32/ISO-HDLC`, and 50GiB/s for calculating
`CRC-64/NVME`.

This is a summary of the best [targets](#acceleration-targets) for the most important and popular CRC checksums. More 
extensive benchmark results, with other targets and variants, can be found in the [benches](benches/README.md) folder.

### CRC-32/ISCSI

AKA `crc32c` in many, but not all, implementations.

| Arch    | Brand | CPU             | System                    | Target                 | 1KiB (GiB/s) | 1MiB (GiB/s) |
|:--------|:------|:----------------|:--------------------------|:-----------------------|-------------:|-------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | avx512_vpclmulqdq_v3x2 |        ~38.0 |       ~111.7 |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx512_vpclmulqdq_v3x2 |        ~21.1 |        ~54.6 |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_blended           |        ~18.5 |        ~31.6 |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_v12e_v1           |        ~54.8 |        ~99.6 |
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_blended           |        ~60.8 |        ~96.3 |
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_blended           |        ~50.3 |        ~87.6 |

### CRC-32/ISO-HDLC

AKA `crc32` in many, but not all, implementations.

| Arch    | Brand | CPU             | System                    | Target       | 1KiB (GiB/s) | 1MiB (GiB/s) |
|:--------|:------|:----------------|:--------------------------|:-------------|-------------:|-------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-48xl        | avx2_blended |        ~16.6 |       ~110.4 |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx2_blended |        ~17.2 |        ~53.8 |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon_blended |        ~18.5 |        ~31.5 |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon_v12e_v1 |        ~56.5 |        ~98.8 |
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon_v12e_v1 |        ~59.2 |       ~105.3 |
| aarch64 | Apple | M2 Ultra        | Mac Studio (24 core)      | neon_blended |        ~50.1 |        ~87.0 |

### CRC-64/NVME

| Arch    | Brand | CPU             | System               | Target              | 1KiB (GiB/s) | 1MiB (GiB/s) |
|:--------|:------|:----------------|:---------------------|:--------------------|-------------:|-------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-24xl   | avx512_vpclmulqdq   |        ~24.9 |       ~109.7 |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl   | avx512_vpclmulqdq   |        ~24.4 |        ~54.6 |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl   | neon_pclmulqdq_eor3 |        ~18.7 |        ~36.8 |
| aarch64 | AWS   | Graviton2       | EC2 c6g.metal        | neon_pclmulqdq      |         ~9.8 |        ~15.9 |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core) | neon_pclmulqdq_eor3 |        ~49.5 |        ~71.9 |

## Other CRC widths

There are [a lot of other known CRC widths and variants](https://reveng.sourceforge.io/crc-catalogue/all.htm), ranging 
from `CRC-3/GSM` to `CRC-82/DARC`, and everything in between. 

Since [Awesome](https://awesome.co) doesn't use any that aren't `CRC-32` or `CRC-64` in length, this library doesn't 
currently support them, either. (It should support any newly created or discovered `CRC-32` and `CRC-64` variants, 
though, with zero changes other than defining the [Rocksoft](http://www.ross.net/crc/download/crc_v3.txt) parameters).

In theory, much of the "heavy lifting" has been done, so it should be possible to add other widths with minimal effort.

PRs welcome!

## Implementations
* [crc-fast-php-ext](https://github.com/awesomized/crc-fast-php-ext) `PHP` extension using this library.

## References

* [crc32-fast](https://crates.io/crates/crc32fast) Original `CRC-32/ISO-HDLC` (`crc32`) implementation in `Rust`.
* [crc64-fast](https://github.com/tikv/crc64fast) Original `CRC-64/XZ` implementation in `Rust`.
* [crc64fast-nvme](https://github.com/awesomized/crc64fast-nvme) Original `CRC-64/NVME` implementation in `Rust`.
* [Fast CRC Computation for Generic Polynomials Using PCLMULQDQ Instruction](https://web.archive.org/web/20131224125630/https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/fast-crc-computation-generic-polynomials-pclmulqdq-paper.pdf) 
  Intel's paper.
* [NVM Express® NVM Command Set Specification](https://nvmexpress.org/wp-content/uploads/NVM-Express-NVM-Command-Set-Specification-1.0d-2023.12.28-Ratified.pdf) 
  The NVMe spec, including `CRC-64-NVME` (with incorrect endian `Check` value in
  `Section 5.2.1.3.4, Figure 120, page 83`).
* [CRC-64/NVME](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-nvme) The `CRC-64/NVME` quick
  definition.
* [A PAINLESS GUIDE TO CRC ERROR DETECTION ALGORITHMS](http://www.ross.net/crc/download/crc_v3.txt) Best description of 
  CRC I've seen to date (and the definition of the Rocksoft model).
* [Linux implementation](https://github.com/torvalds/linux/blob/786c8248dbd33a5a7a07f7c6e55a7bfc68d2ca48/lib/crc64.c)
  Linux implementation of `CRC-64/NVME`.
* [MASM/C++ artifacts implementation](https://github.com/jeffareid/crc/) - Reference MASM/C++ implementation for 
  generating artifacts.
* [Intel isa-l GH issue #88](https://github.com/intel/isa-l/issues/88) - Additional insight into generating artifacts.
* [StackOverflow PCLMULQDQ CRC32 answer](https://stackoverflow.com/questions/71328336/fast-crc-with-pclmulqdq-not-reflected/71329114#71329114) 
  Insightful answer to implementation details for CRC32.
* [StackOverflow PCLMULQDQ CRC32 question](https://stackoverflow.com/questions/21171733/calculating-constants-for-crc32-using-pclmulqdq) 
  Insightful question & answer to CRC32 implementation details.
* [AWS S3 announcement about CRC64-NVME support](https://aws.amazon.com/blogs/aws/introducing-default-data-integrity-protections-for-new-objects-in-amazon-s3/)
* [AWS S3 docs on checking object integrity using CRC64-NVME](https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html)
* [Vector Carry-Less Multiplication of Quadwords (VPCLMULQDQ) details](https://en.wikichip.org/wiki/x86/vpclmulqdq)
* [Linux kernel updates by Eric Biggers to use VPCLMULQDQ, etc](https://lkml.org/lkml/2025/2/10/1367)

## License

`cfc-fast` is dual-licensed under

* Apache 2.0 license ([LICENSE-Apache](./LICENSE-Apache) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or <https://opensource.org/licenses/MIT>)