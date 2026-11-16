// Runs the exact per-thread functions of gpu_walk.cuh on the CPU, thread by thread, and checks them
// against the header arithmetic (gpu/certicom131/certicom131.cuh).  This validates the algorithm and the
// device data layout; it does not execute anything on a GPU.
//
// Gates (nonzero exit on failure):
//   G1  field oracle: pmul / pinv against the header's fp_mont_mul / fp_inv; the bitsliced field (W =
//       uint32_t, 32 lanes) against the header's F2m multiply, square and inverse;
//   G2  step oracle: after one iteration every lane equals +-(P + T[j]) of the header's affine add;
//   G3  every end point is on the curve (header fp_on_curve / f2_on_curve);
//   G4  end states do not depend on how lanes are split into threads and batches;
//   no exceptional lane in any run.
// Output: JSON lines; with --dump FILE the end-state words of the reference configuration are written
// for a later byte comparison with a device run.
#include <string>

#include "host_common.hpp"

using namespace hc;

static u64 fold_words(const std::vector<uint32_t> &v) {
    u64 x = 1469598103934665603ull;
    for (uint32_t w : v) x = (x ^ w) * 1099511628211ull;
    return x;
}

static int prime_gates(int steps, const char *dump) {
    int rc = 0;
    u64 sd = 0x1317;
    // G1
    int bad = 0;
    for (int i = 0; i < 20000; ++i) {
        const h::Fp a = h::fp_from_canonical(h::Fp{{(uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)(splitmix(sd) & 3)}});
        const h::Fp b = h::fp_from_canonical(h::Fp{{(uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)splitmix(sd), (uint32_t)(splitmix(sd) & 3)}});
        if (!h::fp_eq(pmul(a, b), h::fp_mont_mul(a, b))) ++bad;
        if (i < 200 && !pzero(a) && !h::fp_eq(pinv(a), h::fp_inv(a))) ++bad;
    }
    printf("{\"kind\":\"gate\",\"gate\":\"G1\",\"arm\":\"ECCp-131/u32x5\",\"products\":20000,\"inversions\":200,\"mismatches\":%d}\n", bad);
    rc |= bad != 0;
    // G2 + G3 + G4 over (threads, batch) splits of 64 lanes
    const int LANES = 64;
    PrimeSetup su = make_prime(LANES, 0x1318);
    const int cfgs[4][2] = {{64, 1}, {16, 4}, {8, 8}, {4, 16}};
    std::vector<uint32_t> ref_state;
    for (int c = 0; c < 4; ++c) {
        const int nth = cfgs[c][0], B = cfgs[c][1];
        std::vector<uint32_t> S = prime_state(su.start);
        int exc = 0;
        for (int t = 0; t < nth; ++t) exc += prime_thread(su.tab, S.data(), LANES, t, nth, B, steps);
        int off = 0;
        for (int g = 0; g < LANES; ++g) if (!h::fp_on_curve(prime_lane(S, LANES, g))) ++off;
        bool same = true;
        if (c == 0) ref_state = S; else same = (S == ref_state);
        printf("{\"kind\":\"gate\",\"gate\":\"G3G4\",\"arm\":\"ECCp-131/u32x5\",\"threads\":%d,\"batch\":%d,\"steps\":%d,\"off_curve\":%d,\"matches_reference_split\":%s,\"exceptional\":%d,\"state_fold\":\"%016llx\"}\n",
               nth, B, steps, off, same ? "true" : "false", exc, (unsigned long long)fold_words(S));
        rc |= off != 0 || !same || exc != 0;
    }
    {  // G2: one iteration, 8 threads x 8 lanes
        std::vector<uint32_t> S = prime_state(su.start);
        int exc = 0;
        for (int t = 0; t < 8; ++t) exc += prime_thread(su.tab, S.data(), LANES, t, 8, 8, 1);
        int b2 = 0;
        for (int g = 0; g < LANES; ++g) {
            const h::FpPoint P = su.start[g];
            const int j = (int)pjump(P.x);
            const h::FpPoint R = h::fp_add_point(P, su.tabpt[j]);
            const h::FpPoint got = prime_lane(S, LANES, g);
            if (!(h::fp_eq(got.x, R.x) && (h::fp_eq(got.y, R.y) || h::fp_eq(got.y, h::fp_neg(R).y)))) ++b2;
            h::FpPoint chk = got; pcanon(chk.y);
            if (!h::fp_eq(chk.y, got.y)) ++b2;
        }
        printf("{\"kind\":\"gate\",\"gate\":\"G2\",\"arm\":\"ECCp-131/u32x5\",\"lanes_checked\":%d,\"mismatches\":%d,\"exceptional\":%d}\n", LANES, b2, exc);
        rc |= b2 != 0 || exc != 0;
    }
    if (dump) {
        std::vector<uint32_t> S = prime_state(su.start);
        for (int t = 0; t < 16; ++t) prime_thread(su.tab, S.data(), LANES, t, 16, 4, steps);
        FILE *f = fopen((std::string(dump) + ".prime").c_str(), "wb");
        fwrite(S.data(), 4, S.size(), f);
        fclose(f);
    }
    return rc;
}

static int bin_gates(int steps, const char *dump) {
    int rc = 0;
    u64 sd = 0x1319;
    // G1: bitsliced field, 32 lanes per word
    {
        int bad = 0;
        for (int rep = 0; rep < 40; ++rep) {
            std::vector<h::F2m> a(32), b(32);
            El A, B, R;
            memset(&A, 0, sizeof A); memset(&B, 0, sizeof B);
            for (int l = 0; l < 32; ++l) {
                a[l] = h::F2m{{splitmix(sd), splitmix(sd), splitmix(sd) & 7ull}};
                b[l] = h::F2m{{splitmix(sd), splitmix(sd), splitmix(sd) & 7ull}};
                if (h::f2_is_zero(a[l])) a[l] = h::f2_one();
                u64 oa[3], ob[3];
                poly_to_onb(a[l], oa); poly_to_onb(b[l], ob);
                for (int i = 0; i < NW; ++i) { A.w[i] |= (W)onb_bit(oa, i) << l; B.w[i] |= (W)onb_bit(ob, i) << l; }
            }
            auto lane = [&](const El &e, int l) {
                u64 o[3] = {0, 0, 0};
                for (int i = 0; i < NW; ++i) if ((e.w[i] >> l) & 1u) o[i >> 6] |= 1ull << (i & 63);
                return onb_to_poly(o);
            };
            FB::mul(A.w, B.w, R.w);
            for (int l = 0; l < 32; ++l) if (!f2eq(lane(R, l), h::f2_mul(a[l], b[l]))) ++bad;
            if (rep < 4) {
                FB::sqr(A.w, R.w);
                for (int l = 0; l < 32; ++l) if (!f2eq(lane(R, l), h::f2_square(a[l]))) ++bad;
                FB::inv(A.w, R.w);
                for (int l = 0; l < 32; ++l) if (!f2eq(lane(R, l), h::f2_inv(a[l]))) ++bad;
            }
        }
        printf("{\"kind\":\"gate\",\"gate\":\"G1\",\"arm\":\"ECC2-131/bitsliced32\",\"products\":1280,\"squares\":128,\"inversions\":128,\"mismatches\":%d}\n", bad);
        rc |= bad != 0;
    }
    // G2 + G3 + G4: 1024 flat lanes split as (threads, slots) = (32,1), (16,2), (8,4), (4,8)
    const int LANES = 1024;
    BinSetup su = make_bin(LANES, 0x131a);
    const int cfgs[4][2] = {{32, 1}, {16, 2}, {8, 4}, {4, 8}};
    std::vector<h::F2Point> ref_pts;
    for (int c = 0; c < 4; ++c) {
        const int nth = cfgs[c][0], S = cfgs[c][1];
        std::vector<W> X, Y;
        bin_state(su.start, nth, S, X, Y);
        int exc = 0;
        for (int t = 0; t < nth; ++t) exc += bin_thread(su.tabx.data(), su.taby.data(), su.amask.data(), X.data(), Y.data(), t, nth, S, steps);
        std::vector<h::F2Point> pts(LANES);
        int off = 0;
        for (int s = 0; s < S; ++s)
            for (int t = 0; t < nth; ++t)
                for (int l = 0; l < 32; ++l) {
                    const h::F2Point p = bin_lane(X, Y, nth, s, t, l);
                    pts[((size_t)s * nth + t) * 32 + l] = p;
                    if (!h::f2_on_curve(p)) ++off;
                }
        bool same = true;
        if (c == 0) ref_pts = pts;
        else for (int q = 0; q < LANES; ++q) same = same && f2eq(pts[q].x, ref_pts[q].x) && f2eq(pts[q].y, ref_pts[q].y);
        printf("{\"kind\":\"gate\",\"gate\":\"G3G4\",\"arm\":\"ECC2-131/bitsliced32\",\"threads\":%d,\"slots\":%d,\"steps\":%d,\"off_curve\":%d,\"matches_reference_split\":%s,\"exceptional\":%d}\n",
               nth, S, steps, off, same ? "true" : "false", exc);
        rc |= off != 0 || !same || exc != 0;
    }
    {  // G2: one iteration, (8 threads, 4 slots)
        const int nth = 8, S = 4;
        std::vector<W> X, Y;
        bin_state(su.start, nth, S, X, Y);
        int exc = 0;
        for (int t = 0; t < nth; ++t) exc += bin_thread(su.tabx.data(), su.taby.data(), su.amask.data(), X.data(), Y.data(), t, nth, S, 1);
        int b2 = 0;
        for (int s = 0; s < S; ++s)
            for (int t = 0; t < nth; ++t)
                for (int l = 0; l < 32; ++l) {
                    const size_t q = ((size_t)s * nth + t) * 32 + l;
                    const h::F2Point P = su.start[q];
                    const h::F2Point R = h::f2_add_point(P, su.tabpt[jump_of(P.x)]);
                    const h::F2Point got = bin_lane(X, Y, nth, s, t, l);
                    if (!(f2eq(got.x, R.x) && (f2eq(got.y, R.y) || f2eq(got.y, h::f2_add(R.x, R.y))))) ++b2;
                    const h::F2Point cl = bclass(got);
                    if (!f2eq(cl.y, got.y)) ++b2;
                }
        printf("{\"kind\":\"gate\",\"gate\":\"G2\",\"arm\":\"ECC2-131/bitsliced32\",\"lanes_checked\":%d,\"mismatches\":%d,\"exceptional\":%d}\n", LANES, b2, exc);
        rc |= b2 != 0 || exc != 0;
    }
    if (dump) {
        std::vector<W> X, Y;
        bin_state(su.start, 8, 4, X, Y);
        for (int t = 0; t < 8; ++t) bin_thread(su.tabx.data(), su.taby.data(), su.amask.data(), X.data(), Y.data(), t, 8, 4, steps);
        FILE *f = fopen((std::string(dump) + ".bin").c_str(), "wb");
        fwrite(X.data(), 4, X.size(), f);
        fwrite(Y.data(), 4, Y.size(), f);
        fclose(f);
    }
    return rc;
}

int main(int argc, char **argv) {
    int psteps = 12, bsteps = 6;
    const char *dump = nullptr;
    std::string which = "both";
    for (int i = 1; i < argc; ++i) {
        if (!strcmp(argv[i], "--psteps") && i + 1 < argc) psteps = atoi(argv[++i]);
        else if (!strcmp(argv[i], "--bsteps") && i + 1 < argc) bsteps = atoi(argv[++i]);
        else if (!strcmp(argv[i], "--dump") && i + 1 < argc) dump = argv[++i];
        else if (!strcmp(argv[i], "--curve") && i + 1 < argc) which = argv[++i];
    }
    int rc = 0;
    if (which == "both" || which == "p") rc |= prime_gates(psteps, dump);
    if (which == "both" || which == "b") rc |= bin_gates(bsteps, dump);
    printf("{\"kind\":\"summary\",\"pass\":%s}\n", rc ? "false" : "true");
    return rc;
}
