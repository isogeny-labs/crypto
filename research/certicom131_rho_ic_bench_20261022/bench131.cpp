// Rho-step microbenchmark on the exact ECCp-131 and ECC2-131 groups, using the
// native arithmetic in gpu/certicom131/certicom131.cuh (host build).
//
// What is timed: one iteration of an r-adding walk with the negation map,
//   j = hash(x) & 15;  P <- canon(P + T[j])
// on a lane of `batch` independent walks, with ONE field inversion per batch
// (Montgomery's trick).  The jump table T[j] = a_j G + b_j Q is built outside
// the timed region.  Per-iteration scalar bookkeeping is not carried: it is
// integer work outside the field arithmetic and is reported as unpriced.
//
// Gates (the program exits nonzero if any fails):
//   * every lane end point is on the curve;
//   * the end state of every lane is identical for every batch size
//     (lanes are independent, so batching must not change any trajectory);
//   * batch 1 uses the unbatched affine formula of the header
//     (fp_add_point / f2_add_point) as an independent reference.
//
// Output: one JSON object per line on stdout.
#include <algorithm>
#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>

#include "../../gpu/certicom131/certicom131.cuh"

using namespace certicom131;
using Clock = std::chrono::steady_clock;

static double now_s() {
    return std::chrono::duration<double>(Clock::now().time_since_epoch()).count();
}

static uint64_t splitmix(uint64_t &s) {
    uint64_t z = (s += 0x9e3779b97f4a7c15ull);
    z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ull;
    z = (z ^ (z >> 27)) * 0x94d049bb133111ebull;
    return z ^ (z >> 31);
}

static volatile uint64_t g_sink;

// ---------- prime field ----------
struct PrimeCurve {
    static constexpr const char *name = "ECCp-131";
    using Pt = FpPoint;
    using F = Fp;
    static Pt gen() { return fp_generator(); }
    static Pt tgt() { return fp_target(); }
    static Pt add_ref(const Pt &a, const Pt &b) { return fp_add_point(a, b); }
    static bool on(const Pt &p) { return fp_on_curve(p); }
    static bool eq(const Pt &a, const Pt &b) { return fp_point_eq(a, b); }
    static uint32_t hash(const Pt &p) { return p.x.v[0] >> 3; }
    static Pt canon(const Pt &p) {  // negation map: smaller of y, -y
        Pt n = fp_neg(p);
        return fp_ge(p.y, n.y) ? n : p;
    }
    static F den(const Pt &p, const Pt &t) { return fp_sub(t.x, p.x); }
    static bool zero(const F &d) { return fp_is_zero(d); }
    static F mul(const F &a, const F &b) { return fp_mont_mul(a, b); }
    static F inv(const F &a) { return fp_inv(a); }
    static F one() { return fp_r1(); }
    static Pt finish(const Pt &p, const Pt &t, const F &dinv) {
        const F lambda = fp_mont_mul(fp_sub(t.y, p.y), dinv);
        const F x3 = fp_sub(fp_sub(fp_mont_mul(lambda, lambda), p.x), t.x);
        const F y3 = fp_sub(fp_mont_mul(lambda, fp_sub(p.x, x3)), p.y);
        return Pt{x3, y3, 0};
    }
    static uint64_t fold(const Pt &p) {
        uint64_t h = 0;
        for (int i = 0; i < 5; ++i) h = h * 0x100000001b3ull ^ p.x.v[i] ^ ((uint64_t)p.y.v[i] << 7);
        return h;
    }
    // per-iteration field operation counts of the batched step (excluding the
    // amortised inversion): 3 M for the trick + 1 M lambda + 1 S + 1 M y3
    static constexpr double M = 3 + 1 + 1 + 1, S = 0;
};

// ---------- binary field ----------
struct BinaryCurve {
    static constexpr const char *name = "ECC2-131";
    using Pt = F2Point;
    using F = F2m;
    static Pt gen() { return f2_generator(); }
    static Pt tgt() { return f2_target(); }
    static Pt add_ref(const Pt &a, const Pt &b) { return f2_add_point(a, b); }
    static bool on(const Pt &p) { return f2_on_curve(p); }
    static bool eq(const Pt &a, const Pt &b) { return f2_point_eq(a, b); }
    static uint32_t hash(const Pt &p) { return (uint32_t)(p.x.v[0] >> 3); }
    static Pt canon(const Pt &p) {  // -P = (x, x + y); keep the lexicographically smaller y
        const F ny = f2_add(p.x, p.y);
        if (ny.v[2] != p.y.v[2]) return ny.v[2] < p.y.v[2] ? Pt{p.x, ny, 0} : p;
        if (ny.v[1] != p.y.v[1]) return ny.v[1] < p.y.v[1] ? Pt{p.x, ny, 0} : p;
        return ny.v[0] < p.y.v[0] ? Pt{p.x, ny, 0} : p;
    }
    static F den(const Pt &p, const Pt &t) { return f2_add(p.x, t.x); }
    static bool zero(const F &d) { return f2_is_zero(d); }
    static F mul(const F &a, const F &b) { return f2_mul(a, b); }
    static F inv(const F &a) { return f2_inv(a); }
    static F one() { return f2_one(); }
    static Pt finish(const Pt &p, const Pt &t, const F &dinv) {
        const F lambda = f2_mul(f2_add(p.y, t.y), dinv);
        const F x3 = f2_add(f2_add(f2_add(f2_square(lambda), lambda), f2_add(p.x, t.x)), f2_a());
        const F y3 = f2_add(f2_add(f2_mul(lambda, f2_add(p.x, x3)), x3), p.y);
        return Pt{x3, y3, 0};
    }
    static uint64_t fold(const Pt &p) {
        uint64_t h = 0;
        for (int i = 0; i < 3; ++i) h = h * 0x100000001b3ull ^ p.x.v[i] ^ (p.y.v[i] << 7);
        return h;
    }
    static constexpr double M = 3 + 1 + 1, S = 1;
};


// ---------- binary field, pclmulqdq path (same field, same curve as BinaryCurve) ----------
#include <immintrin.h>
namespace fast2 {
static inline void clmul64(uint64_t a, uint64_t b, uint64_t &lo, uint64_t &hi) {
    __m128i r = _mm_clmulepi64_si128(_mm_cvtsi64_si128((long long)a), _mm_cvtsi64_si128((long long)b), 0);
    lo = (uint64_t)_mm_cvtsi128_si64(r);
    hi = (uint64_t)_mm_extract_epi64(r, 1);
}
static inline F2m reduce(const uint64_t p[5]) {
    uint64_t H0 = (p[2] >> 3) | (p[3] << 61), H1 = (p[3] >> 3) | (p[4] << 61), H2 = p[4] >> 3;
    uint64_t s0 = p[0], s1 = p[1], s2 = p[2] & 7ull;
    auto shl = [&](int k) {  // H << k accumulated into s0..s2
        s0 ^= H0 << k;
        s1 ^= (H1 << k) | (H0 >> (64 - k));
        s2 ^= (H2 << k) | (H1 >> (64 - k));
    };
    s0 ^= H0; s1 ^= H1; s2 ^= H2;
    shl(1); shl(2); shl(13);
    uint64_t T = s2 >> 3;
    s2 &= 7ull;
    s0 ^= T ^ (T << 1) ^ (T << 2) ^ (T << 13);
    return F2m{{s0, s1, s2}};
}
static inline F2m mul(const F2m &a, const F2m &b) {
    uint64_t p[6] = {0, 0, 0, 0, 0, 0};  // p[5] is provably 0: a.v[2] * b.v[2] < 2^5
    for (int i = 0; i < 3; ++i)
        for (int j = 0; j < 3; ++j) {
            uint64_t lo, hi;
            clmul64(a.v[i], b.v[j], lo, hi);
            p[i + j] ^= lo;
            p[i + j + 1] ^= hi;
        }
    return reduce(p);
}
static inline F2m sqr(const F2m &a) {
    uint64_t p[6] = {0, 0, 0, 0, 0, 0};  // a.v[2] < 8, so p[5] stays 0 and is not read
    for (int i = 0; i < 3; ++i) {
        uint64_t lo, hi;
        clmul64(a.v[i], a.v[i], lo, hi);
        p[2 * i] ^= lo;
        p[2 * i + 1] ^= hi;
    }
    return reduce(p);
}
static inline F2m sqrn(F2m a, int n) { while (n--) a = sqr(a); return a; }
// Itoh-Tsujii: a^-1 = (a^(2^130-1))^2, 8 M + 130 S.
static inline F2m inv(const F2m &a) {
    F2m b1 = a;
    F2m b2 = mul(sqrn(b1, 1), b1);
    F2m b4 = mul(sqrn(b2, 2), b2);
    F2m b8 = mul(sqrn(b4, 4), b4);
    F2m b16 = mul(sqrn(b8, 8), b8);
    F2m b32 = mul(sqrn(b16, 16), b16);
    F2m b64 = mul(sqrn(b32, 32), b32);
    F2m b65 = mul(sqr(b64), a);
    F2m b130 = mul(sqrn(b65, 65), b65);
    return sqr(b130);
}
}  // namespace fast2

struct BinaryClmulCurve : BinaryCurve {
    static constexpr const char *name = "ECC2-131/clmul";
    using Pt = F2Point;
    using F = F2m;
    static F mul(const F &a, const F &b) { return fast2::mul(a, b); }
    static F inv(const F &a) { return fast2::inv(a); }
    static Pt add_ref(const Pt &p, const Pt &q) {  // one inversion per add, fast field
        if (p.infinity) return q;
        if (q.infinity) return p;
        if (f2_eq(p.x, q.x)) return f2_add_point(p, q);  // doubling / inverse: header path
        const F dinv = fast2::inv(f2_add(p.x, q.x));
        return finish(p, q, dinv);
    }
    static Pt finish(const Pt &p, const Pt &t, const F &dinv) {
        const F lambda = fast2::mul(f2_add(p.y, t.y), dinv);
        const F x3 = f2_add(f2_add(f2_add(fast2::sqr(lambda), lambda), f2_add(p.x, t.x)), f2_a());
        const F y3 = f2_add(f2_add(fast2::mul(lambda, f2_add(p.x, x3)), x3), p.y);
        return Pt{x3, y3, 0};
    }
    static constexpr double M = 3 + 1 + 1, S = 1;
};

static std::vector<uint64_t> g_gate_ref_b131;

template <class C>
struct Bench {
    using Pt = typename C::Pt;
    using F = typename C::F;
    Pt table[16];
    std::vector<Pt> start;

    // Build the jump table and `lanes` starting points as random combinations
    // of G and Q using the reference formulas only.
    void setup(int lanes, uint64_t seed) {
        const Pt g = C::gen(), q = C::tgt();
        uint64_t s = seed;
        auto combo = [&](int ka, int kb) {
            Pt acc = g;  // (ka+1) G + (kb+1) Q by repeated reference additions
            Pt gg = g, qq = q;
            for (int i = 0; i < ka; ++i) acc = C::add_ref(acc, gg);
            for (int i = 0; i < kb + 1; ++i) acc = C::add_ref(acc, qq);
            return acc;
        };
        for (int j = 0; j < 16; ++j) {
            Pt t = combo(1 + (int)(splitmix(s) % 61), (int)(splitmix(s) % 61));
            table[j] = C::canon(t);
        }
        start.resize(lanes);
        for (int i = 0; i < lanes; ++i)
            start[i] = C::canon(combo(1 + (int)(splitmix(s) % 200), (int)(splitmix(s) % 200)));
    }

    // Run `steps` iterations on every lane in groups of `batch`. Returns the
    // number of exceptional (zero-denominator) events handled by the slow path.
    uint64_t run(std::vector<Pt> &st, int batch, uint64_t steps) {
        const int lanes = (int)st.size();
        std::vector<F> pre(batch), den(batch);
        std::vector<Pt> tj(batch);
        uint64_t exceptional = 0;
        for (int base = 0; base < lanes; base += batch) {
            Pt *p = &st[base];
            for (uint64_t it = 0; it < steps; ++it) {
                if (batch == 1) {  // header reference formula, one inversion per add
                    const Pt &t = table[C::hash(p[0]) & 15];
                    Pt n = C::add_ref(p[0], t);
                    p[0] = C::canon(n);
                    if (p[0].infinity) { ++exceptional; p[0] = table[0]; }
                    continue;
                }
                F acc = C::one();
                for (int i = 0; i < batch; ++i) {
                    tj[i] = table[C::hash(p[i]) & 15];
                    den[i] = C::den(p[i], tj[i]);
                    if (C::zero(den[i])) { den[i] = C::one(); ++exceptional; }
                    pre[i] = acc;
                    acc = C::mul(acc, den[i]);
                }
                F inv = C::inv(acc);
                for (int i = batch - 1; i >= 0; --i) {
                    const F di = C::mul(inv, pre[i]);
                    inv = C::mul(inv, den[i]);
                    if (C::eq(p[i], tj[i]) || C::zero(C::den(p[i], tj[i]))) {
                        p[i] = table[(C::hash(p[i]) + 1) & 15];  // exceptional lane: deterministic move
                        continue;
                    }
                    p[i] = C::canon(C::finish(p[i], tj[i], di));
                }
            }
        }
        return exceptional;
    }
};

template <class C>
static int study(int lanes, uint64_t steps, int rounds, const std::vector<int> &batches) {
    Bench<C> b;
    b.setup(lanes, 0x131);
    const char *nm = C::name;
    int rc = 0;

    // ---- field-operation microbench ----
    {
        typename C::F x = C::gen().x, y = C::gen().y;
        const int N = 200000;
        double t0 = now_s();
        for (int i = 0; i < N; ++i) { x = C::mul(x, y); y = C::mul(y, x); }
        double tm = (now_s() - t0) / (2.0 * N);
        g_sink = C::fold(typename C::Pt{x, y, 0});
        const int NI = 2000;
        t0 = now_s();
        typename C::F z = y;
        for (int i = 0; i < NI; ++i) { z = C::inv(z); z = C::mul(z, x); }
        double ti = (now_s() - t0) / NI;
        g_sink = C::fold(typename C::Pt{z, z, 0});
        printf("{\"kind\":\"field\",\"curve\":\"%s\",\"mul_ns\":%.2f,\"inv_ns\":%.1f,\"inv_over_mul\":%.1f}\n", nm,
               tm * 1e9, ti * 1e9, ti / tm);
    }

    // ---- gate: field arithmetic of this arm against the header reference (binary only) ----
    if (!strcmp(nm, "ECC2-131/clmul")) {
        uint64_t sd = 77;
        int bad = 0;
        for (int i = 0; i < 20000; ++i) {
            F2m a{{splitmix(sd), splitmix(sd), splitmix(sd) & 7ull}}, c{{splitmix(sd), splitmix(sd), splitmix(sd) & 7ull}};
            if (!f2_eq(fast2::mul(a, c), f2_mul(a, c))) ++bad;
            if (!f2_eq(fast2::sqr(a), f2_square(a))) ++bad;
            if (i < 200 && !f2_is_zero(a) && !f2_eq(fast2::inv(a), f2_inv(a))) ++bad;
        }
        printf("{\"kind\":\"gate\",\"curve\":\"%s\",\"field_vs_header\":{\"cases\":20000,\"mismatches\":%d}}\n", nm, bad);
        if (bad) rc = 1;
    }
    // ---- gates: end state identical across batch sizes (short run) ----
    std::vector<uint64_t> ref;
    for (int bs : batches) {
        std::vector<typename C::Pt> st = b.start;
        b.run(st, bs, 64);
        std::vector<uint64_t> f;
        for (auto &p : st) {
            if (!C::on(p)) { printf("{\"kind\":\"gate\",\"curve\":\"%s\",\"batch\":%d,\"on_curve\":false}\n", nm, bs); rc = 1; }
            f.push_back(C::fold(p));
        }
        if (ref.empty()) ref = f;
        else if (f != ref) { printf("{\"kind\":\"gate\",\"curve\":\"%s\",\"batch\":%d,\"state_matches_batch1\":false}\n", nm, bs); rc = 1; }
    }
    if (!strncmp(nm, "ECC2-131", 8)) {
        if (g_gate_ref_b131.empty()) g_gate_ref_b131 = ref;
        else if (g_gate_ref_b131 != ref) { printf("{\"kind\":\"gate\",\"curve\":\"%s\",\"state_matches_header_arm\":false}\n", nm); rc = 1; }
        else printf("{\"kind\":\"gate\",\"curve\":\"%s\",\"state_matches_header_arm\":true}\n", nm);
    }
    printf("{\"kind\":\"gate\",\"curve\":\"%s\",\"lanes\":%d,\"gate_steps\":64,\"batches\":%zu,\"pass\":%s}\n", nm, lanes, batches.size(), rc ? "false" : "true");

    // ---- timing: interleave configurations over rounds ----
    std::vector<std::vector<double>> ns(batches.size());
    std::vector<uint64_t> exc(batches.size(), 0);
    for (int r = 0; r < rounds; ++r) {
        for (size_t k = 0; k < batches.size(); ++k) {
            std::vector<typename C::Pt> st = b.start;
            double t0 = now_s();
            exc[k] += b.run(st, batches[k], steps);
            double dt = now_s() - t0;
            uint64_t tot = 0;
            for (auto &p : st) tot += C::fold(p);
            g_sink = tot;
            for (auto &p : st) if (!C::on(p)) rc = 1;
            ns[k].push_back(dt * 1e9 / ((double)steps * lanes));
        }
    }
    for (size_t k = 0; k < batches.size(); ++k) {
        auto v = ns[k];
        std::sort(v.begin(), v.end());
        double med = v[v.size() / 2];
        printf("{\"kind\":\"step\",\"curve\":\"%s\",\"batch\":%d,\"lanes\":%d,\"steps\":%llu,\"rounds\":%d,"
               "\"ns_per_step_median\":%.2f,\"ns_per_step_min\":%.2f,\"ns_per_step_max\":%.2f,"
               "\"steps_per_s_median\":%.0f,\"exceptional\":%llu}\n",
               nm, batches[k], lanes, (unsigned long long)steps, rounds, med, v.front(), v.back(), 1e9 / med,
               (unsigned long long)exc[k]);
    }
    return rc;
}

int main(int argc, char **argv) {
    int lanes = 128;
    uint64_t steps = 2000;
    int rounds = 7;
    std::string which = "both";
    for (int i = 1; i < argc; ++i) {
        if (!strcmp(argv[i], "--lanes") && i + 1 < argc) lanes = atoi(argv[++i]);
        else if (!strcmp(argv[i], "--steps") && i + 1 < argc) steps = strtoull(argv[++i], 0, 10);
        else if (!strcmp(argv[i], "--rounds") && i + 1 < argc) rounds = atoi(argv[++i]);
        else if (!strcmp(argv[i], "--curve") && i + 1 < argc) which = argv[++i];
    }
    const std::vector<int> batches = {1, 8, 32, 128};
    if (lanes % 128) { fprintf(stderr, "--lanes must be a multiple of 128\n"); return 2; }
    int rc = 0;
    if (which == "both" || which == "p") rc |= study<PrimeCurve>(lanes, steps, rounds, batches);
    if (which == "both" || which == "b") rc |= study<BinaryCurve>(lanes, steps, rounds, batches);
    if (which == "both" || which == "c") {
        if (which == "c") { rc |= study<BinaryCurve>(lanes, 1, 1, batches); }
        rc |= study<BinaryClmulCurve>(lanes, steps, rounds, batches);
    }
    return rc;
}
