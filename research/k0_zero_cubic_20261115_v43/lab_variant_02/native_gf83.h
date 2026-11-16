#ifndef ROOTFINDER_LAB_VARIANT_02_NATIVE_GF83_H
#define ROOTFINDER_LAB_VARIANT_02_NATIVE_GF83_H

// Exact GF(2^83) arithmetic for f(T)=T^83+T^45+T^2+T+1.
// Elements are canonical polynomial-basis integers in [0,2^83).
// Intrinsic functions require pclmul_available(); mul/square dispatch safely.
// No -mpclmul compilation flag is required. The modulus must be independently
// certified by the caller: this header performs arithmetic, not irreducibility
// certification. No benchmark or algorithmic performance claim is implied.

#include <cassert>
#include <cstdint>

#if (defined(__x86_64__) || defined(__i386__)) && (defined(__GNUC__) || defined(__clang__))
#define ROOTFINDER_GF83_HAS_PCLMUL_INTRINSICS 1
#include <immintrin.h>
#else
#define ROOTFINDER_GF83_HAS_PCLMUL_INTRINSICS 0
#endif

namespace gf83 {

using Element = unsigned __int128;
constexpr Element mask = (Element(1) << 83) - 1;
constexpr Element reduction_low = (Element(1) << 45) | Element(7);
constexpr Element modulus = (Element(1) << 83) | reduction_low;

inline bool canonical(Element value) { return (value & ~mask) == 0; }

// A product of two degree-at-most-82 polynomials has degree at most164.
// Split it as lo+hi*T^128, then use T^83=T^45+T^2+T+1.
// Successive fold degree ceilings are126,88,82, so exactly three folds
// suffice. All intermediate shifts fit within an unsigned128-bit integer.
inline Element reduce_product(Element lo, std::uint64_t hi) {
    assert((hi >> 37) == 0);
    Element high = (lo >> 83) | (Element(hi) << 45);
    Element value = (lo & mask) ^ high ^ (high << 1) ^ (high << 2) ^ (high << 45);
    high = value >> 83;
    value = (value & mask) ^ high ^ (high << 1) ^ (high << 2) ^ (high << 45);
    high = value >> 83;
    value = (value & mask) ^ high ^ (high << 1) ^ (high << 2) ^ (high << 45);
    assert(canonical(value));
    return value;
}

// Independent shift-and-reduce multiplication, portable to targets with
// unsigned128-bit integers. This does not share product reduction code.
inline Element mul_portable(Element left, Element right) {
    assert(canonical(left) && canonical(right));
    Element result = 0;
    while (right) {
        if (right & 1) result ^= left;
        right >>= 1;
        const bool carry = (left >> 82) != 0;
        left = (left << 1) & mask;
        if (carry) left ^= reduction_low;
    }
    return result;
}

inline std::uint64_t spread32(std::uint32_t source) {
    std::uint64_t value = source;
    value = (value | (value << 16)) & UINT64_C(0x0000ffff0000ffff);
    value = (value | (value << 8)) & UINT64_C(0x00ff00ff00ff00ff);
    value = (value | (value << 4)) & UINT64_C(0x0f0f0f0f0f0f0f0f);
    value = (value | (value << 2)) & UINT64_C(0x3333333333333333);
    return (value | (value << 1)) & UINT64_C(0x5555555555555555);
}

inline Element square_portable(Element value) {
    assert(canonical(value));
    const std::uint64_t lo = std::uint64_t(value);
    const std::uint32_t hi = std::uint32_t(value >> 64);
    const Element product_lo = Element(spread32(std::uint32_t(lo))) |
                               (Element(spread32(std::uint32_t(lo >> 32))) << 64);
    return reduce_product(product_lo, spread32(hi));
}

inline bool pclmul_available() {
#if ROOTFINDER_GF83_HAS_PCLMUL_INTRINSICS
    static const bool supported = []() {
        __builtin_cpu_init();
        return bool(__builtin_cpu_supports("pclmul"));
    }();
    return supported;
#else
    return false;
#endif
}

#if ROOTFINDER_GF83_HAS_PCLMUL_INTRINSICS
__attribute__((target("pclmul,sse2")))
inline Element mul_pclmul(Element left, Element right) {
    assert(canonical(left) && canonical(right));
    const std::uint64_t left_lo = std::uint64_t(left), left_hi = std::uint64_t(left >> 64);
    const std::uint64_t right_lo = std::uint64_t(right), right_hi = std::uint64_t(right >> 64);
    const __m128i a = _mm_set_epi64x(left_hi, left_lo);
    const __m128i b = _mm_set_epi64x(right_hi, right_lo);
    const __m128i p00 = _mm_clmulepi64_si128(a, b, 0x00);
    const __m128i p11 = _mm_clmulepi64_si128(a, b, 0x11);
    const __m128i mixed_a = _mm_set_epi64x(0, left_lo ^ left_hi);
    const __m128i mixed_b = _mm_set_epi64x(0, right_lo ^ right_hi);
    const __m128i cross = _mm_xor_si128(_mm_xor_si128(
        _mm_clmulepi64_si128(mixed_a, mixed_b, 0x00), p00), p11);
    alignas(16) std::uint64_t low_words[2], high_words[2], cross_words[2];
    _mm_store_si128(reinterpret_cast<__m128i*>(low_words), p00);
    _mm_store_si128(reinterpret_cast<__m128i*>(high_words), p11);
    _mm_store_si128(reinterpret_cast<__m128i*>(cross_words), cross);
    assert(high_words[1] == 0);
    const Element product_lo = Element(low_words[0]) |
        (Element(low_words[1] ^ cross_words[0]) << 64);
    return reduce_product(product_lo, high_words[0] ^ cross_words[1]);
}

__attribute__((target("pclmul,sse2")))
inline Element square_pclmul(Element value) {
    assert(canonical(value));
    const __m128i input = _mm_set_epi64x(std::uint64_t(value >> 64), std::uint64_t(value));
    const __m128i p00 = _mm_clmulepi64_si128(input, input, 0x00);
    const __m128i p11 = _mm_clmulepi64_si128(input, input, 0x11);
    alignas(16) std::uint64_t low_words[2], high_words[2];
    _mm_store_si128(reinterpret_cast<__m128i*>(low_words), p00);
    _mm_store_si128(reinterpret_cast<__m128i*>(high_words), p11);
    assert(high_words[1] == 0);
    return reduce_product(Element(low_words[0]) | (Element(low_words[1]) << 64), high_words[0]);
}
#else
inline Element mul_pclmul(Element left, Element right) { return mul_portable(left, right); }
inline Element square_pclmul(Element value) { return square_portable(value); }
#endif

inline Element mul(Element left, Element right) {
    return pclmul_available() ? mul_pclmul(left, right) : mul_portable(left, right);
}

inline Element square(Element value) {
    return pclmul_available() ? square_pclmul(value) : square_portable(value);
}

}  // namespace gf83

#endif
