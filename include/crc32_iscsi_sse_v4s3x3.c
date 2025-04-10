/* Generated by https://github.com/corsix/fast-crc32/ using: */
/* ./generate -i sse -p crc32c -a v4s3x3 */
/* Modified slightly post-generation to improve function name and include build target */
/* MIT licensed */
/* Modified for 32-bit compatibility */

#include <stddef.h>
#include <stdint.h>
#include <nmmintrin.h>
#include <wmmintrin.h>

#if defined(_MSC_VER)
#define CRC_AINLINE static __forceinline
#define CRC_ALIGN(n) __declspec(align(n))
#else
#define CRC_AINLINE static __inline __attribute__((always_inline))
#define CRC_ALIGN(n) __attribute__((aligned(n)))
#endif
#define CRC_EXPORT extern

#define clmul_lo(a, b) (_mm_clmulepi64_si128((a), (b), 0))
#define clmul_hi(a, b) (_mm_clmulepi64_si128((a), (b), 17))

const char *const ISCSI_TARGET = "x86_sse_v4s3x3";

const char *get_iscsi_target() {
    return ISCSI_TARGET;
}

/* Platform-specific 64-bit handling */
#if defined(__x86_64__) || defined(_M_X64)
/* 64-bit platform */
CRC_AINLINE __m128i mm_cvtsi64_si128(uint64_t val) {
  return _mm_cvtsi64_si128(val);
}

CRC_AINLINE uint64_t mm_cvtsi128_si64(__m128i val) {
  return _mm_cvtsi128_si64(val);
}

CRC_AINLINE uint64_t mm_extract_epi64(__m128i val, int idx) {
  /* Even on 64-bit platforms, we need to use constant indices */
  if (idx == 0) {
    return _mm_cvtsi128_si64(val);
  } else {
    /* For the high 64 bits */
    return _mm_cvtsi128_si64(_mm_srli_si128(val, 8));
  }
}

CRC_AINLINE uint32_t mm_crc32_u64(uint32_t crc, uint64_t val) {
  return _mm_crc32_u64(crc, val);
}
#else
/* 32-bit platform */
CRC_AINLINE __m128i mm_cvtsi64_si128(uint64_t val) {
  /* Split 64-bit value into two 32-bit parts for 32-bit platform */
  __m128i result, temp;
  result = _mm_cvtsi32_si128((uint32_t)val);  /* Low 32 bits */
  temp = _mm_cvtsi32_si128((uint32_t)(val >> 32));  /* High 32 bits */

  /* Shift high 32 bits to position 1 */
  temp = _mm_slli_si128(temp, 4);

  /* Combine low and high parts */
  result = _mm_or_si128(result, temp);
  return result;
}

CRC_AINLINE uint64_t mm_cvtsi128_si64(__m128i val) {
  /* Combine two 32-bit values into one 64-bit result */
  uint32_t low = _mm_cvtsi128_si32(val);
  uint32_t high = _mm_extract_epi32(val, 1);
  return ((uint64_t)high << 32) | low;
}

CRC_AINLINE uint64_t mm_extract_epi64(__m128i val, int idx) {
  /* Extract 64 bits (two 32-bit values) */
  uint32_t low, high;

  if (idx == 0) {
    low = _mm_cvtsi128_si32(val);
    high = _mm_extract_epi32(val, 1);
  } else {
    low = _mm_extract_epi32(val, 2);
    high = _mm_extract_epi32(val, 3);
  }

  return ((uint64_t)high << 32) | low;
}

CRC_AINLINE uint32_t mm_crc32_u64(uint32_t crc, uint64_t val) {
  /* Process 64-bit value in two 32-bit chunks on 32-bit platforms */
  crc = _mm_crc32_u32(crc, (uint32_t)val);
  crc = _mm_crc32_u32(crc, (uint32_t)(val >> 32));
  return crc;
}
#endif

CRC_AINLINE __m128i clmul_scalar(uint32_t a, uint32_t b) {
  return _mm_clmulepi64_si128(_mm_cvtsi32_si128(a), _mm_cvtsi32_si128(b), 0);
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
    acc = _mm_crc32_u32(acc, 0);
  }
  while ((low = stack & 1), stack >>= 1) {
    __m128i x = _mm_cvtsi32_si128(acc);
    uint64_t y = mm_cvtsi128_si64(_mm_clmulepi64_si128(x, x, 0));
    acc = mm_crc32_u64(0, y << low);
  }
  return acc;
}

CRC_AINLINE __m128i crc_shift(uint32_t crc, size_t nbytes) {
  return clmul_scalar(crc, xnmodp(nbytes * 8 - 33));
}

CRC_EXPORT uint32_t crc32_iscsi_impl(uint32_t crc0, const char* buf, size_t len) {
  crc0 = ~crc0;
  for (; len && ((uintptr_t)buf & 7); --len) {
    crc0 = _mm_crc32_u8(crc0, *buf++);
  }
  if (((uintptr_t)buf & 8) && len >= 8) {
    crc0 = mm_crc32_u64(crc0, *(const uint64_t*)buf);
    buf += 8;
    len -= 8;
  }
  if (len >= 144) {
    size_t blk = (len - 8) / 136;
    size_t klen = blk * 24;
    const char* buf2 = buf + 0;
    uint32_t crc1 = 0;
    uint32_t crc2 = 0;
    __m128i vc0;
    __m128i vc1;
    uint64_t vc;
    /* First vector chunk. */
    __m128i x0 = _mm_loadu_si128((const __m128i*)buf2), y0;
    __m128i x1 = _mm_loadu_si128((const __m128i*)(buf2 + 16)), y1;
    __m128i x2 = _mm_loadu_si128((const __m128i*)(buf2 + 32)), y2;
    __m128i x3 = _mm_loadu_si128((const __m128i*)(buf2 + 48)), y3;
    __m128i k;
    k = _mm_setr_epi32(0x740eef02, 0, 0x9e4addf8, 0);
    x0 = _mm_xor_si128(_mm_cvtsi32_si128(crc0), x0);
    crc0 = 0;
    buf2 += 64;
    len -= 136;
    buf += blk * 64;
    /* Main loop. */
    while (len >= 144) {
      y0 = clmul_lo(x0, k), x0 = clmul_hi(x0, k);
      y1 = clmul_lo(x1, k), x1 = clmul_hi(x1, k);
      y2 = clmul_lo(x2, k), x2 = clmul_hi(x2, k);
      y3 = clmul_lo(x3, k), x3 = clmul_hi(x3, k);
      y0 = _mm_xor_si128(y0, _mm_loadu_si128((const __m128i*)buf2)), x0 = _mm_xor_si128(x0, y0);
      y1 = _mm_xor_si128(y1, _mm_loadu_si128((const __m128i*)(buf2 + 16))), x1 = _mm_xor_si128(x1, y1);
      y2 = _mm_xor_si128(y2, _mm_loadu_si128((const __m128i*)(buf2 + 32))), x2 = _mm_xor_si128(x2, y2);
      y3 = _mm_xor_si128(y3, _mm_loadu_si128((const __m128i*)(buf2 + 48))), x3 = _mm_xor_si128(x3, y3);
      crc0 = mm_crc32_u64(crc0, *(const uint64_t*)buf);
      crc1 = mm_crc32_u64(crc1, *(const uint64_t*)(buf + klen));
      crc2 = mm_crc32_u64(crc2, *(const uint64_t*)(buf + klen * 2));
      crc0 = mm_crc32_u64(crc0, *(const uint64_t*)(buf + 8));
      crc1 = mm_crc32_u64(crc1, *(const uint64_t*)(buf + klen + 8));
      crc2 = mm_crc32_u64(crc2, *(const uint64_t*)(buf + klen * 2 + 8));
      crc0 = mm_crc32_u64(crc0, *(const uint64_t*)(buf + 16));
      crc1 = mm_crc32_u64(crc1, *(const uint64_t*)(buf + klen + 16));
      crc2 = mm_crc32_u64(crc2, *(const uint64_t*)(buf + klen * 2 + 16));
      buf += 24;
      buf2 += 64;
      len -= 136;
    }
    /* Reduce x0 ... x3 to just x0. */
    k = _mm_setr_epi32(0xf20c0dfe, 0, 0x493c7d27, 0);
    y0 = clmul_lo(x0, k), x0 = clmul_hi(x0, k);
    y2 = clmul_lo(x2, k), x2 = clmul_hi(x2, k);
    y0 = _mm_xor_si128(y0, x1), x0 = _mm_xor_si128(x0, y0);
    y2 = _mm_xor_si128(y2, x3), x2 = _mm_xor_si128(x2, y2);
    k = _mm_setr_epi32(0x3da6d0cb, 0, 0xba4fc28e, 0);
    y0 = clmul_lo(x0, k), x0 = clmul_hi(x0, k);
    y0 = _mm_xor_si128(y0, x2), x0 = _mm_xor_si128(x0, y0);
    /* Final scalar chunk. */
    crc0 = mm_crc32_u64(crc0, *(const uint64_t*)buf);
    crc1 = mm_crc32_u64(crc1, *(const uint64_t*)(buf + klen));
    crc2 = mm_crc32_u64(crc2, *(const uint64_t*)(buf + klen * 2));
    crc0 = mm_crc32_u64(crc0, *(const uint64_t*)(buf + 8));
    crc1 = mm_crc32_u64(crc1, *(const uint64_t*)(buf + klen + 8));
    crc2 = mm_crc32_u64(crc2, *(const uint64_t*)(buf + klen * 2 + 8));
    crc0 = mm_crc32_u64(crc0, *(const uint64_t*)(buf + 16));
    crc1 = mm_crc32_u64(crc1, *(const uint64_t*)(buf + klen + 16));
    crc2 = mm_crc32_u64(crc2, *(const uint64_t*)(buf + klen * 2 + 16));
    buf += 24;
    vc0 = crc_shift(crc0, klen * 2 + 8);
    vc1 = crc_shift(crc1, klen + 8);
    vc = mm_extract_epi64(_mm_xor_si128(vc0, vc1), 0);
    /* Reduce 128 bits to 32 bits, and multiply by x^32. */
    /* Extract the two 64-bit parts of x0 and combine them */
    uint64_t x0_low = mm_extract_epi64(x0, 0);
    uint64_t x0_high = mm_extract_epi64(x0, 1);
    uint64_t x0_combined = mm_extract_epi64(crc_shift(mm_crc32_u64(mm_crc32_u64(0, x0_low), x0_high), klen * 3 + 8), 0);
    vc ^= x0_combined;
    /* Final 8 bytes. */
    buf += klen * 2;
    crc0 = crc2;
    crc0 = mm_crc32_u64(crc0, *(const uint64_t*)buf ^ vc), buf += 8;
    len -= 8;
  }
  for (; len >= 8; buf += 8, len -= 8) {
    crc0 = mm_crc32_u64(crc0, *(const uint64_t*)buf);
  }
  for (; len; --len) {
    crc0 = _mm_crc32_u8(crc0, *buf++);
  }
  return ~crc0;
}