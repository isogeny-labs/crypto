// Bitsliced ECC2-131 rho walk on the ECC2K-130 client's field code.
//
// ECC2-131 and ECC2K-130 live in the same field GF(2^131) = GF(2)[z]/(z^131 + z^13 + z^2 + z + 1).
// The ECC2K-130 client computes in the type-II optimal normal basis, bitsliced over 512 lanes
// (ecc2k130/include/fieldbs.h, generated/eccF131.h).  This program moves the ECC2-131 curve
// y^2 + xy = x^3 + a x^2 + b into those normal-basis coordinates once, through the generated
// change-of-basis table, and runs the negation-map r-adding walk on the same bitsliced
// multiplier, with Montgomery's trick across slots (one inversion per `slots` slots).
//
//   j = x_0 x_1 x_2 x_3 (four coordinate words, class-invariant),  P <- canon(P + T[j])
//   canon: of (x, y) and (x, x + y) keep the one with y_k = 0 at k = first set coordinate of x
//          (flipping y -> x + y flips y_k when x_k = 1, so this is a function of the class).
//
// Gates (nonzero exit on failure):
//   G1 field oracle: 10240 random products, 1024 random squares and 1024 inversions in the
//      header's polynomial basis (gpu/certicom131/certicom131.cuh) against the bitsliced
//      arithmetic through the basis change, 0 mismatches;
//   G2 step oracle: after one iteration every lane equals +-(P + T[j]) of the header's
//      affine add on the same inputs;
//   G3 end points on the curve (header f2_on_curve) after the run;
//   G4 trajectories do not depend on the slot count (8 vs 16 vs 32 slots on the same lanes);
//   no exceptional (zero denominator) event in any run, else the run is reported and fails.
//
// Output: JSON lines.
#include <algorithm>
#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>

#include "../../ecc2k130/include/curveparams.h"
#include "../../gpu/certicom131/certicom131.cuh"

namespace h = certicom131;
using u64 = unsigned long long;
using W = Bits512;
using FB = FieldBs<CfgF131, W>;
static const int M = 131;
static const int LANES = 512;
using Clock = std::chrono::steady_clock;
static double now_s() { return std::chrono::duration<double>(Clock::now().time_since_epoch()).count(); }
static u64 splitmix(u64 &s) {
    u64 z = (s += 0x9e3779b97f4a7c15ull);
    z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ull;
    z = (z ^ (z >> 27)) * 0x94d049bb133111ebull;
    return z ^ (z >> 31);
}
static volatile u64 g_sink;

// ---------- basis change (scalar) ----------
static void poly_to_onb(const u64 pb[3], u64 out[3]) {
    out[0] = out[1] = out[2] = 0;
    for (int i = 0; i < M; ++i)
        if ((pb[i >> 6] >> (i & 63)) & 1)
            for (int l = 0; l < 3; ++l) out[l] ^= eccF131::Z_TO_ONB[i][l];
}
static void onb_to_poly(const u64 on[3], u64 out[3]) {
    out[0] = out[1] = out[2] = 0;
    for (int i = 0; i < M; ++i)
        if ((on[i >> 6] >> (i & 63)) & 1)
            for (int l = 0; l < 3; ++l) out[l] ^= eccF131::GAMMA_TO_PB[i][l];
}
static h::F2m onb_to_f2m(const u64 on[3]) {
    u64 p[3];
    onb_to_poly(on, p);
    return h::F2m{{p[0], p[1], p[2]}};
}
static void f2m_to_onb(const h::F2m &a, u64 out[3]) {
    const u64 pb[3] = {a.v[0], a.v[1], a.v[2]};
    poly_to_onb(pb, out);
}

// ---------- bitsliced element helpers ----------
struct El { W w[M]; };
static void set_lane(El &e, int lane, const h::F2m &poly) {
    u64 on[3];
    f2m_to_onb(poly, on);
    FB::setLane(e.w, lane, on);
}
static h::F2m get_lane(const El &e, int lane) {
    u64 on[3];
    FB::getLane(e.w, lane, on);
    return onb_to_f2m(on);
}
static void broadcast_poly(El &e, const h::F2m &poly) {
    u64 on[3];
    f2m_to_onb(poly, on);
    FB::broadcast(on, e.w);
}
static bool f2eq(const h::F2m &a, const h::F2m &b) { return a.v[0] == b.v[0] && a.v[1] == b.v[1] && a.v[2] == b.v[2]; }

// ---------- walk ----------
struct Walk {
    int S;                       // slots per inversion batch
    std::vector<El> X, Y;        // lane state, S slots
    El TX[16], TY[16];           // jump table (broadcast), kept for reference
    u64 tx_on[16][3], ty_on[16][3];
    W a_mask[M];                 // curve coefficient a in normal-basis coordinates, as word masks
    std::vector<El> Tx, Ty, den, pre;
    El acc, inv, tmp, l, x3, y3;
    u64 exceptional = 0;

    void init_slots(int slots) {
        S = slots;
        Tx.resize(S); Ty.resize(S); den.resize(S); pre.resize(S);
    }
    void load_table(const h::F2Point *t) {
        for (int j = 0; j < 16; ++j) {
            f2m_to_onb(t[j].x, tx_on[j]);
            f2m_to_onb(t[j].y, ty_on[j]);
        }
        u64 a_on[3];
        f2m_to_onb(h::f2_a(), a_on);
        for (int i = 0; i < M; ++i) a_mask[i] = ((a_on[i >> 6] >> (i & 63)) & 1) ? W(1) : W(0);
    }
    // select table entry by the lane's x_0..x_3 through 16 minterm masks
    void mux(const El &x, El &tx, El &ty) {
        W i0 = x.w[0], i1 = x.w[1], i2 = x.w[2], i3 = x.w[3];
        W m[16];
        const W a0 = ~i0, a1 = ~i1, a2 = ~i2, a3 = ~i3;
        W m00 = a0 & a1, m01 = i0 & a1, m10 = a0 & i1, m11 = i0 & i1;
        W n00 = a2 & a3, n01 = i2 & a3, n10 = a2 & i3, n11 = i2 & i3;
        const W lo[4] = {m00, m01, m10, m11}, hi[4] = {n00, n01, n10, n11};
        for (int q = 0; q < 4; ++q)
            for (int p = 0; p < 4; ++p) m[q * 4 + p] = lo[p] & hi[q];
        for (int i = 0; i < M; ++i) {
            W ax(0), ay(0);
            for (int j = 0; j < 16; ++j) {
                if ((tx_on[j][i >> 6] >> (i & 63)) & 1) ax |= m[j];
                if ((ty_on[j][i >> 6] >> (i & 63)) & 1) ay |= m[j];
            }
            tx.w[i] = ax;
            ty.w[i] = ay;
        }
    }
    void canon(El &x, El &y) {
        W found(0), sel(0);
        for (int k = 0; k < M; ++k) {
            const W first = x.w[k] & ~found;
            sel = sel ^ (first & y.w[k]);
            found = found | x.w[k];
        }
        for (int k = 0; k < M; ++k) y.w[k] = y.w[k] ^ (sel & x.w[k]);
    }
    void iterate() {
        // pass 1: table lookup, denominators, running product
        FB::setOne(acc.w);
        for (int s = 0; s < S; ++s) {
            mux(X[s], Tx[s], Ty[s]);
            W z(0);
            for (int i = 0; i < M; ++i) {
                den[s].w[i] = X[s].w[i] ^ Tx[s].w[i];
                z = z | den[s].w[i];
            }
            const W zl = ~z;
            if (zl != W(0)) {
                ++exceptional;
                for (int i = 0; i < M; ++i) den[s].w[i] = den[s].w[i] | zl;
            }
            pre[s] = acc;
            FB::mul(acc.w, den[s].w, tmp.w);
            acc = tmp;
        }
        FB::inv(acc.w, inv.w);
        // pass 2: back-substitution and the affine additions (FB::mul copies its inputs first,
        // so the output may alias an input)
        El num;
        for (int s = S - 1; s >= 0; --s) {
            FB::mul(inv.w, pre[s].w, tmp.w);          // tmp = 1 / den[s]
            FB::mul(inv.w, den[s].w, inv.w);          // inv <- inv * den[s]
            for (int i = 0; i < M; ++i) num.w[i] = Y[s].w[i] ^ Ty[s].w[i];
            FB::mul(num.w, tmp.w, l.w);               // lambda
            FB::sqr(l.w, x3.w);                       // free permutation
            for (int i = 0; i < M; ++i) x3.w[i] = x3.w[i] ^ l.w[i] ^ X[s].w[i] ^ Tx[s].w[i] ^ a_mask[i];
            for (int i = 0; i < M; ++i) num.w[i] = X[s].w[i] ^ x3.w[i];
            FB::mul(l.w, num.w, y3.w);
            for (int i = 0; i < M; ++i) y3.w[i] = y3.w[i] ^ x3.w[i] ^ Y[s].w[i];
            canon(x3, y3);
            X[s] = x3;
            Y[s] = y3;
        }
    }
};

// ---------- setup ----------
struct Points {
    std::vector<h::F2Point> base;           // header scalar multiples of G
    h::F2Point table[16];
    std::vector<h::F2Point> lane_start;     // canonical representatives, polynomial coordinates
};

// fast polynomial-basis affine add (clmul) used only to build start points; checked through G2
static h::F2m pmul(const h::F2m &a, const h::F2m &b) { return h::f2_mul(a, b); }

static h::F2Point padd(const h::F2Point &p, const h::F2Point &q) { return h::f2_add_point(p, q); }
static h::F2Point pcanon(const h::F2Point &p) {  // same rule as Walk::canon, in polynomial coordinates
    u64 on[3], oy[3];
    f2m_to_onb(p.x, on);
    f2m_to_onb(p.y, oy);
    int k = -1;
    for (int i = 0; i < M; ++i) if ((on[i >> 6] >> (i & 63)) & 1) { k = i; break; }
    if (k >= 0 && ((oy[k >> 6] >> (k & 63)) & 1)) {
        h::F2Point r = p;
        r.y = h::f2_add(p.x, p.y);
        return r;
    }
    return p;
}

static void set_state(Walk &w, const std::vector<h::F2Point> &st) {
    w.X.assign(w.S, El());
    w.Y.assign(w.S, El());
    for (int s = 0; s < w.S; ++s)
        for (int l = 0; l < LANES; ++l) {
            set_lane(w.X[s], l, st[s * LANES + l].x);
            set_lane(w.Y[s], l, st[s * LANES + l].y);
        }
}

int main(int argc, char **argv) {
    int rounds = 5;
    u64 steps = 40;
    bool timing_only = false;
    int tslots = 16;
    for (int i = 1; i < argc; ++i) {
        if (!strcmp(argv[i], "--timing-only")) timing_only = true;
        else if (!strcmp(argv[i], "--slots") && i + 1 < argc) tslots = atoi(argv[++i]);
        if (!strcmp(argv[i], "--steps") && i + 1 < argc) steps = strtoull(argv[++i], 0, 10);
        else if (!strcmp(argv[i], "--rounds") && i + 1 < argc) rounds = atoi(argv[++i]);
    }
    int rc = 0;
    u64 sd = 0x131;

    // ---- G1: field oracle ----
    if (!timing_only) {
        int bad = 0;
        El A, B, R;
        for (int rep = 0; rep < 20; ++rep) {
            std::vector<h::F2m> a(LANES), b(LANES);
            for (int l = 0; l < LANES; ++l) {
                a[l] = h::F2m{{splitmix(sd), splitmix(sd), splitmix(sd) & 7ull}};
                b[l] = h::F2m{{splitmix(sd), splitmix(sd), splitmix(sd) & 7ull}};
                set_lane(A, l, a[l]);
                set_lane(B, l, b[l]);
            }
            FB::mul(A.w, B.w, R.w);
            for (int l = 0; l < LANES; ++l) if (!f2eq(get_lane(R, l), h::f2_mul(a[l], b[l]))) ++bad;
            if (rep < 2) {
                FB::sqr(A.w, R.w);
                for (int l = 0; l < LANES; ++l) if (!f2eq(get_lane(R, l), h::f2_square(a[l]))) ++bad;
                for (int l = 0; l < LANES; ++l) if (h::f2_is_zero(a[l])) set_lane(A, l, h::f2_one());
                FB::inv(A.w, R.w);
                for (int l = 0; l < LANES; ++l) {
                    const h::F2m x = h::f2_is_zero(a[l]) ? h::f2_one() : a[l];
                    if (!f2eq(get_lane(R, l), h::f2_inv(x))) ++bad;
                }
            }
        }
        printf("{\"kind\":\"gate\",\"gate\":\"G1\",\"arm\":\"ECC2-131/bitsliced512\",\"products\":10240,\"squares\":1024,\"inversions\":1024,\"mismatches\":%d}\n", bad);
        rc |= bad != 0;
    }

    // ---- points ----
    Points pts;
    for (int i = 0; i < 48; ++i) {
        h::U160 k{{(uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)(splitmix(sd) & 3)}};
        pts.base.push_back(h::f2_scalar_mul(k, i % 2 ? h::f2_target() : h::f2_generator()));
    }
    for (int j = 0; j < 16; ++j) {
        const auto &p = pts.base[splitmix(sd) % 48];
        const auto &q = pts.base[splitmix(sd) % 48];
        h::F2Point t = h::f2_add_point(p, q);
        if (t.infinity) { --j; continue; }
        pts.table[j] = pcanon(t);
    }
    const int MAXS = 32;
    pts.lane_start.resize((size_t)MAXS * LANES);
    auto hits_table = [&](const h::F2Point &p) {
        for (int j = 0; j < 16; ++j) if (f2eq(p.x, pts.table[j].x)) return true;
        return false;
    };
    int rejected = 0;
    for (auto &p : pts.lane_start) {
        for (;;) {
            const auto &p0 = pts.base[splitmix(sd) % 48];
            const auto &p1 = pts.base[splitmix(sd) % 48];
            const auto &p2 = pts.base[splitmix(sd) % 48];
            h::F2Point q = h::f2_add_point(p0, p1);
            if (q.infinity) { ++rejected; continue; }
            q = h::f2_add_point(q, p2);
            if (q.infinity) { ++rejected; continue; }
            q = pcanon(q);
            if (hits_table(q)) { ++rejected; continue; }
            p = q;
            break;
        }
    }
    printf("{\"kind\":\"setup\",\"arm\":\"ECC2-131/bitsliced512\",\"starts\":%zu,\"rejected_degenerate\":%d}\n", pts.lane_start.size(), rejected);

    // ---- G2: one iteration against the header's affine add ----
    if (!timing_only) {
        Walk w;
        w.init_slots(8);
        w.load_table(pts.table);
        std::vector<h::F2Point> st(pts.lane_start.begin(), pts.lane_start.begin() + 8 * LANES);
        set_state(w, st);
        // expected jump index per lane from the start state, via the same coordinates
        std::vector<h::F2Point> before = st;
        w.iterate();
        int bad = 0, checked = 0;
        for (int s = 0; s < 8; ++s)
            for (int l = 0; l < LANES; ++l) {
                u64 on[3];
                f2m_to_onb(before[s * LANES + l].x, on);
                const int j = (int)((on[0] & 1) | ((on[0] >> 1 & 1) << 1) | ((on[0] >> 2 & 1) << 2) | ((on[0] >> 3 & 1) << 3));
                const h::F2Point ref = h::f2_add_point(before[s * LANES + l], pts.table[j]);
                const h::F2Point got{get_lane(w.X[s], l), get_lane(w.Y[s], l), 0};
                const bool same = f2eq(got.x, ref.x) && (f2eq(got.y, ref.y) || f2eq(got.y, h::f2_add(ref.x, ref.y)));
                if (!same) ++bad;
                ++checked;
            }
        printf("{\"kind\":\"gate\",\"gate\":\"G2\",\"arm\":\"ECC2-131/bitsliced512\",\"lanes_checked\":%d,\"mismatches\":%d,\"exceptional\":%llu}\n", checked, bad, w.exceptional);
        rc |= (bad != 0) || (w.exceptional != 0);
    }

    // ---- G4 + G3: slot-count independence and on-curve ----
    std::vector<std::vector<std::pair<h::F2m, h::F2m>>> endpts;
    for (int slots : {8, 16, 32}) {
        if (timing_only) break;
        Walk w;
        w.init_slots(slots);
        w.load_table(pts.table);
        std::vector<h::F2Point> st(pts.lane_start.begin(), pts.lane_start.begin() + (size_t)slots * LANES);
        set_state(w, st);
        for (int i = 0; i < 24; ++i) w.iterate();
        std::vector<std::pair<h::F2m, h::F2m>> v;
        int onbad = 0;
        for (int s = 0; s < 8; ++s)
            for (int l = 0; l < LANES; l += 7) {
                const h::F2m x = get_lane(w.X[s], l), y = get_lane(w.Y[s], l);
                v.push_back({x, y});
                if (!h::f2_on_curve(h::F2Point{x, y, 0})) ++onbad;
            }
        endpts.push_back(v);
        bool same = true;
        for (size_t i = 0; i < v.size(); ++i) same = same && f2eq(v[i].first, endpts[0][i].first) && f2eq(v[i].second, endpts[0][i].second);
        printf("{\"kind\":\"gate\",\"gate\":\"G3G4\",\"arm\":\"ECC2-131/bitsliced512\",\"slots\":%d,\"iterations\":24,\"sampled_lanes\":%zu,\"off_curve\":%d,\"matches_8_slot_run\":%s,\"exceptional\":%llu}\n",
               slots, v.size(), onbad, same ? "true" : "false", w.exceptional);
        rc |= onbad != 0 || !same || w.exceptional != 0;
    }

    // ---- phase profile (per slot, one pinned core; informational) ----
    if (!timing_only) {
        Walk w;
        w.init_slots(16);
        w.load_table(pts.table);
        std::vector<h::F2Point> st(pts.lane_start.begin(), pts.lane_start.begin() + (size_t)16 * LANES);
        set_state(w, st);
        El a = w.X[0], b = w.Y[0], r;
        const int N = 400;
        double t0 = now_s();
        for (int i = 0; i < N; ++i) { FB::mul(a.w, b.w, r.w); a = r; }
        const double t_mul = (now_s() - t0) / N;
        g_sink = (u64)r.w[0].limb(0);
        t0 = now_s();
        El tx, ty;
        for (int i = 0; i < N; ++i) { w.mux(w.X[i % 16], tx, ty); }
        const double t_mux = (now_s() - t0) / N;
        g_sink = (u64)tx.w[0].limb(0);
        t0 = now_s();
        El x = w.X[0], y = w.Y[0];
        for (int i = 0; i < N; ++i) w.canon(x, y);
        const double t_canon = (now_s() - t0) / N;
        g_sink = (u64)y.w[0].limb(0);
        t0 = now_s();
        for (int i = 0; i < 40; ++i) { FB::inv(a.w, r.w); a = r; }
        const double t_inv = (now_s() - t0) / 40;
        g_sink = (u64)r.w[0].limb(0);
        printf("{\"kind\":\"profile\",\"arm\":\"ECC2-131/bitsliced512\",\"slot_mul_us\":%.2f,\"slot_mux_us\":%.2f,\"slot_canon_us\":%.2f,\"slot_inv_us\":%.2f,\"mul_ns_per_lane\":%.2f}\n",
               t_mul * 1e6, t_mux * 1e6, t_canon * 1e6, t_inv * 1e6, t_mul * 1e9 / LANES);
    }

    // ---- timing: interleaved over rounds ----
    int cfg[3] = {8, 16, 32};
    if (timing_only) { cfg[0] = cfg[1] = cfg[2] = tslots; }
    std::vector<std::vector<double>> ns(3);
    for (int r = 0; r < rounds; ++r)
        for (int k = 0; k < 3; ++k) {
            Walk w;
            w.init_slots(cfg[k]);
            w.load_table(pts.table);
            std::vector<h::F2Point> st(pts.lane_start.begin(), pts.lane_start.begin() + (size_t)cfg[k] * LANES);
            set_state(w, st);
            const double t0 = now_s();
            for (u64 i = 0; i < steps; ++i) w.iterate();
            const double dt = now_s() - t0;
            g_sink = (u64)w.X[0].w[0].limb(0);
            ns[k].push_back(dt * 1e9 / ((double)steps * cfg[k] * LANES));
            int off = 0;  // end-of-run check on 32 sampled lanes
            for (int q = 0; q < 32; ++q) {
                const int s = (q * 7) % cfg[k], l = (q * 37) % LANES;
                if (!h::f2_on_curve(h::F2Point{get_lane(w.X[s], l), get_lane(w.Y[s], l), 0})) ++off;
            }
            if (off) { rc = 1; printf("{\"kind\":\"gate\",\"gate\":\"G3\",\"arm\":\"ECC2-131/bitsliced512\",\"timing_run_off_curve\":%d}\n", off); }
            if (w.exceptional) { rc = 1; printf("{\"kind\":\"exceptional\",\"arm\":\"ECC2-131/bitsliced512\",\"slots\":%d,\"events\":%llu}\n", cfg[k], w.exceptional); }
        }
    for (int k = 0; k < 3; ++k) {
        auto v = ns[k];
        std::sort(v.begin(), v.end());
        const double med = v[v.size() / 2];
        printf("{\"kind\":\"step\",\"arm\":\"ECC2-131/bitsliced512\",\"slots\":%d,\"lanes_per_slot\":%d,\"iterations\":%llu,\"rounds\":%d,\"ns_per_step_median\":%.2f,\"ns_per_step_min\":%.2f,\"ns_per_step_max\":%.2f,\"steps_per_s_median\":%.0f}\n",
               cfg[k], LANES, (unsigned long long)steps, rounds, med, v.front(), v.back(), 1e9 / med);
    }
    return rc;
}
