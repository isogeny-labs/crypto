#ifndef CERTICOM131_FIXTURES_HPP
#define CERTICOM131_FIXTURES_HPP

#include <cstring>

#include "verify.cuh"
#include "vectors131.h"

namespace certicom131 {

inline Fixtures load_fixtures() {
    static_assert(VEC_COUNT == VECTOR_COUNT, "fixture count mismatch");
    Fixtures f{};
    std::memcpy(f.vp_a, VP_A, sizeof(VP_A));
    std::memcpy(f.vp_b, VP_B, sizeof(VP_B));
    std::memcpy(f.vp_mul, VP_MUL, sizeof(VP_MUL));
    std::memcpy(f.vp_inv, VP_INV, sizeof(VP_INV));
    std::memcpy(f.vb_a, VB_A, sizeof(VB_A));
    std::memcpy(f.vb_b, VB_B, sizeof(VB_B));
    std::memcpy(f.vb_mul, VB_MUL, sizeof(VB_MUL));
    std::memcpy(f.vb_sqr, VB_SQR, sizeof(VB_SQR));
    std::memcpy(f.vb_inv, VB_INV, sizeof(VB_INV));
    std::memcpy(f.vpk, VPK, sizeof(VPK));
    std::memcpy(f.vpkg, VPKG, sizeof(VPKG));
    std::memcpy(f.vp_gq, VP_GQ, sizeof(VP_GQ));
    std::memcpy(f.vp_2g, VP_2G, sizeof(VP_2G));
    std::memcpy(f.vbk, VBK, sizeof(VBK));
    std::memcpy(f.vbkg, VBKG, sizeof(VBKG));
    std::memcpy(f.vb_gq, VB_GQ, sizeof(VB_GQ));
    std::memcpy(f.vb_2g, VB_2G, sizeof(VB_2G));
    return f;
}

} // namespace certicom131

#endif
