/* fp256x64.cuh -- P-256 field arithmetic on 4 x 64-bit limbs, Montgomery
 * form with R = 2^256, for CUDA devices and host emulation.
 *
 * Mirrors `src/ecc/p256_field.rs` + `src/ct_bignum.rs::mont_mul`: the same
 * modulus limbs, the same `m' = -p^-1 mod 2^64`, the same `R mod p` and
 * `R^2 mod p` constants, the same SOS (separated operand scanning)
 * Montgomery multiplication with a final conditional subtraction.  Inputs
 * and outputs are canonical representatives in [0, p), so a Montgomery
 * value produced here equals the one `P256FieldElement` holds bit for bit.
 *
 * Why 64-bit limbs here when gpu/ecc/fp256.cuh argues for 32-bit limbs?
 * This header exists to mirror the crate's P-256 field exactly (the task
 * is a cross-checkable oracle, not peak throughput), and 64-bit limbs are
 * what the Rust code uses.  On sm_120 the 64x64->128 partial products
 * become IMAD.WIDE.U32 quads plus carry material, so a multiply costs
 * ~4x the instructions of the 32-bit CIOS in gpu/ecc/fp256.cuh with
 * FP_PTX=1.  A production P-256 kernel should instantiate `rootfind.cuh`
 * on that field instead; the kernel is templated on the field type for
 * exactly this reason.  See RESEARCH_GPU_ROOTFIND_DECOMPOSITION.md §2.
 */
#ifndef GPU_ROOTFIND_FP256X64_CUH
#define GPU_ROOTFIND_FP256X64_CUH

#include <stdint.h>
#include "fp64.cuh"   /* RF_HD / RF_BIG / rf_mulhi64 */

struct U256x64 {
    uint64_t v[4];
};

/* P-256 constants, little-endian limbs (p256_field.rs: P, R_MOD_P,
 * R_SQUARED_MOD_P; P_INV_LOW = compute_minv64(P[0]) = 1 since
 * P[0] = 2^64 - 1 and (2^64-1) * 1 = -1 mod 2^64). */
#define P256_P_LIMBS  {0xFFFFFFFFFFFFFFFFull, 0x00000000FFFFFFFFull, 0x0000000000000000ull, 0xFFFFFFFF00000001ull}
#define P256_R1_LIMBS {0x0000000000000001ull, 0xFFFFFFFF00000000ull, 0xFFFFFFFFFFFFFFFFull, 0x00000000FFFFFFFEull}
#define P256_R2_LIMBS {0x0000000000000003ull, 0xFFFFFFFBFFFFFFFFull, 0xFFFFFFFFFFFFFFFEull, 0x00000004FFFFFFFDull}
#define P256_PINV_LOW 1ull

struct Fp256Ctx {
    U256x64 p;
    U256x64 one;
    U256x64 r2;
    uint64_t pinv;
};

static inline Fp256Ctx fp256_ctx_p256() {
    Fp256Ctx c;
    const uint64_t P[4] = P256_P_LIMBS, R1[4] = P256_R1_LIMBS, R2[4] = P256_R2_LIMBS;
    for (int i = 0; i < 4; i++) { c.p.v[i] = P[i]; c.one.v[i] = R1[i]; c.r2.v[i] = R2[i]; }
    c.pinv = P256_PINV_LOW;
    return c;
}

/* --- limb primitives ------------------------------------------------- */

/* r = a + b, returns carry (0/1). */
RF_HD uint64_t u256_adc(U256x64 &r, const U256x64 &a, const U256x64 &b) {
    uint64_t c = 0;
#pragma unroll
    for (int i = 0; i < 4; i++) {
        uint64_t s = a.v[i] + b.v[i];
        uint64_t c1 = s < a.v[i];
        uint64_t t = s + c;
        uint64_t c2 = t < s;
        r.v[i] = t;
        c = c1 | c2;
    }
    return c;
}

/* r = a - b, returns borrow (0/1). */
RF_HD uint64_t u256_sbb(U256x64 &r, const U256x64 &a, const U256x64 &b) {
    uint64_t bw = 0;
#pragma unroll
    for (int i = 0; i < 4; i++) {
        uint64_t d = a.v[i] - b.v[i];
        uint64_t b1 = a.v[i] < b.v[i];
        uint64_t t = d - bw;
        uint64_t b2 = d < bw;
        r.v[i] = t;
        bw = b1 | b2;
    }
    return bw;
}

RF_HD void u256_cmov(U256x64 &r, const U256x64 &a, uint64_t flag) {
    uint64_t mask = (uint64_t)0 - (flag & 1);
#pragma unroll
    for (int i = 0; i < 4; i++) r.v[i] = (r.v[i] & ~mask) | (a.v[i] & mask);
}

RF_HD int u256_eq(const U256x64 &a, const U256x64 &b) {
    uint64_t acc = 0;
#pragma unroll
    for (int i = 0; i < 4; i++) acc |= a.v[i] ^ b.v[i];
    return acc == 0;
}

RF_HD int u256_is_zero(const U256x64 &a) {
    return (a.v[0] | a.v[1] | a.v[2] | a.v[3]) == 0;
}

/* acc = cur + a*b + carry, as (hi, lo).
 * Host: let the compiler lower a single 128-bit multiply/accumulate into
 * MUL/ADC instructions.  This is equivalent to the explicit carry chain.
 * Device: preserve the original 64-bit implementation and register use.
 * The maximum sum is 2^128-1, so the wide host accumulator cannot wrap. */
RF_HD void u64_mac(uint64_t cur, uint64_t a, uint64_t b, uint64_t carry, uint64_t &lo, uint64_t &hi) {
#ifdef __CUDA_ARCH__
    uint64_t plo = a * b;
    uint64_t phi = rf_mulhi64(a, b);
    uint64_t s1 = plo + cur;
    phi += (s1 < plo);
    uint64_t s2 = s1 + carry;
    phi += (s2 < s1);
    lo = s2;
    hi = phi;
#else
    const unsigned __int128 sum = (unsigned __int128)a * b + cur + carry;
    lo = (uint64_t)sum;
    hi = (uint64_t)(sum >> 64);
#endif
}

/* --- field operations ------------------------------------------------ */

RF_HD U256x64 fp256_add(const Fp256Ctx &f, const U256x64 &a, const U256x64 &b) {
    U256x64 s, t;
    uint64_t c = u256_adc(s, a, b);
    uint64_t bw = u256_sbb(t, s, f.p);
    u256_cmov(s, t, c | (bw ^ 1));
    return s;
}

RF_HD U256x64 fp256_sub(const Fp256Ctx &f, const U256x64 &a, const U256x64 &b) {
    U256x64 d, t;
    uint64_t bw = u256_sbb(d, a, b);
    u256_adc(t, d, f.p);
    u256_cmov(d, t, bw);
    return d;
}

RF_HD U256x64 fp256_neg(const Fp256Ctx &f, const U256x64 &a) {
    U256x64 z = {{0, 0, 0, 0}};
    return fp256_sub(f, z, a);
}

/* SOS Montgomery multiplication, identical phase structure to
 * ct_bignum::mont_mul: full 8-limb schoolbook product, then four
 * reduction rounds each zeroing one low limb, then one conditional
 * subtraction driven by (extra | !borrow). */
RF_HD U256x64 fp256_mul(const Fp256Ctx &f, const U256x64 &a, const U256x64 &b) {
    uint64_t t[9];
#pragma unroll
    for (int i = 0; i < 9; i++) t[i] = 0;
    /* phase 1: t[0..8) = a * b */
#pragma unroll
    for (int i = 0; i < 4; i++) {
        uint64_t carry = 0;
#pragma unroll
        for (int j = 0; j < 4; j++) {
            uint64_t lo, hi;
            u64_mac(t[i + j], a.v[j], b.v[i], carry, lo, hi);
            t[i + j] = lo;
            carry = hi;
        }
        t[i + 4] = carry;
    }
    /* phase 2: reduction */
#pragma unroll
    for (int i = 0; i < 4; i++) {
#ifdef __CUDA_ARCH__
        /* Device path is unchanged until GPU occupancy/throughput validation. */
        uint64_t m = t[i] * f.pinv;
        uint64_t carry = 0;
#pragma unroll
        for (int j = 0; j < 4; j++) {
            uint64_t lo, hi;
            u64_mac(t[i + j], m, f.p.v[j], carry, lo, hi);
            t[i + j] = lo;
            carry = hi;
        }
#else
        /* P-256 specialization: pinv=1, p[0]=B-1, p[2]=0, B=2^64.
         * Therefore m=t[i], and t[i]+m*(B-1)=m*B exactly:
         * eliminate the first wide MAC, set the cleared limb to zero,
         * and use m as the next carry.  The zero p[2] limb also needs
         * just a single add-with-carry, not a wide multiplication.
         * fp256_ctx_p256() is the only constructor of this field. */
        const uint64_t m = t[i];
        uint64_t carry = m, lo, hi;
        t[i] = 0;
        u64_mac(t[i + 1], m, 0x00000000ffffffffull, carry, lo, hi);
        t[i + 1] = lo;
        carry = hi;
        const uint64_t sum = t[i + 2] + carry;
        carry = sum < t[i + 2];
        t[i + 2] = sum;
        u64_mac(t[i + 3], m, 0xffffffff00000001ull, carry, lo, hi);
        t[i + 3] = lo;
        carry = hi;
#endif
        /* propagate through t[i+4 .. 8] */
#pragma unroll
        for (int k = i + 4; k < 9; k++) {
            uint64_t s = t[k] + carry;
            carry = s < t[k];
            t[k] = s;
        }
    }
    U256x64 r, s;
#pragma unroll
    for (int i = 0; i < 4; i++) r.v[i] = t[i + 4];
    uint64_t bw = u256_sbb(s, r, f.p);
    u256_cmov(r, s, (t[8] & 1) | (bw ^ 1));
    return r;
}

RF_HD U256x64 fp256_sqr(const Fp256Ctx &f, const U256x64 &a) { return fp256_mul(f, a, a); }

RF_HD U256x64 fp256_to_mont(const Fp256Ctx &f, const U256x64 &a) { return fp256_mul(f, a, f.r2); }

RF_HD U256x64 fp256_from_mont(const Fp256Ctx &f, const U256x64 &a) {
    U256x64 one = {{1, 0, 0, 0}};
    return fp256_mul(f, a, one);
}

/* Invert a Montgomery residue with the fixed, public P-256 exponent p-2.
 *
 * The CUDA path preserves the original uniform 256-bit square-and-multiply
 * schedule until device throughput and register occupancy are measured.
 * On the host, the hexadecimal digits of p-2 belong to {0, 1, D, F}
 * (31 zeroes, 31 F's, one D and one 1). Precompute a^13 and a^15
 * with three squares and three multiplies, then use a radix-16 chain.
 * This eliminates roughly 93 multiplications relative to the original
 * 256-bit bitwise exponent. The exponent-dependent branches are public.
 * Specific to P-256: Fp256Ctx is only constructed by fp256_ctx_p256(). */
RF_BIG U256x64 fp256_inv(const Fp256Ctx &f, const U256x64 &a) {
#ifdef __CUDA_ARCH__
    U256x64 e = f.p;
    e.v[0] -= 2;
    U256x64 acc = f.one;
#pragma unroll 1
    for (int i = 255; i >= 0; i--) {
        acc = fp256_sqr(f, acc);
        if ((e.v[i >> 6] >> (i & 63)) & 1) acc = fp256_mul(f, acc, a);
    }
    return acc;
#else
    const U256x64 a2 = fp256_sqr(f, a);
    const U256x64 a3 = fp256_mul(f, a2, a);
    const U256x64 a12 = fp256_sqr(f, fp256_sqr(f, a3));
    const U256x64 a13 = fp256_mul(f, a12, a);
    const U256x64 a15 = fp256_mul(f, a12, a3);
    U256x64 e = f.p;
    e.v[0] -= 2; /* p[0] = 2^64 - 1, no borrow */
    U256x64 acc = a15; /* leading hexadecimal digit of p-2 is F */
    for (int i = 62; i >= 0; --i) {
        acc = fp256_sqr(f, acc);
        acc = fp256_sqr(f, acc);
        acc = fp256_sqr(f, acc);
        acc = fp256_sqr(f, acc);
        const unsigned digit = (unsigned)((e.v[i >> 4] >> (4 * (i & 15))) & 15u);
        if (digit == 15) acc = fp256_mul(f, acc, a15);
        else if (digit == 13) acc = fp256_mul(f, acc, a13);
        else if (digit == 1) acc = fp256_mul(f, acc, a);
    }
    return acc;
#endif
}

/* Montgomery simultaneous inversion; for n>0 omit the three redundant
 * products with Montgomery one. scratch[0] is intentionally unused. */
RF_HD void fp256_batch_inv(const Fp256Ctx &f, U256x64 *x, int n, U256x64 *scratch) {
    if (n <= 0) return;
    U256x64 acc = x[0];
    for (int i = 1; i < n; i++) {
        scratch[i] = acc;
        acc = fp256_mul(f, acc, x[i]);
    }
    U256x64 inv_acc = fp256_inv(f, acc);
    for (int i = n - 1; i > 0; i--) {
        U256x64 xi = x[i];
        x[i] = fp256_mul(f, inv_acc, scratch[i]);
        inv_acc = fp256_mul(f, inv_acc, xi);
    }
    x[0] = inv_acc;
}

#endif /* GPU_ROOTFIND_FP256X64_CUH */
