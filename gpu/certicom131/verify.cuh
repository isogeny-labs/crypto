#ifndef CERTICOM131_VERIFY_CUH
#define CERTICOM131_VERIFY_CUH

#include <stdint.h>

#include "certicom131.cuh"

#if defined(__CUDACC__)
#define C131_VERIFY_HD __host__ __device__ __forceinline__
#else
#define C131_VERIFY_HD inline
#endif

namespace certicom131 {

static constexpr int VECTOR_COUNT = 256;
static constexpr int SCALAR_VECTOR_COUNT = 16;

struct Fixtures {
    uint32_t vp_a[VECTOR_COUNT][5];
    uint32_t vp_b[VECTOR_COUNT][5];
    uint32_t vp_mul[VECTOR_COUNT][5];
    uint32_t vp_inv[VECTOR_COUNT][5];
    uint64_t vb_a[VECTOR_COUNT][3];
    uint64_t vb_b[VECTOR_COUNT][3];
    uint64_t vb_mul[VECTOR_COUNT][3];
    uint64_t vb_sqr[VECTOR_COUNT][3];
    uint64_t vb_inv[VECTOR_COUNT][3];
    uint32_t vpk[SCALAR_VECTOR_COUNT][5];
    uint32_t vpkg[SCALAR_VECTOR_COUNT][2][5];
    uint32_t vp_gq[2][5];
    uint32_t vp_2g[2][5];
    uint64_t vbk[SCALAR_VECTOR_COUNT][3];
    uint64_t vbkg[SCALAR_VECTOR_COUNT][2][3];
    uint64_t vb_gq[2][3];
    uint64_t vb_2g[2][3];
};

C131_VERIFY_HD Fp fp_from_words(const uint32_t words[5]) {
    return Fp{{words[0], words[1], words[2], words[3], words[4]}};
}

C131_VERIFY_HD U160 u160_from_words(const uint32_t words[5]) {
    return U160{{words[0], words[1], words[2], words[3], words[4]}};
}

C131_VERIFY_HD F2m f2_from_words(const uint64_t words[3]) {
    return F2m{{words[0], words[1], words[2]}};
}

C131_VERIFY_HD U160 u160_from_u64_words(const uint64_t words[3]) {
    return U160{{
        static_cast<uint32_t>(words[0]), static_cast<uint32_t>(words[0] >> 32),
        static_cast<uint32_t>(words[1]), static_cast<uint32_t>(words[1] >> 32),
        static_cast<uint32_t>(words[2])
    }};
}

C131_VERIFY_HD bool fp_point_matches(const FpPoint &p, const uint32_t expected[2][5]) {
    if (p.infinity) return false;
    return fp_eq(fp_to_canonical(p.x), fp_from_words(expected[0]))
        && fp_eq(fp_to_canonical(p.y), fp_from_words(expected[1]));
}

C131_VERIFY_HD bool f2_point_matches(const F2Point &p, const uint64_t expected[2][3]) {
    if (p.infinity) return false;
    return f2_eq(p.x, f2_from_words(expected[0])) && f2_eq(p.y, f2_from_words(expected[1]));
}

// Returns zero on success.  Nonzero codes identify the failed contract and
// are stable so host and device receipts can be compared directly.
C131_VERIFY_HD int verify_all(const Fixtures &f) {
    for (int i = 0; i < VECTOR_COUNT; ++i) {
        const Fp a = fp_from_canonical(fp_from_words(f.vp_a[i]));
        const Fp b = fp_from_canonical(fp_from_words(f.vp_b[i]));
        if (!fp_eq(fp_to_canonical(fp_mont_mul(a, b)), fp_from_words(f.vp_mul[i]))) return 1000 + i;
        if (!fp_eq(fp_to_canonical(fp_inv(a)), fp_from_words(f.vp_inv[i]))) return 2000 + i;

        const F2m ba = f2_from_words(f.vb_a[i]);
        const F2m bb = f2_from_words(f.vb_b[i]);
        if (!f2_eq(f2_mul(ba, bb), f2_from_words(f.vb_mul[i]))) return 3000 + i;
        if (!f2_eq(f2_square(ba), f2_from_words(f.vb_sqr[i]))) return 4000 + i;
        if (!f2_eq(f2_inv(ba), f2_from_words(f.vb_inv[i]))) return 5000 + i;
    }

    const FpPoint pg = fp_generator();
    const FpPoint pq = fp_target();
    if (!fp_on_curve(pg)) return 6001;
    if (!fp_on_curve(pq)) return 6002;
    if (!fp_point_matches(fp_add_point(pg, pq), f.vp_gq)) return 6003;
    if (!fp_point_matches(fp_double(pg), f.vp_2g)) return 6004;
    if (!fp_point_eq(fp_add_point(pg, fp_infinity()), pg)) return 6005;
    if (!fp_add_point(pg, fp_neg(pg)).infinity) return 6006;
    for (int i = 0; i < SCALAR_VECTOR_COUNT; ++i) {
        if (!fp_point_matches(fp_scalar_mul(u160_from_words(f.vpk[i]), pg), f.vpkg[i])) return 6100 + i;
    }
    if (!fp_scalar_mul(fp_order(), pg).infinity) return 6201;
    if (!fp_scalar_mul(fp_order(), pq).infinity) return 6202;

    const F2Point bg = f2_generator();
    const F2Point bq = f2_target();
    if (!f2_on_curve(bg)) return 7001;
    if (!f2_on_curve(bq)) return 7002;
    if (!f2_point_matches(f2_add_point(bg, bq), f.vb_gq)) return 7003;
    if (!f2_point_matches(f2_double(bg), f.vb_2g)) return 7004;
    if (!f2_point_eq(f2_add_point(bg, f2_infinity()), bg)) return 7005;
    if (!f2_add_point(bg, f2_neg(bg)).infinity) return 7006;
    for (int i = 0; i < SCALAR_VECTOR_COUNT; ++i) {
        if (!f2_point_matches(f2_scalar_mul(u160_from_u64_words(f.vbk[i]), bg), f.vbkg[i])) return 7100 + i;
    }
    if (!f2_scalar_mul(f2_order(), bg).infinity) return 7201;
    if (!f2_scalar_mul(f2_order(), bq).infinity) return 7202;
    return 0;
}

} // namespace certicom131

#undef C131_VERIFY_HD

#endif
