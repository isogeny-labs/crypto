// gpu/fes/test_cpu.cpp -- verify the FES Gray-code search on the host, with no
// GPU.  It compiles the same fes.cuh the CUDA/Metal kernels use, runs the
// per-sub-cube search for every prefix (exactly what one GPU thread does), and
// checks the collected solutions against (a) a from-scratch brute force and
// (b) the independently brute-forced set the generator emitted.
#include <algorithm>
#include <cstdint>
#include <cstdio>
#include <vector>

#include "fes.cuh"
#include "system.h"

static std::vector<uint64_t> brute_force() {
  std::vector<uint64_t> sols;
  for (uint64_t x = 0; x < (1ULL << FES_N); ++x) {
    if (fes_eval(FES_CONST, FES_LIN, FES_QUAD_TRI, FES_N, x) == 0) {
      sols.push_back(x);
    }
  }
  return sols;
}

static std::vector<uint64_t> fes_all(int prefix_bits) {
  const int low = FES_N - prefix_bits;
  std::vector<uint64_t> out(1u << 16);
  std::vector<uint64_t> df(FES_N);
  std::vector<uint64_t> sols;
  for (uint64_t p = 0; p < (1ULL << prefix_bits); ++p) {
    int found = 0;
    fes_search_subcube(FES_CONST, FES_LIN, FES_QUAD_TRI, FES_N, low, p,
                       out.data(), &found, (int)out.size(), df.data());
    for (int i = 0; i < found && i < (int)out.size(); ++i) sols.push_back(out[i]);
  }
  std::sort(sols.begin(), sols.end());
  return sols;
}

int main() {
  int failures = 0;

  auto brute = brute_force();
  std::sort(brute.begin(), brute.end());

  // The generator's independent solution set must match our brute force.
  std::vector<uint64_t> gen(FES_SOLUTIONS, FES_SOLUTIONS + FES_NUM_SOLUTIONS);
  std::sort(gen.begin(), gen.end());
  if (gen != brute) {
    printf("FAIL: generator solution set disagrees with brute force (%zu vs %zu)\n",
           gen.size(), brute.size());
    ++failures;
  }

  // The planted solution must be a genuine zero.
  if (fes_eval(FES_CONST, FES_LIN, FES_QUAD_TRI, FES_N, FES_PLANTED) != 0) {
    printf("FAIL: planted solution is not a zero\n");
    ++failures;
  }

  // The sharded Gray-code search must find exactly the brute-force set, at
  // several shard granularities (different thread counts).
  for (int pb : {0, 2, 4, 6}) {
    if (pb >= FES_N) continue;
    auto got = fes_all(pb);
    if (got != brute) {
      printf("FAIL: FES search with %d prefix bits found %zu, expected %zu\n", pb,
             got.size(), brute.size());
      ++failures;
    } else {
      printf("ok: %d prefix bits (%d threads) -> %zu solutions\n", pb, 1 << pb,
             got.size());
    }
  }

  // The shared-memory walk (fes_search_subcube_sym, what fes.cu runs) must
  // match fes_search_subcube exactly: same solutions in the same order and
  // the same final derivatives, for every prefix at several splits.
  {
    std::vector<uint64_t> qsym((size_t)FES_N * FES_N);
    fes_build_qsym(FES_QUAD_TRI, FES_N, qsym.data());
    std::vector<uint64_t> oa(1u << 16), ob(1u << 16), da(FES_N), db(FES_N);
    long prefixes = 0, mismatches = 0;
    for (int pb : {0, 2, 4, 6, 10}) {
      if (pb > FES_N) continue;
      const int low = FES_N - pb;
      for (uint64_t p = 0; p < (1ULL << pb); ++p) {
        int fa = 0, fb = 0;
        fes_search_subcube(FES_CONST, FES_LIN, FES_QUAD_TRI, FES_N, low, p,
                           oa.data(), &fa, (int)oa.size(), da.data());
        fes_search_subcube_sym(FES_CONST, FES_LIN, FES_QUAD_TRI, qsym.data(),
                               FES_N, low, p, ob.data(), &fb, (int)ob.size(),
                               db.data());
        ++prefixes;
        bool same = fa == fb && da == db;
        for (int i = 0; same && i < fa && i < (int)oa.size(); ++i)
          same = oa[i] == ob[i];
        if (!same) ++mismatches;
      }
    }
    if (mismatches) {
      printf("FAIL: symmetric-table walk differs on %ld of %ld sub-cubes\n",
             mismatches, prefixes);
      ++failures;
    } else {
      printf("ok: symmetric-table walk identical on %ld sub-cubes\n", prefixes);
    }
  }

  // The O(1) walk (fes_walk_gray) must emit exactly fes_search_subcube's
  // list, in the same order, for every prefix at every split, including
  // the tiny sub-cubes (low_bits 0..3) where the initial correction and
  // the "no second set bit" case dominate.
  {
    std::vector<uint64_t> qsym((size_t)FES_N * FES_N);
    fes_build_qsym(FES_QUAD_TRI, FES_N, qsym.data());
    std::vector<uint64_t> oa(1u << 16), ob(1u << 16), da(FES_N), db(FES_N);
    long prefixes = 0, mismatches = 0, zeros = 0;
    for (int pb = 0; pb <= FES_N && pb <= 12; ++pb) {
      const int low = FES_N - pb;
      for (uint64_t p = 0; p < (1ULL << pb); ++p) {
        int fa = 0, fb = 0;
        fes_search_subcube(FES_CONST, FES_LIN, FES_QUAD_TRI, FES_N, low, p,
                           oa.data(), &fa, (int)oa.size(), da.data());
        fes_search_subcube_gray(FES_CONST, FES_LIN, FES_QUAD_TRI, qsym.data(),
                                FES_N, low, p, ob.data(), &fb, (int)ob.size(),
                                db.data());
        ++prefixes;
        zeros += fa;
        bool same = fa == fb;
        for (int i = 0; same && i < fa && i < (int)oa.size(); ++i)
          same = oa[i] == ob[i];
        if (!same) ++mismatches;
      }
    }
    for (int low = 0; low <= 3 && low <= FES_N; ++low) {
      for (uint64_t p = 0; p < (1ULL << (FES_N - low)) && p < 4096; ++p) {
        int fa = 0, fb = 0;
        fes_search_subcube(FES_CONST, FES_LIN, FES_QUAD_TRI, FES_N, low, p,
                           oa.data(), &fa, (int)oa.size(), da.data());
        fes_search_subcube_gray(FES_CONST, FES_LIN, FES_QUAD_TRI, qsym.data(),
                                FES_N, low, p, ob.data(), &fb, (int)ob.size(),
                                db.data());
        ++prefixes;
        bool same = fa == fb;
        for (int i = 0; same && i < fa && i < (int)oa.size(); ++i)
          same = oa[i] == ob[i];
        if (!same) ++mismatches;
      }
    }
    if (mismatches) {
      printf("FAIL: O(1) Gray walk differs on %ld of %ld sub-cubes\n",
             mismatches, prefixes);
      ++failures;
    } else {
      printf("ok: O(1) Gray walk identical on %ld sub-cubes (%ld zeros at splits 0..12)\n",
             prefixes, zeros);
    }
  }

  // The workers' default split must also find exactly the brute-force set.
  {
    const int low = fes_default_low_bits(FES_N);
    auto got = fes_all(FES_N - low);
    if (got != brute) {
      printf("FAIL: default split (low_bits=%d) found %zu, expected %zu\n", low,
             got.size(), brute.size());
      ++failures;
    } else {
      printf("ok: default split low_bits=%d (%d threads) -> %zu solutions\n",
             low, 1 << (FES_N - low), got.size());
    }
  }

  if (failures == 0) {
    printf("PASS: FES host verification (n=%d, m=%d, %zu solutions)\n", FES_N,
           FES_M, brute.size());
  }
  return failures == 0 ? 0 : 1;
}
