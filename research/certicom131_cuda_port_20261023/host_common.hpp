// Host-side set-up shared by host_emul.cpp and walk_cuda.cu: start points, jump tables and the
// state layouts of gpu_walk.cuh, built from the header's verified affine arithmetic.
#pragma once
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <vector>

#include "gpu_walk.cuh"

namespace hc {
using namespace gw;
typedef unsigned long long u64;
static inline u64 splitmix(u64 &s) {
    u64 z = (s += 0x9e3779b97f4a7c15ull);
    z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ull;
    z = (z ^ (z >> 27)) * 0x94d049bb133111ebull;
    return z ^ (z >> 31);
}
static inline h::U160 rand_scalar(u64 &sd) {
    return h::U160{{(uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)(splitmix(sd) & 3)}};
}

// ---------------- ECCp-131 ----------------
struct PrimeSetup {
    PrimeTable tab;
    h::FpPoint tabpt[16];
    std::vector<h::FpPoint> start;  // canonical class representatives, header Montgomery form
};
static inline h::FpPoint pclass(h::FpPoint p) { pcanon(p.y); return p; }
static inline PrimeSetup make_prime(int lanes, u64 seed) {
    PrimeSetup s;
    u64 sd = seed;
    std::vector<h::FpPoint> base;
    for (int i = 0; i < 48; ++i) base.push_back(h::fp_scalar_mul(rand_scalar(sd), i % 2 ? h::fp_target() : h::fp_generator()));
    for (int j = 0; j < 16; ++j) {
        const h::FpPoint t = h::fp_add_point(base[splitmix(sd) % 48], base[splitmix(sd) % 48]);
        if (t.infinity) { --j; continue; }
        s.tabpt[j] = pclass(t);
        s.tab.tx[j] = s.tabpt[j].x;
        s.tab.ty[j] = s.tabpt[j].y;
    }
    s.start.resize(lanes);
    for (auto &p : s.start)
        for (;;) {
            h::FpPoint q = h::fp_add_point(base[splitmix(sd) % 48], base[splitmix(sd) % 48]);
            if (q.infinity) continue;
            q = h::fp_add_point(q, base[splitmix(sd) % 48]);
            if (q.infinity) continue;
            q = pclass(q);
            bool hit = false;
            for (int j = 0; j < 16; ++j) hit = hit || h::fp_eq(q.x, s.tabpt[j].x);
            if (hit) continue;
            p = q;
            break;
        }
    return s;
}
static inline std::vector<uint32_t> prime_state(const std::vector<h::FpPoint> &pts) {
    const int L = (int)pts.size();
    std::vector<uint32_t> S((size_t)10 * L);
    for (int g = 0; g < L; ++g)
        for (int k = 0; k < 5; ++k) {
            S[(size_t)(0 * 5 + k) * L + g] = pts[g].x.v[k];
            S[(size_t)(1 * 5 + k) * L + g] = pts[g].y.v[k];
        }
    return S;
}
static inline h::FpPoint prime_lane(const std::vector<uint32_t> &S, int L, int g) {
    h::FpPoint p;
    p.infinity = 0;
    for (int k = 0; k < 5; ++k) { p.x.v[k] = S[(size_t)(0 * 5 + k) * L + g]; p.y.v[k] = S[(size_t)(1 * 5 + k) * L + g]; }
    return p;
}

// ---------------- ECC2-131 ----------------
static inline void poly_to_onb(const h::F2m &a, u64 out[3]) {
    out[0] = out[1] = out[2] = 0;
    for (int i = 0; i < NW; ++i)
        if ((a.v[i >> 6] >> (i & 63)) & 1)
            for (int l = 0; l < 3; ++l) out[l] ^= eccF131::Z_TO_ONB[i][l];
}
static inline h::F2m onb_to_poly(const u64 on[3]) {
    u64 out[3] = {0, 0, 0};
    for (int i = 0; i < NW; ++i)
        if ((on[i >> 6] >> (i & 63)) & 1)
            for (int l = 0; l < 3; ++l) out[l] ^= eccF131::GAMMA_TO_PB[i][l];
    return h::F2m{{out[0], out[1], out[2]}};
}
static inline bool f2eq(const h::F2m &a, const h::F2m &b) { return a.v[0] == b.v[0] && a.v[1] == b.v[1] && a.v[2] == b.v[2]; }
static inline bool onb_bit(const u64 on[3], int i) { return (on[i >> 6] >> (i & 63)) & 1; }
// ONB coordinate j of the pair (x, y) of a header point: used for the jump index
static inline int jump_of(const h::F2m &x) {
    u64 on[3];
    poly_to_onb(x, on);
    return (int)(onb_bit(on, 0) | (onb_bit(on, 1) << 1) | (onb_bit(on, 2) << 2) | (onb_bit(on, 3) << 3));
}
static inline h::F2Point bclass(const h::F2Point &p) {  // same rule as bcanon
    u64 on[3], oy[3];
    poly_to_onb(p.x, on);
    poly_to_onb(p.y, oy);
    for (int k = 0; k < NW; ++k)
        if (onb_bit(on, k)) {
            if (onb_bit(oy, k)) { h::F2Point r = p; r.y = h::f2_add(p.x, p.y); return r; }
            return p;
        }
    return p;
}
struct BinSetup {
    std::vector<W> tabx, taby, amask;  // tabx[i*16 + j]
    h::F2Point tabpt[16];
    std::vector<h::F2Point> start;
};
static inline BinSetup make_bin(int lanes, u64 seed) {
    BinSetup s;
    u64 sd = seed;
    std::vector<h::F2Point> base;
    for (int i = 0; i < 48; ++i) base.push_back(h::f2_scalar_mul(rand_scalar(sd), i % 2 ? h::f2_target() : h::f2_generator()));
    for (int j = 0; j < 16; ++j) {
        const h::F2Point t = h::f2_add_point(base[splitmix(sd) % 48], base[splitmix(sd) % 48]);
        if (t.infinity) { --j; continue; }
        s.tabpt[j] = bclass(t);
    }
    s.tabx.assign((size_t)NW * 16, 0);
    s.taby.assign((size_t)NW * 16, 0);
    for (int j = 0; j < 16; ++j) {
        u64 ox[3], oy[3];
        poly_to_onb(s.tabpt[j].x, ox);
        poly_to_onb(s.tabpt[j].y, oy);
        for (int i = 0; i < NW; ++i) {
            s.tabx[(size_t)i * 16 + j] = onb_bit(ox, i) ? ~(W)0 : 0;
            s.taby[(size_t)i * 16 + j] = onb_bit(oy, i) ? ~(W)0 : 0;
        }
    }
    u64 oa[3];
    poly_to_onb(h::f2_a(), oa);
    s.amask.assign(NW, 0);
    for (int i = 0; i < NW; ++i) s.amask[i] = onb_bit(oa, i) ? ~(W)0 : 0;
    s.start.resize(lanes);
    for (auto &p : s.start)
        for (;;) {
            h::F2Point q = h::f2_add_point(base[splitmix(sd) % 48], base[splitmix(sd) % 48]);
            if (q.infinity) continue;
            q = h::f2_add_point(q, base[splitmix(sd) % 48]);
            if (q.infinity) continue;
            q = bclass(q);
            bool hit = false;
            for (int j = 0; j < 16; ++j) hit = hit || f2eq(q.x, s.tabpt[j].x);
            if (hit) continue;
            p = q;
            break;
        }
    return s;
}
// flat lane q = (s*nthreads + tid)*32 + l  <->  bit l of word (s*NW + i)*nthreads + tid
static inline void bin_state(const std::vector<h::F2Point> &pts, int nthreads, int S, std::vector<W> &X, std::vector<W> &Y) {
    X.assign((size_t)S * NW * nthreads, 0);
    Y.assign((size_t)S * NW * nthreads, 0);
    for (int s = 0; s < S; ++s)
        for (int t = 0; t < nthreads; ++t)
            for (int l = 0; l < 32; ++l) {
                const size_t q = ((size_t)s * nthreads + t) * 32 + l;
                u64 ox[3], oy[3];
                poly_to_onb(pts[q].x, ox);
                poly_to_onb(pts[q].y, oy);
                for (int i = 0; i < NW; ++i) {
                    X[((size_t)s * NW + i) * nthreads + t] |= (W)onb_bit(ox, i) << l;
                    Y[((size_t)s * NW + i) * nthreads + t] |= (W)onb_bit(oy, i) << l;
                }
            }
}
static inline h::F2Point bin_lane(const std::vector<W> &X, const std::vector<W> &Y, int nthreads, int s, int t, int l) {
    u64 ox[3] = {0, 0, 0}, oy[3] = {0, 0, 0};
    for (int i = 0; i < NW; ++i) {
        if ((X[((size_t)s * NW + i) * nthreads + t] >> l) & 1u) ox[i >> 6] |= 1ull << (i & 63);
        if ((Y[((size_t)s * NW + i) * nthreads + t] >> l) & 1u) oy[i >> 6] |= 1ull << (i & 63);
    }
    return h::F2Point{onb_to_poly(ox), onb_to_poly(oy), 0};
}
}  // namespace hc
