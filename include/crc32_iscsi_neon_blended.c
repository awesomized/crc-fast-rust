/* Generated by https://github.com/corsix/fast-crc32/ */
/* Modified post-generation to improve function names, include build targets,
   and bifurcate large (>1KiB) and small payloads for optimized performance */
/* MIT licensed */

#include <stddef.h>
#include <stdint.h>
#include <arm_acle.h>
#include <arm_neon.h>

#if defined(_MSC_VER)
#define CRC_AINLINE static __forceinline
#define CRC_ALIGN(n) __declspec(align(n))
#else
#define CRC_AINLINE static __inline __attribute__((always_inline))
#define CRC_ALIGN(n) __attribute__((aligned(n)))
#endif
#define CRC_EXPORT extern

const char *const ISCSI_TARGET = "aarch64_neon_blended";

const char *get_iscsi_target() {
    return ISCSI_TARGET;
}

CRC_AINLINE uint64x2_t clmul_lo_eor3(uint64x2_t a, uint64x2_t b) {
  uint64x2_t r;
  __asm("pmull %0.1q, %1.1d, %2.1d\n" : "=w"(r) : "w"(a), "w"(b));
  return r;
}

CRC_AINLINE uint64x2_t clmul_hi_eor3(uint64x2_t a, uint64x2_t b) {
  uint64x2_t r;
  __asm("pmull2 %0.1q, %1.2d, %2.2d\n" : "=w"(r) : "w"(a), "w"(b));
  return r;
}

CRC_AINLINE uint64x2_t clmul_scalar(uint32_t a, uint32_t b) {
  uint64x2_t r;
  __asm("pmull %0.1q, %1.1d, %2.1d\n" : "=w"(r) : "w"(vmovq_n_u64(a)), "w"(vmovq_n_u64(b)));
  return r;
}

static uint32_t xnmodp(uint64_t n) /* x^n mod P, in log(n) time */ {
  uint64_t stack = ~(uint64_t)1;
  uint32_t acc, low;
  for (; n > 191; n = (n >> 1) - 16) {
    stack = (stack << 1) + (n & 1);
  }
  stack = ~stack;
  acc = ((uint32_t)0x80000000) >> (n & 31);
  for (n >>= 5; n; --n) {
    acc = __crc32cw(acc, 0);
  }
  while ((low = stack & 1), stack >>= 1) {
    poly8x8_t x = vreinterpret_p8_u64(vmov_n_u64(acc));
    uint64_t y = vgetq_lane_u64(vreinterpretq_u64_p16(vmull_p8(x, x)), 0);
    acc = __crc32cd(0, y << low);
  }
  return acc;
}

CRC_AINLINE uint64x2_t crc_shift(uint32_t crc, size_t nbytes) {
  return clmul_scalar(crc, xnmodp(nbytes * 8 - 33));
}

CRC_AINLINE uint32_t crc32_iscsi_large_impl(uint32_t crc0, const char* buf, size_t len) {
  crc0 = ~crc0;
  for (; len && ((uintptr_t)buf & 7); --len) {
    crc0 = __crc32cb(crc0, *buf++);
  }
  if (((uintptr_t)buf & 8) && len >= 8) {
    crc0 = __crc32cd(crc0, *(const uint64_t*)buf);
    buf += 8;
    len -= 8;
  }
  if (len >= 192) {
    const char* end = buf + len;
    size_t blk = (len - 0) / 192;
    size_t klen = blk * 16;
    const char* buf2 = buf + klen * 3;
    const char* limit = buf + klen - 32;
    uint32_t crc1 = 0;
    uint32_t crc2 = 0;
    uint64x2_t vc0;
    uint64x2_t vc1;
    uint64x2_t vc2;
    uint64_t vc;
    /* First vector chunk. */
    uint64x2_t x0 = vld1q_u64((const uint64_t*)buf2), y0;
    uint64x2_t x1 = vld1q_u64((const uint64_t*)(buf2 + 16)), y1;
    uint64x2_t x2 = vld1q_u64((const uint64_t*)(buf2 + 32)), y2;
    uint64x2_t x3 = vld1q_u64((const uint64_t*)(buf2 + 48)), y3;
    uint64x2_t x4 = vld1q_u64((const uint64_t*)(buf2 + 64)), y4;
    uint64x2_t x5 = vld1q_u64((const uint64_t*)(buf2 + 80)), y5;
    uint64x2_t x6 = vld1q_u64((const uint64_t*)(buf2 + 96)), y6;
    uint64x2_t x7 = vld1q_u64((const uint64_t*)(buf2 + 112)), y7;
    uint64x2_t x8 = vld1q_u64((const uint64_t*)(buf2 + 128)), y8;
    uint64x2_t k;
    { static const uint64_t CRC_ALIGN(16) k_[] = {0x7e908048, 0xc96cfdc0}; k = vld1q_u64(k_); }
    buf2 += 144;
    /* Main loop. */
    while (buf <= limit) {
      y0 = clmul_lo_eor3(x0, k), x0 = clmul_hi_eor3(x0, k);
      y1 = clmul_lo_eor3(x1, k), x1 = clmul_hi_eor3(x1, k);
      y2 = clmul_lo_eor3(x2, k), x2 = clmul_hi_eor3(x2, k);
      y3 = clmul_lo_eor3(x3, k), x3 = clmul_hi_eor3(x3, k);
      y4 = clmul_lo_eor3(x4, k), x4 = clmul_hi_eor3(x4, k);
      y5 = clmul_lo_eor3(x5, k), x5 = clmul_hi_eor3(x5, k);
      y6 = clmul_lo_eor3(x6, k), x6 = clmul_hi_eor3(x6, k);
      y7 = clmul_lo_eor3(x7, k), x7 = clmul_hi_eor3(x7, k);
      y8 = clmul_lo_eor3(x8, k), x8 = clmul_hi_eor3(x8, k);
      x0 = veor3q_u64(x0, y0, vld1q_u64((const uint64_t*)buf2));
      x1 = veor3q_u64(x1, y1, vld1q_u64((const uint64_t*)(buf2 + 16)));
      x2 = veor3q_u64(x2, y2, vld1q_u64((const uint64_t*)(buf2 + 32)));
      x3 = veor3q_u64(x3, y3, vld1q_u64((const uint64_t*)(buf2 + 48)));
      x4 = veor3q_u64(x4, y4, vld1q_u64((const uint64_t*)(buf2 + 64)));
      x5 = veor3q_u64(x5, y5, vld1q_u64((const uint64_t*)(buf2 + 80)));
      x6 = veor3q_u64(x6, y6, vld1q_u64((const uint64_t*)(buf2 + 96)));
      x7 = veor3q_u64(x7, y7, vld1q_u64((const uint64_t*)(buf2 + 112)));
      x8 = veor3q_u64(x8, y8, vld1q_u64((const uint64_t*)(buf2 + 128)));
      crc0 = __crc32cd(crc0, *(const uint64_t*)buf);
      crc1 = __crc32cd(crc1, *(const uint64_t*)(buf + klen));
      crc2 = __crc32cd(crc2, *(const uint64_t*)(buf + klen * 2));
      crc0 = __crc32cd(crc0, *(const uint64_t*)(buf + 8));
      crc1 = __crc32cd(crc1, *(const uint64_t*)(buf + klen + 8));
      crc2 = __crc32cd(crc2, *(const uint64_t*)(buf + klen * 2 + 8));
      buf += 16;
      buf2 += 144;
    }
    /* Reduce x0 ... x8 to just x0. */
    { static const uint64_t CRC_ALIGN(16) k_[] = {0xf20c0dfe, 0x493c7d27}; k = vld1q_u64(k_); }
    y0 = clmul_lo_eor3(x0, k), x0 = clmul_hi_eor3(x0, k);
    x0 = veor3q_u64(x0, y0, x1);
    x1 = x2, x2 = x3, x3 = x4, x4 = x5, x5 = x6, x6 = x7, x7 = x8;
    y0 = clmul_lo_eor3(x0, k), x0 = clmul_hi_eor3(x0, k);
    y2 = clmul_lo_eor3(x2, k), x2 = clmul_hi_eor3(x2, k);
    y4 = clmul_lo_eor3(x4, k), x4 = clmul_hi_eor3(x4, k);
    y6 = clmul_lo_eor3(x6, k), x6 = clmul_hi_eor3(x6, k);
    x0 = veor3q_u64(x0, y0, x1);
    x2 = veor3q_u64(x2, y2, x3);
    x4 = veor3q_u64(x4, y4, x5);
    x6 = veor3q_u64(x6, y6, x7);
    { static const uint64_t CRC_ALIGN(16) k_[] = {0x3da6d0cb, 0xba4fc28e}; k = vld1q_u64(k_); }
    y0 = clmul_lo_eor3(x0, k), x0 = clmul_hi_eor3(x0, k);
    y4 = clmul_lo_eor3(x4, k), x4 = clmul_hi_eor3(x4, k);
    x0 = veor3q_u64(x0, y0, x2);
    x4 = veor3q_u64(x4, y4, x6);
    { static const uint64_t CRC_ALIGN(16) k_[] = {0x740eef02, 0x9e4addf8}; k = vld1q_u64(k_); }
    y0 = clmul_lo_eor3(x0, k), x0 = clmul_hi_eor3(x0, k);
    x0 = veor3q_u64(x0, y0, x4);
    /* Final scalar chunk. */
    crc0 = __crc32cd(crc0, *(const uint64_t*)buf);
    crc1 = __crc32cd(crc1, *(const uint64_t*)(buf + klen));
    crc2 = __crc32cd(crc2, *(const uint64_t*)(buf + klen * 2));
    crc0 = __crc32cd(crc0, *(const uint64_t*)(buf + 8));
    crc1 = __crc32cd(crc1, *(const uint64_t*)(buf + klen + 8));
    crc2 = __crc32cd(crc2, *(const uint64_t*)(buf + klen * 2 + 8));
    vc0 = crc_shift(crc0, klen * 2 + blk * 144);
    vc1 = crc_shift(crc1, klen + blk * 144);
    vc2 = crc_shift(crc2, 0 + blk * 144);
    vc = vgetq_lane_u64(veor3q_u64(vc0, vc1, vc2), 0);
    /* Reduce 128 bits to 32 bits, and multiply by x^32. */
    crc0 = __crc32cd(0, vgetq_lane_u64(x0, 0));
    crc0 = __crc32cd(crc0, vc ^ vgetq_lane_u64(x0, 1));
    buf = buf2;
    len = end - buf;
  }
  if (len >= 32) {
    size_t klen = ((len - 8) / 24) * 8;
    uint32_t crc1 = 0;
    uint32_t crc2 = 0;
    uint64x2_t vc0;
    uint64x2_t vc1;
    uint64_t vc;
    /* Main loop. */
    do {
      crc0 = __crc32cd(crc0, *(const uint64_t*)buf);
      crc1 = __crc32cd(crc1, *(const uint64_t*)(buf + klen));
      crc2 = __crc32cd(crc2, *(const uint64_t*)(buf + klen * 2));
      buf += 8;
      len -= 24;
    } while (len >= 32);
    vc0 = crc_shift(crc0, klen * 2 + 8);
    vc1 = crc_shift(crc1, klen + 8);
    vc = vgetq_lane_u64(veorq_u64(vc0, vc1), 0);
    /* Final 8 bytes. */
    buf += klen * 2;
    crc0 = crc2;
    crc0 = __crc32cd(crc0, *(const uint64_t*)buf ^ vc), buf += 8;
    len -= 8;
  }
  for (; len >= 8; buf += 8, len -= 8) {
    crc0 = __crc32cd(crc0, *(const uint64_t*)buf);
  }
  for (; len; --len) {
    crc0 = __crc32cb(crc0, *buf++);
  }
  return ~crc0;
}


CRC_AINLINE uint64x2_t clmul_lo_e(uint64x2_t a, uint64x2_t b, uint64x2_t c) {
  uint64x2_t r;
  __asm("pmull %0.1q, %2.1d, %3.1d\neor %0.16b, %0.16b, %1.16b\n" : "=w"(r), "+w"(c) : "w"(a), "w"(b));
  return r;
}

CRC_AINLINE uint64x2_t clmul_hi_e(uint64x2_t a, uint64x2_t b, uint64x2_t c) {
  uint64x2_t r;
  __asm("pmull2 %0.1q, %2.2d, %3.2d\neor %0.16b, %0.16b, %1.16b\n" : "=w"(r), "+w"(c) : "w"(a), "w"(b));
  return r;
}

CRC_AINLINE uint32_t crc32_iscsi_small_impl(uint32_t crc0, const char* buf, size_t len) {
  crc0 = ~crc0;
  for (; len && ((uintptr_t)buf & 7); --len) {
    crc0 = __crc32cb(crc0, *buf++);
  }
  if (((uintptr_t)buf & 8) && len >= 8) {
    crc0 = __crc32cd(crc0, *(const uint64_t*)buf);
    buf += 8;
    len -= 8;
  }
  if (len >= 192) {
    const char* end = buf + len;
    const char* limit = buf + len - 192;
    /* First vector chunk. */
    uint64x2_t x0 = vld1q_u64((const uint64_t*)buf), y0;
    uint64x2_t x1 = vld1q_u64((const uint64_t*)(buf + 16)), y1;
    uint64x2_t x2 = vld1q_u64((const uint64_t*)(buf + 32)), y2;
    uint64x2_t x3 = vld1q_u64((const uint64_t*)(buf + 48)), y3;
    uint64x2_t x4 = vld1q_u64((const uint64_t*)(buf + 64)), y4;
    uint64x2_t x5 = vld1q_u64((const uint64_t*)(buf + 80)), y5;
    uint64x2_t x6 = vld1q_u64((const uint64_t*)(buf + 96)), y6;
    uint64x2_t x7 = vld1q_u64((const uint64_t*)(buf + 112)), y7;
    uint64x2_t x8 = vld1q_u64((const uint64_t*)(buf + 128)), y8;
    uint64x2_t x9 = vld1q_u64((const uint64_t*)(buf + 144)), y9;
    uint64x2_t x10 = vld1q_u64((const uint64_t*)(buf + 160)), y10;
    uint64x2_t x11 = vld1q_u64((const uint64_t*)(buf + 176)), y11;
    uint64x2_t k;
    { static const uint64_t CRC_ALIGN(16) k_[] = {0xa87ab8a8, 0xab7aff2a}; k = vld1q_u64(k_); }
    x0 = veorq_u64((uint64x2_t){crc0, 0}, x0);
    buf += 192;
    /* Main loop. */
    while (buf <= limit) {
      y0 = clmul_lo_e(x0, k, vld1q_u64((const uint64_t*)buf)), x0 = clmul_hi_e(x0, k, y0);
      y1 = clmul_lo_e(x1, k, vld1q_u64((const uint64_t*)(buf + 16))), x1 = clmul_hi_e(x1, k, y1);
      y2 = clmul_lo_e(x2, k, vld1q_u64((const uint64_t*)(buf + 32))), x2 = clmul_hi_e(x2, k, y2);
      y3 = clmul_lo_e(x3, k, vld1q_u64((const uint64_t*)(buf + 48))), x3 = clmul_hi_e(x3, k, y3);
      y4 = clmul_lo_e(x4, k, vld1q_u64((const uint64_t*)(buf + 64))), x4 = clmul_hi_e(x4, k, y4);
      y5 = clmul_lo_e(x5, k, vld1q_u64((const uint64_t*)(buf + 80))), x5 = clmul_hi_e(x5, k, y5);
      y6 = clmul_lo_e(x6, k, vld1q_u64((const uint64_t*)(buf + 96))), x6 = clmul_hi_e(x6, k, y6);
      y7 = clmul_lo_e(x7, k, vld1q_u64((const uint64_t*)(buf + 112))), x7 = clmul_hi_e(x7, k, y7);
      y8 = clmul_lo_e(x8, k, vld1q_u64((const uint64_t*)(buf + 128))), x8 = clmul_hi_e(x8, k, y8);
      y9 = clmul_lo_e(x9, k, vld1q_u64((const uint64_t*)(buf + 144))), x9 = clmul_hi_e(x9, k, y9);
      y10 = clmul_lo_e(x10, k, vld1q_u64((const uint64_t*)(buf + 160))), x10 = clmul_hi_e(x10, k, y10);
      y11 = clmul_lo_e(x11, k, vld1q_u64((const uint64_t*)(buf + 176))), x11 = clmul_hi_e(x11, k, y11);
      buf += 192;
    }
    /* Reduce x0 ... x11 to just x0. */
    { static const uint64_t CRC_ALIGN(16) k_[] = {0xf20c0dfe, 0x493c7d27}; k = vld1q_u64(k_); }
    y0 = clmul_lo_e(x0, k, x1), x0 = clmul_hi_e(x0, k, y0);
    y2 = clmul_lo_e(x2, k, x3), x2 = clmul_hi_e(x2, k, y2);
    y4 = clmul_lo_e(x4, k, x5), x4 = clmul_hi_e(x4, k, y4);
    y6 = clmul_lo_e(x6, k, x7), x6 = clmul_hi_e(x6, k, y6);
    y8 = clmul_lo_e(x8, k, x9), x8 = clmul_hi_e(x8, k, y8);
    y10 = clmul_lo_e(x10, k, x11), x10 = clmul_hi_e(x10, k, y10);
    { static const uint64_t CRC_ALIGN(16) k_[] = {0x3da6d0cb, 0xba4fc28e}; k = vld1q_u64(k_); }
    y0 = clmul_lo_e(x0, k, x2), x0 = clmul_hi_e(x0, k, y0);
    y4 = clmul_lo_e(x4, k, x6), x4 = clmul_hi_e(x4, k, y4);
    y8 = clmul_lo_e(x8, k, x10), x8 = clmul_hi_e(x8, k, y8);
    { static const uint64_t CRC_ALIGN(16) k_[] = {0x740eef02, 0x9e4addf8}; k = vld1q_u64(k_); }
    y0 = clmul_lo_e(x0, k, x4), x0 = clmul_hi_e(x0, k, y0);
    x4 = x8;
    y0 = clmul_lo_e(x0, k, x4), x0 = clmul_hi_e(x0, k, y0);
    /* Reduce 128 bits to 32 bits, and multiply by x^32. */
    crc0 = __crc32cd(0, vgetq_lane_u64(x0, 0));
    crc0 = __crc32cd(crc0, vgetq_lane_u64(x0, 1));
    len = end - buf;
  }
  if (len >= 16) {
    /* First vector chunk. */
    uint64x2_t x0 = vld1q_u64((const uint64_t*)buf), y0;
    uint64x2_t k;
    { static const uint64_t CRC_ALIGN(16) k_[] = {0xf20c0dfe, 0x493c7d27}; k = vld1q_u64(k_); }
    x0 = veorq_u64((uint64x2_t){crc0, 0}, x0);
    buf += 16;
    len -= 16;
    /* Main loop. */
    while (len >= 16) {
      y0 = clmul_lo_e(x0, k, vld1q_u64((const uint64_t*)buf)), x0 = clmul_hi_e(x0, k, y0);
      buf += 16;
      len -= 16;
    }
    /* Reduce 128 bits to 32 bits, and multiply by x^32. */
    crc0 = __crc32cd(0, vgetq_lane_u64(x0, 0));
    crc0 = __crc32cd(crc0, vgetq_lane_u64(x0, 1));
  }
  for (; len >= 8; buf += 8, len -= 8) {
    crc0 = __crc32cd(crc0, *(const uint64_t*)buf);
  }
  for (; len; --len) {
    crc0 = __crc32cb(crc0, *buf++);
  }
  return ~crc0;
}

CRC_EXPORT uint32_t crc32_iscsi_impl(uint32_t crc0, const char* buf, size_t len) {
    // Define 1 KiB threshold (1024 bytes)
    const size_t LARGE_BUFFER_THRESHOLD = 1024;

    // Select implementation based on buffer size
    if (len <= LARGE_BUFFER_THRESHOLD) {
        return crc32_iscsi_small_impl(crc0, buf, len);
    } else {
        return crc32_iscsi_large_impl(crc0, buf, len);
    }
}
