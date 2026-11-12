/* syz.c -- FOLLOWUP-selector-extra-syzygies.
 *
 * Kernel basis of the homogeneous selector system of emin.c (same instance format, same row
 * builder, same column layout j*nmons+t, lambda = sum_j lambda_j x^j), quotient by the trivial
 * family T_e = { q (1+ell_0) zeta : deg q <= e-1 } (lane_A_falldegree.md Lead A1, F3), and a
 * characterisation of the extra kernel.  emin.c is NOT modified.
 *
 * Usage:  syz E PERMILLE SEED < instance_line      (kernel analysis)
 *         syz dual D < instance_line             (dual section count, see dual_main)
 *         syz coset H E < instance_line          (sections over the half H_H only, see coset_main)
 *   PERMILLE = 1000: rows of all 2^k points.  < 1000: rows of a random sample of points; the
 *   sample kernel (a superset) is then cut down to the exact kernel by evaluating every sample-
 *   kernel vector on all 2^k points (exact result either way).
 * Output: one JSON line on stdout; progress on stderr.
 */
#include <m4ri/m4ri.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/resource.h>

static int n, k, e, kr; /* kr = number of row vectors v (= k except in coset mode, where k = kr-1) */
static uint64_t poly, a6, z, V[64], VR[64], X[64]; /* X[j] = x^j */
static uint64_t *rowsC;                    /* k+1 trace rows per point */
static uint64_t *span;
static int coset_half = -1; /* -1: whole V; 0/1: restrict to H_0/H_1 (k-1 coordinates) */
static size_t P;

static uint64_t gmul(uint64_t a, uint64_t b) {
  uint64_t r = 0, top = 1ULL << n;
  while (b) {
    if (b & 1) r ^= a;
    b >>= 1;
    a <<= 1;
    if (a & top) a ^= poly;
  }
  return r;
}
static uint64_t gsq(uint64_t a) { return gmul(a, a); }
static uint64_t gpow(uint64_t a, uint64_t ex) {
  uint64_t r = 1;
  while (ex) {
    if (ex & 1) r = gmul(r, a);
    a = gsq(a);
    ex >>= 1;
  }
  return r;
}
static uint64_t ginv(uint64_t a) { return gpow(a, (1ULL << n) - 2); }
static int gtr(uint64_t a) {
  uint64_t t = 0, x = a;
  for (int i = 0; i < n; i++) { t ^= x; x = gsq(x); }
  if (t > 1) { fprintf(stderr, "trace not in F2\n"); exit(2); }
  return (int)t;
}
static int par(uint64_t w) { return __builtin_parityll(w); }
static double now(void) {
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return ts.tv_sec + 1e-9 * ts.tv_nsec;
}
static uint64_t rs;
static uint64_t rnd(void) {
  uint64_t x = (rs += 0x9E3779B97F4A7C15ULL);
  x = (x ^ (x >> 30)) * 0xBF58476D1CE4E5B9ULL;
  x = (x ^ (x >> 27)) * 0x94D049BB133111EBULL;
  return x ^ (x >> 31);
}

static uint64_t Tcol[64];
static uint64_t trrow(uint64_t w) { /* bit j = Tr(x^j w) */
  uint64_t r = 0;
  for (int b = 0; b < n; b++)
    if (w >> b & 1) r ^= Tcol[b];
  return r;
}
static uint64_t phistar(uint64_t C, uint64_t A) {
  uint64_t a = gsq(C) ^ z, b = gmul(z, C), A2 = gsq(A);
  return gsq(A2) ^ gmul(a, A2) ^ gmul(b, A);
}
static uint64_t hC(uint64_t C) { return gmul(gsq(z), gsq(C)) ^ a6; }
static int rank64(uint64_t *w, int m) {
  uint64_t piv[64] = {0};
  int r = 0;
  for (int i = 0; i < m; i++) {
    uint64_t u = w[i];
    while (u) {
      int p = 63 - __builtin_clzll(u);
      if (piv[p]) u ^= piv[p];
      else { piv[p] = u; r++; break; }
    }
  }
  return r;
}

static void setup(void) {
  unsigned long long t;
  if (scanf("%d %d %llu", &n, &k, &t) != 3) exit(2);
  poly = t;
  if (scanf("%llu", &t) != 1) exit(2);
  a6 = t;
  if (scanf("%llu", &t) != 1) exit(2);
  z = t;
  for (int i = 0; i < k; i++) {
    if (scanf("%llu", &t) != 1) exit(2);
    V[i] = t;
  }
  X[0] = 1;
  for (int j = 1; j < n; j++) X[j] = gmul(X[j - 1], 2);
  uint64_t off = 0;
  kr = k;
  for (int i = 0; i < k; i++) VR[i] = V[i];
  if (coset_half >= 0) {
    /* H_h = {C in V : Tr(C) = h + Tr(a6/z^2)} = off + V0, V0 = V cap ker Tr, coordinates on V0 */
    int t0b = gtr(gmul(a6, ginv(gsq(z))));
    int is = -1;
    for (int i = 0; i < k; i++)
      if (gtr(V[i])) { is = i; break; }
    if (is < 0) { fprintf(stderr, "V inside ker Tr: no split\n"); exit(2); }
    for (int i = 0; i < k; i++) VR[i] = V[i];
    kr = k;
    uint64_t vs = V[is], W[64];
    int m = 0;
    for (int i = 0; i < k; i++)
      if (i != is) W[m++] = V[i] ^ (gtr(V[i]) ? vs : 0);
    for (int i = 0; i < m; i++) V[i] = W[i];
    k = m;
    off = ((coset_half ^ t0b) & 1) ? vs : 0;
  }
  int trp[128];
  uint64_t xm = 1;
  for (int m = 0; m < 2 * n - 1; m++) { trp[m] = gtr(xm); xm = gmul(xm, 2); }
  for (int i = 0; i < n; i++) {
    Tcol[i] = 0;
    for (int j = 0; j < n; j++)
      if (trp[i + j]) Tcol[i] |= 1ULL << j;
  }
  P = (size_t)1 << k;
  span = malloc(sizeof(uint64_t) * P);
  span[0] = 0;
  for (size_t m = 1; m < P; m++) {
    size_t low = m & (~m + 1);
    span[m] = span[m ^ low] ^ V[__builtin_ctzll(low)];
  }
  for (size_t m = 0; m < P; m++) span[m] ^= off;
  rowsC = malloc(sizeof(uint64_t) * P * (kr + 1));
  for (size_t ci = 0; ci < P; ci++) {
    uint64_t C = span[ci], *R = rowsC + ci * (kr + 1);
    for (int i = 0; i < kr; i++) R[i] = trrow(phistar(C, VR[i]));
    R[kr] = trrow(hC(C));
  }
}

static int nmons;
static uint32_t *mons;
static int32_t *midx;
static void make_mons(void) {
  mons = malloc(sizeof(uint32_t) * P);
  midx = malloc(sizeof(int32_t) * P);
  nmons = 0;
  for (uint32_t m = 0; m < P; m++) {
    midx[m] = -1;
    if (__builtin_popcount(m) <= e) { midx[m] = nmons; mons[nmons++] = m; }
  }
}

static void put_point(mzd_t *A, rci_t r0, uint32_t ci) {
  static int *sub = NULL;
  if (!sub) sub = malloc(sizeof(int) * nmons);
  int ns = 0;
  for (int t = 0; t < nmons; t++)
    if ((mons[t] & ci) == mons[t]) sub[ns++] = t;
  const uint64_t *R = rowsC + (size_t)ci * (kr + 1);
  for (int r = 0; r <= kr; r++)
    for (int j = 0; j < n; j++)
      if (R[r] >> j & 1)
        for (int s = 0; s < ns; s++) mzd_write_bit(A, r0 + r, j * nmons + sub[s], 1);
}

static int first_bit(const mzd_t *A, rci_t r) {
  for (wi_t w = 0; w < A->width; w++) {
    word x = mzd_row((mzd_t *)A, r)[w];
    if (x) return w * m4ri_radix + __builtin_ctzll(x);
  }
  return -1;
}

/* Right kernel {x : A x = 0} as rows of a new matrix.  A is destroyed (RREF in place). */
static mzd_t *kernel_rows(mzd_t *A) {
  rci_t r = mzd_echelonize_m4ri(A, 1, 0);
  rci_t nc = A->ncols;
  int *piv = malloc(sizeof(int) * (r ? r : 1));
  uint8_t *isp = calloc(nc, 1);
  for (rci_t i = 0; i < r; i++) { piv[i] = first_bit(A, i); isp[piv[i]] = 1; }
  int *fidx = malloc(sizeof(int) * nc);
  rci_t nf = 0;
  for (rci_t c = 0; c < nc; c++) fidx[c] = isp[c] ? -1 : nf++;
  mzd_t *K = mzd_init(nf ? nf : 1, nc);
  if (!nf) K->nrows = 0;
  for (rci_t c = 0; c < nc; c++)
    if (fidx[c] >= 0) mzd_write_bit(K, fidx[c], c, 1);
  for (rci_t i = 0; i < r; i++) {
    word *row = mzd_row(A, i);
    for (wi_t w = 0; w < A->width; w++) {
      word x = row[w];
      while (x) {
        int b = __builtin_ctzll(x);
        x &= x - 1;
        rci_t c = w * m4ri_radix + b;
        if (c < nc && fidx[c] >= 0) mzd_write_bit(K, fidx[c], piv[i], 1);
      }
    }
  }
  free(piv); free(isp); free(fidx);
  return K;
}

/* lambda(C) for all C (index = binary coordinates of C) from an ANF row vector */
static void eval_row(const mzd_t *M, rci_t i, uint64_t *L) {
  memset(L, 0, sizeof(uint64_t) * P);
  word *row = mzd_row((mzd_t *)M, i);
  for (wi_t w = 0; w < M->width; w++) {
    word x = row[w];
    while (x) {
      int b = __builtin_ctzll(x);
      x &= x - 1;
      rci_t p = w * m4ri_radix + b;
      if (p >= M->ncols) break;
      L[mons[p % nmons]] ^= 1ULL << (p / nmons);
    }
  }
  for (int i2 = 0; i2 < k; i2++) {
    size_t bit = (size_t)1 << i2;
    for (size_t c = 0; c < P; c++)
      if (c & bit) L[c] ^= L[c ^ bit];
  }
}

static int rank_of(const mzd_t *M) {
  if (M->nrows == 0) return 0;
  mzd_t *C = mzd_copy(NULL, M);
  int r = mzd_echelonize_m4ri(C, 0, 0);
  mzd_free(C);
  return r;
}

/* violation matrix: row i = parities of the homogeneous rows at every point for vector i */
static mzd_t *viol_matrix(const mzd_t *M) {
  mzd_t *Vm = mzd_init(M->nrows ? M->nrows : 1, (rci_t)(P * (k + 1)));
  uint64_t *L = malloc(sizeof(uint64_t) * P);
  for (rci_t i = 0; i < M->nrows; i++) {
    eval_row(M, i, L);
    for (size_t c = 0; c < P; c++) {
      const uint64_t *R = rowsC + c * (k + 1);
      for (int r = 0; r <= k; r++)
        if (par(R[r] & L[c])) mzd_write_bit(Vm, i, (rci_t)(c * (k + 1) + r), 1);
    }
  }
  free(L);
  return Vm;
}

/* Generic: matrix whose row i has one bit per (selected point, check) */
typedef int (*checkfn)(size_t c, uint64_t Lc, int r); /* returns parity bit */
static uint64_t *urows; /* universal rows: n+1 per point */
static mzd_t *eval_matrix(const mzd_t *M, int sel /*0 all,1 H0,2 H1*/, int mode /*0 values,1 universal*/,
                          const uint8_t *ell0) {
  size_t np = 0;
  for (size_t c = 0; c < P; c++)
    if (!sel || ell0[c] == sel - 1) np++;
  int per = mode == 0 ? n : n + 1;
  mzd_t *Em = mzd_init(M->nrows ? M->nrows : 1, (rci_t)(np * per));
  uint64_t *L = malloc(sizeof(uint64_t) * P);
  for (rci_t i = 0; i < M->nrows; i++) {
    eval_row(M, i, L);
    size_t q = 0;
    for (size_t c = 0; c < P; c++) {
      if (sel && ell0[c] != sel - 1) continue;
      for (int r = 0; r < per; r++) {
        int bit = mode == 0 ? (int)(L[c] >> r & 1) : par(urows[c * (n + 1) + r] & L[c]);
        if (bit) mzd_write_bit(Em, i, (rci_t)(q * per + r), 1);
      }
      q++;
    }
  }
  free(L);
  return Em;
}

/* coefficientwise field map on the ANF coefficients: beta_t -> f(beta_t) */
static mzd_t *map_coeffs(const mzd_t *M, int which /*0 square, 1 mult by x*/) {
  mzd_t *O = mzd_init(M->nrows ? M->nrows : 1, M->ncols);
  for (rci_t i = 0; i < M->nrows; i++)
    for (int t = 0; t < nmons; t++) {
      uint64_t b = 0;
      for (int j = 0; j < n; j++)
        if (mzd_read_bit(M, i, j * nmons + t)) b |= 1ULL << j;
      if (!b) continue;
      uint64_t f = which == 0 ? gsq(b) : gmul(b, 2);
      for (int j = 0; j < n; j++)
        if (f >> j & 1) mzd_write_bit(O, i, j * nmons + t, 1);
    }
  return O;
}

/* projection onto the columns whose monomial has degree > d */
static int rank_above(const mzd_t *M, int d) {
  int cnt = 0;
  for (int t = 0; t < nmons; t++)
    if (__builtin_popcount(mons[t]) > d) cnt++;
  if (!cnt || !M->nrows) return 0;
  mzd_t *Q = mzd_init(M->nrows, cnt * n);
  for (rci_t i = 0; i < M->nrows; i++) {
    int q = 0;
    for (int j = 0; j < n; j++)
      for (int t = 0; t < nmons; t++)
        if (__builtin_popcount(mons[t]) > d) {
          if (mzd_read_bit(M, i, j * nmons + t)) mzd_write_bit(Q, i, q, 1);
          q++;
        }
  }
  int r = mzd_echelonize_m4ri(Q, 0, 0);
  mzd_free(Q);
  return r;
}

static long Mbin(int kk, int ee) {
  long s = 0, c = 1;
  for (int d = 0; d <= ee && d <= kk; d++) {
    s += c;
    c = c * (kk - d) / (d + 1);
  }
  return ee < 0 ? 0 : s;
}

/* Dual count (post-hoc, see RESULT.md): S_D = { G : V -> K, deg G <= D, G(C) in W_C for all C },
   W_C = span(Phi*_C(v_1..v_k), h_C).  Rows: Tr(G(C) a) = 0 for a in a basis of A_C = W_C^perp.
   Prints dim S_D and Ann0 = sum_C (k+1 - rank W_C).  Identity to test:
   nullity(e) = n M(k,e) - (k+1) 2^k + Ann0 + dim S_{k-e-1}. */
static int dual_main(int D) {
  e = D;
  setup();
  make_mons();
  rci_t ncol = (rci_t)n * nmons;
  long ann0 = 0, nrows = 0;
  uint64_t *Ab = malloc(sizeof(uint64_t) * P * n);
  int *na = malloc(sizeof(int) * P);
  for (size_t c = 0; c < P; c++) {
    /* nullspace of the (k+1) x n trace-row matrix: lambda with par(R[r] & lambda) = 0 */
    const uint64_t *R = rowsC + c * (k + 1);
    uint64_t piv[64] = {0};
    int rk = 0;
    for (int r = 0; r <= k; r++) {
      uint64_t u = R[r];
      for (int b = 63; b >= 0 && u; b--)
        if (u >> b & 1) {
          if (piv[b]) u ^= piv[b];
          else { piv[b] = u; rk++; break; }
        }
    }
    ann0 += k + 1 - rk;
    /* reduce to RREF over the pivots, then read off the nullspace */
    for (int b = 63; b >= 0; b--)
      if (piv[b])
        for (int b2 = 63; b2 >= 0; b2--)
          if (b2 != b && piv[b2] && (piv[b2] >> b & 1)) piv[b2] ^= piv[b];
    int m = 0;
    for (int f = 0; f < n; f++) {
      if (piv[f]) continue;
      uint64_t v = 1ULL << f;
      for (int b = 0; b < n; b++)
        if (piv[b] && (piv[b] >> f & 1)) v |= 1ULL << b;
      Ab[c * n + m++] = v;
    }
    na[c] = m;
    nrows += m;
  }
  mzd_t *A = mzd_init((rci_t)nrows, ncol);
  rci_t r0 = 0;
  int *sub = malloc(sizeof(int) * nmons);
  for (size_t c = 0; c < P; c++) {
    int ns = 0;
    for (int t = 0; t < nmons; t++)
      if ((mons[t] & c) == mons[t]) sub[ns++] = t;
    for (int a = 0; a < na[c]; a++, r0++) {
      uint64_t lam = Ab[c * n + a];
      for (int i2 = 0; i2 <= k; i2++)
        if (par(rowsC[c * (k + 1) + i2] & lam)) { fprintf(stderr, "annihilator bug\n"); exit(3); }
      uint64_t row = trrow(lam);
      for (int j = 0; j < n; j++)
        if (row >> j & 1)
          for (int s2 = 0; s2 < ns; s2++) mzd_write_bit(A, r0, j * nmons + sub[s2], 1);
    }
  }
  double t1 = now();
  rci_t rk = mzd_echelonize_m4ri(A, 0, 0);
  struct rusage ru_;
  getrusage(RUSAGE_SELF, &ru_);
  printf("{\"n\": %d, \"k\": %d, \"D\": %d, \"cols\": %d, \"rows\": %ld, \"rank\": %d, \"dim_S\": %d, "
         "\"ann0\": %ld, \"maxrss_kb\": %ld, \"seconds\": %.1f}\n",
         n, k, D, ncol, nrows, rk, ncol - rk, ann0, ru_.ru_maxrss, now() - t1);
  return 0;
}

/* Sections over one half: nullity of the homogeneous system on the points of H_h only, with
   unknowns of degree <= E in the k-1 coordinates of H_h (post-hoc, see RESULT.md). */
static int coset_main(int h, int E) {
  coset_half = h;
  e = E;
  setup();
  make_mons();
  rci_t ncol = (rci_t)n * nmons;
  mzd_t *A = mzd_init((rci_t)(P * (kr + 1)), ncol);
  for (size_t c = 0; c < P; c++) put_point(A, (rci_t)(c * (kr + 1)), (uint32_t)c);
  rci_t r = mzd_echelonize_m4ri(A, 0, 0);
  printf("{\"n\": %d, \"half\": %d, \"kcoords\": %d, \"e\": %d, \"cols\": %d, \"rank\": %d, \"dim_sections\": %d}\n",
         n, h, k, e, ncol, r, ncol - r);
  return 0;
}

int main(int argc, char **argv) {
  if (argc >= 3 && !strcmp(argv[1], "dual")) return dual_main(atoi(argv[2]));
  if (argc >= 4 && !strcmp(argv[1], "coset")) return coset_main(atoi(argv[2]), atoi(argv[3]));
  if (argc < 4) { fprintf(stderr, "usage: syz E PERMILLE SEED < instance | syz dual D < instance\n"); return 2; }
  e = atoi(argv[1]);
  int permille = atoi(argv[2]);
  rs = strtoull(argv[3], NULL, 10) * 0x1000193ULL + 77;
  double t0 = now();
  setup();
  make_mons();
  rci_t ncol = (rci_t)n * nmons;
  /* ell_0 */
  uint64_t zeta = ginv(gsq(z));
  int t0bit = gtr(gmul(a6, zeta));
  uint8_t *ell0 = malloc(P);
  for (size_t c = 0; c < P; c++) ell0[c] = (uint8_t)(gtr(span[c]) ^ t0bit);
  /* consistency of F1/F2 at every point: Tr(zeta Phi*_C(v)) = 0, Tr(zeta h_C) = ell_0(C) */
  long f12_bad = 0;
  for (size_t c = 0; c < P; c++) {
    for (int i = 0; i < k; i++) f12_bad += gtr(gmul(zeta, phistar(span[c], V[i])));
    f12_bad += gtr(gmul(zeta, hC(span[c]))) != ell0[c];
  }
  /* 1. sample kernel */
  size_t npts = P;
  uint32_t *pts = malloc(sizeof(uint32_t) * P);
  if (permille >= 1000) {
    for (size_t c = 0; c < P; c++) pts[c] = (uint32_t)c;
  } else {
    npts = P * permille / 1000;
    for (size_t c = 0; c < P; c++) pts[c] = (uint32_t)c;
    for (size_t i = 0; i < npts; i++) {
      size_t j = i + rnd() % (P - i);
      uint32_t tt = pts[i]; pts[i] = pts[j]; pts[j] = tt;
    }
  }
  mzd_t *H = mzd_init((rci_t)(npts * (k + 1)), ncol);
  for (size_t i = 0; i < npts; i++) put_point(H, (rci_t)(i * (k + 1)), pts[i]);
  double t1 = now();
  mzd_t *Ks = kernel_rows(H);
  mzd_free(H);
  fprintf(stderr, "e=%d: %zu points, sample kernel %d (%.1fs)\n", e, npts, Ks->nrows, now() - t1);
  /* 2. exact kernel: cut the sample kernel by the violations on all points */
  mzd_t *Vm = viol_matrix(Ks);
  int vr = rank_of(Vm);
  mzd_t *K;
  if (vr == 0) {
    K = Ks;
  } else {
    mzd_t *Vt = mzd_transpose(NULL, Vm);
    mzd_t *Cm = kernel_rows(Vt); /* coefficient combinations with no violation */
    mzd_free(Vt);
    K = mzd_mul(NULL, Cm, Ks, 0);
    mzd_free(Cm);
    mzd_free(Ks);
  }
  mzd_free(Vm);
  int N = K->nrows;
  /* independent check of the final basis */
  mzd_t *Vk = viol_matrix(K);
  int kernel_check_ok = mzd_is_zero(Vk);
  mzd_free(Vk);
  int rankK = rank_of(K);
  fprintf(stderr, "kernel %d (rank %d), check %d\n", N, rankK, kernel_check_ok);
  /* 3. trivial family */
  int nq = 0;
  for (int t = 0; t < nmons; t++)
    if (__builtin_popcount(mons[t]) <= e - 1) nq++;
  mzd_t *T = mzd_init(nq ? nq : 1, ncol);
  if (!nq) T->nrows = 0;
  int qi = 0;
  uint32_t tvmask = 0;
  for (int i = 0; i < k; i++)
    if (gtr(V[i])) tvmask |= 1u << i;
  for (int t = 0; t < nmons; t++) {
    uint32_t m = mons[t];
    if (__builtin_popcount(m) > e - 1) continue;
    /* g = m * ((1+t0) + sum_{i in tvmask} c_i), multilinear */
    uint8_t *g = calloc(nmons, 1);
    if (!t0bit) g[midx[m]] ^= 1;
    for (int i = 0; i < k; i++)
      if (tvmask >> i & 1) g[midx[m | (1u << i)]] ^= 1;
    for (int j = 0; j < n; j++)
      if (zeta >> j & 1)
        for (int u = 0; u < nmons; u++)
          if (g[u]) mzd_write_bit(T, qi, j * nmons + u, 1);
    free(g);
    qi++;
  }
  mzd_t *VT = viol_matrix(T);
  int T_in_kernel = mzd_is_zero(VT);
  mzd_free(VT);
  int rankT = rank_of(T);
  mzd_t *KT = mzd_stack(NULL, K, T);
  int rankKT = rank_of(KT);
  mzd_free(KT);
  int excess = rankK - rankT;
  /* 4. degree filtration: dim(X cap deg<=d) = rank X - rank(proj_{deg>d} X) */
  char filt[2048] = "";
  for (int d = 0; d <= e; d++) {
    int dk = rankK - rank_above(K, d), dt = rankT - rank_above(T, d);
    char b[64];
    snprintf(b, sizeof b, "%s[%d, %d, %d]", d ? ", " : "", d, dk, dt);
    strcat(filt, b);
  }
  /* 5. support on the halves H0 = {ell_0=0}, H1 = {ell_0=1} */
  mzd_t *E0 = eval_matrix(K, 1, 0, ell0), *E1 = eval_matrix(K, 2, 0, ell0);
  int r0 = rank_of(E0), r1 = rank_of(E1);
  mzd_free(E0); mzd_free(E1);
  /* 6. universal (V-independent) subfamily */
  urows = malloc(sizeof(uint64_t) * P * (n + 1));
  for (size_t c = 0; c < P; c++) {
    for (int i = 0; i < n; i++) urows[c * (n + 1) + i] = trrow(phistar(span[c], X[i]));
    urows[c * (n + 1) + n] = trrow(hC(span[c]));
  }
  mzd_t *U = eval_matrix(K, 0, 1, ell0);
  int ru = rank_of(U);
  mzd_free(U);
  /* 7. fibre ranks against pointwise annihilator dims */
  /* hist key: half, dimA_C, dimU_C, fibre rank of K, fibre rank of T */
  static long hist[2][65][65][65];
  memset(hist, 0, sizeof hist);
  {
    uint64_t *LK = malloc(sizeof(uint64_t) * P * (size_t)(N ? N : 1));
    uint64_t *L = malloc(sizeof(uint64_t) * P);
    for (int i = 0; i < N; i++) {
      eval_row(K, i, L);
      for (size_t c = 0; c < P; c++) LK[c * N + i] = L[c];
    }
    uint64_t *w = malloc(sizeof(uint64_t) * (N + n + 2));
    for (size_t c = 0; c < P; c++) {
      memcpy(w, rowsC + c * (k + 1), sizeof(uint64_t) * (k + 1));
      int dA = n - rank64(w, k + 1);
      memcpy(w, urows + c * (n + 1), sizeof(uint64_t) * (n + 1));
      int dU = n - rank64(w, n + 1);
      memcpy(w, LK + c * N, sizeof(uint64_t) * N);
      int fk = rank64(w, N);
      hist[ell0[c]][dA][dU][fk]++;
    }
    free(LK); free(L); free(w);
  }
  /* 8. closure: Frobenius (coefficientwise squaring) and multiplication by x */
  mzd_t *S2 = map_coeffs(K, 0), *SX = map_coeffs(K, 1);
  mzd_t *V2 = viol_matrix(S2), *VX = viol_matrix(SX);
  int rs2 = rank_of(V2), rsx = rank_of(VX);
  mzd_free(S2); mzd_free(SX); mzd_free(V2); mzd_free(VX);
  /* output */
  printf("{\"n\": %d, \"k\": %d, \"e\": %d, \"cols\": %d, \"points_sampled\": %zu, \"sample_kernel_violation_rank\": %d, "
         "\"F1F2_violations\": %ld, \"kernel_dim\": %d, \"kernel_check_ok\": %s, \"trivial_pred\": %ld, "
         "\"trivial_rank\": %d, \"trivial_in_kernel\": %s, \"rank_kernel_plus_trivial\": %d, \"excess\": %d, "
         "\"t0\": %d, \"trV_mask\": %u, "
         "\"filtration_d_dimK_dimT\": [%s], \"rank_on_H0\": %d, \"rank_on_H1\": %d, "
         "\"dim_vanish_on_H1\": %d, \"dim_vanish_on_H0\": %d, \"dim_universal\": %d, "
         "\"dim_frobenius_closed\": %d, \"dim_xmult_closed\": %d, \"fibre_hist\": [",
         n, k, e, ncol, npts, vr, f12_bad, N, kernel_check_ok ? "true" : "false", Mbin(k - 1, e - 1), rankT,
         T_in_kernel ? "true" : "false", rankKT, excess, t0bit, tvmask, filt, r0, r1, rankK - r1,
         rankK - r0, rankK - ru, rankK - rs2, rankK - rsx);
  int first = 1;
  for (int h = 0; h < 2; h++)
    for (int a = 0; a <= n; a++)
      for (int u = 0; u <= n; u++)
        for (int f = 0; f <= n; f++)
          if (hist[h][a][u][f]) {
            printf("%s{\"ell0\": %d, \"dimA\": %d, \"dimU\": %d, \"fibre_rank\": %d, \"points\": %ld}",
                   first ? "" : ", ", h, a, u, f, hist[h][a][u][f]);
            first = 0;
          }
  struct rusage ru_;
  getrusage(RUSAGE_SELF, &ru_);
  printf("], \"maxrss_kb\": %ld, \"seconds\": %.1f}\n", ru_.ru_maxrss, now() - t0);
  return 0;
}
