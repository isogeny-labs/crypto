// spmv_math.cuh -- the SPMV1 residue arithmetic, shared by the CUDA worker
// (spmv.cu) and the host oracle (spmv-oracle-check.cpp).  Compiles as host
// C++ and as CUDA.
#pragma once

#include <cstdint>

// Exact a*b mod m for a, b < m < 2^63 by a 128-bit Barrett quotient.
// The previous loop spent up to 63 dependent add-and-double steps (each a
// compare-and-subtract) per product; this is four 64x64 multiply-high and five
// multiply-low operations plus one conditional subtraction.  The result is the
// same residue: the host-side oracle in spmv-oracle-check.cpp compares the two
// on edge-case and random moduli, and the kernel's per-task body against the
// CPU worker's loop.  Untested on a GPU.
#if defined(__CUDACC__)
#define SPMV_HD __host__ __device__ __forceinline__
#else
#define SPMV_HD inline
#endif

SPMV_HD std::uint64_t spmv_mulhi(std::uint64_t a, std::uint64_t b) {
#if defined(__CUDA_ARCH__)
    return __umul64hi(a, b);
#else
    return (std::uint64_t)(((unsigned __int128)a * b) >> 64);
#endif
}

struct SpmvModulus {
    std::uint64_t m;
    std::uint64_t mu_hi, mu_lo; // floor((2^128 - 1) / m)
};

inline SpmvModulus spmv_modulus(std::uint64_t m) {
    const unsigned __int128 mu = ~(unsigned __int128)0 / m;
    return {m, (std::uint64_t)(mu >> 64), (std::uint64_t)mu};
}

SPMV_HD std::uint64_t spmv_add_mod(std::uint64_t a, std::uint64_t b,
                                   std::uint64_t modulus) {
    const std::uint64_t sum = a + b; // a,b < 2^63, so this cannot overflow.
    return sum >= modulus ? sum - modulus : sum;
}

// x = a*b < m^2 < 2^126, so q = floor(x / m) < m < 2^63.  With
// mu = floor((2^128-1)/m) >= 2^128/m - 1, q_est = floor(x*mu / 2^128)
// satisfies q - 1 <= q_est <= q, hence 0 <= x - q_est*m < 2m and one
// conditional subtraction finishes.  x - q_est*m < 2^64, so it is exact
// in wrapping 64-bit arithmetic.
SPMV_HD std::uint64_t spmv_mul_mod(std::uint64_t a, std::uint64_t b,
                                   const SpmvModulus &mod) {
    const std::uint64_t xl = a * b, xh = spmv_mulhi(a, b);
    const std::uint64_t t = spmv_mulhi(xl, mod.mu_lo);
    const std::uint64_t m1l = xh * mod.mu_lo, m1h = spmv_mulhi(xh, mod.mu_lo);
    const std::uint64_t m2l = xl * mod.mu_hi, m2h = spmv_mulhi(xl, mod.mu_hi);
    std::uint64_t s = m1l + m2l;
    std::uint64_t carry = s < m1l;
    s += t;
    carry += s < t;
    const std::uint64_t q = xh * mod.mu_hi + m1h + m2h + carry;
    std::uint64_t r = xl - q * mod.m;
    return r >= mod.m ? r - mod.m : r;
}

// One output residue: row `row`, lane `lane`.  Coefficients and x are
// already reduced below the modulus on the host (the residue of a*b is
// unchanged by reducing a and b first).
SPMV_HD std::uint64_t spmv_task(const std::uint64_t *__restrict__ row_ptr,
                                const std::uint64_t *__restrict__ column_index,
                                const std::uint64_t *__restrict__ coefficient,
                                const std::uint64_t *__restrict__ x,
                                std::uint64_t row, std::uint64_t lane,
                                std::uint64_t lanes, const SpmvModulus &mod) {
    std::uint64_t sum = 0;
    const std::uint64_t end = row_ptr[row + 1];
    for (std::uint64_t at = row_ptr[row]; at < end; ++at) {
        const auto column = column_index[at];
        sum = spmv_add_mod(sum, spmv_mul_mod(coefficient[at], x[column * lanes + lane], mod),
                           mod.m);
    }
    return sum;
}
