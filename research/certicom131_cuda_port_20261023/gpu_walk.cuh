// Per-thread rho walks for the exact ECCp-131 and ECC2-131 groups, written once and compiled both as
// CUDA device code and as plain C++ (the host build runs the very same functions thread by thread).
//
//   ECCp-131 : each thread advances B independent walks; 5 x 32-bit limbs, interleaved Montgomery
//              (R = 2^160, the header's representation), one Fermat inversion per batch (Montgomery's
//              trick), negation map, 16-entry r-adding table.
//   ECC2-131 : each thread advances 32 walks per slot (one bit lane each) on the ECC2K-130 client's
//              bitsliced GF(2^131) code (ecc2k130/include/fieldbs.h, W = uint32_t), S slots per thread,
//              one inversion per batch; the curve is carried in the client's normal-basis coordinates.
//
// State is struct-of-arrays in global memory so that consecutive threads touch consecutive words.
// Neither walk keeps jump-scalar bookkeeping or a distinguished-point test (unpriced work, as in the
// CPU studies).
#pragma once
#include <stdint.h>

#include "../../ecc2k130/include/curveparams.h"
#include "../../gpu/certicom131/certicom131.cuh"

#if defined(__CUDACC__)
#define GW_HD __host__ __device__ __forceinline__
#define GW_ND __host__ __device__ __noinline__
#else
#define GW_HD inline
#define GW_ND inline
#endif

namespace gw {
namespace h = certicom131;
using h::Fp;
static const int MAXB = 16;  // walks per thread (prime)
static const int MAXS = 8;   // slots per thread (binary)
static const int NW = 131;   // field degree

// =====================================================================
// ECCp-131
// =====================================================================
struct PrimeTable { Fp tx[16], ty[16]; };  // Montgomery form, canonical class representatives

GW_HD uint32_t mac(uint32_t a, uint32_t b, uint32_t c, uint32_t &carry) {
    const uint64_t t = (uint64_t)a * b + c + carry;
    carry = (uint32_t)(t >> 32);
    return (uint32_t)t;
}
// Interleaved (CIOS) Montgomery product, R = 2^160.  Same function as the header's fp_mont_mul.
GW_HD Fp pmul(const Fp &a, const Fp &b) {
    const Fp p = h::fp_modulus();
    uint32_t t[7] = {0, 0, 0, 0, 0, 0, 0};
#pragma unroll
    for (int i = 0; i < 5; ++i) {
        uint32_t c = 0;
#pragma unroll
        for (int j = 0; j < 5; ++j) t[j] = mac(a.v[j], b.v[i], t[j], c);
        uint64_t s = (uint64_t)t[5] + c;
        t[5] = (uint32_t)s;
        t[6] = (uint32_t)(s >> 32);
        const uint32_t m = t[0] * P131_NINV;
        c = 0;
        (void)mac(m, p.v[0], t[0], c);
#pragma unroll
        for (int j = 1; j < 5; ++j) t[j - 1] = mac(m, p.v[j], t[j], c);
        s = (uint64_t)t[5] + c;
        t[4] = (uint32_t)s;
        t[5] = t[6] + (uint32_t)(s >> 32);
    }
    Fp out{{t[0], t[1], t[2], t[3], t[4]}};
    // t[5] is zero for inputs below p; a final conditional subtraction reduces into [0, p)
    uint32_t d[5];
    uint64_t br = 0;
#pragma unroll
    for (int i = 0; i < 5; ++i) {
        const uint64_t rhs = (uint64_t)p.v[i] + br;
        d[i] = (uint32_t)((uint64_t)out.v[i] - rhs);
        br = (uint64_t)out.v[i] < rhs;
    }
    const bool keep = br != 0 && t[5] == 0;  // out < p and no overflow limb: keep out
    Fp r;
#pragma unroll
    for (int i = 0; i < 5; ++i) r.v[i] = keep ? out.v[i] : d[i];
    return r;
}
GW_HD Fp padd(const Fp &a, const Fp &b) { return h::fp_add(a, b); }
GW_HD Fp psub(const Fp &a, const Fp &b) { return h::fp_sub(a, b); }
GW_HD bool pzero(const Fp &a) { return (a.v[0] | a.v[1] | a.v[2] | a.v[3] | a.v[4]) == 0; }
GW_HD bool peq(const Fp &a, const Fp &b) { return h::fp_eq(a, b); }
// a^(p-2), exponent bits fixed by the modulus (square-and-multiply, high to low)
GW_ND Fp pinv(const Fp &a) {
    const Fp pm2 = h::fp_pm2();
    Fp r = h::fp_r1();
    for (int bit = 130; bit >= 0; --bit) {
        r = pmul(r, r);
        if ((pm2.v[bit >> 5] >> (bit & 31)) & 1u) r = pmul(r, a);
    }
    return r;
}
// class representative: of y and p - y keep the one whose Montgomery limb 0 is even (p odd: exactly one)
GW_HD void pcanon(Fp &y) {
    if (y.v[0] & 1u) y = psub(h::fp_zero(), y);
}
GW_HD uint32_t pjump(const Fp &x) { return (x.v[0] >> 3) & 15u; }

// State layout: word k of x for global lane g at S[(0*5 + k)*L + g], of y at S[(1*5 + k)*L + g].
// Thread tid advances lanes g = tid + nthreads * b, b < B, for `steps` iterations.
// Returns the number of exceptional lanes seen (zero denominator or equal x), which take a fixed move.
GW_ND int prime_thread(const PrimeTable &T, uint32_t *S, int L, int tid, int nthreads, int B, int steps) {
    Fp x[MAXB], y[MAXB], den[MAXB], pre[MAXB];
    uint8_t jx[MAXB], bad[MAXB];
    int exceptional = 0;
    for (int b = 0; b < B; ++b) {
        const int g = tid + nthreads * b;
        for (int k = 0; k < 5; ++k) {
            x[b].v[k] = S[(0 * 5 + k) * (size_t)L + g];
            y[b].v[k] = S[(1 * 5 + k) * (size_t)L + g];
        }
    }
    for (int it = 0; it < steps; ++it) {
        Fp acc = h::fp_r1();
        for (int b = 0; b < B; ++b) {
            const uint32_t j = pjump(x[b]);
            jx[b] = (uint8_t)j;
            Fp d = psub(T.tx[j], x[b]);
            const bool z = pzero(d);
            bad[b] = z;
            if (z) d = h::fp_r1();
            den[b] = d;
            pre[b] = acc;
            acc = pmul(acc, d);
        }
        Fp inv = pinv(acc);
        for (int b = B - 1; b >= 0; --b) {
            const Fp di = pmul(inv, pre[b]);
            inv = pmul(inv, den[b]);
            const uint32_t j = jx[b];
            if (bad[b]) {
                ++exceptional;
                const uint32_t j2 = (j + 1) & 15u;
                x[b] = T.tx[j2];
                y[b] = T.ty[j2];
                continue;
            }
            const Fp lam = pmul(psub(T.ty[j], y[b]), di);
            const Fp x3 = psub(psub(pmul(lam, lam), x[b]), T.tx[j]);
            Fp y3 = psub(pmul(lam, psub(x[b], x3)), y[b]);
            pcanon(y3);
            x[b] = x3;
            y[b] = y3;
        }
    }
    for (int b = 0; b < B; ++b) {
        const int g = tid + nthreads * b;
        for (int k = 0; k < 5; ++k) {
            S[(0 * 5 + k) * (size_t)L + g] = x[b].v[k];
            S[(1 * 5 + k) * (size_t)L + g] = y[b].v[k];
        }
    }
    return exceptional;
}

// =====================================================================
// ECC2-131 on the client's bitsliced field (W = uint32_t: 32 lanes per word)
// =====================================================================
typedef uint32_t W;
typedef FieldBs<CfgF131, W> FB;
struct El { W w[NW]; };

// Jump table as lane masks: m[j][i] = all-ones when bit i (normal-basis coordinate) of entry j is set.
// Layout: tabx[i*16 + j], taby[i*16 + j].
GW_HD void bmux(const El &x, const W *tabx, const W *taby, El &tx, El &ty) {
    const W i0 = x.w[0], i1 = x.w[1], i2 = x.w[2], i3 = x.w[3];
    W m[16];
    const W a0 = ~i0, a1 = ~i1, a2 = ~i2, a3 = ~i3;
    const W lo[4] = {a0 & a1, i0 & a1, a0 & i1, i0 & i1};
    const W hi[4] = {a2 & a3, i2 & a3, a2 & i3, i2 & i3};
#pragma unroll
    for (int q = 0; q < 4; ++q)
#pragma unroll
        for (int p = 0; p < 4; ++p) m[q * 4 + p] = lo[p] & hi[q];
    for (int i = 0; i < NW; ++i) {
        W ax = 0, ay = 0;
#pragma unroll
        for (int j = 0; j < 16; ++j) {
            ax |= m[j] & tabx[i * 16 + j];
            ay |= m[j] & taby[i * 16 + j];
        }
        tx.w[i] = ax;
        ty.w[i] = ay;
    }
}
// class representative of {(x, y), (x, x + y)}: keep y_k = 0 at the first set coordinate k of x
GW_HD void bcanon(const El &x, El &y) {
    W found = 0, sel = 0;
    for (int k = 0; k < NW; ++k) {
        const W first = x.w[k] & ~found;
        sel ^= first & y.w[k];
        found |= x.w[k];
    }
    for (int k = 0; k < NW; ++k) y.w[k] ^= sel & x.w[k];
}

// State layout: word i of x, slot s, thread t at X[(s*NW + i)*nthreads + t] (likewise Y).
// `amask[i]` = all-ones when coordinate i of the curve coefficient a is set.
// Returns the number of lanes with a zero denominator (a start on a table point), which the caller
// treats as a failed run: the denominators are patched to 1 so the batch inverse stays defined.
GW_ND int bin_thread(const W *tabx, const W *taby, const W *amask, W *X, W *Y, int tid, int nthreads, int S, int steps) {
    El x[MAXS], y[MAXS], den[MAXS], pre[MAXS], tx, ty;
    int exceptional = 0;
    for (int s = 0; s < S; ++s)
        for (int i = 0; i < NW; ++i) {
            x[s].w[i] = X[((size_t)s * NW + i) * nthreads + tid];
            y[s].w[i] = Y[((size_t)s * NW + i) * nthreads + tid];
        }
    for (int it = 0; it < steps; ++it) {
        El acc, inv, tmp, lam, x3, y3, num;
        FB::setOne(acc.w);
        for (int s = 0; s < S; ++s) {
            bmux(x[s], tabx, taby, tx, ty);
            W z = 0;
            for (int i = 0; i < NW; ++i) {
                den[s].w[i] = x[s].w[i] ^ tx.w[i];
                z |= den[s].w[i];
            }
            const W zl = ~z;
            if (zl) {
                for (int l = 0; l < 32; ++l) exceptional += (zl >> l) & 1u;
                for (int i = 0; i < NW; ++i) den[s].w[i] |= zl;
            }
            pre[s] = acc;
            FB::mul(acc.w, den[s].w, tmp.w);
            acc = tmp;
        }
        FB::inv(acc.w, inv.w);
        for (int s = S - 1; s >= 0; --s) {
            bmux(x[s], tabx, taby, tx, ty);
            FB::mul(inv.w, pre[s].w, tmp.w);      // 1 / den[s]
            FB::mul(inv.w, den[s].w, inv.w);      // inv <- inv * den[s]
            for (int i = 0; i < NW; ++i) num.w[i] = y[s].w[i] ^ ty.w[i];
            FB::mul(num.w, tmp.w, lam.w);
            FB::sqr(lam.w, x3.w);
            for (int i = 0; i < NW; ++i) x3.w[i] ^= lam.w[i] ^ x[s].w[i] ^ tx.w[i] ^ amask[i];
            for (int i = 0; i < NW; ++i) num.w[i] = x[s].w[i] ^ x3.w[i];
            FB::mul(lam.w, num.w, y3.w);
            for (int i = 0; i < NW; ++i) y3.w[i] ^= x3.w[i] ^ y[s].w[i];
            bcanon(x3, y3);
            x[s] = x3;
            y[s] = y3;
        }
    }
    for (int s = 0; s < S; ++s)
        for (int i = 0; i < NW; ++i) {
            X[((size_t)s * NW + i) * nthreads + tid] = x[s].w[i];
            Y[((size_t)s * NW + i) * nthreads + tid] = y[s].w[i];
        }
    return exceptional;
}
}  // namespace gw
