// AVX-512 ECCp-131 rho walk: 8 independent walks per register, radix-2^27 Montgomery arithmetic
// (5 limbs, R = 2^135) on vpmuludq with lazy carries, Montgomery's trick across `slots` registers
// (one vector inversion per batch, i.e. one inversion chain per lane).
//
// Walk: j = canonical(x) mod 16; P <- canon(P + T[j]); canon keeps the one of (x, y), (x, p - y)
// whose canonical y is even (p is odd, so exactly one is), which is a function of the class.
//
// Invariants: field values are held as 5 limbs below 2^27 and numerically in [0, 2p]; a
// Montgomery product of inputs below 4p is below 2p (R = 2^135 > 16p/1).  State points are kept
// fully reduced (x, y in [0, p)) so end states compare limb for limb.
//
// Gates (nonzero exit on failure):
//   G1 field oracle: 8192 random products and 512 inversions against the header's prime field
//      (gpu/certicom131/certicom131.cuh), compared through scalar big-integer conversions
//      that do not use the vector code, 0 mismatches;
//   G2 step oracle: every lane after one iteration equals +-(P + T[j]) of the header's affine add;
//   G3 end points on the curve (header fp_on_curve) after the run (a zero denominator, which has
//      probability about 2^-131 per step, would leave a lane off the curve);
//   G4 trajectories do not depend on the slot count.
//
// Output: JSON lines.
#include <immintrin.h>

#include <algorithm>
#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>

#include "pf64.h"

using Clock = std::chrono::steady_clock;
static double now_s() { return std::chrono::duration<double>(Clock::now().time_since_epoch()).count(); }
static u64 splitmix(u64 &s) {
    u64 z = (s += 0x9e3779b97f4a7c15ull);
    z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ull;
    z = (z ^ (z >> 27)) * 0x94d049bb133111ebull;
    return z ^ (z >> 31);
}
static volatile u64 g_sink;

static const int NL = 5;
static const int RB = 27;
static const u64 MASK = (1ull << RB) - 1;

// ---------- scalar helpers (big integers as 3 x 64 canonical limbs) ----------
static void to_limbs27(const pf::F &c, u64 out[NL]) {
    for (int k = 0; k < NL; ++k) {
        u64 v = 0;
        for (int b = 0; b < RB; ++b) {
            const int pos = k * RB + b;
            if (pos < 192 && ((c.v[pos >> 6] >> (pos & 63)) & 1)) v |= 1ull << b;
        }
        out[k] = v;
    }
}
static pf::F from_limbs27(const u64 in[NL]) {
    pf::F r{{0, 0, 0}};
    for (int k = 0; k < NL; ++k)
        for (int b = 0; b < RB; ++b)
            if ((in[k] >> b) & 1) {
                const int pos = k * RB + b;
                r.v[pos >> 6] |= 1ull << (pos & 63);
            }
    return r;
}
static pf::F reduce_p(pf::F x) {  // value below 4p -> canonical
    for (int i = 0; i < 3 && pf::ge_p(x.v); ++i) {
        u128 br = 0;
        for (int j = 0; j < 3; ++j) { u128 d = (u128)x.v[j] - pf::P[j] - (u64)br; x.v[j] = (u64)d; br = (d >> 64) & 1; }
    }
    return x;
}
static pf::F mont135(const pf::F &c) {  // c * 2^135 mod p, canonical in/out, by doubling
    pf::F x = c;
    for (int i = 0; i < 135; ++i) x = pf::add(x, x);
    return x;
}

// ---------- vector field ----------
struct V { __m512i l[NL]; };
static __m512i vmask, vN0;
static __m512i vP[NL], vP2[NL];  // p and 2p in limbs
static V vOne;                   // Montgomery one (R mod p)
static u64 g_pl[NL], g_p2l[NL];
static u64 g_n0;

static inline V vmul(const V &a, const V &b) {
    __m512i t[2 * NL];
    for (int i = 0; i < 2 * NL; ++i) t[i] = _mm512_setzero_si512();
    for (int i = 0; i < NL; ++i)
        for (int j = 0; j < NL; ++j) t[i + j] = _mm512_add_epi64(t[i + j], _mm512_mul_epu32(a.l[i], b.l[j]));
    for (int i = 0; i < NL; ++i) {
        const __m512i q = _mm512_and_si512(_mm512_mul_epu32(_mm512_and_si512(t[i], vmask), vN0), vmask);
        for (int j = 0; j < NL; ++j) t[i + j] = _mm512_add_epi64(t[i + j], _mm512_mul_epu32(q, vP[j]));
        t[i + 1] = _mm512_add_epi64(t[i + 1], _mm512_srli_epi64(t[i], RB));
    }
    V r;
    __m512i c = _mm512_setzero_si512();
    for (int k = 0; k < NL; ++k) {
        const __m512i v = _mm512_add_epi64(t[NL + k], c);
        r.l[k] = _mm512_and_si512(v, vmask);
        c = _mm512_srli_epi64(v, RB);
    }
    return r;
}
// carry-propagate limbwise sums to limbs below 2^27
static inline void vnorm(V &a) {
    __m512i c = _mm512_setzero_si512();
    for (int k = 0; k < NL; ++k) {
        const __m512i v = _mm512_add_epi64(a.l[k], c);
        a.l[k] = _mm512_and_si512(v, vmask);
        c = _mm512_srli_epi64(v, RB);
    }
}
// r = a - m (limbwise with borrow); returns r and the final borrow mask (1 when a < m)
static inline __m512i vsub_borrow(const V &a, const __m512i *m, V &r) {
    __m512i bw = _mm512_setzero_si512();
    for (int k = 0; k < NL; ++k) {
        const __m512i v = _mm512_sub_epi64(_mm512_sub_epi64(a.l[k], m[k]), bw);
        bw = _mm512_srli_epi64(v, 63);
        r.l[k] = _mm512_and_si512(v, vmask);
    }
    return bw;
}
static inline V vselect(const __m512i flag, const V &if_set, const V &if_clear) {
    const __mmask8 k = _mm512_test_epi64_mask(flag, flag);
    V r;
    for (int i = 0; i < NL; ++i) r.l[i] = _mm512_mask_blend_epi64(k, if_clear.l[i], if_set.l[i]);
    return r;
}
// a + b in [0, 2p] for a, b in [0, 2p]
static inline V vadd(const V &a, const V &b) {
    V s;
    for (int k = 0; k < NL; ++k) s.l[k] = _mm512_add_epi64(a.l[k], b.l[k]);
    vnorm(s);
    V d;
    const __m512i bw = vsub_borrow(s, vP2, d);  // borrow set: s < 2p, keep s
    return vselect(bw, s, d);
}
// a - b mod p in [0, 2p] for a, b in [0, 2p]
static inline V vsub(const V &a, const V &b) {
    V d;
    __m512i brw = _mm512_setzero_si512();
    for (int k = 0; k < NL; ++k) {
        const __m512i v = _mm512_sub_epi64(_mm512_sub_epi64(a.l[k], b.l[k]), brw);
        brw = _mm512_srli_epi64(v, 63);
        d.l[k] = _mm512_and_si512(v, vmask);
    }
    // negative result: add 2p (the final carry out of limb 4 wraps modulo 2^135 and is dropped)
    V e;
    __m512i c = _mm512_setzero_si512();
    for (int k = 0; k < NL; ++k) {
        const __m512i v = _mm512_add_epi64(_mm512_add_epi64(d.l[k], vP2[k]), c);
        e.l[k] = _mm512_and_si512(v, vmask);
        c = _mm512_srli_epi64(v, RB);
    }
    return vselect(brw, e, d);
}
// fully reduce a value in [0, 2p) to [0, p)
static inline V vreduce(const V &a) {
    V d;
    const __m512i bw = vsub_borrow(a, vP, d);
    return vselect(bw, a, d);
}
static inline V vsplat(const u64 limbs[NL]) {
    V r;
    for (int k = 0; k < NL; ++k) r.l[k] = _mm512_set1_epi64((long long)limbs[k]);
    return r;
}
static V vinv(const V &a, const int *ebits, int nbits) {  // Fermat, exponent bits high to low
    V r = vOne;
    for (int i = 0; i < nbits; ++i) {
        r = vmul(r, r);
        if (ebits[i]) r = vmul(r, a);
    }
    return r;
}
static void vset_lane(V &v, int lane, const u64 limbs[NL]) {
    alignas(64) u64 tmp[8];
    for (int k = 0; k < NL; ++k) {
        _mm512_store_si512((void *)tmp, v.l[k]);
        tmp[lane] = limbs[k];
        v.l[k] = _mm512_load_si512((const void *)tmp);
    }
}
static void vget_lane(const V &v, int lane, u64 limbs[NL]) {
    alignas(64) u64 tmp[8];
    for (int k = 0; k < NL; ++k) {
        _mm512_store_si512((void *)tmp, v.l[k]);
        limbs[k] = tmp[lane];
    }
}
static void init_field() {
    to_limbs27(pf::F{{pf::P[0], pf::P[1], pf::P[2]}}, g_pl);
    const pf::F p2 = pf::F{{pf::P[0] << 1, (pf::P[1] << 1) | (pf::P[0] >> 63), (pf::P[2] << 1) | (pf::P[1] >> 63)}};
    to_limbs27(p2, g_p2l);
    u64 x = g_pl[0];
    for (int i = 0; i < 6; ++i) x *= 2 - g_pl[0] * x;  // inverse of p0 modulo 2^64
    g_n0 = ((u64)0 - x) & MASK;
    vmask = _mm512_set1_epi64((long long)MASK);
    vN0 = _mm512_set1_epi64((long long)g_n0);
    for (int k = 0; k < NL; ++k) {
        vP[k] = _mm512_set1_epi64((long long)g_pl[k]);
        vP2[k] = _mm512_set1_epi64((long long)g_p2l[k]);
    }
    u64 one[NL];
    pf::init();
    to_limbs27(mont135(pf::F{{1, 0, 0}}), one);
    vOne = vsplat(one);
}

// ---------- walk ----------
struct Table { __m512i xlo[NL], xhi[NL], ylo[NL], yhi[NL]; };

struct Walk {
    int S;
    std::vector<V> X, Y, Tx, Ty, den, pre;
    Table tab;
    int ebits[131];
    int nbits = 131;

    void init(int slots) {
        S = slots;
        Tx.resize(S); Ty.resize(S); den.resize(S); pre.resize(S);
        // exponent p - 2, bits high to low
        u64 e[3] = {pf::P[0] - 2, pf::P[1], pf::P[2]};
        for (int i = 0; i < 131; ++i) ebits[i] = (int)((e[(130 - i) >> 6] >> ((130 - i) & 63)) & 1);
    }
    void load_table(const u64 tx[16][NL], const u64 ty[16][NL]) {
        for (int k = 0; k < NL; ++k) {
            alignas(64) u64 a[8], b[8], c[8], d[8];
            for (int j = 0; j < 8; ++j) { a[j] = tx[j][k]; b[j] = tx[j + 8][k]; c[j] = ty[j][k]; d[j] = ty[j + 8][k]; }
            tab.xlo[k] = _mm512_load_si512((const void *)a);
            tab.xhi[k] = _mm512_load_si512((const void *)b);
            tab.ylo[k] = _mm512_load_si512((const void *)c);
            tab.yhi[k] = _mm512_load_si512((const void *)d);
        }
    }
    inline void lookup(const V &x, V &tx, V &ty) const {
        const __m512i idx = _mm512_and_si512(x.l[0], _mm512_set1_epi64(15));
        for (int k = 0; k < NL; ++k) {
            tx.l[k] = _mm512_permutex2var_epi64(tab.xlo[k], idx, tab.xhi[k]);
            ty.l[k] = _mm512_permutex2var_epi64(tab.ylo[k], idx, tab.yhi[k]);
        }
    }
    void iterate() {
        V acc = vOne;
        for (int s = 0; s < S; ++s) {
            lookup(X[s], Tx[s], Ty[s]);
            den[s] = vsub(Tx[s], X[s]);
            pre[s] = acc;
            acc = vmul(acc, den[s]);
        }
        V inv = vinv(acc, ebits, nbits);
        for (int s = S - 1; s >= 0; --s) {
            const V di = vmul(inv, pre[s]);
            inv = vmul(inv, den[s]);
            const V num = vsub(Ty[s], Y[s]);
            const V lam = vmul(num, di);
            const V l2 = vmul(lam, lam);
            const V x3 = vsub(vsub(l2, X[s]), Tx[s]);
            const V t = vsub(X[s], x3);
            const V y3 = vsub(vmul(lam, t), Y[s]);
            const V x3c = vreduce(x3);
            V yc = vreduce(y3);
            // canon: keep the representative with even canonical y; -y = p - y flips the parity
            V neg;
            __m512i bw = _mm512_setzero_si512();
            for (int k = 0; k < NL; ++k) {
                const __m512i v = _mm512_sub_epi64(_mm512_sub_epi64(vP[k], yc.l[k]), bw);
                bw = _mm512_srli_epi64(v, 63);
                neg.l[k] = _mm512_and_si512(v, vmask);
            }
            const __m512i odd = _mm512_and_si512(yc.l[0], _mm512_set1_epi64(1));
            X[s] = x3c;
            Y[s] = vselect(odd, neg, yc);
        }
    }
};

// ---------- main ----------
struct Pt { pf::F x, y; };  // canonical, Montgomery R = 2^135, as 3 x 64 integers
static Pt to_mont_pt(const h::FpPoint &p) {
    return Pt{mont135(pf::from_header(h::fp_to_canonical(p.x))), mont135(pf::from_header(h::fp_to_canonical(p.y)))};
}
static void limbs_of(const pf::F &c, u64 out[NL]) { to_limbs27(c, out); }
static pf::F canon_of_lane(const V &v, int lane) {
    u64 l[NL];
    vget_lane(v, lane, l);
    return reduce_p(from_limbs27(l));
}
static bool eqF(const pf::F &a, const pf::F &b) { return a.v[0] == b.v[0] && a.v[1] == b.v[1] && a.v[2] == b.v[2]; }
static pf::F negF(const pf::F &a) {
    if (a.v[0] == 0 && a.v[1] == 0 && a.v[2] == 0) return a;
    pf::F r;
    u128 br = 0;
    for (int j = 0; j < 3; ++j) { u128 d = (u128)pf::P[j] - a.v[j] - (u64)br; r.v[j] = (u64)d; br = (d >> 64) & 1; }
    return r;
}
static h::FpPoint header_point(const Pt &p) {  // Montgomery-135 canonical -> header point (via 2^-135: repeated halving)
    auto half = [](pf::F x) {
        if (x.v[0] & 1) {
            u128 c = 0;
            for (int j = 0; j < 3; ++j) { c += (u128)x.v[j] + pf::P[j]; x.v[j] = (u64)c; c >>= 64; }
            // carry out of the top limb cannot occur: x < p and p < 2^131
        }
        x.v[0] = (x.v[0] >> 1) | (x.v[1] << 63);
        x.v[1] = (x.v[1] >> 1) | (x.v[2] << 63);
        x.v[2] >>= 1;
        return x;
    };
    pf::F x = p.x, y = p.y;
    for (int i = 0; i < 135; ++i) { x = half(x); y = half(y); }
    return h::FpPoint{h::fp_from_canonical(pf::to_header_canonical(x)), h::fp_from_canonical(pf::to_header_canonical(y)), 0};
}

int main(int argc, char **argv) {
    int rounds = 5;
    u64 steps = 40;
    bool timing_only = false;
    int tslots = 64;
    for (int i = 1; i < argc; ++i) {
        if (!strcmp(argv[i], "--timing-only")) timing_only = true;
        else if (!strcmp(argv[i], "--slots") && i + 1 < argc) tslots = atoi(argv[++i]);
        else if (!strcmp(argv[i], "--steps") && i + 1 < argc) steps = strtoull(argv[++i], 0, 10);
        else if (!strcmp(argv[i], "--rounds") && i + 1 < argc) rounds = atoi(argv[++i]);
    }
    init_field();
    int rc = 0;
    u64 sd = 0x131;

    // ---- G1: field oracle ----
    if (!timing_only) {
        int bad = 0;
        Walk w;
        w.init(1);
        for (int rep = 0; rep < 1024; ++rep) {
            V a, b;
            pf::F ca[8], cb[8];
            for (int l = 0; l < 8; ++l) {
                do { ca[l] = pf::F{{splitmix(sd), splitmix(sd), splitmix(sd) & 3}}; } while (pf::ge_p(ca[l].v));
                do { cb[l] = pf::F{{splitmix(sd), splitmix(sd), splitmix(sd) & 3}}; } while (pf::ge_p(cb[l].v));
                u64 la[NL], lb[NL];
                limbs_of(mont135(ca[l]), la);
                limbs_of(mont135(cb[l]), lb);
                vset_lane(a, l, la);
                vset_lane(b, l, lb);
            }
            const V r = vmul(a, b);
            for (int l = 0; l < 8; ++l) {
                // Montgomery product of x*R and y*R is x*y*R: expected = mont135(c) with c = x*y mod p from the header
                const h::Fp ha = pf::to_header_canonical(ca[l]), hb = pf::to_header_canonical(cb[l]);
                const h::Fp c = h::fp_to_canonical(h::fp_mont_mul(h::fp_from_canonical(ha), h::fp_from_canonical(hb)));
                if (!eqF(canon_of_lane(r, l), mont135(pf::from_header(c)))) ++bad;
            }
            if (rep < 64) {
                // inversion check on 8 lanes: a * inv(a) = 1
                const V ia = vinv(a, w.ebits, w.nbits);
                const V one = vmul(a, ia);
                for (int l = 0; l < 8; ++l) {
                    pf::F expect_one = mont135(pf::F{{1, 0, 0}});
                    if (!eqF(canon_of_lane(one, l), expect_one)) ++bad;
                }
            }
        }
        printf("{\"kind\":\"gate\",\"gate\":\"G1\",\"arm\":\"ECCp-131/avx512x8\",\"products\":8192,\"inversions\":512,\"mismatches\":%d}\n", bad);
        rc |= bad != 0;
    }

    // ---- points ----
    std::vector<h::FpPoint> base;
    for (int i = 0; i < 48; ++i) {
        h::U160 k{{(uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)(splitmix(sd) & 3)}};
        base.push_back(h::fp_scalar_mul(k, i % 2 ? h::fp_target() : h::fp_generator()));
    }
    auto addp = [&](const h::FpPoint &p, const h::FpPoint &q) { return h::fp_add_point(p, q); };
    auto canon_hp = [&](const h::FpPoint &p) {  // same class rule as the vector code: even canonical (Montgomery-135) y
        Pt m = to_mont_pt(p);
        if (m.y.v[0] & 1) return header_point(Pt{m.x, negF(m.y)});
        return p;
    };
    h::FpPoint tab_pts[16];
    u64 tx[16][NL], ty[16][NL];
    for (int j = 0; j < 16; ++j) {
        const h::FpPoint t = addp(base[splitmix(sd) % 48], base[splitmix(sd) % 48]);
        if (t.infinity) { --j; continue; }
        tab_pts[j] = canon_hp(t);
        const Pt m = to_mont_pt(tab_pts[j]);
        limbs_of(m.x, tx[j]);
        limbs_of(m.y, ty[j]);
    }
    const int MAXS = 64;
    std::vector<h::FpPoint> start((size_t)MAXS * 8);
    int rejected = 0;
    for (auto &p : start) {
        for (;;) {
            h::FpPoint q = addp(base[splitmix(sd) % 48], base[splitmix(sd) % 48]);
            if (q.infinity) { ++rejected; continue; }
            q = addp(q, base[splitmix(sd) % 48]);
            if (q.infinity) { ++rejected; continue; }
            q = canon_hp(q);
            bool hit = false;
            for (int j = 0; j < 16; ++j) hit = hit || h::fp_eq(q.x, tab_pts[j].x);
            if (hit) { ++rejected; continue; }
            p = q;
            break;
        }
    }
    printf("{\"kind\":\"setup\",\"arm\":\"ECCp-131/avx512x8\",\"starts\":%zu,\"rejected_degenerate\":%d}\n", start.size(), rejected);
    auto set_state = [&](Walk &w, int slots) {
        w.X.assign(slots, V());
        w.Y.assign(slots, V());
        for (int s = 0; s < slots; ++s)
            for (int l = 0; l < 8; ++l) {
                const Pt m = to_mont_pt(start[s * 8 + l]);
                u64 lx[NL], ly[NL];
                limbs_of(m.x, lx);
                limbs_of(m.y, ly);
                vset_lane(w.X[s], l, lx);
                vset_lane(w.Y[s], l, ly);
            }
    };

    // ---- G2: one iteration against the header's affine add ----
    if (!timing_only) {
        Walk w;
        w.init(64);
        w.load_table(tx, ty);
        set_state(w, 64);
        std::vector<int> jidx;
        for (int s = 0; s < 64; ++s)
            for (int l = 0; l < 8; ++l) jidx.push_back((int)(canon_of_lane(w.X[s], l).v[0] & 15));
        w.iterate();
        int bad = 0, checked = 0;
        for (int s = 0; s < 64; ++s)
            for (int l = 0; l < 8; ++l) {
                const h::FpPoint ref = h::fp_add_point(start[s * 8 + l], tab_pts[jidx[s * 8 + l]]);
                const Pt rm = to_mont_pt(ref);
                const pf::F gx = canon_of_lane(w.X[s], l), gy = canon_of_lane(w.Y[s], l);
                const bool same = eqF(gx, rm.x) && (eqF(gy, rm.y) || eqF(gy, negF(rm.y)));
                const bool even = (gy.v[0] & 1) == 0;
                if (!same || !even) ++bad;
                ++checked;
            }
        printf("{\"kind\":\"gate\",\"gate\":\"G2\",\"arm\":\"ECCp-131/avx512x8\",\"lanes_checked\":%d,\"mismatches\":%d}\n", checked, bad);
        rc |= bad != 0;
    }

    // ---- G3 + G4 ----
    if (!timing_only) {
        std::vector<std::vector<pf::F>> ends;
        for (int slots : {16, 32, 64}) {
            Walk w;
            w.init(slots);
            w.load_table(tx, ty);
            set_state(w, slots);
            for (int i = 0; i < 24; ++i) w.iterate();
            std::vector<pf::F> v;
            int offcurve = 0;
            for (int s = 0; s < 16; ++s)
                for (int l = 0; l < 8; ++l) {
                    const pf::F x = canon_of_lane(w.X[s], l), y = canon_of_lane(w.Y[s], l);
                    v.push_back(x);
                    v.push_back(y);
                    if (!h::fp_on_curve(header_point(Pt{x, y}))) ++offcurve;
                }
            ends.push_back(v);
            bool same = true;
            for (size_t i = 0; i < v.size(); ++i) same = same && eqF(v[i], ends[0][i]);
            printf("{\"kind\":\"gate\",\"gate\":\"G3G4\",\"arm\":\"ECCp-131/avx512x8\",\"slots\":%d,\"iterations\":24,\"sampled_lanes\":128,\"off_curve\":%d,\"matches_16_slot_run\":%s}\n", slots, offcurve, same ? "true" : "false");
            rc |= offcurve != 0 || !same;
        }
        // phase profile
        Walk w;
        w.init(16);
        set_state(w, 16);
        V a = w.X[0], b = w.Y[0];
        const int N = 20000;
        double t0 = now_s();
        for (int i = 0; i < N; ++i) { a = vmul(a, b); b = vmul(b, a); }
        const double tm = (now_s() - t0) / (2.0 * N);
        g_sink = (u64)_mm_cvtsi128_si64(_mm512_castsi512_si128(a.l[0]));
        printf("{\"kind\":\"profile\",\"arm\":\"ECCp-131/avx512x8\",\"dependent_vmul_ns\":%.2f,\"vmul_ns_per_lane\":%.2f}\n", tm * 1e9, tm * 1e9 / 8);
    }

    // ---- timing ----
    int cfg[4] = {16, 32, 64, 64};
    if (timing_only) cfg[0] = tslots;
    std::vector<std::vector<double>> ns(4);
    const int ncfg = timing_only ? 1 : 4;
    for (int r = 0; r < rounds; ++r)
        for (int k = 0; k < ncfg; ++k) {
            Walk w;
            w.init(cfg[k]);
            w.load_table(tx, ty);
            set_state(w, cfg[k]);
            const double t0 = now_s();
            for (u64 i = 0; i < steps; ++i) w.iterate();
            const double dt = now_s() - t0;
            g_sink = (u64)_mm_cvtsi128_si64(_mm512_castsi512_si128(w.X[0].l[0]));
            ns[k].push_back(dt * 1e9 / ((double)steps * cfg[k] * 8));
            int off = 0;  // end-of-run check: 32 sampled lanes on the curve (a zero denominator would leave a lane off it)
            for (int q = 0; q < 32; ++q) {
                const int s = (q * 7) % cfg[k], l = (q * 3) % 8;
                const Pt e{canon_of_lane(w.X[s], l), canon_of_lane(w.Y[s], l)};
                if (!h::fp_on_curve(header_point(e))) ++off;
            }
            if (off) { rc = 1; printf("{\"kind\":\"gate\",\"gate\":\"G3\",\"arm\":\"ECCp-131/avx512x8\",\"timing_run_off_curve\":%d}\n", off); }
        }
    for (int k = 0; k < ncfg; ++k) {
        auto v = ns[k];
        std::sort(v.begin(), v.end());
        const double med = v[v.size() / 2];
        printf("{\"kind\":\"step\",\"arm\":\"ECCp-131/avx512x8\",\"slots\":%d,\"lanes_per_slot\":8,\"iterations\":%llu,\"rounds\":%d,\"ns_per_step_median\":%.2f,\"ns_per_step_min\":%.2f,\"ns_per_step_max\":%.2f,\"steps_per_s_median\":%.0f}\n",
               cfg[k], (unsigned long long)steps, rounds, med, v.front(), v.back(), 1e9 / med);
    }
    return rc;
}
