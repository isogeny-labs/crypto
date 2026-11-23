/* polyclose.c -- native degree-<=D closure W_D(F) over GF(2) for an ARBITRARY Boolean polynomial
 * system (LEAD-A-bilinear-lift-size-degree, research/leads_20261111/lane_A_falldegree.md, Lead A5).
 *
 * Same closure as research/lfd_lower_bound_20261111/lfd_small.py `closure` (and the A3 tool
 * research/leads_20261111/runs/A_fall_effective_variable/fallclose.c, whose M4RI batch/RREF core
 * insert_batch() is copied here unchanged): W_D = least subspace of B_{<=D} (B = multilinear
 * Boolean ring, x_i^2 = x_i) containing F and closed under f -> x_i f for f in W_D cap B_{<D},
 * all i in 0..NV-1.  The difference from fallclose.c: the generators are read from stdin as an
 * explicit list of monomials, and monomials are index lists (NV up to 1000), not 64-bit masks,
 * so lifted systems with N + k^2 = 80..99 variables, and the Lemma 2 lift (298 variables at
 * n = 11), fit.
 *
 * Input (stdin, whitespace separated):
 *   NV NP
 *   NP times:  T  then T monomials, each  d i_1 .. i_d   (indices < NV; repeats are reduced
 *              x_i^2 = x_i, equal monomials cancel mod 2; d = 0 is the constant 1)
 * Usage: polyclose D [BATCH=8192] [FULL=0] [ROWCAP=0]
 *   FULL=0 stops as soon as 1 is in W (a fall); FULL=1 runs to the fixed point.
 *   ROWCAP>0 allocates W with at most ROWCAP rows (memory cap); exceeding it aborts with exit 4.
 * Output: one JSON line: one_in, W_dims (cumulative dims of W cap B_{<=e}, e=0..D), ncols, ...
 */
#include <m4ri/m4ri.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/resource.h>

#define MAXD 8
#define MAXNV 1000
static int NV, D;
static long long Bin[MAXNV + 1][MAXD + 2];
static int ncols, nhigh, nlow;
static uint16_t *mons; /* column -> MAXD sorted indices */
static uint8_t *mdeg;  /* column -> degree */
static int off[MAXD + 1];
static int32_t *multab;

static double now(void) {
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return ts.tv_sec + 1e-9 * ts.tv_nsec;
}
static int colidx_sorted(const int *c, int d) {
  long long r = 0;
  for (int j = 0; j < d; j++) r += Bin[c[j]][j + 1];
  return off[d] + (int)r;
}

/* ---- W in RREF (core copied from A3 fallclose.c) ---- */
static mzd_t *W;
static int r = 0, *piv, *ispiv;
static word **queue;
static int qn = 0, qcap = 0, qhead = 0;
static int lw;
static int rowcap;

static void queue_push_row(word const *row) {
  if (qn == qcap) {
    qcap = qcap ? 2 * qcap : 1024;
    queue = realloc(queue, sizeof(word *) * qcap);
  }
  word *q = calloc(lw, sizeof(word));
  for (int c = 0; c < nlow; c++) {
    int col = nhigh + c;
    if (row[col >> 6] >> (col & 63) & 1) q[c >> 6] |= (word)1 << (c & 63);
  }
  queue[qn++] = q;
}

static int first_bit(word const *row, int wpr) {
  for (int w = 0; w < wpr; w++)
    if (row[w]) return w * 64 + __builtin_ctzll(row[w]);
  return -1;
}

static void insert_batch(mzd_t *B) {
  int nb = B->nrows;
  if (r > 0) {
    mzd_t *T = mzd_transpose(NULL, B);
    mzd_t *BPt = mzd_init(r, nb);
    for (int j = 0; j < r; j++) memcpy(mzd_row(BPt, j), mzd_row(T, piv[j]), sizeof(word) * T->width);
    mzd_free(T);
    mzd_t *BP = mzd_transpose(NULL, BPt);
    mzd_free(BPt);
    mzd_t *Wwin = mzd_init_window(W, 0, 0, r, ncols);
    mzd_addmul(B, BP, Wwin, 0);
    mzd_free_window(Wwin);
    mzd_free(BP);
  }
  int rk = mzd_echelonize_m4ri(B, 1, 0);
  if (rk == 0) return;
  if (r + rk > rowcap) { fprintf(stderr, "ROWCAP exceeded (r=%d + rk=%d > %d)\n", r, rk, rowcap); exit(4); }
  int *pv = malloc(sizeof(int) * rk);
  for (int i = 0; i < rk; i++) {
    pv[i] = first_bit(mzd_row(B, i), B->width);
    if (pv[i] < 0 || ispiv[pv[i]] >= 0) { fprintf(stderr, "internal: bad pivot\n"); exit(3); }
  }
  mzd_t *Bn = mzd_init_window(B, 0, 0, rk, ncols);
  if (r > 0) {
    mzd_t *G = mzd_init(r, rk);
    for (int j = 0; j < r; j++) {
      word const *wr = mzd_row(W, j);
      for (int i = 0; i < rk; i++)
        if (wr[pv[i] >> 6] >> (pv[i] & 63) & 1) mzd_write_bit(G, j, i, 1);
    }
    mzd_t *Wwin = mzd_init_window(W, 0, 0, r, ncols);
    mzd_addmul(Wwin, G, Bn, 0);
    mzd_free_window(Wwin);
    mzd_free(G);
  }
  for (int i = 0; i < rk; i++) {
    memcpy(mzd_row(W, r), mzd_row(Bn, i), sizeof(word) * W->width);
    piv[r] = pv[i];
    ispiv[pv[i]] = r;
    if (pv[i] >= nhigh) queue_push_row(mzd_row(W, r));
    r++;
  }
  mzd_free_window(Bn);
  free(pv);
}

static void dims(int *cum) {
  int cnt[MAXD + 1] = {0};
  for (int j = 0; j < r; j++) cnt[mdeg[piv[j]]]++;
  int s = 0;
  for (int e = 0; e <= D; e++) { s += cnt[e]; cum[e] = s; }
}

static int cmpint(const void *a, const void *b) { return *(const int *)a - *(const int *)b; }

int main(int argc, char **argv) {
  if (argc < 2) { fprintf(stderr, "usage: polyclose D [BATCH] [FULL] [ROWCAP]\n"); return 2; }
  D = atoi(argv[1]);
  int BS = argc > 2 ? atoi(argv[2]) : 8192;
  int full = argc > 3 ? atoi(argv[3]) : 0;
  rowcap = argc > 4 ? atoi(argv[4]) : 0;
  int NP;
  if (scanf("%d %d", &NV, &NP) != 2) return 2;
  if (NV < 1 || NV > MAXNV || D < 1 || D > MAXD) { fprintf(stderr, "size\n"); return 2; }
  for (int a = 0; a <= MAXNV; a++)
    for (int b = 0; b < MAXD + 2; b++) Bin[a][b] = b == 0 ? 1 : (a == 0 ? 0 : Bin[a - 1][b - 1] + Bin[a - 1][b]);
  long long nc = 0;
  for (int d = D; d >= 0; d--) { off[d] = (int)nc; nc += Bin[NV][d]; }
  if (nc > 2000000000LL) { fprintf(stderr, "too many columns\n"); return 2; }
  ncols = (int)nc;
  nhigh = (int)Bin[NV][D];
  nlow = ncols - nhigh;
  lw = (nlow + 63) / 64;
  if (rowcap <= 0 || rowcap > ncols) rowcap = ncols;
  mons = calloc((size_t)ncols * MAXD, sizeof(uint16_t));
  mdeg = calloc(ncols, 1);
  /* enumerate combinations of each degree, place at their colex rank */
  for (int d = 0; d <= D; d++) {
    int c[MAXD];
    for (int j = 0; j < d; j++) c[j] = j;
    for (;;) {
      int col = colidx_sorted(c, d);
      mdeg[col] = d;
      for (int j = 0; j < d; j++) mons[(size_t)col * MAXD + j] = c[j];
      if (d == 0) break;
      int j = d - 1;
      while (j >= 0 && c[j] == NV - d + j) j--;
      if (j < 0) break;
      c[j]++;
      for (int t = j + 1; t < d; t++) c[t] = c[t - 1] + 1;
    }
  }
  multab = malloc(sizeof(int32_t) * (size_t)nlow * NV);
  for (int cc = 0; cc < nlow; cc++) {
    int col = nhigh + cc, d = mdeg[col];
    const uint16_t *m = mons + (size_t)col * MAXD;
    for (int i = 0; i < NV; i++) {
      int t[MAXD + 1], tn = 0, ins = 0, dup = 0;
      for (int j = 0; j < d; j++) {
        if (!ins && i < m[j]) { t[tn++] = i; ins = 1; }
        if (i == m[j]) dup = 1;
        t[tn++] = m[j];
      }
      if (dup) { multab[(size_t)cc * NV + i] = col; continue; }
      if (!ins) t[tn++] = i;
      multab[(size_t)cc * NV + i] = colidx_sorted(t, tn);
    }
  }

  double t0 = now();
  W = mzd_init(rowcap, ncols);
  piv = malloc(sizeof(int) * ncols);
  ispiv = malloc(sizeof(int) * ncols);
  for (int c = 0; c < ncols; c++) ispiv[c] = -1;

  /* generators */
  mzd_t *B = mzd_init(NP, ncols);
  int maxgdeg = 0, skipped = 0;
  for (int p = 0; p < NP; p++) {
    int T;
    if (scanf("%d", &T) != 1) return 2;
    int pdeg = 0;
    for (int t = 0; t < T; t++) {
      int d, idx[64];
      if (scanf("%d", &d) != 1 || d < 0 || d > 64) return 2;
      for (int j = 0; j < d; j++) {
        if (scanf("%d", &idx[j]) != 1 || idx[j] < 0 || idx[j] >= NV) return 2;
      }
      qsort(idx, d, sizeof(int), cmpint);
      int u = 0;
      for (int j = 0; j < d; j++)
        if (u == 0 || idx[u - 1] != idx[j]) idx[u++] = idx[j];
      if (u > pdeg) pdeg = u;
      if (u > D) { pdeg = 99; continue; }
      int col = colidx_sorted(idx, u);
      mzd_write_bit(B, p, col, mzd_read_bit(B, p, col) ^ 1);
    }
    if (pdeg > D) { /* generator of degree > D is not in B_{<=D}: drop it, as lfd_small.closure */
      mzd_row_clear_offset(B, p, 0);
      skipped++;
    } else if (pdeg > maxgdeg) maxgdeg = pdeg;
  }
  insert_batch(B);
  mzd_free(B);
  int gen_rank = r;
  long long products = 0;
  int batches = 0;
  int one_col = off[0];
  int per = BS / NV;
  if (per < 1) per = 1;
  while (qhead < qn && (full || ispiv[one_col] < 0)) {
    int take = qn - qhead < per ? qn - qhead : per;
    mzd_t *Bt = mzd_init(take * NV, ncols);
    for (int q = 0; q < take; q++) {
      word *f = queue[qhead + q];
      for (int w = 0; w < lw; w++) {
        word x = f[w];
        while (x) {
          int c = w * 64 + __builtin_ctzll(x);
          x &= x - 1;
          int32_t *mt = multab + (size_t)c * NV;
          for (int i = 0; i < NV; i++) {
            word *row = mzd_row(Bt, q * NV + i);
            row[mt[i] >> 6] ^= (word)1 << (mt[i] & 63);
          }
        }
      }
      free(f);
      queue[qhead + q] = NULL;
    }
    qhead += take;
    products += (long long)take * NV;
    insert_batch(Bt);
    mzd_free(Bt);
    batches++;
    int cum[MAXD + 1];
    dims(cum);
    fprintf(stderr, "[%.1fs] batch %d products %lld rank %d queue %d low %d one %d\n", now() - t0, batches, products, r,
            qn - qhead, cum[D - 1], ispiv[one_col] >= 0);
  }
  int cum[MAXD + 1];
  dims(cum);
  struct rusage ru;
  getrusage(RUSAGE_SELF, &ru);
  printf("{\"NV\": %d, \"NP\": %d, \"D\": %d, \"one_in\": %d, \"W_dims\": [", NV, NP, D, ispiv[one_col] >= 0);
  for (int e = 0; e <= D; e++) printf("%s%d", e ? ", " : "", cum[e]);
  printf("], \"ncols\": %d, \"codim\": %d, \"gen_rank\": %d, \"gens_skipped_deg_gt_D\": %d, \"products\": %lld, "
         "\"batches\": %d, \"stopped_early\": %d, \"full\": %d, \"sec\": %.1f, \"maxrss_kb\": %ld}\n",
         ncols, ncols - r, gen_rank, skipped, products, batches, (!full && ispiv[one_col] >= 0 && qhead < qn), full,
         now() - t0, ru.ru_maxrss);
  return 0;
}
