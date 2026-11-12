/* emin.c — C port of selector_check.e_min (NOTE.md, Proposition S).
 *
 * For an unsatisfiable m=2 instance (n, field polynomial, a6, z, V = (v_1..v_k)), e_min is the
 * least e for which there is lambda: V -> K, each F2-coordinate of degree <= e in the k
 * coordinates of C, with for every C in V
 *     Tr(lambda(C) * Phi*_C(v_i)) = 0  (i = 1..k),   Phi*_C(A) = A^4 + (C^2+z) A^2 + z C A,
 *     Tr(lambda(C) * h_C)         = 1,               h_C = z^2 C^2 + a6.
 * Unknowns: the ANF coefficients x[j,t] of lambda_j on the monomials mu_t of degree <= e
 * (n * M columns).  Each point C contributes k+1 rows.
 *
 * Method (lazy constraint generation; both verdicts carry a proof):
 *   1. Echelonise (M4RI, reduced) the rows of a random batch of points.
 *   2. If the rhs column becomes a pivot, the batch alone is inconsistent: INFEASIBLE at e
 *      (sound: any solution of the full system solves the subsystem).  The point list is
 *      written so that `verify` can rebuild and re-check it independently.
 *   3. Otherwise take the particular solution (free variables 0), evaluate lambda on all 2^k
 *      points by a Moebius (zeta) transform, and test every one of the 2^k (k+1) rows.
 *      No violation: FEASIBLE at e (the full check is the proof).  Otherwise add the points
 *      with violated rows and repeat.  A violated row is never in the span of the current
 *      consistent rows, so the rank grows every round.
 *
 * Input (stdin, one line): n k poly a6 z v_1 ... v_k   (integers; poly includes x^n).
 * Usage:
 *   emin count                      print the number of (A,C) in V x V with S(A,C) = 0
 *   emin solve EMAX MAXCOLS SEED [CERTDIR]
 *                                   test e = 0..EMAX; stop at the first feasible e, or when
 *                                   n*M(e) exceeds MAXCOLS (then e_min > last tested e)
 *   emin verify E POINTSFILE        rebuild the rows of the listed points with independent
 *                                   code (PLUQ elimination) and report consistency
 */
#include <m4ri/m4ri.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

static int n, k;
static uint64_t poly, a6, z, V[64];
static uint64_t *rowsC;   /* (k+1) rows per point, bit j = coefficient on x^j  */

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

/* splitmix64 */
static uint64_t rs;
static uint64_t rnd(void) {
  uint64_t x = (rs += 0x9E3779B97F4A7C15ULL);
  x = (x ^ (x >> 30)) * 0xBF58476D1CE4E5B9ULL;
  x = (x ^ (x >> 27)) * 0x94D049BB133111EBULL;
  return x ^ (x >> 31);
}

static void read_instance(void) {
  unsigned long long t;
  if (scanf("%d %d %llu", &n, &k, &t) != 3) { fprintf(stderr, "bad input\n"); exit(2); }
  poly = t;
  if (scanf("%llu", &t) != 1) exit(2);
  a6 = t;
  if (scanf("%llu", &t) != 1) exit(2);
  z = t;
  for (int i = 0; i < k; i++) {
    if (scanf("%llu", &t) != 1) exit(2);
    V[i] = t;
  }
  if (n > 62 || k > 30) { fprintf(stderr, "n or k too large\n"); exit(2); }
}

/* Build the k+1 trace rows per point, and count solutions of S(A,C) = 0 on the way. */
static uint64_t build_rows(void) {
  size_t P = (size_t)1 << k;
  rowsC = malloc(sizeof(uint64_t) * P * (k + 1));
  /* tr_row(w): bit j = Tr(x^j w) = XOR_i w_i Tr(x^(i+j)) */
  int trp[128];
  for (int m = 0; m < 2 * n - 1; m++) {
    uint64_t xm = 1;
    for (int s = 0; s < m; s++) xm = gmul(xm, 2);
    trp[m] = gtr(xm);
  }
  uint64_t Tcol[64];
  for (int i = 0; i < n; i++) {
    Tcol[i] = 0;
    for (int j = 0; j < n; j++)
      if (trp[i + j]) Tcol[i] |= 1ULL << j;
  }
  uint64_t vsq[64], v4[64];
  for (int i = 0; i < k; i++) { vsq[i] = gsq(V[i]); v4[i] = gsq(vsq[i]); }
  uint64_t z2 = gsq(z), nz = 0, C = 0;
  /* span[mask] = sum of V[i] over the bits of mask (as lfd_small's span) */
  uint64_t *span = malloc(sizeof(uint64_t) * P);
  span[0] = 0;
  for (size_t m = 1; m < P; m++) {
    size_t low = m & (~m + 1);
    span[m] = span[m ^ low] ^ V[__builtin_ctzll(low)];
  }
  for (size_t ci = 0; ci < P; ci++) {
    C = span[ci];
    uint64_t a = gsq(C) ^ z, b = gmul(z, C), w[64], piv[64] = {0};
    int rank = 0;
    for (int i = 0; i < k; i++) {
      w[i] = v4[i] ^ gmul(a, vsq[i]) ^ gmul(b, V[i]);
      uint64_t u = w[i];
      while (u) {
        int p = 63 - __builtin_clzll(u);
        if (piv[p]) u ^= piv[p];
        else { piv[p] = u; rank++; break; }
      }
    }
    uint64_t h = gmul(z2, gsq(C)) ^ a6, u = h;
    while (u) {
      int p = 63 - __builtin_clzll(u);
      if (piv[p]) u ^= piv[p];
      else break;
    }
    if (u == 0) nz += 1ULL << (k - rank);
    uint64_t *R = rowsC + ci * (k + 1);
    for (int i = 0; i <= k; i++) {
      uint64_t ww = i < k ? w[i] : h, r = 0;
      for (int b2 = 0; b2 < n; b2++)
        if (ww >> b2 & 1) r ^= Tcol[b2];
      R[i] = r;
    }
  }
  free(span);
  return nz;
}

static int nmons;
static uint32_t *mons;
static void make_mons(int e) {
  nmons = 0;
  size_t cap = 1;
  for (int d = 1; d <= e; d++) {
    /* C(k,d) */
    uint64_t c = 1;
    for (int i = 0; i < d; i++) c = c * (k - i) / (i + 1);
    cap += c;
  }
  free(mons);
  mons = malloc(sizeof(uint32_t) * cap);
  for (uint32_t m = 0; m < (1u << k); m++)
    if (__builtin_popcount(m) <= e) mons[nmons++] = m;
}

/* write the k+1 rows of point ci into rows r0.. of A (ncol = n*nmons, rhs in last column) */
static void put_point(mzd_t *A, rci_t r0, uint32_t ci) {
  /* monomials contained in ci */
  static int *sub = NULL;
  static int subcap = 0;
  if (subcap < nmons) { free(sub); sub = malloc(sizeof(int) * nmons); subcap = nmons; }
  int ns = 0;
  for (int t = 0; t < nmons; t++)
    if ((mons[t] & ci) == mons[t]) sub[ns++] = t;
  const uint64_t *R = rowsC + (size_t)ci * (k + 1);
  rci_t ncol = (rci_t)n * nmons;
  for (int r = 0; r <= k; r++) {
    mzd_row_clear_offset(A, r0 + r, 0);
    for (int j = 0; j < n; j++)
      if (R[r] >> j & 1)
        for (int s = 0; s < ns; s++) mzd_write_bit(A, r0 + r, j * nmons + sub[s], 1);
    if (r == k) mzd_write_bit(A, r0 + r, ncol, 1);
  }
}

static int first_bit(const mzd_t *A, rci_t r) {
  for (wi_t w = 0; w < A->width; w++) {
    word x = mzd_row((mzd_t *)A, r)[w];
    if (x) return w * m4ri_radix + __builtin_ctzll(x);
  }
  return -1;
}

static int cmp_u32(const void *a, const void *b) {
  uint32_t x = *(const uint32_t *)a, y = *(const uint32_t *)b;
  return x < y ? -1 : x > y;
}

/* returns 1 feasible, 0 infeasible; fills the points used */
static int solve_e(int e, uint64_t seed, const char *certdir, size_t *npts_used, int *rounds_out) {
  make_mons(e);
  size_t P = (size_t)1 << k;
  rci_t ncol = (rci_t)n * nmons;
  rs = seed * 0x1000193ULL + e;
  uint8_t *used = calloc(P, 1);
  uint32_t *pts = malloc(sizeof(uint32_t) * P);
  size_t npts = 0;
  /* first batch: enough points to fill ncol + 64 rows */
  size_t want = ((size_t)ncol + 64) / (k + 1) + 1;
  if (want > P) want = P;
  uint32_t *batch = malloc(sizeof(uint32_t) * P);
  size_t nb = 0;
  while (nb < want) {
    uint32_t c = (uint32_t)(rnd() & (P - 1));
    if (!used[c]) { used[c] = 1; batch[nb++] = c; }
  }
  mzd_t *E = NULL;
  rci_t rank = 0;
  int rounds = 0, result = -1;
  uint64_t *L = malloc(sizeof(uint64_t) * P);
  while (1) {
    rounds++;
    double t0 = now();
    rci_t nrows = rank + (rci_t)nb * (k + 1);
    mzd_t *A = mzd_init(nrows, ncol + 1);
    if (E) {
      for (rci_t r = 0; r < rank; r++) mzd_copy_row(A, r, E, r);
      mzd_free(E);
    }
    for (size_t b = 0; b < nb; b++) {
      put_point(A, rank + (rci_t)b * (k + 1), batch[b]);
      pts[npts++] = batch[b];
    }
    rank = mzd_echelonize_m4ri(A, 1, 0);
    int incons = 0;
    for (rci_t r = 0; r < rank; r++)
      if (first_bit(A, r) == ncol) { incons = 1; break; }
    fprintf(stderr, "  e=%d round %d: rows %d cols %d rank %d (%.1fs)%s\n", e, rounds, nrows, ncol + 1,
            rank, now() - t0, incons ? " INCONSISTENT" : "");
    if (incons) { result = 0; mzd_free(A); break; }
    /* particular solution -> ANF coefficients */
    memset(L, 0, sizeof(uint64_t) * P);
    for (rci_t r = 0; r < rank; r++) {
      if (!mzd_read_bit(A, r, ncol)) continue;
      int p = first_bit(A, r);
      int j = p / nmons, t = p % nmons;
      L[mons[t]] ^= 1ULL << j;
    }
    for (int i = 0; i < k; i++) {
      size_t bit = (size_t)1 << i;
      for (size_t c = 0; c < P; c++)
        if (c & bit) L[c] ^= L[c ^ bit];
    }
    nb = 0;
    size_t nviol = 0;
    size_t cap = (size_t)(ncol / 8 + 1024) / (k + 1) + 1;
    for (size_t c = 0; c < P; c++) {
      const uint64_t *R = rowsC + c * (k + 1);
      int bad = 0;
      for (int r = 0; r <= k; r++)
        if (par(R[r] & L[c]) != (r == k)) { bad = 1; break; }
      if (bad) {
        nviol++;
        if (used[c]) { fprintf(stderr, "violated row at a used point %zu: bug\n", c); exit(3); }
        /* reservoir-style subsample of violated points */
        if (nb < cap) batch[nb++] = (uint32_t)c;
        else {
          uint64_t q = rnd() % nviol;
          if (q < cap) batch[q] = (uint32_t)c;
        }
      }
    }
    fprintf(stderr, "  e=%d round %d: %zu violated points, adding %zu\n", e, rounds, nviol, nb);
    if (nviol == 0) {
      result = 1;
      if (certdir) {
        char fn[512];
        snprintf(fn, sizeof fn, "%s/lambda_e%d.bin", certdir, e);
        FILE *f = fopen(fn, "wb");
        fwrite(L, sizeof(uint64_t), P, f); /* lambda(C) for C = binary index, bit j = coord j */
        fclose(f);
      }
      mzd_free(A);
      break;
    }
    for (size_t b = 0; b < nb; b++) used[batch[b]] = 1;
    E = mzd_submatrix(NULL, A, 0, 0, rank, ncol + 1);
    mzd_free(A);
  }
  if (certdir && result == 0) {
    char fn[512];
    snprintf(fn, sizeof fn, "%s/points_e%d.txt", certdir, e);
    FILE *f = fopen(fn, "w");
    qsort(pts, npts, sizeof(uint32_t), cmp_u32);
    for (size_t i = 0; i < npts; i++) fprintf(f, "%u\n", pts[i]);
    fclose(f);
  }
  *npts_used = npts;
  *rounds_out = rounds;
  free(L); free(used); free(pts); free(batch);
  return result;
}

int main(int argc, char **argv) {
  if (argc < 2) { fprintf(stderr, "usage: see header\n"); return 2; }
  read_instance();
  double t0 = now();
  uint64_t nz = build_rows();
  if (!strcmp(argv[1], "count")) {
    printf("%llu\n", (unsigned long long)nz);
    return 0;
  }
  if (!strcmp(argv[1], "solve")) {
    int emax = atoi(argv[2]);
    long maxcols = atol(argv[3]);
    uint64_t seed = strtoull(argv[4], NULL, 10);
    const char *cert = argc > 5 ? argv[5] : NULL;
    if (nz) { printf("{\"error\": \"satisfiable instance\", \"solutions\": %llu}\n", (unsigned long long)nz); return 1; }
    int last_infeasible = -1, emin = -1;
    printf("{\"n\": %d, \"k\": %d, \"rows_build_s\": %.2f, \"tests\": [", n, k, now() - t0);
    for (int e = 0; e <= emax; e++) {
      make_mons(e);
      long ncol = (long)n * nmons;
      if (ncol > maxcols) break;
      size_t np;
      int rounds;
      double t1 = now();
      int r = solve_e(e, seed, cert, &np, &rounds);
      printf("%s{\"e\": %d, \"cols\": %ld, \"feasible\": %s, \"rounds\": %d, \"points_used\": %zu, \"seconds\": %.1f}",
             e ? ", " : "", e, ncol, r ? "true" : "false", rounds, np, now() - t1);
      fflush(stdout);
      if (r) { emin = e; break; }
      last_infeasible = e;
    }
    if (emin >= 0) printf("], \"e_min\": %d}\n", emin);
    else printf("], \"e_min\": null, \"e_min_gt\": %d}\n", last_infeasible);
    return 0;
  }
  if (!strcmp(argv[1], "verify")) {
    /* independent rebuild: rows from the definitions via a second code path (per-row
       construction from the field values, not from rowsC), PLUQ elimination */
    int e = atoi(argv[2]);
    FILE *f = fopen(argv[3], "r");
    uint32_t *pl = malloc(sizeof(uint32_t) << k);
    size_t np = 0;
    unsigned u;
    while (fscanf(f, "%u", &u) == 1) pl[np++] = u;
    fclose(f);
    make_mons(e);
    rci_t ncol = (rci_t)n * nmons;
    mzd_t *A = mzd_init((rci_t)np * (k + 1), ncol + 1);
    uint64_t z2 = gsq(z);
    for (size_t i = 0; i < np; i++) {
      uint64_t C = 0;
      for (int b = 0; b < k; b++)
        if (pl[i] >> b & 1) C ^= V[b];
      uint64_t a = gsq(C) ^ z, bb = gmul(z, C);
      for (int r = 0; r <= k; r++) {
        uint64_t w = r < k ? gsq(gsq(V[r])) ^ gmul(a, gsq(V[r])) ^ gmul(bb, V[r]) : gmul(z2, gsq(C)) ^ a6;
        for (int j = 0; j < n; j++) {
          uint64_t xj = 1;
          for (int s = 0; s < j; s++) xj = gmul(xj, 2);
          if (!gtr(gmul(xj, w))) continue;   /* Tr(x^j w) directly */
          for (int t = 0; t < nmons; t++)
            if ((mons[t] & pl[i]) == mons[t]) mzd_write_bit(A, (rci_t)i * (k + 1) + r, j * nmons + t, 1);
        }
        if (r == k) mzd_write_bit(A, (rci_t)i * (k + 1) + r, ncol, 1);
      }
    }
    mzd_t *B = mzd_submatrix(NULL, A, 0, 0, A->nrows, ncol);
    rci_t rb = mzd_echelonize_pluq(B, 0);
    rci_t ra = mzd_echelonize_pluq(A, 0);
    printf("{\"points\": %zu, \"rank_A\": %d, \"rank_A_b\": %d, \"inconsistent\": %s}\n", np, rb, ra,
           ra > rb ? "true" : "false");
    return 0;
  }
  return 2;
}
