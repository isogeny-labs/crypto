// gpu/fes/fes_io.hpp -- the stable text I/O contract every FES worker speaks,
// so the Rust binary can drive a CUDA, Metal, or host-emulation worker through
// the exact same wire. Host-only (plain C++), included by fes_solve.cpp and by
// fes.cu's host side.
//
// Input file (whitespace-separated decimal integers):
//     n m
//     <const>
//     <lin_0> ... <lin_{n-1}>
//     <quad_tri_0> ... <quad_tri_{T-1}>        T = n*(n+1)/2
// where each coefficient is a u64 whose bit e is that term's coefficient in
// equation e (see fes.cuh / fref.py). Output on stdout:
//     SOLUTIONS <k>
//     <x_0>
//     ...
//     <x_{k-1}>
// each x a decimal u64 assignment that zeroes the system. A worker may cap how
// many it lists; the Rust side re-verifies every returned x, so a worker can
// only ever propose candidates.
#pragma once

#include <chrono>
#include <cstdint>
#include <cstdlib>
#include <cstdio>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#if defined(__unix__) || defined(__APPLE__)
#include <poll.h>
#endif

struct FesSystem {
  int n = 0;
  int m = 0;
  uint64_t cst = 0;
  std::vector<uint64_t> lin;      // length n
  std::vector<uint64_t> quad_tri; // length n*(n+1)/2
};

// Parse a system file. Returns false on any malformed input.
inline bool fes_read_system(const std::string &path, FesSystem &sys) {
  std::ifstream f(path);
  if (!f) return false;
  if (!(f >> sys.n >> sys.m)) return false;
  if (sys.n < 0 || sys.n > 62 || sys.m < 0 || sys.m > 64) return false;
  if (!(f >> sys.cst)) return false;
  sys.lin.assign(sys.n, 0);
  for (int i = 0; i < sys.n; ++i)
    if (!(f >> sys.lin[i])) return false;
  const int t = sys.n * (sys.n + 1) / 2;
  sys.quad_tri.assign(t, 0);
  for (int i = 0; i < t; ++i)
    if (!(f >> sys.quad_tri[i])) return false;
  return true;
}

// Print solutions in the contract's format.
inline void fes_write_solutions(const std::vector<uint64_t> &sols) {
  printf("SOLUTIONS %zu\n", sols.size());
  for (uint64_t x : sols) printf("%llu\n", (unsigned long long)x);
}

// Serve protocol (`--serve`, CUDA and host workers): requests are systems in
// the contract above, back to back on stdin; each reply is
//     SOLUTIONS <listed> <total>
//     <x_0> ... (listed lines)
// flushed at once; total > listed means a capped (incomplete) list.
// fes_serve_read returns 1 for a request, 0 at a clean EOF, -1 if malformed.
inline int fes_serve_read(FesSystem &sys, int max_vars) {
  if (scanf("%d %d", &sys.n, &sys.m) != 2) return 0;
  if (sys.n < 1 || sys.n > max_vars || sys.m < 0 || sys.m > 64) return -1;
  unsigned long long v = 0;
  if (scanf("%llu", &v) != 1) return -1;
  sys.cst = v;
  const int t = sys.n * (sys.n + 1) / 2;
  sys.lin.assign(sys.n, 0);
  sys.quad_tri.assign(t, 0);
  for (int i = 0; i < sys.n; ++i) { if (scanf("%llu", &v) != 1) return -1; sys.lin[i] = v; }
  for (int i = 0; i < t; ++i) { if (scanf("%llu", &v) != 1) return -1; sys.quad_tri[i] = v; }
  return 1;
}

inline void fes_serve_reply(const std::vector<uint64_t> &sols, unsigned long long total) {
  printf("SOLUTIONS %zu %llu\n", sols.size(), total);
  for (uint64_t x : sols) printf("%llu\n", (unsigned long long)x);
  fflush(stdout);
}

// Binary serve protocol (`--serve-bin`), little-endian, no text parsing:
//   request: u32 magic 0x31534546 ("FES1"), u32 n, u32 m, u32 0,
//            u64 cst, u64 lin[n], u64 quad_tri[n*(n+1)/2]
//   reply:   u32 magic, u32 status (0 ok, 1 malformed request),
//            u64 listed, u64 total, u64 x[listed]
// Same contract as the text protocol: total > listed means a capped list.
static const uint32_t FES_BIN_MAGIC = 0x31534546u;

// Opt-in (FES_SERVE_SPIN_US=N, default 0 = off, no extra system call): before the blocking read of the next
// request, poll stdin without sleeping for up to N microseconds, so a worker that idles between requests does not
// pay a cold wake-up.  Probe for the per-call wake-up cost; it changes no reply.  Skipped when a request is
// already buffered (glibc) or on a platform without poll().
inline void fes_serve_spin_for_request() {
#if defined(__unix__) || defined(__APPLE__)
  static const int spin_us = [] { const char *v = getenv("FES_SERVE_SPIN_US"); return v ? atoi(v) : 0; }();
  if (spin_us <= 0) return;
#ifdef __GLIBC__
  if (stdin->_IO_read_ptr < stdin->_IO_read_end) return;
#endif
  const auto t0 = std::chrono::steady_clock::now();
  pollfd p{0, POLLIN, 0};
  while (poll(&p, 1, 0) == 0 &&
         std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::steady_clock::now() - t0).count() < spin_us) {
  }
#endif
}

// 1 = request read, 0 = clean EOF, -1 = malformed.
inline int fes_serve_bin_read(FesSystem &sys, int max_vars) {
  fes_serve_spin_for_request();
  uint32_t h[4];
  if (fread(h, sizeof h, 1, stdin) != 1) return 0;
  if (h[0] != FES_BIN_MAGIC || h[1] < 1 || (int)h[1] > max_vars || h[2] > 64) return -1;
  sys.n = (int)h[1];
  sys.m = (int)h[2];
  const size_t t = (size_t)sys.n * (sys.n + 1) / 2;
  sys.lin.resize(sys.n);
  sys.quad_tri.resize(t);
  uint64_t cst = 0;
  if (fread(&cst, 8, 1, stdin) != 1) return -1;
  sys.cst = cst;
  if (fread(sys.lin.data(), 8, sys.n, stdin) != (size_t)sys.n) return -1;
  if (t && fread(sys.quad_tri.data(), 8, t, stdin) != t) return -1;
  return 1;
}

inline void fes_serve_bin_reply(const uint64_t *sols, uint64_t listed, uint64_t total, uint32_t status) {
  const uint32_t h[2] = {FES_BIN_MAGIC, status};
  const uint64_t c[2] = {listed, total};
  fwrite(h, sizeof h, 1, stdout);
  fwrite(c, sizeof c, 1, stdout);
  if (listed) fwrite(sols, 8, listed, stdout);
  fflush(stdout);
}
