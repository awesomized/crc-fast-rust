# CRC64-NVMe combination benchmarks

Repeated CRC64-NVMe combines rebuild GF(2) shift matrices for every fragment. The cached path constructs the 64 zero-byte operators once, then
applies only the operators selected by the second fragment's length. It is used
by `checksum_combine`, `checksum_combine_with_params`, `Digest::combine`, and the
corresponding FFI calls for the reflected NVMe polynomial.

Upstream's existing cache stores 23 fixed SIMD folding keys for `CrcParams::new()`.
Its static slicing tables process payload bytes, and its CPU cache selects a
hardware implementation. All three Rust combine entry points and their FFI
wrappers instead reach `combine::checksums`, which rebuilds two matrices on every
call. None of those existing caches supplies the length operators needed here.

The cached and generic paths share a matrix-vector multiplication helper that
visits only set bits, avoiding the old branch for every bit of the CRC state.

## Method

The baseline is upstream `3a853cc7daf2cd47cc4466f198680cabdfb0b5fa` (1.10.0), with
the PR benchmark harness (originally added in
`aa46eff2d1967a9e35ffe217259ed6c950a11fa9`).
Both versions use the same harness, locked dependencies, default features, and
release profile (optimization level 3, LTO, one codegen unit). There are no custom
`RUSTFLAGS` or CPU tuning flags.

Measurements were taken on an Apple M2 Max (aarch64), 32 GiB RAM, macOS 26.6.2,
using rustc 1.98.1 (LLVM 22.1.8), on 2026-09-12. These are native measurements on
one machine; they do not establish performance on x86 or embedded targets.

The Criterion 0.7 group uses 100 samples, a 1-second warmup, and a 3-second target
measurement time per case. Algorithm, checksums, and length pass through
`black_box`; Criterion consumes the result. Inputs are precomputed synthetic CRC
values, so the timed region excludes payload hashing and allocation. The fold
case performs 256 dependent combines with a changing accumulator. Power-of-two
lengths exercise one operator; 16 KiB minus one and `u64::MAX` exercise many. The
CRC32-ISCSI and forward CRC64-ECMA182 cases exercise the generic path, which
also benefits from visiting only set bits during matrix multiplication.

## Results

Warmed Criterion estimates; brackets are 95% confidence intervals in the same
unit as the estimate. The point estimate is Criterion's slope for linear
sampling, or mean for flat sampling. Speedup divides the before/after point
estimates. The measured library implementation is
`b1972ea5963045034d2d7eb44b21fc34bd44f644`. All timings measure composition of
supplied CRC values, with no payload reading.

| Case | Before | After | Speedup |
|---|---:|---:|---:|
| NVMe, zero length | 2.664 us [2.657, 2.673] | 10.855 ns [10.831, 10.882] | 245.4x |
| NVMe, 1 B | 2.719 us [2.709, 2.730] | 22.952 ns [22.669, 23.467] | 118.5x |
| NVMe, 1 KiB | 28.223 us [27.825, 28.609] | 22.707 ns [22.662, 22.750] | 1,242.9x |
| NVMe, 16 KiB - 1 | 47.365 us [46.858, 47.888] | 312.665 ns [311.737, 313.649] | 151.5x |
| NVMe, 16 KiB | 53.475 us [53.048, 53.933] | 22.910 ns [22.863, 22.955] | 2,334.1x |
| NVMe, 1 MiB | 97.358 us [96.928, 97.792] | 22.796 ns [22.710, 22.881] | 4,270.8x |
| NVMe, 1 GiB | 200.833 us [199.908, 201.735] | 22.856 ns [22.807, 22.912] | 8,786.7x |
| NVMe, 2^63 B (synthetic) | 457.364 us [456.121, 458.837] | 22.770 ns [22.722, 22.823] | 20,086.3x |
| NVMe, u64::MAX B (synthetic) | 467.995 us [466.356, 469.689] | 1.543 us [1.540, 1.545] | 303.4x |
| NVMe, empty first CRC | 53.204 us [52.801, 53.625] | 10.630 ns [10.612, 10.648] | 5,005.1x |
| NVMe, 256 x 16 KiB combines | 13.605 ms [13.501, 13.707] | 10.120 us [10.025, 10.266] | 1,344.3x |
| CRC32-ISCSI, 16 KiB | 9.270 us [9.231, 9.306] | 3.579 us [3.560, 3.598] | 2.6x |
| CRC64-ECMA182, 16 KiB | 53.552 us [53.112, 53.970] | 17.077 us [17.018, 17.140] | 3.1x |

## First-call cost and storage

Fresh-process medians, with 100 samples per build and length; process order was
alternated to reduce ordering bias. This includes initialization on every sample.

| Second length | Before | After |
|---|---:|---:|
| 0 B | 5.334 us | 0.791 us |
| 1 B | 5.584 us | 99.813 us |
| 16 KiB | 93.771 us | 96.000 us |

A one-off 1-byte combine is about 18x slower. Using these cold medians and
warmed latencies, the estimated break-even is about 36 total 1-byte combines
per process. At 16 KiB, the first call is about 96 us versus 94 us, and the
second call already recovers that difference. These estimates are specific to
this machine and workload; the optimization targets repeated composition.

The cache retains 32 KiB of matrices plus synchronization metadata. It uses static
storage, no heap allocation, and initializes on the first call with a nonzero
adjusted first CRC and nonzero second length. Empty cases avoid initialization.
`std` builds enable it automatically. `no_std` builds enable it with the existing
`cache` feature; without that feature they continue using the generic algorithm.
The operators are shared by matching custom parameters, including different
initial and final XOR values. Other polynomials keep the existing generic path.

## Reproduce

From the PR checkout, create a separate worktree containing the identical harness
and the upstream implementation. Keep Cargo's build directories separate and
share only Criterion's result directory:

```sh
git worktree add --detach ../crc-fast-before aa46eff2d1967a9e35ffe217259ed6c950a11fa9
cp benches/benchmark.rs ../crc-fast-before/benches/benchmark.rs
cp examples/combine_first_call.rs ../crc-fast-before/examples/combine_first_call.rs
CRITERION_HOME="$PWD/target/criterion" cargo bench --locked \
  --manifest-path ../crc-fast-before/Cargo.toml --bench benchmark -- \
  combine --save-baseline before --noplot
CRITERION_HOME="$PWD/target/criterion" cargo bench --locked \
  --bench benchmark -- combine --baseline before --noplot
```

Run the first-call probe in a fresh process for every sample. The timer is inside
the process, around the first public API call and consumption of its result;
it excludes process startup and printing. Alternate the two executables:

```sh
cargo build --locked --release \
  --manifest-path ../crc-fast-before/Cargo.toml --example combine_first_call
cp ../crc-fast-before/target/release/examples/combine_first_call /tmp/combine-before
cargo build --locked --release --example combine_first_call
cp target/release/examples/combine_first_call /tmp/combine-after
python3 - <<'PYTHON'
import statistics
import subprocess

for length in (0, 1, 16384):
    samples = {"before": [], "after": []}
    for i in range(100):
        order = ("before", "after") if i % 2 == 0 else ("after", "before")
        for version in order:
            samples[version].append(int(subprocess.check_output([
                "/tmp/combine-" + version, str(length)
            ])))
    for version, values in samples.items():
        print(version, length, "median ns:", statistics.median(values),
              "range ns:", min(values), max(values))
PYTHON
```

Run builds and benchmarks serially so compiler jobs do not compete with timed
work. First-call and warmed Criterion measurements are separate experiments;
first-call timings include cold code/data access and have more process-to-process
variation than the warmed measurements.
