// Scalar 3 x 64-bit Montgomery field (R = 2^192) for the ECCp-131 prime; shared by walk131.cpp and
// pv131.cpp (as an independent scalar big-integer oracle there).
#pragma once
#include <stdint.h>
#include "../../gpu/certicom131/certicom131.cuh"
using u64 = uint64_t;
using u128 = unsigned __int128;
namespace h = certicom131;

namespace pf {
struct F { u64 v[3]; };
static const u64 P[3] = {0x194c43186b3abc0bull, 0x8e1d43f293469e33ull, 0x4ull};
static u64 N0;  // -p^-1 mod 2^64
static F R2;    // 2^384 mod p

static inline bool ge_p(const u64 a[3]) {
    for (int i = 2; i >= 0; --i)
        if (a[i] != P[i]) return a[i] > P[i];
    return true;
}
static inline F add(const F &a, const F &b) {
    u128 c = 0;
    F r;
    for (int i = 0; i < 3; ++i) { c += (u128)a.v[i] + b.v[i]; r.v[i] = (u64)c; c >>= 64; }
    if (ge_p(r.v)) {
        u128 br = 0;
        for (int i = 0; i < 3; ++i) { u128 d = (u128)r.v[i] - P[i] - (u64)br; r.v[i] = (u64)d; br = (d >> 64) & 1; }
    }
    return r;
}
static inline F sub(const F &a, const F &b) {
    F r;
    u128 br = 0;
    for (int i = 0; i < 3; ++i) { u128 d = (u128)a.v[i] - b.v[i] - (u64)br; r.v[i] = (u64)d; br = (d >> 64) & 1; }
    if (br) {
        u128 c = 0;
        for (int i = 0; i < 3; ++i) { c += (u128)r.v[i] + P[i]; r.v[i] = (u64)c; c >>= 64; }
    }
    return r;
}
static inline F mul(const F &a, const F &b) {
    u64 t0 = 0, t1 = 0, t2 = 0, t3 = 0, t4 = 0;
    for (int i = 0; i < 3; ++i) {
        u128 c = 0, s;
        s = (u128)a.v[0] * b.v[i] + t0; t0 = (u64)s; c = s >> 64;
        s = (u128)a.v[1] * b.v[i] + t1 + c; t1 = (u64)s; c = s >> 64;
        s = (u128)a.v[2] * b.v[i] + t2 + c; t2 = (u64)s; c = s >> 64;
        s = (u128)t3 + c; t3 = (u64)s; t4 = (u64)(s >> 64);
        u64 m = t0 * N0;
        s = (u128)m * P[0] + t0; c = s >> 64;
        s = (u128)m * P[1] + t1 + c; t0 = (u64)s; c = s >> 64;
        s = (u128)m * P[2] + t2 + c; t1 = (u64)s; c = s >> 64;
        s = (u128)t3 + c; t2 = (u64)s; c = s >> 64;
        t3 = t4 + (u64)c;
    }
    F r{{t0, t1, t2}};
    if (t3 || ge_p(r.v)) {
        u128 br = 0;
        for (int i = 0; i < 3; ++i) { u128 d = (u128)r.v[i] - P[i] - (u64)br; r.v[i] = (u64)d; br = (d >> 64) & 1; }
    }
    return r;
}
static void init() {
    u64 inv = 1;  // Newton: inv = p0^-1 mod 2^64
    for (int i = 0; i < 6; ++i) inv *= 2 - P[0] * inv;
    N0 = (u64)0 - inv;
    F x{{1, 0, 0}};
    for (int i = 0; i < 384; ++i) x = add(x, x);
    R2 = x;
}
static inline F to_mont(const F &c) { return mul(c, R2); }
static inline F from_mont(const F &a) { return mul(a, F{{1, 0, 0}}); }
static inline F one() { return to_mont(F{{1, 0, 0}}); }
static inline bool is_zero(const F &a) { return (a.v[0] | a.v[1] | a.v[2]) == 0; }
static inline bool eq(const F &a, const F &b) { return a.v[0] == b.v[0] && a.v[1] == b.v[1] && a.v[2] == b.v[2]; }
static F inv(const F &a) {  // Fermat, exponent p-2
    u64 e[3] = {P[0] - 2, P[1], P[2]};
    F r = one();
    for (int bit = 130; bit >= 0; --bit) {
        r = mul(r, r);
        if ((e[bit >> 6] >> (bit & 63)) & 1) r = mul(r, a);
    }
    return r;
}
static F from_header(const h::Fp &c) {  // header canonical (5 x 32) -> canonical 3 x 64
    return F{{(u64)c.v[0] | ((u64)c.v[1] << 32), (u64)c.v[2] | ((u64)c.v[3] << 32), (u64)c.v[4]}};
}
static h::Fp to_header_canonical(const F &c) {
    return h::Fp{{(uint32_t)c.v[0], (uint32_t)(c.v[0] >> 32), (uint32_t)c.v[1], (uint32_t)(c.v[1] >> 32), (uint32_t)c.v[2]}};
}
}  // namespace pf
