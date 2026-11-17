/* iclose.c -- independently coded degree-<=D closure W_D(F) over GF(2) (closure-falls-crosscheck, 2026-11-12).
 *
 * Second implementation of the closure of research/lfd_lower_bound_20261111/lfd_small.py `closure`:
 *   W_D = least subspace of B_{<=D} (Boolean multilinear ring in N variables) containing F and closed
 *   under f -> x_i f for f in W_D cap B_{<D}.
 * It shares NO code with ../fallclose.c and does not use M4RI:
 *   - input is a polynomial system file (gen_system.py: independent Weil descent by evaluation);
 *   - columns: degree D first ... degree 0 last (graded), but WITHIN a degree the monomials are put in a
 *     seeded pseudo-random order (seed 0 = lexicographic), looked up through a hash table;
 *   - W is kept in SEMI-echelon form (each row's pivot = its first column; rows are never back-reduced),
 *     not RREF.  With a graded column order a row whose pivot has degree < D lies entirely in B_{<D}, and
 *     these rows span W cap B_{<D}; likewise the rows with pivot degree <= e span W cap B_{<=e};
 *   - reduction uses own 8-bit Gray tables ("Four Russians" style), several groups per pass, OpenMP over rows;
 *   - the work list of low rows is processed LIFO by default (FIFO with ORDER=1).
 * The least fixed point does not depend on the order, so W_dims and one_in must agree with fallclose.
 * Usage: iclose D SYSFILE [BATCH=8192] [FULL=0] [SEED=1] [ORDER=0]
 *   FULL=0 stops when 1 in W; FULL=1 runs to the fixed point.  Output: one JSON line.
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/resource.h>
#ifdef _OPENMP
#include <omp.h>
#endif

typedef uint64_t u64;
static int N, D, ncols, nhigh, WW; /* WW = words per row */
static u64 *mon;                   /* col -> monomial */
static int *cdeg;                  /* col -> degree */
static u64 *hkey;
static int *hval;
static u64 hmask;

static double wall(void) {
  struct timespec t;
  clock_gettime(CLOCK_MONOTONIC, &t);
  return t.tv_sec + 1e-9 * t.tv_nsec;
}
static u64 hh(u64 x) {
  x ^= x >> 33; x *= 0xff51afd7ed558ccdULL; x ^= x >> 33; x *= 0xc4ceb9fe1a85ec53ULL; x ^= x >> 33;
  return x;
}
static void hput(u64 m, int c) {
  u64 h = hh(m) & hmask;
  while (hval[h] >= 0) h = (h + 1) & hmask;
  hkey[h] = m; hval[h] = c;
}
static int hget(u64 m) {
  u64 h = hh(m) & hmask;
  while (hval[h] >= 0) {
    if (hkey[h] == m) return hval[h];
    h = (h + 1) & hmask;
  }
  return -1;
}
static u64 rs;
static u64 rnd(void) { rs ^= rs << 13; rs ^= rs >> 7; rs ^= rs << 17; return rs; }

/* enumerate degree-d monomials in lex order of their sorted index tuples */
static u64 *tmpm; static int tmpn;
static void gen(int start, int left, u64 m) {
  if (left == 0) { tmpm[tmpn++] = m; return; }
  for (int i = start; i <= N - left; i++) gen(i + 1, left - 1, m | (1ULL << i));
}

#define GETB(row, c) (((row)[(c) >> 6] >> ((c) & 63)) & 1)
#define FLIPB(row, c) ((row)[(c) >> 6] ^= 1ULL << ((c) & 63))

static u64 *Wm;           /* rank rows x WW */
static int rank_ = 0;
static int *pivrow;       /* col -> row or -1 */
static int *lowmul;       /* (col-nhigh)*N + i -> col of mon*x_i */
static int nthreads = 1;

static int lead(const u64 *r) {
  for (int w = 0; w < WW; w++)
    if (r[w]) return w * 64 + __builtin_ctzll(r[w]);
  return -1;
}
static void xor_from(u64 *a, const u64 *b, int w0) {
  for (int w = w0; w < WW; w++) a[w] ^= b[w];
}

/* --- reduce rows R[0..nr) (stride WW) against a list of pivot rows, given as pivot columns pc[0..np)
 *     (strictly increasing) with rows prow[j]; rows prow are semi-echelon with lead pc[j].
 *     Tables: groups of 8 pivots, NT groups per pass. --- */
#define NT 8
static u64 *tabs; /* NT * 256 * WW */
static void reduce_rows(u64 *R, int nr, const int *pc, u64 *const *prow, int np) {
  u64 *T8 = malloc(sizeof(u64) * 8 * WW);
  for (int g0 = 0; g0 < np; g0 += 8 * NT) {
    int ng = 0, gc[NT][8], gt[NT], gw[NT];
    for (int g = 0; g < NT && g0 + 8 * g < np; g++, ng++) {
      int b = g0 + 8 * g, t = np - b < 8 ? np - b : 8;
      gt[g] = t;
      gw[g] = pc[b] >> 6;
      for (int j = 0; j < t; j++) { gc[g][j] = pc[b + j]; memcpy(T8 + (size_t)j * WW, prow[b + j], sizeof(u64) * WW); }
      /* local reduction: T8[j] gets zeros at the other group pivots (descending j) */
      for (int j = t - 1; j >= 0; j--)
        for (int jj = j + 1; jj < t; jj++)
          if (GETB(T8 + (size_t)j * WW, gc[g][jj])) xor_from(T8 + (size_t)j * WW, T8 + (size_t)jj * WW, gw[g]);
      u64 *tb = tabs + (size_t)g * 256 * WW;
      memset(tb, 0, sizeof(u64) * WW);
      for (int m = 1; m < (1 << t); m++) {
        int lo = __builtin_ctz(m);
        u64 *dst = tb + (size_t)m * WW;
        const u64 *a = tb + (size_t)(m & (m - 1)) * WW, *bb = T8 + (size_t)lo * WW;
        for (int w = gw[g]; w < WW; w++) dst[w] = a[w] ^ bb[w];
      }
    }
#pragma omp parallel for schedule(dynamic, 16) num_threads(nthreads)
    for (int r = 0; r < nr; r++) {
      u64 *row = R + (size_t)r * WW;
      for (int g = 0; g < ng; g++) {
        int idx = 0;
        for (int j = 0; j < gt[g]; j++) idx |= (int)GETB(row, gc[g][j]) << j;
        if (idx) xor_from(row, tabs + ((size_t)g * 256 + idx) * WW, gw[g]);
      }
    }
  }
  free(T8);
}

/* queue of low polynomials (sparse: list of columns) */
typedef struct { int n; int *c; } spoly;
static spoly *Q; static long qn = 0, qcap = 0;
static long queued_total = 0;

static void add_row_to_W(const u64 *r, int p) {
  memcpy(Wm + (size_t)rank_ * WW, r, sizeof(u64) * WW);
  pivrow[p] = rank_++;
  if (p >= nhigh) { /* whole row has degree < D: queue it */
    int cnt = 0;
    for (int w = nhigh >> 6; w < WW; w++) cnt += __builtin_popcountll(r[w]);
    spoly s; s.n = 0; s.c = malloc(sizeof(int) * (cnt ? cnt : 1));
    for (int w = 0; w < WW; w++) {
      u64 x = r[w];
      while (x) { int c = w * 64 + __builtin_ctzll(x); x &= x - 1;
        if (c < nhigh) { fprintf(stderr, "internal: low-lead row has high column\n"); exit(3); }
        s.c[s.n++] = c; }
    }
    if (qn == qcap) { qcap = qcap ? 2 * qcap : 4096; Q = realloc(Q, sizeof(spoly) * qcap); }
    Q[qn++] = s; queued_total++;
  }
}

/* insert a batch: reduce against W, then echelonise inside the batch with 8-row groups */
static void insert_batch(u64 *B, int nb) {
  /* pivot list of W in column order */
  int *pc = malloc(sizeof(int) * (rank_ + 1));
  u64 **prow = malloc(sizeof(u64 *) * (rank_ + 1));
  int np = 0;
  for (int c = 0; c < ncols; c++)
    if (pivrow[c] >= 0) { pc[np] = c; prow[np] = Wm + (size_t)pivrow[c] * WW; np++; }
  if (np) reduce_rows(B, nb, pc, prow, np);
  free(pc); free(prow);
  /* echelonise B */
  int pos = 0;
  while (pos < nb) {
    int gp[8], gr[8], t = 0;
    while (pos < nb && t < 8) {
      u64 *row = B + (size_t)pos * WW;
      for (int j = 0; j < t; j++)
        if (GETB(row, gp[j])) xor_from(row, B + (size_t)gr[j] * WW, gp[j] >> 6);
      int p = lead(row);
      if (p >= 0) {
        if (pivrow[p] >= 0) { fprintf(stderr, "internal: lead on W pivot\n"); exit(3); }
        for (int j = 0; j < t; j++)
          if (GETB(B + (size_t)gr[j] * WW, p)) xor_from(B + (size_t)gr[j] * WW, row, p >> 6);
        gp[t] = p; gr[t] = pos; t++;
      }
      pos++;
    }
    if (t == 0) break;
    /* sort group by pivot column (needed by reduce_rows) */
    for (int a = 1; a < t; a++)
      for (int b = a; b > 0 && gp[b - 1] > gp[b]; b--) {
        int x = gp[b]; gp[b] = gp[b - 1]; gp[b - 1] = x; x = gr[b]; gr[b] = gr[b - 1]; gr[b - 1] = x;
      }
    u64 *pr[8];
    for (int j = 0; j < t; j++) pr[j] = B + (size_t)gr[j] * WW;
    if (pos < nb) reduce_rows(B + (size_t)pos * WW, nb - pos, gp, pr, t);
    for (int j = 0; j < t; j++) add_row_to_W(pr[j], gp[j]);
  }
}

int main(int argc, char **argv) {
  if (argc < 3) { fprintf(stderr, "usage: iclose D SYSFILE [BATCH] [FULL] [SEED] [ORDER]\n"); return 2; }
  D = atoi(argv[1]);
  int BS = argc > 3 ? atoi(argv[3]) : 8192, full = argc > 4 ? atoi(argv[4]) : 0;
  u64 seed = argc > 5 ? strtoull(argv[5], 0, 10) : 1;
  int fifo = argc > 6 ? atoi(argv[6]) : 0;
#ifdef _OPENMP
  nthreads = omp_get_max_threads();
#endif
  FILE *fp = fopen(argv[2], "r");
  if (!fp) { perror("sysfile"); return 2; }
  int n;
  if (fscanf(fp, "%d %d", &N, &n) != 2 || N < 1 || N > 62 || D < 2 || D > 8) { fprintf(stderr, "bad input\n"); return 2; }
  /* columns */
  long tot = 0, cntd[10];
  for (int d = 0; d <= D; d++) { long c = 1; for (int j = 0; j < d; j++) c = c * (N - j) / (j + 1); cntd[d] = c; tot += c; }
  ncols = (int)tot; nhigh = (int)cntd[D]; WW = (ncols + 63) / 64;
  mon = malloc(sizeof(u64) * ncols); cdeg = malloc(sizeof(int) * ncols);
  u64 hs = 1; while (hs < 4 * (u64)ncols) hs <<= 1;
  hmask = hs - 1; hkey = malloc(sizeof(u64) * hs); hval = malloc(sizeof(int) * hs);
  for (u64 i = 0; i < hs; i++) hval[i] = -1;
  rs = seed * 0x9E3779B97F4A7C15ULL + 12345;
  int col = 0;
  for (int d = D; d >= 0; d--) {
    tmpm = malloc(sizeof(u64) * cntd[d]); tmpn = 0;
    gen(0, d, 0);
    if (seed)
      for (int i = tmpn - 1; i > 0; i--) { int j = (int)(rnd() % (u64)(i + 1)); u64 x = tmpm[i]; tmpm[i] = tmpm[j]; tmpm[j] = x; }
    for (int i = 0; i < tmpn; i++) { mon[col] = tmpm[i]; cdeg[col] = d; hput(tmpm[i], col); col++; }
    free(tmpm);
  }
  int one_col = hget(0);
  lowmul = malloc(sizeof(int) * (size_t)(ncols - nhigh) * N);
  for (int c = nhigh; c < ncols; c++)
    for (int i = 0; i < N; i++) lowmul[(size_t)(c - nhigh) * N + i] = hget(mon[c] | (1ULL << i));
  Wm = malloc(sizeof(u64) * (size_t)ncols * WW);
  pivrow = malloc(sizeof(int) * ncols);
  for (int c = 0; c < ncols; c++) pivrow[c] = -1;
  tabs = malloc(sizeof(u64) * (size_t)NT * 256 * WW);
  double t0 = wall();
  /* generators */
  u64 *G = calloc((size_t)n * WW, sizeof(u64));
  for (int b = 0; b < n; b++) {
    int cnt; if (fscanf(fp, "%d", &cnt) != 1) return 2;
    for (int j = 0; j < cnt; j++) {
      unsigned long long m; if (fscanf(fp, "%llu", &m) != 1) return 2;
      int c = hget((u64)m);
      if (c < 0) { fprintf(stderr, "monomial outside B_<=D\n"); return 2; }
      FLIPB(G + (size_t)b * WW, c);
    }
  }
  fclose(fp);
  insert_batch(G, n); free(G);
  int gen_rank = rank_;
  int per = BS / N; if (per < 1) per = 1;
  long products = 0; int batches = 0;
  long qhead = 0; /* FIFO pointer; LIFO pops from the end */
  while ((fifo ? qhead < qn : qn > 0) && (full || pivrow[one_col] < 0)) {
    int avail = (int)(fifo ? qn - qhead : qn), take = avail < per ? avail : per;
    spoly *src = malloc(sizeof(spoly) * take);
    if (fifo) { memcpy(src, Q + qhead, sizeof(spoly) * take); qhead += take; }
    else { qn -= take; memcpy(src, Q + qn, sizeof(spoly) * take); }
    u64 *B = calloc((size_t)take * N * WW, sizeof(u64));
    for (int q = 0; q < take; q++) {
      for (int a = 0; a < src[q].n; a++) {
        const int *lm = lowmul + (size_t)(src[q].c[a] - nhigh) * N;
        for (int i = 0; i < N; i++) FLIPB(B + ((size_t)q * N + i) * WW, lm[i]);
      }
      free(src[q].c);
    }
    free(src);
    if (fifo && qhead == qn) { qhead = qn = 0; }
    insert_batch(B, take * N);
    free(B);
    products += (long)take * N; batches++;
    fprintf(stderr, "[%.1fs] batch %d products %ld rank %d pending %ld one %d\n", wall() - t0, batches, products, rank_,
            fifo ? qn - qhead : qn, pivrow[one_col] >= 0);
  }
  long cnt[10] = {0};
  for (int c = 0; c < ncols; c++) if (pivrow[c] >= 0) cnt[cdeg[c]]++;
  struct rusage ru; getrusage(RUSAGE_SELF, &ru);
  printf("{\"tool\": \"iclose\", \"N\": %d, \"n_eq\": %d, \"D\": %d, \"seed\": %llu, \"order\": \"%s\", \"one_in\": %d, \"W_dims\": [",
         N, n, D, (unsigned long long)seed, fifo ? "fifo" : "lifo", pivrow[one_col] >= 0);
  long s = 0;
  for (int e = 0; e <= D; e++) { s += cnt[e]; printf("%s%ld", e ? ", " : "", s); }
  printf("], \"ncols\": %d, \"rank\": %d, \"codim\": %d, \"gen_rank\": %d, \"products\": %ld, \"batches\": %d, "
         "\"low_rows_queued\": %ld, \"threads\": %d, \"sec\": %.1f, \"maxrss_kb\": %ld}\n",
         ncols, rank_, ncols - rank_, gen_rank, products, batches, queued_total, nthreads, wall() - t0, ru.ru_maxrss);
  return 0;
}
