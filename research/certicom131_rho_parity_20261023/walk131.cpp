// Rho-step parity study on the exact ECCp-131 and ECC2-131 groups.
//
// Successor of research/certicom131_rho_ic_bench_20261022/bench131.cpp (left
// unchanged).  Same walk: j = hash(x) & 15, P <- canon(P + T[j]), negation map,
// one field inversion per batch (Montgomery's trick).  New here:
//   * `chains`: the batch is split into C interleaved product chains, so the C
//     serial multiply chains of the trick overlap in the pipeline.  One
//     inversion per batch still; the inverses are mathematically identical for
//     every C, so lane trajectories must not depend on C.
//   * PrimeFast: 3 x 64-bit Montgomery field (R = 2^192) for the exact p.
//   * BinFast: GF(2^131) with 3 pclmulqdq per product (Karatsuba on the low two
//     limbs, masked shifts for the 3-bit top limb), 2 per square.
//
// Gates (nonzero exit if any fails):
//   G1 field oracle: 20000 random mul / sqr and 200 inversions against the
//      header arithmetic (gpu/certicom131/certicom131.cuh), 0 mismatches;
//   G2 step oracle: every pair of 24 curve points and the 16 table slots, the
//      arm's affine add equals the header's add exactly;
//   G3 every lane end point is on the curve;
//   G4 end states after 64 steps are identical for every (batch, chains).
//
// Output: JSON lines on stdout.
#include <immintrin.h>

#include <algorithm>
#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>

#include "../../gpu/certicom131/certicom131.cuh"

namespace h = certicom131;
using u64 = uint64_t;
using u128 = unsigned __int128;
using Clock = std::chrono::steady_clock;

static double now_s() { return std::chrono::duration<double>(Clock::now().time_since_epoch()).count(); }
static u64 splitmix(u64 &s) {
    u64 z = (s += 0x9e3779b97f4a7c15ull);
    z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ull;
    z = (z ^ (z >> 27)) * 0x94d049bb133111ebull;
    return z ^ (z >> 31);
}
static volatile u64 g_sink;

// =====================================================================
// Prime field, 3 x 64-bit Montgomery, R = 2^192
// =====================================================================
#include "pf64.h"

struct PrimeFast {
    static constexpr const char *name = "ECCp-131/mont64";
    using F = pf::F;
    struct Pt { F x, y; };
    static F mul(const F &a, const F &b) { return pf::mul(a, b); }
    static F inv(const F &a) { return pf::inv(a); }
    static F one() { return pf::one(); }
    static bool zero(const F &a) { return pf::is_zero(a); }
    static F den(const Pt &p, const Pt &t) { return pf::sub(t.x, p.x); }
    static bool eq_pt(const Pt &a, const Pt &b) { return pf::eq(a.x, b.x) && pf::eq(a.y, b.y); }
    static u64 hash(const Pt &p) { return p.x.v[0] >> 3; }
    static Pt neg(const Pt &p) { return Pt{p.x, pf::sub(F{{0, 0, 0}}, p.y)}; }
    static bool same_x(const Pt &a, const Pt &b) { return pf::eq(a.x, b.x); }
    static Pt canon(const Pt &p) {  // smaller of y, -y (Montgomery form), branchless
        const Pt n = neg(p);
        // take n when p.y > n.y lexicographically from the top limb
        u64 gt = 0, eqs = ~0ull;
        for (int i = 2; i >= 0; --i) {
            gt |= eqs & (u64)(p.y.v[i] > n.y.v[i]);
            eqs &= (u64)0 - (u64)(p.y.v[i] == n.y.v[i]);
            eqs = (u64)0 - (eqs & 1);
        }
        const u64 m = (u64)0 - (gt & 1);
        Pt r;
        for (int i = 0; i < 3; ++i) r.y.v[i] = (n.y.v[i] & m) | (p.y.v[i] & ~m);
        r.x = p.x;
        return r;
    }
    static F lam(const Pt &p, const Pt &t, const F &dinv) { return pf::mul(pf::sub(t.y, p.y), dinv); }
    static F x3f(const F &l, const Pt &p, const Pt &t) { return pf::sub(pf::sub(pf::mul(l, l), p.x), t.x); }
    static F y3f(const F &l, const Pt &p, const F &x3) { return pf::sub(pf::mul(l, pf::sub(p.x, x3)), p.y); }
    static Pt finish(const Pt &p, const Pt &t, const F &dinv) {
        const F lambda = pf::mul(pf::sub(t.y, p.y), dinv);
        const F x3 = pf::sub(pf::sub(pf::mul(lambda, lambda), p.x), t.x);
        const F y3 = pf::sub(pf::mul(lambda, pf::sub(p.x, x3)), p.y);
        return Pt{x3, y3};
    }
    static Pt add_ref(const Pt &p, const Pt &q) { return finish(p, q, inv(den(p, q))); }  // p.x != q.x assumed
    static u64 fold(const Pt &p) {
        u64 x = 0;
        for (int i = 0; i < 3; ++i) x = x * 0x100000001b3ull ^ p.x.v[i] ^ (p.y.v[i] << 7);
        return x;
    }
    static Pt from_header(const h::FpPoint &p) { return Pt{pf::to_mont(pf::from_header(h::fp_to_canonical(p.x))), pf::to_mont(pf::from_header(h::fp_to_canonical(p.y)))}; }
    static h::FpPoint to_header(const Pt &p) {
        return h::FpPoint{h::fp_from_canonical(pf::to_header_canonical(pf::from_mont(p.x))),
                          h::fp_from_canonical(pf::to_header_canonical(pf::from_mont(p.y))), 0};
    }
    // exceptional cases (doubling, inverse points) go through the header reference formulas
    static Pt safe_add(const Pt &p, const Pt &q) {
        if (!pf::eq(p.x, q.x)) return add_ref(p, q);
        return from_header(h::fp_add_point(to_header(p), to_header(q)));
    }
    static bool matches_header(const Pt &p, const h::FpPoint &q) {
        return pf::eq(pf::from_mont(p.x), pf::from_header(h::fp_to_canonical(q.x))) &&
               pf::eq(pf::from_mont(p.y), pf::from_header(h::fp_to_canonical(q.y)));
    }
    static bool on_curve(const Pt &p) {  // y^2 = x^3 + a x + b, header constants converted
        const F a = pf::to_mont(pf::from_header(h::fp_a_canonical()));
        const F b = pf::to_mont(pf::from_header(h::fp_b_canonical()));
        const F y2 = pf::mul(p.y, p.y), x2 = pf::mul(p.x, p.x), x3 = pf::mul(x2, p.x);
        return pf::eq(y2, pf::add(pf::add(x3, pf::mul(a, p.x)), b));
    }
};

// =====================================================================
// Binary field GF(2^131) = GF(2)[z]/(z^131 + z^13 + z^2 + z + 1), pclmulqdq
// =====================================================================
namespace bf {
using F = h::F2m;
static inline __m128i load(u64 a) { return _mm_cvtsi64_si128((long long)a); }
static inline void cl(u64 a, u64 b, u64 &lo, u64 &hi) {
    __m128i r = _mm_clmulepi64_si128(load(a), load(b), 0);
    lo = (u64)_mm_cvtsi128_si64(r);
    hi = (u64)_mm_extract_epi64(r, 1);
}
// x * a for a 3-bit x and a 128-bit (a0,a1): result up to 131 bits in r[0..2]
template <int K>
static inline void small_k(u64 x, u64 a0, u64 a1, u64 r[3]) {
    const u64 m = (u64)0 - ((x >> K) & 1);
    r[0] ^= m & (a0 << K);
    r[1] ^= m & ((a1 << K) | (K ? a0 >> (64 - (K ? K : 1)) : 0));
    r[2] ^= m & (K ? a1 >> (64 - (K ? K : 1)) : 0);
}
static inline void small_mul(u64 x, u64 a0, u64 a1, u64 r[3]) {
    r[0] = r[1] = r[2] = 0;
    small_k<0>(x, a0, a1, r);
    small_k<1>(x, a0, a1, r);
    small_k<2>(x, a0, a1, r);
}
static inline F reduce(const u64 p[5]) {
    u64 H0 = (p[2] >> 3) | (p[3] << 61), H1 = (p[3] >> 3) | (p[4] << 61), H2 = p[4] >> 3;
    u64 s0 = p[0] ^ H0, s1 = p[1] ^ H1, s2 = (p[2] & 7ull) ^ H2;
    s0 ^= H0 << 1; s1 ^= (H1 << 1) | (H0 >> 63); s2 ^= (H2 << 1) | (H1 >> 63);
    s0 ^= H0 << 2; s1 ^= (H1 << 2) | (H0 >> 62); s2 ^= (H2 << 2) | (H1 >> 62);
    s0 ^= H0 << 13; s1 ^= (H1 << 13) | (H0 >> 51); s2 ^= (H2 << 13) | (H1 >> 51);
    const u64 T = s2 >> 3;
    s2 &= 7ull;
    s0 ^= T ^ (T << 1) ^ (T << 2) ^ (T << 13);
    return F{{s0, s1, s2}};
}
static inline F mul(const F &a, const F &b) {
    u64 l0, h0, l1, h1, lm, hm;
    cl(a.v[0], b.v[0], l0, h0);
    cl(a.v[1], b.v[1], l1, h1);
    cl(a.v[0] ^ a.v[1], b.v[0] ^ b.v[1], lm, hm);
    u64 p[5];
    p[0] = l0;
    p[1] = h0 ^ l0 ^ l1 ^ lm;
    p[2] = h0 ^ h1 ^ l1 ^ hm;
    p[3] = h1;
    p[4] = 0;
    // (a2 * X^128) * b01 and (b2 * X^128) * a01, each 131 bits
    u64 ra[3], rb[3];
    small_mul(a.v[2], b.v[0], b.v[1], ra);
    small_mul(b.v[2], a.v[0], a.v[1], rb);
    p[2] ^= ra[0] ^ rb[0];
    p[3] ^= ra[1] ^ rb[1];
    p[4] ^= ra[2] ^ rb[2];
    // a2 * b2 * X^256: at most 5 bits
    u64 t = 0;
    for (int k = 0; k < 3; ++k) t ^= ((u64)0 - ((a.v[2] >> k) & 1)) & (b.v[2] << k);
    p[4] ^= t;
    return reduce(p);
}
static inline F sqr(const F &a) {
    u64 l0, h0, l1, h1, l2, h2;
    cl(a.v[0], a.v[0], l0, h0);
    cl(a.v[1], a.v[1], l1, h1);
    cl(a.v[2], a.v[2], l2, h2);
    const u64 p[5] = {l0, h0, l1, h1, l2};
    (void)h2;
    return reduce(p);
}
static inline F sqrn(F a, int n) { while (n--) a = sqr(a); return a; }
static F inv(const F &a) {  // Itoh-Tsujii, 8 M + 130 S
    F b2 = mul(sqrn(a, 1), a);
    F b4 = mul(sqrn(b2, 2), b2);
    F b8 = mul(sqrn(b4, 4), b4);
    F b16 = mul(sqrn(b8, 8), b8);
    F b32 = mul(sqrn(b16, 16), b16);
    F b64 = mul(sqrn(b32, 32), b32);
    F b65 = mul(sqr(b64), a);
    F b130 = mul(sqrn(b65, 65), b65);
    return sqr(b130);
}
}  // namespace bf

struct BinFast {
    static constexpr const char *name = "ECC2-131/clmul3";
    using F = bf::F;
    struct Pt { F x, y; };
    static F add(const F &a, const F &b) { return F{{a.v[0] ^ b.v[0], a.v[1] ^ b.v[1], a.v[2] ^ b.v[2]}}; }
    static F mul(const F &a, const F &b) { return bf::mul(a, b); }
    static F inv(const F &a) { return bf::inv(a); }
    static F one() { return F{{1, 0, 0}}; }
    static bool zero(const F &a) { return (a.v[0] | a.v[1] | a.v[2]) == 0; }
    static bool eqf(const F &a, const F &b) { return a.v[0] == b.v[0] && a.v[1] == b.v[1] && a.v[2] == b.v[2]; }
    static F den(const Pt &p, const Pt &t) { return add(p.x, t.x); }
    static bool eq_pt(const Pt &a, const Pt &b) { return eqf(a.x, b.x) && eqf(a.y, b.y); }
    static bool same_x(const Pt &a, const Pt &b) { return eqf(a.x, b.x); }
    static u64 hash(const Pt &p) { return p.x.v[0] >> 3; }
    static Pt canon(const Pt &p) {  // -P = (x, x + y); keep the lexicographically smaller y, branchless
        const F ny = add(p.x, p.y);
        u64 lt = 0, eqs = 1;
        for (int i = 2; i >= 0; --i) {
            lt |= eqs & (u64)(ny.v[i] < p.y.v[i]);
            eqs &= (u64)(ny.v[i] == p.y.v[i]);
        }
        const u64 m = (u64)0 - (lt & 1);
        Pt r;
        r.x = p.x;
        for (int i = 0; i < 3; ++i) r.y.v[i] = (ny.v[i] & m) | (p.y.v[i] & ~m);
        return r;
    }
    static F lam(const Pt &p, const Pt &t, const F &dinv) { return bf::mul(add(p.y, t.y), dinv); }
    static F x3f(const F &l, const Pt &p, const Pt &t) { return add(add(add(bf::sqr(l), l), add(p.x, t.x)), h::f2_a()); }
    static F y3f(const F &l, const Pt &p, const F &x3) { return add(add(bf::mul(l, add(p.x, x3)), x3), p.y); }
    static Pt finish(const Pt &p, const Pt &t, const F &dinv) {
        const F a = h::f2_a();
        const F lambda = bf::mul(add(p.y, t.y), dinv);
        const F x3 = add(add(add(bf::sqr(lambda), lambda), add(p.x, t.x)), a);
        const F y3 = add(add(bf::mul(lambda, add(p.x, x3)), x3), p.y);
        return Pt{x3, y3};
    }
    static Pt add_ref(const Pt &p, const Pt &q) { return finish(p, q, inv(den(p, q))); }
    static u64 fold(const Pt &p) {
        u64 x = 0;
        for (int i = 0; i < 3; ++i) x = x * 0x100000001b3ull ^ p.x.v[i] ^ (p.y.v[i] << 7);
        return x;
    }
    static Pt from_header(const h::F2Point &p) { return Pt{p.x, p.y}; }
    static Pt safe_add(const Pt &p, const Pt &q) {
        if (!eqf(p.x, q.x)) return add_ref(p, q);
        const h::F2Point r = h::f2_add_point(h::F2Point{p.x, p.y, 0}, h::F2Point{q.x, q.y, 0});
        return Pt{r.x, r.y};
    }
    static bool matches_header(const Pt &p, const h::F2Point &q) { return eqf(p.x, q.x) && eqf(p.y, q.y); }
    static bool on_curve(const Pt &p) {
        h::F2Point q{p.x, p.y, 0};
        return h::f2_on_curve(q);
    }
};

// =====================================================================
// Walk driver
// =====================================================================
template <class C>
struct Walk {
    using Pt = typename C::Pt;
    using F = typename C::F;
    Pt table[16];
    std::vector<Pt> start;
    int rejected = 0;

    void setup(int lanes, u64 seed) {
        // generator and target taken from the header, converted once
        Pt g, q;
        from_hdr_gq(g, q);
        u64 s = seed;
        auto combo = [&](int ka, int kb) {
            Pt acc = g;
            for (int i = 0; i < ka; ++i) acc = C::safe_add(acc, g);
            for (int i = 0; i < kb + 1; ++i) acc = C::safe_add(acc, q);
            return acc;
        };
        for (int j = 0; j < 16; ++j) table[j] = C::canon(combo(1 + (int)(splitmix(s) % 61), (int)(splitmix(s) % 61)));
        start.resize(lanes);
        rejected = 0;
        for (int i = 0; i < lanes; ++i) {
            for (;;) {
                const Pt c = C::canon(combo(1 + (int)(splitmix(s) % 200), (int)(splitmix(s) % 200)));
                bool hit = false;
                for (int j = 0; j < 16; ++j) hit = hit || C::same_x(c, table[j]);
                if (hit) { ++rejected; continue; }
                start[i] = c;
                break;
            }
        }
    }
    static void from_hdr_gq(Pt &g, Pt &q);

    // `batch` lanes share one inversion, split into `chains` interleaved product chains; the
    // finishing pass advances groups of GROUP lanes in lock-step so independent field
    // products overlap in the out-of-order window.
    static constexpr int GROUP = 4;
    u64 run(std::vector<Pt> &st, int batch, int chains, u64 steps) {
        const int lanes = (int)st.size();
        std::vector<F> pre(batch), den(batch), di(batch);
        std::vector<uint8_t> jx(batch), bad(batch);
        u64 exceptional = 0;
        F acc[16], pf_[16], sf[16], invc[16];
        for (int base = 0; base < lanes; base += batch) {
            Pt *p = &st[base];
            for (u64 it = 0; it < steps; ++it) {
                for (int c = 0; c < chains; ++c) acc[c] = C::one();
                for (int i = 0; i < batch; ++i) {
                    const int c = i % chains;
                    const int j = (int)(C::hash(p[i]) & 15);
                    jx[i] = (uint8_t)j;
                    F d = C::den(p[i], table[j]);
                    const bool z = C::zero(d);
                    bad[i] = z || C::eq_pt(p[i], table[j]);
                    if (z) { d = C::one(); }
                    den[i] = d;
                    pre[i] = acc[c];
                    acc[c] = C::mul(acc[c], d);
                }
                F run_ = C::one();
                for (int c = 0; c < chains; ++c) { pf_[c] = run_; run_ = C::mul(run_, acc[c]); }
                const F it_ = C::inv(run_);
                run_ = C::one();
                for (int c = chains - 1; c >= 0; --c) { sf[c] = run_; run_ = C::mul(run_, acc[c]); }
                for (int c = 0; c < chains; ++c) invc[c] = C::mul(it_, C::mul(pf_[c], sf[c]));
                for (int i = batch - 1; i >= 0; --i) {
                    const int c = i % chains;
                    di[i] = C::mul(invc[c], pre[i]);
                    invc[c] = C::mul(invc[c], den[i]);
                }
                for (int i = 0; i < batch; i += GROUP) {
                    F l[GROUP], x3[GROUP], y3[GROUP];
                    for (int k = 0; k < GROUP; ++k) l[k] = C::lam(p[i + k], table[jx[i + k]], di[i + k]);
                    for (int k = 0; k < GROUP; ++k) x3[k] = C::x3f(l[k], p[i + k], table[jx[i + k]]);
                    for (int k = 0; k < GROUP; ++k) y3[k] = C::y3f(l[k], p[i + k], x3[k]);
                    for (int k = 0; k < GROUP; ++k) {
                        if (bad[i + k]) {  // exceptional lane: deterministic move
                            ++exceptional;
                            p[i + k] = table[(C::hash(p[i + k]) + 1) & 15];
                        } else {
                            p[i + k] = C::canon(Pt{x3[k], y3[k]});
                        }
                    }
                }
            }
        }
        return exceptional;
    }
};
template <>
void Walk<PrimeFast>::from_hdr_gq(Pt &g, Pt &q) { g = PrimeFast::from_header(h::fp_generator()); q = PrimeFast::from_header(h::fp_target()); }
template <>
void Walk<BinFast>::from_hdr_gq(Pt &g, Pt &q) { g = BinFast::from_header(h::f2_generator()); q = BinFast::from_header(h::f2_target()); }

// ---------- gates G1, G2 per arm ----------
static int gate_prime() {
    int bad = 0;
    u64 sd = 11;
    for (int i = 0; i < 20000; ++i) {
        pf::F a{{splitmix(sd), splitmix(sd), splitmix(sd) & 3}}, b{{splitmix(sd), splitmix(sd), splitmix(sd) & 3}};
        if (pf::ge_p(a.v) || pf::ge_p(b.v)) { --i; continue; }
        h::Fp ha = pf::to_header_canonical(a), hb = pf::to_header_canonical(b);
        const h::Fp ref = h::fp_to_canonical(h::fp_mont_mul(h::fp_from_canonical(ha), h::fp_from_canonical(hb)));
        const pf::F got = pf::from_mont(pf::mul(pf::to_mont(a), pf::to_mont(b)));
        if (!pf::eq(got, pf::from_header(ref))) ++bad;
        if (i < 200 && !pf::is_zero(a)) {
            const h::Fp rinv = h::fp_to_canonical(h::fp_inv(h::fp_from_canonical(ha)));
            if (!pf::eq(pf::from_mont(pf::inv(pf::to_mont(a))), pf::from_header(rinv))) ++bad;
        }
    }
    printf("{\"kind\":\"gate\",\"gate\":\"G1\",\"arm\":\"ECCp-131/mont64\",\"cases\":20000,\"mismatches\":%d}\n", bad);
    return bad != 0;
}
static int gate_binary() {
    int bad = 0;
    u64 sd = 13;
    for (int i = 0; i < 20000; ++i) {
        h::F2m a{{splitmix(sd), splitmix(sd), splitmix(sd) & 7ull}}, b{{splitmix(sd), splitmix(sd), splitmix(sd) & 7ull}};
        if (!BinFast::eqf(bf::mul(a, b), h::f2_mul(a, b))) ++bad;
        if (!BinFast::eqf(bf::sqr(a), h::f2_square(a))) ++bad;
        if (i < 200 && !BinFast::zero(a) && !BinFast::eqf(bf::inv(a), h::f2_inv(a))) ++bad;
    }
    printf("{\"kind\":\"gate\",\"gate\":\"G1\",\"arm\":\"ECC2-131/clmul3\",\"cases\":20000,\"mismatches\":%d}\n", bad);
    return bad != 0;
}
static int step_oracle_prime() {
    std::vector<h::FpPoint> pts;
    u64 sd = 5;
    for (int i = 0; i < 24; ++i) {
        h::U160 k{{(uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)(splitmix(sd) & 3)}};
        pts.push_back(h::fp_scalar_mul(k, h::fp_generator()));
    }
    int bad = 0, n = 0;
    for (auto &p : pts)
        for (int j = 0; j < 16; ++j) {
            const auto &t = pts[(j * 5 + 3) % pts.size()];
            if (h::fp_eq(p.x, t.x)) continue;
            const h::FpPoint ref = h::fp_add_point(p, t);
            const PrimeFast::Pt pf_ = PrimeFast::from_header(p), tf = PrimeFast::from_header(t);
            const PrimeFast::Pt got = PrimeFast::add_ref(pf_, tf);
            if (!PrimeFast::matches_header(got, ref) || !PrimeFast::on_curve(got)) ++bad;
            const PrimeFast::Pt c = PrimeFast::canon(got);
            if (!PrimeFast::eq_pt(c, got) && !PrimeFast::eq_pt(c, PrimeFast::neg(got))) ++bad;
            ++n;
        }
    printf("{\"kind\":\"gate\",\"gate\":\"G2\",\"arm\":\"ECCp-131/mont64\",\"pairs\":%d,\"mismatches\":%d}\n", n, bad);
    return bad != 0;
}
static int step_oracle_binary() {
    std::vector<h::F2Point> pts;
    u64 sd = 7;
    for (int i = 0; i < 24; ++i) {
        h::U160 k{{(uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)(splitmix(sd) & 3)}};
        pts.push_back(h::f2_scalar_mul(k, h::f2_generator()));
    }
    int bad = 0, n = 0;
    for (auto &p : pts)
        for (int j = 0; j < 16; ++j) {
            const auto &t = pts[(j * 5 + 3) % pts.size()];
            if (h::f2_eq(p.x, t.x)) continue;
            const h::F2Point ref = h::f2_add_point(p, t);
            const BinFast::Pt got = BinFast::add_ref(BinFast::from_header(p), BinFast::from_header(t));
            if (!BinFast::matches_header(got, ref) || !BinFast::on_curve(got)) ++bad;
            ++n;
        }
    printf("{\"kind\":\"gate\",\"gate\":\"G2\",\"arm\":\"ECC2-131/clmul3\",\"pairs\":%d,\"mismatches\":%d}\n", n, bad);
    return bad != 0;
}

template <class C>
static int study(int lanes, u64 steps, int rounds, const std::vector<std::pair<int, int>> &cfgs) {
    int rc = 0;
    u64 exc_total = 0;
    Walk<C> w;
    w.setup(lanes, 0x131);
    printf("{\"kind\":\"setup\",\"arm\":\"%s\",\"starts\":%d,\"rejected_degenerate\":%d}\n", C::name, lanes, w.rejected);
    // G3, G4
    std::vector<u64> ref;
    for (auto [b, c] : cfgs) {
        auto st = w.start;
        exc_total += w.run(st, b, c, 64);
        std::vector<u64> f;
        for (auto &p : st) {
            if (!C::on_curve(p)) { printf("{\"kind\":\"gate\",\"gate\":\"G3\",\"arm\":\"%s\",\"batch\":%d,\"chains\":%d,\"on_curve\":false}\n", C::name, b, c); rc = 1; }
            f.push_back(C::fold(p));
        }
        if (ref.empty()) ref = f;
        else if (f != ref) { printf("{\"kind\":\"gate\",\"gate\":\"G4\",\"arm\":\"%s\",\"batch\":%d,\"chains\":%d,\"state_matches\":false}\n", C::name, b, c); rc = 1; }
    }
    printf("{\"kind\":\"gate\",\"gate\":\"G3G4\",\"arm\":\"%s\",\"lanes\":%d,\"configs\":%zu,\"gate_steps\":64,\"pass\":%s}\n", C::name, lanes, cfgs.size(), rc ? "false" : "true");
    // timing, interleaved over rounds
    std::vector<std::vector<double>> ns(cfgs.size());
    for (int r = 0; r < rounds; ++r)
        for (size_t k = 0; k < cfgs.size(); ++k) {
            auto st = w.start;
            const double t0 = now_s();
            exc_total += w.run(st, cfgs[k].first, cfgs[k].second, steps);
            const double dt = now_s() - t0;
            u64 tot = 0;
            for (auto &p : st) tot += C::fold(p);
            g_sink = tot;
            ns[k].push_back(dt * 1e9 / ((double)steps * lanes));
        }
    for (size_t k = 0; k < cfgs.size(); ++k) {
        auto v = ns[k];
        std::sort(v.begin(), v.end());
        const double med = v[v.size() / 2];
        printf("{\"kind\":\"step\",\"arm\":\"%s\",\"batch\":%d,\"chains\":%d,\"lanes\":%d,\"steps\":%llu,\"rounds\":%d,"
               "\"ns_per_step_median\":%.2f,\"ns_per_step_min\":%.2f,\"ns_per_step_max\":%.2f,\"steps_per_s_median\":%.0f}\n",
               C::name, cfgs[k].first, cfgs[k].second, lanes, (unsigned long long)steps, rounds, med, v.front(), v.back(), 1e9 / med);
    }
    printf("{\"kind\":\"exceptional\",\"arm\":\"%s\",\"zero_denominator_events\":%llu}\n", C::name, (unsigned long long)exc_total);
    if (exc_total) rc = 1;
    // field costs
    {
        typename C::F x = w.start[0].x, y = w.start[0].y;
        const int N = 400000;
        double t0 = now_s();
        for (int i = 0; i < N; ++i) { x = C::mul(x, y); y = C::mul(y, x); }
        const double tm = (now_s() - t0) / (2.0 * N);
        g_sink = C::fold(typename C::Pt{x, y});
        printf("{\"kind\":\"field\",\"arm\":\"%s\",\"dependent_mul_ns\":%.2f}\n", C::name, tm * 1e9);
    }
    return rc;
}

int main(int argc, char **argv) {
    int lanes = 512, rounds = 7;
    u64 steps = 2000;
    std::string which = "both";
    for (int i = 1; i < argc; ++i) {
        if (!strcmp(argv[i], "--lanes") && i + 1 < argc) lanes = atoi(argv[++i]);
        else if (!strcmp(argv[i], "--steps") && i + 1 < argc) steps = strtoull(argv[++i], 0, 10);
        else if (!strcmp(argv[i], "--rounds") && i + 1 < argc) rounds = atoi(argv[++i]);
        else if (!strcmp(argv[i], "--curve") && i + 1 < argc) which = argv[++i];
    }
    if (lanes % 512) { fprintf(stderr, "--lanes must be a multiple of 512\n"); return 2; }
    pf::init();
    const std::vector<std::pair<int, int>> cfgs = {{128, 1}, {128, 2}, {128, 4}, {128, 8}, {256, 4}, {256, 8}, {512, 8}, {512, 16}};
    int rc = 0;
    if (which == "both" || which == "p") {
        rc |= gate_prime();
        rc |= step_oracle_prime();
        rc |= study<PrimeFast>(lanes, steps, rounds, cfgs);
    }
    if (which == "both" || which == "b") {
        rc |= gate_binary();
        rc |= step_oracle_binary();
        rc |= study<BinFast>(lanes, steps, rounds, cfgs);
    }
    return rc;
}
