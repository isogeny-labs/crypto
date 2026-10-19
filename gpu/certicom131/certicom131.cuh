#ifndef CERTICOM131_ARITHMETIC_CUH
#define CERTICOM131_ARITHMETIC_CUH

#include <stdint.h>

#include "params131.h"

#if defined(__CUDACC__)
#define C131_HD __host__ __device__ __forceinline__
#else
#define C131_HD inline
#endif

namespace certicom131 {

struct U160 {
    uint32_t v[5];
};

struct Fp {
    uint32_t v[5];
};

static constexpr Fp FP_MOD = P131_P;
static constexpr Fp FP_R1 = P131_R1;
static constexpr Fp FP_R2 = P131_R2;
static constexpr Fp FP_PM2 = P131_PM2;
static constexpr Fp FP_A_CANON = P131_A;
static constexpr Fp FP_B_CANON = P131_B;
static constexpr Fp FP_GX_CANON = P131_GX;
static constexpr Fp FP_GY_CANON = P131_GY;
static constexpr Fp FP_QX_CANON = P131_QX;
static constexpr Fp FP_QY_CANON = P131_QY;
static constexpr U160 FP_ORDER = P131_N;

C131_HD Fp fp_zero() {
    return Fp{{0, 0, 0, 0, 0}};
}

C131_HD bool fp_eq(const Fp &a, const Fp &b) {
    uint32_t d = 0;
    for (int i = 0; i < 5; ++i) d |= a.v[i] ^ b.v[i];
    return d == 0;
}

C131_HD bool fp_is_zero(const Fp &a) {
    return fp_eq(a, fp_zero());
}

C131_HD bool fp_ge(const Fp &a, const Fp &b) {
    for (int i = 4; i >= 0; --i) {
        if (a.v[i] != b.v[i]) return a.v[i] > b.v[i];
    }
    return true;
}

C131_HD uint32_t fp_sub_raw(Fp &out, const Fp &a, const Fp &b) {
    uint64_t borrow = 0;
    for (int i = 0; i < 5; ++i) {
        const uint64_t rhs = static_cast<uint64_t>(b.v[i]) + borrow;
        out.v[i] = static_cast<uint32_t>(static_cast<uint64_t>(a.v[i]) - rhs);
        borrow = static_cast<uint64_t>(a.v[i]) < rhs;
    }
    return static_cast<uint32_t>(borrow);
}

C131_HD void fp_add_raw(Fp &out, const Fp &a, const Fp &b) {
    uint64_t carry = 0;
    for (int i = 0; i < 5; ++i) {
        const uint64_t sum = static_cast<uint64_t>(a.v[i]) + b.v[i] + carry;
        out.v[i] = static_cast<uint32_t>(sum);
        carry = sum >> 32;
    }
}

C131_HD Fp fp_add(const Fp &a, const Fp &b) {
    Fp out;
    fp_add_raw(out, a, b);
    // Inputs are reduced below a 131-bit modulus, so their sum fits in 132
    // bits and cannot overflow the five-limb container.
    if (fp_ge(out, FP_MOD)) {
        Fp reduced;
        fp_sub_raw(reduced, out, FP_MOD);
        out = reduced;
    }
    return out;
}

C131_HD Fp fp_sub(const Fp &a, const Fp &b) {
    Fp out;
    if (fp_sub_raw(out, a, b)) {
        Fp corrected;
        fp_add_raw(corrected, out, FP_MOD);
        out = corrected;
    }
    return out;
}

C131_HD Fp fp_mont_mul(const Fp &a, const Fp &b) {
    // Product followed by REDC with radix 2^160.  Eleven limbs leave one
    // explicit carry limb during each multiply-add propagation.
    uint32_t t[11] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
    for (int i = 0; i < 5; ++i) {
        uint64_t carry = 0;
        for (int j = 0; j < 5; ++j) {
            const uint64_t uv = static_cast<uint64_t>(t[i + j])
                              + static_cast<uint64_t>(a.v[j]) * b.v[i] + carry;
            t[i + j] = static_cast<uint32_t>(uv);
            carry = uv >> 32;
        }
        for (int k = i + 5; k < 11 && carry; ++k) {
            const uint64_t uv = static_cast<uint64_t>(t[k]) + carry;
            t[k] = static_cast<uint32_t>(uv);
            carry = uv >> 32;
        }
    }

    for (int i = 0; i < 5; ++i) {
        const uint32_t m = static_cast<uint32_t>(static_cast<uint64_t>(t[i]) * P131_NINV);
        uint64_t carry = 0;
        for (int j = 0; j < 5; ++j) {
            const uint64_t uv = static_cast<uint64_t>(t[i + j])
                              + static_cast<uint64_t>(m) * FP_MOD.v[j] + carry;
            t[i + j] = static_cast<uint32_t>(uv);
            carry = uv >> 32;
        }
        for (int k = i + 5; k < 11 && carry; ++k) {
            const uint64_t uv = static_cast<uint64_t>(t[k]) + carry;
            t[k] = static_cast<uint32_t>(uv);
            carry = uv >> 32;
        }
    }

    Fp out{{t[5], t[6], t[7], t[8], t[9]}};
    if (fp_ge(out, FP_MOD)) {
        Fp reduced;
        fp_sub_raw(reduced, out, FP_MOD);
        out = reduced;
    }
    return out;
}

C131_HD Fp fp_from_canonical(const Fp &a) {
    return fp_mont_mul(a, FP_R2);
}

C131_HD Fp fp_to_canonical(const Fp &a) {
    static constexpr Fp one{{1, 0, 0, 0, 0}};
    return fp_mont_mul(a, one);
}

C131_HD bool u160_bit(const U160 &a, int bit) {
    return ((a.v[bit >> 5] >> (bit & 31)) & 1u) != 0;
}

C131_HD Fp fp_pow(const Fp &a, const U160 &exponent) {
    Fp out = FP_R1;
    for (int bit = 159; bit >= 0; --bit) {
        out = fp_mont_mul(out, out);
        if (u160_bit(exponent, bit)) out = fp_mont_mul(out, a);
    }
    return out;
}

C131_HD Fp fp_inv(const Fp &a) {
    const U160 exponent{{FP_PM2.v[0], FP_PM2.v[1], FP_PM2.v[2], FP_PM2.v[3], FP_PM2.v[4]}};
    return fp_pow(a, exponent);
}

C131_HD Fp fp_mul_small(const Fp &a, unsigned n) {
    Fp out = fp_zero();
    for (unsigned i = 0; i < n; ++i) out = fp_add(out, a);
    return out;
}

struct FpPoint {
    Fp x;
    Fp y;
    uint32_t infinity;
};

C131_HD FpPoint fp_infinity() {
    return FpPoint{fp_zero(), fp_zero(), 1};
}

C131_HD bool fp_point_eq(const FpPoint &p, const FpPoint &q) {
    if (p.infinity || q.infinity) return p.infinity == q.infinity;
    return fp_eq(p.x, q.x) && fp_eq(p.y, q.y);
}

C131_HD FpPoint fp_point_from_canonical(const Fp &x, const Fp &y) {
    return FpPoint{fp_from_canonical(x), fp_from_canonical(y), 0};
}

C131_HD FpPoint fp_generator() {
    return fp_point_from_canonical(FP_GX_CANON, FP_GY_CANON);
}

C131_HD FpPoint fp_target() {
    return fp_point_from_canonical(FP_QX_CANON, FP_QY_CANON);
}

C131_HD bool fp_on_curve(const FpPoint &p) {
    if (p.infinity) return true;
    const Fp aa = fp_from_canonical(FP_A_CANON);
    const Fp bb = fp_from_canonical(FP_B_CANON);
    const Fp y2 = fp_mont_mul(p.y, p.y);
    const Fp x2 = fp_mont_mul(p.x, p.x);
    const Fp x3 = fp_mont_mul(x2, p.x);
    return fp_eq(y2, fp_add(fp_add(x3, fp_mont_mul(aa, p.x)), bb));
}

C131_HD FpPoint fp_double(const FpPoint &p) {
    if (p.infinity || fp_is_zero(p.y)) return fp_infinity();
    const Fp aa = fp_from_canonical(FP_A_CANON);
    const Fp numerator = fp_add(fp_mul_small(fp_mont_mul(p.x, p.x), 3), aa);
    const Fp denominator = fp_mul_small(p.y, 2);
    const Fp lambda = fp_mont_mul(numerator, fp_inv(denominator));
    const Fp x3 = fp_sub(fp_mont_mul(lambda, lambda), fp_mul_small(p.x, 2));
    const Fp y3 = fp_sub(fp_mont_mul(lambda, fp_sub(p.x, x3)), p.y);
    return FpPoint{x3, y3, 0};
}

C131_HD FpPoint fp_add_point(const FpPoint &p, const FpPoint &q) {
    if (p.infinity) return q;
    if (q.infinity) return p;
    if (fp_eq(p.x, q.x)) {
        if (fp_eq(p.y, q.y)) return fp_double(p);
        return fp_infinity();
    }
    const Fp lambda = fp_mont_mul(fp_sub(q.y, p.y), fp_inv(fp_sub(q.x, p.x)));
    const Fp x3 = fp_sub(fp_sub(fp_mont_mul(lambda, lambda), p.x), q.x);
    const Fp y3 = fp_sub(fp_mont_mul(lambda, fp_sub(p.x, x3)), p.y);
    return FpPoint{x3, y3, 0};
}

C131_HD FpPoint fp_neg(const FpPoint &p) {
    if (p.infinity || fp_is_zero(p.y)) return p;
    return FpPoint{p.x, fp_sub(fp_zero(), p.y), 0};
}

C131_HD FpPoint fp_scalar_mul(const U160 &k, const FpPoint &p) {
    FpPoint out = fp_infinity();
    for (int bit = 159; bit >= 0; --bit) {
        out = fp_double(out);
        if (u160_bit(k, bit)) out = fp_add_point(out, p);
    }
    return out;
}

struct F2m {
    uint64_t v[3];
};

static constexpr F2m F2_A = B131_A;
static constexpr F2m F2_B = B131_B;
static constexpr F2m F2_GX = B131_GX;
static constexpr F2m F2_GY = B131_GY;
static constexpr F2m F2_QX = B131_QX;
static constexpr F2m F2_QY = B131_QY;
static constexpr U160 F2_ORDER = B131_N;

C131_HD F2m f2_zero() {
    return F2m{{0, 0, 0}};
}

C131_HD F2m f2_one() {
    return F2m{{1, 0, 0}};
}

C131_HD F2m f2_add(const F2m &a, const F2m &b) {
    return F2m{{a.v[0] ^ b.v[0], a.v[1] ^ b.v[1], a.v[2] ^ b.v[2]}};
}

C131_HD bool f2_eq(const F2m &a, const F2m &b) {
    return ((a.v[0] ^ b.v[0]) | (a.v[1] ^ b.v[1]) | (a.v[2] ^ b.v[2])) == 0;
}

C131_HD bool f2_is_zero(const F2m &a) {
    return (a.v[0] | a.v[1] | a.v[2]) == 0;
}

C131_HD bool f2_product_bit(const uint64_t product[5], int bit) {
    return ((product[bit >> 6] >> (bit & 63)) & 1ull) != 0;
}

C131_HD void f2_product_toggle(uint64_t product[5], int bit) {
    product[bit >> 6] ^= 1ull << (bit & 63);
}

C131_HD F2m f2_mul(const F2m &a, const F2m &b) {
    uint64_t product[5] = {0, 0, 0, 0, 0};
    for (int bit = 0; bit < 131; ++bit) {
        if (((b.v[bit >> 6] >> (bit & 63)) & 1ull) == 0) continue;
        const int word = bit >> 6;
        const int shift = bit & 63;
        for (int j = 0; j < 3; ++j) {
            product[word + j] ^= a.v[j] << shift;
            if (shift && word + j + 1 < 5) product[word + j + 1] ^= a.v[j] >> (64 - shift);
        }
    }
    // x^131 = x^13 + x^2 + x + 1.
    for (int bit = 260; bit >= 131; --bit) {
        if (!f2_product_bit(product, bit)) continue;
        f2_product_toggle(product, bit);
        const int base = bit - 131;
        f2_product_toggle(product, base);
        f2_product_toggle(product, base + 1);
        f2_product_toggle(product, base + 2);
        f2_product_toggle(product, base + 13);
    }
    return F2m{{product[0], product[1], product[2] & 7ull}};
}

C131_HD F2m f2_square(const F2m &a) {
    return f2_mul(a, a);
}

C131_HD F2m f2_inv(const F2m &a) {
    F2m out = f2_one();
    // 2^131 - 2 has bits 130..1 set and bit 0 clear.
    for (int bit = 130; bit >= 0; --bit) {
        out = f2_square(out);
        if (bit != 0) out = f2_mul(out, a);
    }
    return out;
}

struct F2Point {
    F2m x;
    F2m y;
    uint32_t infinity;
};

C131_HD F2Point f2_infinity() {
    return F2Point{f2_zero(), f2_zero(), 1};
}

C131_HD bool f2_point_eq(const F2Point &p, const F2Point &q) {
    if (p.infinity || q.infinity) return p.infinity == q.infinity;
    return f2_eq(p.x, q.x) && f2_eq(p.y, q.y);
}

C131_HD F2Point f2_generator() {
    return F2Point{F2_GX, F2_GY, 0};
}

C131_HD F2Point f2_target() {
    return F2Point{F2_QX, F2_QY, 0};
}

C131_HD bool f2_on_curve(const F2Point &p) {
    if (p.infinity) return true;
    const F2m x2 = f2_square(p.x);
    const F2m left = f2_add(f2_square(p.y), f2_mul(p.x, p.y));
    const F2m right = f2_add(f2_add(f2_mul(x2, p.x), f2_mul(F2_A, x2)), F2_B);
    return f2_eq(left, right);
}

C131_HD F2Point f2_double(const F2Point &p) {
    if (p.infinity || f2_is_zero(p.x)) return f2_infinity();
    const F2m lambda = f2_add(p.x, f2_mul(p.y, f2_inv(p.x)));
    const F2m x3 = f2_add(f2_add(f2_square(lambda), lambda), F2_A);
    const F2m y3 = f2_add(f2_square(p.x), f2_mul(f2_add(lambda, f2_one()), x3));
    return F2Point{x3, y3, 0};
}

C131_HD F2Point f2_add_point(const F2Point &p, const F2Point &q) {
    if (p.infinity) return q;
    if (q.infinity) return p;
    if (f2_eq(p.x, q.x)) {
        if (f2_eq(p.y, q.y)) return f2_double(p);
        return f2_infinity();
    }
    const F2m lambda = f2_mul(f2_add(p.y, q.y), f2_inv(f2_add(p.x, q.x)));
    const F2m x3 = f2_add(f2_add(f2_add(f2_add(f2_square(lambda), lambda), p.x), q.x), F2_A);
    const F2m y3 = f2_add(f2_add(f2_mul(lambda, f2_add(p.x, x3)), x3), p.y);
    return F2Point{x3, y3, 0};
}

C131_HD F2Point f2_neg(const F2Point &p) {
    if (p.infinity) return p;
    return F2Point{p.x, f2_add(p.y, p.x), 0};
}

C131_HD F2Point f2_scalar_mul(const U160 &k, const F2Point &p) {
    F2Point out = f2_infinity();
    for (int bit = 159; bit >= 0; --bit) {
        out = f2_double(out);
        if (u160_bit(k, bit)) out = f2_add_point(out, p);
    }
    return out;
}

} // namespace certicom131

#undef C131_HD

#endif
