/* fp64.cuh -- Montgomery arithmetic mod an odd prime p < 2^63 on one
 * 64-bit word, for CUDA devices and for host emulation of the same code.
 *
 * This is a line-for-line mirror of `Fp64` in
 * `src/cryptanalysis/prime_fast.rs` (R = 2^64, values held as a*R mod p,
 * reduction `t -> (t + m*p) >> 64` with m = lo(t) * (-p^-1 mod 2^64)),
 * so a Montgomery-form value produced here is bit-identical to the one
 * the Rust engine produces for the same canonical input.  That is what
 * lets the Rust harness compare the GPU oracle against `prime_fast`
 * exactly rather than modulo re-encoding.
 *
 * Every function is `__host__ __device__`: the CPU test harness and the
 * emulation backend compile *this file* with g++/clang++, the kernels
 * compile it with nvcc.  The only device/host split is the 64x64->128
 * product: `__umul64hi` on the device, `unsigned __int128` on the host.
 *
 * Blackwell (sm_120) notes
 * ------------------------
 * - The integer datapath is 32-bit.  A 64x64->64 low product is three
 *   IMAD.WIDE.U32, `__umul64hi` is four plus carry handling, so one
 *   Montgomery multiplication here is roughly 12-16 IMAD-class
 *   instructions plus a handful of IADD3/ISETP.  Reductions are branch
 *   free (the final conditional subtraction is a select), so a warp never
 *   diverges inside the field layer.
 * - A thread holds a few dozen 64-bit values at most (see rootfind.cuh),
 *   which keeps the kernel well under the 255-register ceiling and lets
 *   ptxas place >= 8 warps per scheduler on sm_120.  Register pressure
 *   comes from the batched-inversion scratch, not from the field.
 * - For p < 2^63, a+b < 2^64 and REDC input T < p*2^64 gives
 *   T+m*p < 2*p*2^64 < 2^128. The high-word sum is <2p<2^64.
 *   See HAC 14.32/14.33. The mirrored Rust engine retains its stricter
 *   MAX_BITS guard; widening this documented arithmetic domain changes
 *   no operation or value for existing callers.
 */
#ifndef GPU_ROOTFIND_FP64_CUH
#define GPU_ROOTFIND_FP64_CUH

#include <stdint.h>

#ifdef __CUDACC__
#define RF_HD __host__ __device__ __forceinline__
#define RF_BIG __host__ __device__ __noinline__ inline
#else
#define RF_HD inline
#define RF_BIG inline
#endif

/* Field context.  Plain-old-data so it can be passed to a kernel by value
 * (it lands in the constant parameter bank, which is broadcast to every
 * thread for free). */
struct Fp64Ctx {
    uint64_t p;      /* odd prime, 3 < p < 2^63 */
    uint64_t n_inv;  /* -p^-1 mod 2^64 */
    uint64_t one;    /* R mod p  (Montgomery 1) */
    uint64_t r2;     /* R^2 mod p (canonical -> Montgomery) */
    uint64_t pm2;    /* p - 2, the Fermat exponent */
};

RF_HD uint64_t rf_mulhi64(uint64_t a, uint64_t b) {
#ifdef __CUDA_ARCH__
    return __umul64hi(a, b);
#else
    return (uint64_t)(((unsigned __int128)a * b) >> 64);
#endif
}

/* Montgomery reduction of the 128-bit value (hi:lo) < p * 2^64:
 * returns (hi:lo) * R^-1 mod p.  Written without a 128-bit type so the
 * device path is exactly the arithmetic the hardware offers:
 *   m  = lo * n_inv                 (mod 2^64)
 *   u  = hi + mulhi(m, p) + carry(lo + lo(m*p))
 * and lo + lo(m*p) == 0 mod 2^64 by construction, so the carry is simply
 * (lo != 0).  Result is < 2p, one conditional subtraction finishes. */
RF_HD uint64_t fp64_reduce(const Fp64Ctx &f, uint64_t hi, uint64_t lo) {
    uint64_t m = lo * f.n_inv;
    uint64_t u = hi + rf_mulhi64(m, f.p) + (lo != 0 ? 1u : 0u);
    return u >= f.p ? u - f.p : u;
}

RF_HD uint64_t fp64_mul(const Fp64Ctx &f, uint64_t a, uint64_t b) {
#ifdef __CUDA_ARCH__
    return fp64_reduce(f, rf_mulhi64(a, b), a * b);
#else
    /* Host: add the full 128-bit product and Montgomery correction in a
     * single wide expression.  For p < 2^63 and canonical operands,
     * t + m*p fits in 128 bits.  This lets x86-64 compilers combine
     * carry propagation rather than synthesizing the hi + mulhi + flag
     * sequence of the device-oriented reduction. */
    const unsigned __int128 t = (unsigned __int128)a * b;
    const uint64_t m = (uint64_t)t * f.n_inv;
    const uint64_t u = (uint64_t)((t + (unsigned __int128)m * f.p) >> 64);
    return u >= f.p ? u - f.p : u;
#endif
}

RF_HD uint64_t fp64_sqr(const Fp64Ctx &f, uint64_t a) { return fp64_mul(f, a, a); }

RF_HD uint64_t fp64_add(const Fp64Ctx &f, uint64_t a, uint64_t b) {
    uint64_t s = a + b;
    return s >= f.p ? s - f.p : s;
}

RF_HD uint64_t fp64_sub(const Fp64Ctx &f, uint64_t a, uint64_t b) {
#ifdef __CUDA_ARCH__
    return a >= b ? a - b : a + f.p - b;
#else
    /* Host: branchless correction avoids unpredictable a>=b branches
     * in the two-sign rootfinder sweep.  a,b are canonical [0,p)
     * and unsigned wrap is intentional when a<b.  Device unchanged. */
    const uint64_t d = a - b;
    const uint64_t mask = (uint64_t)0 - (uint64_t)(a < b);
    return d + (f.p & mask);
#endif
}

RF_HD uint64_t fp64_neg(const Fp64Ctx &f, uint64_t a) {
#ifdef __CUDA_ARCH__
    return a == 0 ? 0 : f.p - a;
#else
    /* Canonical a: p-a is the negative except for zero.  Branchless
     * zero masking avoids an unpredictable conditional on public data. */
    return (f.p - a) & ((uint64_t)0 - (uint64_t)(a != 0));
#endif
}

/* canonical -> Montgomery (input must already be < p) */
RF_HD uint64_t fp64_to_mont(const Fp64Ctx &f, uint64_t a) { return fp64_mul(f, a, f.r2); }

/* Montgomery -> canonical */
RF_HD uint64_t fp64_from_mont(const Fp64Ctx &f, uint64_t a) { return fp64_reduce(f, 0, a); }

/* Invert a Montgomery residue.
 *
 * On the GPU, Fermat exponentiation is uniform across the warp because
 * the exponent is the public modulus. A data-dependent Euclidean loop
 * would serialize lanes. Retain the original device implementation.
 *
 * On the CPU host-emulation path, use signed extended Euclid on the
 * canonical value and convert the result back to Montgomery form.
 * For the documented p < 2^63, Euclidean coefficients are bounded by p,
 * so the signed intermediate products fit in int64_t. This is
 * VARIABLE-TIME and intended only for public root-finding inputs, not
 * secret-dependent modular arithmetic or signing. inv(0) = 0.
 *
 * Both paths return the unique Montgomery representation of a^{-1}. */
RF_BIG uint64_t fp64_inv(const Fp64Ctx &f, uint64_t a) {
#ifdef __CUDA_ARCH__
    uint64_t acc = f.one;
    uint64_t base = a;
    uint64_t e = f.pm2;
#pragma unroll 1
    while (e) {
        if (e & 1) acc = fp64_mul(f, acc, base);
        base = fp64_sqr(f, base);
        e >>= 1;
    }
    return acc;
#else
    if (a == 0) return 0;
    uint64_t r0 = f.p, r1 = fp64_from_mont(f, a);
    int64_t t0 = 0, t1 = 1;
    while (r1 != 0) {
        const uint64_t q = r0 / r1;
        const uint64_t r2 = r0 - q * r1;
        const int64_t t2 = t0 - (int64_t)q * t1;
        r0 = r1;
        r1 = r2;
        t0 = t1;
        t1 = t2;
    }
    /* All callers use prime moduli and nonzero inputs (gcd == 1). */
    const uint64_t inverse = (uint64_t)(t0 < 0 ? t0 + (int64_t)f.p : t0);
    return fp64_to_mont(f, inverse);
#endif
}

/* Montgomery's simultaneous inversion of x[0..n) in place using scratch
 * of n words: one fp64_inv plus 3(n-1) multiplies, omitting the first
 * prefix multiply by one and both redundant final multiplies by one.
 * Inputs must be non-zero (the caller substitutes `one` for degenerate
 * entries and flags them). n==0 is a no-op. */
RF_HD void fp64_batch_inv(const Fp64Ctx &f, uint64_t *x, int n, uint64_t *scratch) {
    if (n <= 0) return;
    uint64_t acc = x[0];
    for (int i = 1; i < n; i++) {
        scratch[i] = acc;
        acc = fp64_mul(f, acc, x[i]);
    }
    uint64_t inv_acc = fp64_inv(f, acc);
    for (int i = n - 1; i > 0; i--) {
        uint64_t xi = x[i];
        x[i] = fp64_mul(f, inv_acc, scratch[i]);
        inv_acc = fp64_mul(f, inv_acc, xi);
    }
    x[0] = inv_acc;
}

/* Host-side context construction (mirrors `Fp64::new`). */
static inline Fp64Ctx fp64_ctx_new(uint64_t p) {
    Fp64Ctx f;
    f.p = p;
    uint64_t inv = 1;
    for (int i = 0; i < 6; i++) inv *= 2u - p * inv;   /* Newton: p^-1 mod 2^64 */
    f.n_inv = (uint64_t)0 - inv;
    unsigned __int128 R = ((unsigned __int128)1) << 64;
    f.one = (uint64_t)(R % p);
    f.r2 = (uint64_t)(((unsigned __int128)f.one * f.one) % p);
    f.pm2 = p - 2;
    return f;
}

#endif /* GPU_ROOTFIND_FP64_CUH */
