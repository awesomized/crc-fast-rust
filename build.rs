#![allow(dead_code)]
#![allow(unused)]

extern crate cc;

use cc::Build;
use std::env;

#[cfg(target_arch = "aarch64")]
use std::arch::is_aarch64_feature_detected;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use std::arch::is_x86_feature_detected;

fn main() {
    // Windows doesn't build the C bindings automatically, and since they're auto-generated from
    // another project, I'm not inclined to fix it. The Rust implementation is still very fast.
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-cfg=rust_implementation_only");
        return;
    }

    // build hardware optimized version
    build_optimized();
}

/// Builds hardware-optimized versions of the CRC32 functions
fn build_optimized() {
    // in build scripts, the target architecture is only available via an environment variable
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if "aarch64" == target_arch {
        return build_optimized_aarch64();
    }

    if "x86_64" == target_arch || "x86" == target_arch {
        return build_optimized_x86()
    }

    // fall back to Rust implementation
    println!("cargo:rustc-cfg=rust_implementation_only");
}

fn build_optimized_target_crc32_iscsi(name: &str, flags: &[String]) {
    build_optimized_target(name, flags);

    println!("cargo:rustc-cfg=optimized_crc32_iscsi");
}

fn build_optimized_target_crc32_iso_hdlc(name: &str, flags: &[String]) {
    build_optimized_target(name, flags);

    println!("cargo:rustc-cfg=optimized_crc32_iso_hdlc");
}

fn build_optimized_target(name: &str, flags: &[String]) {
    // Create a longer-lived binding as suggested by the error message
    let mut binding = Build::new();
    let mut build = binding.file(format!("include/{name}.c")).include("include");

    // Apply each flag individually
    for flag in flags {
        build = build.flag(flag);
    }

    build.compile(name);
}

fn build_optimized_aarch64() {
    // feature flag overrides to allow forcing a specific implementation

    // NEON EOR3, which seems to be faster for larger payloads,
    // but slower for smaller ones than v12e_v1
    #[cfg(feature = "optimize_crc32_neon_eor3_v9s3x2e_s3")]
    return build_neon_eor3_v9s3x2e_s3();

    // NEON w/o EOR3, tuned for Apple M1, which is MUCH faster at smaller payloads, and slightly
    // slower at larger ones, on my Apple M2 Ultra
    #[cfg(feature = "optimize_crc32_neon_v12e_v1")]
    return build_neon_v12e_v1();

    // NEON w/o EOR3, tuned for Ampere Altra Arm (GCP Tau T2A)
    #[cfg(feature = "optimize_crc32_neon_v3s4x2e_v2")]
    return build_neon_v3s4x2e_v2();

    // NEON w/EOR3 for large payloads (>1KiB), NEON w/o EOR3 for small ones
    #[cfg(feature = "optimize_crc32_neon_blended")]
    return build_neon_blended();

    // no auto-optimize enabled, return and use the internal Rust implementation
    #[cfg(feature = "optimize_crc32_auto")]
    {
        // for auto, default to NEON blended with EOR3 for large (>1KiB) payloads, w/o EOR3 for
        // small ones
        #[allow(unreachable_code)]
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        if is_aarch64_feature_detected!("crc") && is_aarch64_feature_detected!("sha3") {
            return build_neon_blended();
        }

        // for auto, fallback to non-EOR3 if SHA3 is not available
        #[allow(unreachable_code)]
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        if is_aarch64_feature_detected!("crc") {
            return build_neon_v12e_v1()
        }
    }

    // fall through to internal Rust implementation
    println!("cargo:rustc-cfg=rust_implementation_only");
}

fn build_neon_blended() {
    println!("Building NEON blended");

    let flags = [String::from("-march=armv8.2-a+crypto+crc+sha3")];

    build_optimized_target_crc32_iscsi("crc32_iscsi_neon_blended", &flags);
    build_optimized_target_crc32_iso_hdlc("crc32_iso_hdlc_neon_blended", &flags);
}

fn build_neon_eor3_v9s3x2e_s3() {
    println!("Building NEON EOR3 v9s3x2e s3");

    let flags = [String::from("-march=armv8.2-a+crypto+crc+sha3")];

    build_optimized_target_crc32_iscsi("crc32_iscsi_neon_eor3_v9s3x2e_s3", &flags);
    build_optimized_target_crc32_iso_hdlc("crc32_iso_hdlc_neon_eor3_v9s3x2e_s3", &flags);
}

fn build_neon_v12e_v1() {
    println!("Building NEON v12e v1");

    let flags = [String::from("-march=armv8-a+crypto+crc")];

    build_optimized_target_crc32_iscsi("crc32_iscsi_neon_v12e_v1", &flags);
    build_optimized_target_crc32_iso_hdlc("crc32_iso_hdlc_neon_v12e_v1", &flags);
}

fn build_neon_v3s4x2e_v2() {
    println!("Building NEON v12e v1");

    let flags = [String::from("-march=armv8-a+crypto+crc")];

    build_optimized_target_crc32_iscsi("crc32_iscsi_neon_v3s4x2e_v2", &flags);
    build_optimized_target_crc32_iso_hdlc("crc32_iso_hdlc_neon_v3s4x2e_v2", &flags);
}

fn build_optimized_x86() {
    // feature flag overrides to allow forcing a specific implementation

    #[cfg(feature = "optimize_crc32_avx512_vpclmulqdq_v3x2")]
    return build_avx512_vpclmulqdq_v3x2();

    #[cfg(feature = "optimize_crc32_avx512_v4s3x3")]
    return build_avx512_v4s3x3();

    #[cfg(feature = "optimize_crc32_sse_v4s3x3")]
    return build_sse_v4s3x3();

    // no auto-optimize enabled, return and use the internal Rust implementation
    #[cfg(feature = "optimize_crc32_auto")]
    {
        // for auto, default to the best available implementation based on CPU features

        // in build scripts, the target architecture is only available via an environment variable
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
        if "x86" == target_arch {
            // this is the only one supported on 32-bit x86 systems
            return crate::build_sse_v4s3x3()
        }

        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        if is_x86_feature_detected!("vpclmulqdq")
            && is_x86_feature_detected!("avx512vl")
            && is_x86_feature_detected!("avx512f")
        {
            return build_avx512_vpclmulqdq_v3x2();
        }

        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        if is_x86_feature_detected!("avx512vl")
            && is_x86_feature_detected!("avx512f")
            && is_x86_feature_detected!("pclmulqdq")
        {
            return crate::build_avx512_v4s3x3();
        }

        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        if is_x86_feature_detected!("sse4.2") && is_x86_feature_detected!("pclmulqdq") {
            return crate::build_sse_v4s3x3()
        }
    }

    // fall through to internal Rust implementation
    println!("cargo:rustc-cfg=rust_implementation_only");
}

fn build_avx512_vpclmulqdq_v3x2() {
    println!("Building AVX512 VPCLMULQDQ v3x2");

    let flags = [
        String::from("-msse4.2"),
        String::from("-mpclmul"),
        String::from("-mavx512f"),
        String::from("-mavx512vl"),
        String::from("-mvpclmulqdq"),
    ];

    build_optimized_target_crc32_iscsi("crc32_iscsi_avx512_vpclmulqdq_v3x2", &flags);
    build_optimized_target_crc32_iso_hdlc("crc32_iso_hdlc_avx512_vpclmulqdq_v3x2", &flags);
}

fn build_avx512_v4s3x3() {
    println!("Building AVX512 v4s3x3");

    let flags = [
        String::from("-msse4.2"),
        String::from("-mpclmul"),
        String::from("-mavx512f"),
        String::from("-mavx512vl"),
    ];

    build_optimized_target_crc32_iscsi("crc32_iscsi_avx512_v4s3x3", &flags);
    build_optimized_target_crc32_iso_hdlc("crc32_iso_hdlc_avx512_v4s3x3", &flags);
}

fn build_sse_v4s3x3() {
    println!("Building SSE v4s3x3 for x86 / x86_64");

    let flags = [String::from("-msse4.2"), String::from("-mpclmul")];

    build_optimized_target_crc32_iscsi("crc32_iscsi_sse_v4s3x3", &flags);
    build_optimized_target_crc32_iso_hdlc("crc32_iso_hdlc_sse_v4s3x3", &flags);
}
