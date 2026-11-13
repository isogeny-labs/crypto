// gpu/fes/fes.cuh -- Gray-code fast exhaustive search (FES) for quadratic
// Boolean systems, written to compile BOTH as host C++ (g++/clang++) and as
// CUDA (nvcc), so the algorithm is verified on a GPU-less machine and only the
// launch is device-specific.  This is the parallelizable core of a binary
// index-calculus decomposition: the 2^n cube of assignments splits into
// independent sub-cubes, one per thread.
//
// System (packed, matching fref.py):
//   value(x) = CONST ^ (+)_{i: x_i=1} LIN[i] ^ (+)_{i>=j: x_i=x_j=1} QUAD(i,j)
// where each coefficient is a u64 whose bit e is that term's coefficient in
// equation e, and QUAD is stored as a lower-triangular flat array
//   QUAD_TRI[i*(i+1)/2 + j] = coefficient of x_i x_j   (i >= j).
// A point x is a solution iff value(x) == 0.
//
// The walk maintains, at the current point, the first derivative df[i] of the
// whole system along each variable i (df[i] = f(x ^ e_i) ^ f(x)).  Flipping
// variable i costs one XOR into the value plus n XORs to update the other
// derivatives by the (constant) second derivatives QUAD(i,j).  That is O(n)
// per point; the classic libfes O(1)-amortized refinement layers a second
// Gray code over the df updates and is a drop-in optimization of this loop.
#pragma once

#include <stdint.h>

#if defined(__CUDACC__)
#define FES_HD __host__ __device__
#define FES_HDM __host__ __device__
#define FES_FI __host__ __device__ __forceinline__
#else
#define FES_HD static inline
#define FES_HDM inline
#define FES_FI inline
#endif

// Lower-triangular index for the unordered pair {i, j}.
FES_HD int fes_tri_index(int i, int j) {
  int a = i > j ? i : j;
  int b = i > j ? j : i;
  return a * (a + 1) / 2 + b;
}

FES_HD uint64_t fes_quad(const uint64_t *quad_tri, int i, int j) {
  return quad_tri[fes_tri_index(i, j)];
}

// Direct evaluation, used to verify the walk and to initialize a sub-cube.
FES_HD uint64_t fes_eval(uint64_t cst, const uint64_t *lin,
                         const uint64_t *quad_tri, int n, uint64_t x) {
  uint64_t v = cst;
  for (int i = 0; i < n; ++i) {
    if ((x >> i) & 1ULL) {
      v ^= lin[i];
      for (int j = 0; j <= i; ++j) {
        if ((x >> j) & 1ULL) {
          v ^= quad_tri[i * (i + 1) / 2 + j];
        }
      }
    }
  }
  return v;
}

// Reflected binary Gray code of s.
FES_HD uint64_t fes_gray(uint64_t s) { return s ^ (s >> 1); }

// Trailing-zero count for a nonzero 64-bit word.  `__CUDA_ARCH__` is defined
// only in the *device* pass, so the host pass of this __host__ __device__
// function (and pure host builds) uses the compiler builtin; the device intrinsic
// __ffsll is a device-only function and must not be referenced from host code.
FES_HD int fes_ctz(uint64_t s) {
#if defined(__CUDA_ARCH__)
  return __ffsll((unsigned long long)s) - 1;
#elif defined(__GNUC__)
  return __builtin_ctzll((unsigned long long)s);
#else
  int c = 0;
  while (((s >> c) & 1ULL) == 0ULL) ++c;
  return c;
#endif
}

// Enumerate one sub-cube: the top (n - low_bits) variables are fixed to
// `prefix` (bit p of prefix sets variable low_bits + p), and the low
// `low_bits` variables run over all 2^low_bits assignments in Gray-code order.
// Solutions x (full n-bit assignments) are written to `out` up to `max_out`;
// `*found` is the true count even if it exceeds `max_out`.
FES_HD void fes_search_subcube(uint64_t cst, const uint64_t *lin,
                               const uint64_t *quad_tri, int n, int low_bits,
                               uint64_t prefix, uint64_t *out, int *found,
                               int max_out, uint64_t *df_scratch) {
  const uint64_t base = prefix << low_bits;
  // Initialize df[] and the value at the sub-cube origin (low bits = 0).
  uint64_t *df = df_scratch; // length n, caller-provided (no dynamic alloc)
  for (int i = 0; i < n; ++i) {
    uint64_t d = lin[i] ^ fes_quad(quad_tri, i, i);
    for (int j = 0; j < n; ++j) {
      if (j != i && ((base >> j) & 1ULL)) {
        d ^= fes_quad(quad_tri, i, j);
      }
    }
    df[i] = d;
  }
  uint64_t val = fes_eval(cst, lin, quad_tri, n, base);
  if (val == 0) {
    if (*found < max_out) out[*found] = base;
    ++*found;
  }
  const uint64_t steps = (low_bits >= 63) ? 0 : (1ULL << low_bits);
  for (uint64_t s = 1; s < steps; ++s) {
    int i1 = fes_ctz(s); // flips a low variable, 0 <= i1 < low_bits
    val ^= df[i1];
    // x_{i1} toggled: refresh the other derivatives by the second derivative.
    for (int j = 0; j < n; ++j) {
      if (j != i1) df[j] ^= fes_quad(quad_tri, i1, j);
    }
    if (val == 0) {
      uint64_t x = base | fes_gray(s);
      if (*found < max_out) out[*found] = x;
      ++*found;
    }
  }
}

// Symmetric second-derivative table for the walk: qsym[i*n + j] =
// QUAD(i, j) for i != j and 0 on the diagonal, so the derivative refresh
// `df[j] ^= QUAD(i1, j)  for j != i1` becomes the branch-free row XOR
// `df[j] ^= qsym[i1*n + j]  for all j` (XOR with the zero diagonal is the
// identity).  The CUDA kernel stages this n*n table in shared memory, where
// the warp-uniform row index i1 makes every load a broadcast.
FES_HD void fes_build_qsym(const uint64_t *quad_tri, int n, uint64_t *qsym) {
  for (int i = 0; i < n; ++i)
    for (int j = 0; j < n; ++j)
      qsym[i * n + j] = (i == j) ? 0ULL : fes_quad(quad_tri, i, j);
}

// The walk of fes_search_subcube with the hot loop reading `qsym`
// (fes_build_qsym), reporting each zero through `emit(x)` in visiting order.
// The origin set-up is the same code as fes_search_subcube; the visited
// points, their order, and every value/derivative are identical, which
// test_cpu.cpp checks prefix by prefix.  The CUDA kernel's emitter appends
// straight to the global output, so no per-thread solution cap applies.
template <class Emit>
FES_HD void fes_walk_sym(uint64_t cst, const uint64_t *lin,
                         const uint64_t *quad_tri, const uint64_t *qsym, int n,
                         int low_bits, uint64_t prefix, uint64_t *df_scratch,
                         Emit &emit) {
  const uint64_t base = prefix << low_bits;
  uint64_t *df = df_scratch;
  for (int i = 0; i < n; ++i) {
    uint64_t d = lin[i] ^ fes_quad(quad_tri, i, i);
    for (int j = 0; j < n; ++j) {
      if (j != i && ((base >> j) & 1ULL)) {
        d ^= fes_quad(quad_tri, i, j);
      }
    }
    df[i] = d;
  }
  uint64_t val = fes_eval(cst, lin, quad_tri, n, base);
  if (val == 0) emit(base);
  const uint64_t steps = (low_bits >= 63) ? 0 : (1ULL << low_bits);
  for (uint64_t s = 1; s < steps; ++s) {
    const int i1 = fes_ctz(s);
    val ^= df[i1];
    const uint64_t *row = qsym + i1 * n;
    for (int j = 0; j < n; ++j) df[j] ^= row[j];
    if (val == 0) emit(base | fes_gray(s));
  }
}

// Array emitter with fes_search_subcube's contract: list up to max_out,
// count all.
struct FesArrayEmit {
  uint64_t *out;
  int *found;
  int max_out;
  FES_HDM void operator()(uint64_t x) {
    if (*found < max_out) out[*found] = x;
    ++*found;
  }
};

FES_HD void fes_search_subcube_sym(uint64_t cst, const uint64_t *lin,
                                   const uint64_t *quad_tri,
                                   const uint64_t *qsym, int n, int low_bits,
                                   uint64_t prefix, uint64_t *out, int *found,
                                   int max_out, uint64_t *df_scratch) {
  FesArrayEmit emit{out, found, max_out};
  fes_walk_sym(cst, lin, quad_tri, qsym, n, low_bits, prefix, df_scratch, emit);
}

// The same walk in O(1) per point (the Gray-code bookkeeping of
// Bouillaguet et al., "Fast exhaustive search for polynomial systems in
// F_2", CHES 2010).  fes_walk_sym refreshes all n first derivatives at
// every step; only D_{i1}, the one about to be used, is needed.  Between
// two consecutive flips of variable k (at s = 2^k(2t-1) and 2^k(2t+1)),
// every lower variable flips an even number of times (2^(k-j) times for
// j < k) and exactly one higher variable flips once, k' = ctz(2^(k+1) t),
// which is ctz(s ^ 2^k) at the second flip.  D_k is affine in the other
// variables, so it changes by exactly qsym[k][k'] in between.  Before the
// first flip of k (s = 2^k) the only odd-count lower flip is k - 1, hence
// the initial df[k] ^= qsym[k][k-1].
//
// Every value `val` takes is the one fes_walk_sym computes at the same
// step, so the zeros are the same points in the same order; df holds the
// lazily-updated derivatives, not the eager ones (test_cpu.cpp checks the
// emitted lists against fes_walk_sym and fes_search_subcube).
template <class Emit>
FES_HD void fes_walk_gray(uint64_t cst, const uint64_t *lin,
                          const uint64_t *quad_tri, const uint64_t *qsym, int n,
                          int low_bits, uint64_t prefix, uint64_t *df_scratch,
                          Emit &emit) {
  const uint64_t base = prefix << low_bits;
  uint64_t *df = df_scratch;
  for (int i = 0; i < low_bits; ++i) {
    uint64_t d = lin[i] ^ fes_quad(quad_tri, i, i);
    for (int j = 0; j < n; ++j) {
      if (j != i && ((base >> j) & 1ULL)) {
        d ^= fes_quad(quad_tri, i, j);
      }
    }
    if (i > 0) d ^= qsym[i * n + (i - 1)];
    df[i] = d;
  }
  uint64_t val = fes_eval(cst, lin, quad_tri, n, base);
  if (val == 0) emit(base);
  const uint64_t steps = (low_bits >= 63) ? 0 : (1ULL << low_bits);
  for (uint64_t s = 1; s < steps; ++s) {
    const int k1 = fes_ctz(s);
    const uint64_t rest = s & (s - 1);  // s with its lowest set bit cleared
    if (rest) df[k1] ^= qsym[k1 * n + fes_ctz(rest)];
    val ^= df[k1];
    if (val == 0) emit(base | fes_gray(s));
  }
}

FES_HD void fes_search_subcube_gray(uint64_t cst, const uint64_t *lin,
                                    const uint64_t *quad_tri,
                                    const uint64_t *qsym, int n, int low_bits,
                                    uint64_t prefix, uint64_t *out, int *found,
                                    int max_out, uint64_t *df_scratch) {
  FesArrayEmit emit{out, found, max_out};
  fes_walk_gray(cst, lin, quad_tri, qsym, n, low_bits, prefix, df_scratch, emit);
}

// fes_walk_gray with the walk cut into blocks of 2^K steps, s = h*2^K + l.
// Inside a block (l = 1 .. 2^K - 1) the flipped variable k1 = ctz(l) and,
// when l has a second set bit, the partner k2 = ctz(l & (l - 1)) are
// compile-time constants: the unrolled steps keep D_0..D_{K-1} and the
// K(K-1)/2 second derivatives among them in registers, with no ctz and no
// table load.  When l = 2^k1 the partner is the block's lowest high bit,
// c = K + ctz(h) (none in block 0), so the K values qsym[j][c] are loaded
// once per block.  The step at l = 0 (s = h*2^K) flips variable K + ctz(h)
// and is the walk's only run-time-indexed step.  The arithmetic is
// fes_walk_gray's, step for step, so the zeros are the same points in the
// same order; sub-cubes with low_bits < K fall back to fes_walk_gray.
template <int K, int L, class Emit>
FES_FI void fes_block_steps(uint64_t (&dl)[K], const uint64_t (&q)[K][K],
                            const uint64_t (&qc)[K], bool high, uint64_t &val,
                            uint64_t point, Emit &emit) {
  constexpr int k1 = (L & 1) ? 0 : (L & 2) ? 1 : (L & 4) ? 2 : (L & 8) ? 3
                   : (L & 16) ? 4 : (L & 32) ? 5 : (L & 64) ? 6 : 7;
  constexpr int r = L & (L - 1);
  constexpr int k2 = r == 0 ? -1 : (r & 1) ? 0 : (r & 2) ? 1 : (r & 4) ? 2
                   : (r & 8) ? 3 : (r & 16) ? 4 : (r & 32) ? 5 : (r & 64) ? 6 : 7;
  if constexpr (k2 >= 0) {
    dl[k1] ^= q[k1][k2];
  } else {
    if (high) dl[k1] ^= qc[k1];
  }
  val ^= dl[k1];
  // point = base | gray(h * 2^K); gray is linear over GF(2) and h * 2^K, L
  // have disjoint bits, so gray(h * 2^K + L) = gray(h * 2^K) ^ gray(L).
  if (val == 0) emit(point ^ (uint64_t)(L ^ (L >> 1)));
  if constexpr (L + 1 < (1 << K)) fes_block_steps<K, L + 1>(dl, q, qc, high, val, point, emit);
}

template <int K, class Emit>
FES_HD void fes_walk_block(uint64_t cst, const uint64_t *lin,
                           const uint64_t *quad_tri, const uint64_t *qsym, int n,
                           int low_bits, uint64_t prefix, uint64_t *df_scratch,
                           Emit &emit) {
  // K = 8 (255 unrolled steps) exceeds nvcc's template instantiation depth.
  static_assert(K >= 1 && K <= 7, "block of 2..128 steps");
  if (low_bits < K) {
    fes_walk_gray(cst, lin, quad_tri, qsym, n, low_bits, prefix, df_scratch, emit);
    return;
  }
  const uint64_t base = prefix << low_bits;
  uint64_t *df = df_scratch;
  for (int i = 0; i < low_bits; ++i) {
    uint64_t d = lin[i] ^ fes_quad(quad_tri, i, i);
    for (int j = 0; j < n; ++j) {
      if (j != i && ((base >> j) & 1ULL)) d ^= fes_quad(quad_tri, i, j);
    }
    if (i > 0) d ^= qsym[i * n + (i - 1)];
    df[i] = d;
  }
  uint64_t val = fes_eval(cst, lin, quad_tri, n, base);
  if (val == 0) emit(base);
  uint64_t dl[K], q[K][K], qc[K];
#pragma unroll
  for (int i = 0; i < K; ++i) {
    dl[i] = df[i];
    qc[i] = 0;
#pragma unroll
    for (int j = 0; j < K; ++j) q[i][j] = (i < j) ? qsym[i * n + j] : 0ULL;
  }
  const uint64_t nblocks = 1ULL << (low_bits - K);
  for (uint64_t h = 0; h < nblocks; ++h) {
    const uint64_t s0 = h << K;
    const uint64_t point = base | fes_gray(s0);
    if (h) {
      const int c = K + fes_ctz(h);
      const uint64_t rest = h & (h - 1);
      if (rest) df[c] ^= qsym[c * n + K + fes_ctz(rest)];
      val ^= df[c];
      if (val == 0) emit(point);
#pragma unroll
      for (int j = 0; j < K; ++j) qc[j] = qsym[j * n + c];
    }
    fes_block_steps<K, 1>(dl, q, qc, h != 0, val, point, emit);
  }
}

FES_HD void fes_search_subcube_block6(uint64_t cst, const uint64_t *lin,
                                      const uint64_t *quad_tri,
                                      const uint64_t *qsym, int n, int low_bits,
                                      uint64_t prefix, uint64_t *out, int *found,
                                      int max_out, uint64_t *df_scratch) {
  FesArrayEmit emit{out, found, max_out};
  fes_walk_block<6>(cst, lin, quad_tri, qsym, n, low_bits, prefix, df_scratch, emit);
}

// Default sub-cube split for the workers when no --low-bits is given: at
// most 2^16 sub-cubes (threads), each walking 2^low_bits points, and no
// split below 2^8 points per sub-cube.  The solution set does not depend on
// the split (test_cpu.cpp checks several); only the thread count does.
FES_HD int fes_default_low_bits(int n) {
  const int floor_bits = n < 8 ? n : 8;
  return n - 16 > floor_bits ? n - 16 : floor_bits;
}
