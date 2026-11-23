/* fallclose_q.c -- memory-lean variant of fallclose.c (compressed RREF, non-pivot columns only);
 * same closure, same output.  Original header follows.
 * fallclose.c -- native degree-<=D closure W_D(F) over GF(2) for the binary m=2 descent
 * (LEAD-A-fall-effective-variable-law, research/leads_20261111/lane_A_falldegree.md).
 *
 * System (same as research/lfd_lower_bound_20261111/lfd_small.py, descend(.., 'ab')):
 *   S(a,b) = X1^2 X2^2 + z^2 (X1^2 + X2^2) + z X1 X2 + a6,  X1 = sum a_i v_i, X2 = sum b_j v_j,
 *   variables a_0..a_{k-1} = x_0..x_{k-1}, b_0..b_{k-1} = x_k..x_{2k-1}, N = 2k.
 *   Built symbolically (no 2^N enumeration):
 *     coeff(a_i b_j) = v_i^2 v_j^2 + z v_i v_j,  coeff(a_i) = z^2 v_i^2,  coeff(b_j) = z^2 v_j^2,
 *     constant a6; F = the n bit-coordinates (Boolean multilinear ring B).
 * Closure (lfd_small.closure): W_D = least subspace of B_{<=D} containing F and closed under
 *   f -> x_i f for f in W_D cap B_{<D}.  Order-independent least fixed point.
 * Method: columns ordered degree D first, constant last; W kept in reduced row-echelon form
 *   (M4RI), so the rows whose pivot has degree < D span W cap B_{<D}.  New products are added
 *   in batches: B += B[:,P] W (reduce), echelonise B, back-reduce W on the new pivots, append.
 *   Every new row with a low pivot is queued for multiplication by x_0..x_{N-1} (semi-naive;
 *   the span of queued rows equals W cap B_{<D}).
 * Input (stdin, one line, as emin.c): n k poly a6 z v_1 ... v_k
 * Usage: fallclose D [BATCH=8192] [FULL=0]
 *   FULL=0 stops as soon as 1 is in W (a fall); FULL=1 runs to the fixed point;
 *   FULL=2 prints the degree filtration of span(F) itself and exits.
 * Output: one JSON line: one_in, W_dims (cumulative dims of W cap B_{<=e}, e=0..D), ncols, ...
 */
#include <m4ri/m4ri.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/resource.h>

static int n, k, N, D;
static uint64_t poly, a6, z, V[64];
static long long Bin[70][12];
static int ncols, nhigh, nlow; /* nhigh = C(N,D) columns of degree D, first */
static uint64_t *mons;         /* column -> monomial mask */
static int off[12];            /* first column of each degree */
static int32_t *multab;        /* low col c (0..nlow-1), var i -> full column of m|x_i */

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
static double now(void) {
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return ts.tv_sec + 1e-9 * ts.tv_nsec;
}
static int colidx(uint64_t m) {
  int d = __builtin_popcountll(m), j = 0;
  long long r = 0;
  while (m) {
    int c = __builtin_ctzll(m);
    m &= m - 1;
    r += Bin[c][j + 1];
    j++;
  }
  return off[d] + (int)r;
}

/* ---- W in RREF, stored compressed: only the non-pivot columns Q (sorted) are kept ----
 * Row j of W (pivot piv[j]) is e_{piv[j]} + sum_c WQ[j][c] e_{Q[c]}; all other pivot columns are 0 (RREF).
 * Memory r*(ncols-r) bits instead of ncols^2 (fallclose_q, LEAD-A-closure-dimension-laws). */
static mzd_t *WQ = NULL;      /* r x q */
static int *Q, q;             /* alive (non-pivot) columns, ascending */
static int r = 0, *piv, *ispiv; /* ispiv[col] = row or -1 */
static word **queue;
static int qn = 0, qcap = 0, qhead = 0;
static int lw; /* words per low row */

static void queue_push_low(int pcol, word const *qrow, int const *Qold, int qold) {
  if (qn == qcap) {
    qcap = qcap ? 2 * qcap : 1024;
    queue = realloc(queue, sizeof(word *) * qcap);
  }
  word *v = calloc(lw, sizeof(word));
  (void)pcol; /* the Q-row already carries its own pivot bit (pivot columns are dropped only after this) */
  for (int c = 0; c < qold; c++)
    if (qrow[c >> 6] >> (c & 63) & 1) {
      int col = Qold[c];
      if (col < nhigh) { fprintf(stderr, "internal: low pivot row with high bit\n"); exit(3); }
      int lc = col - nhigh;
      v[lc >> 6] ^= (word)1 << (lc & 63);
    }
  queue[qn++] = v;
}

static int first_bit(word const *row, int wpr) {
  for (int w = 0; w < wpr; w++)
    if (row[w]) return w * 64 + __builtin_ctzll(row[w]);
  return -1;
}

/* gather rows idx[0..m-1] of A into a new m x A->ncols matrix */
static mzd_t *gather_rows(mzd_t const *A, int const *idx, int m) {
  mzd_t *R = mzd_init(m, A->ncols);
  for (int j = 0; j < m; j++) memcpy(mzd_row(R, j), mzd_row((mzd_t *)A, idx[j]), sizeof(word) * A->width);
  return R;
}

static void insert_batch(mzd_t *B) { /* B: nb x ncols (full coordinates) */
  if (q == 0) return; /* W = B_{<=D} already */
  mzd_t *BQ;
  if (r > 0) {
    mzd_t *T = mzd_transpose(NULL, B); /* ncols x nb */
    mzd_t *BPt = gather_rows(T, piv, r);
    mzd_t *BQt = gather_rows(T, Q, q);
    mzd_free(T);
    mzd_t *BP = mzd_transpose(NULL, BPt); mzd_free(BPt);
    BQ = mzd_transpose(NULL, BQt); mzd_free(BQt);
    mzd_addmul(BQ, BP, WQ, 0);
    mzd_free(BP);
  } else {
    BQ = mzd_copy(NULL, B); /* Q = all columns, in order */
  }
  int rk = mzd_echelonize_m4ri(BQ, 1, 0);
  if (rk == 0) { mzd_free(BQ); return; }
  int *pv = malloc(sizeof(int) * rk);
  for (int i = 0; i < rk; i++) {
    pv[i] = first_bit(mzd_row(BQ, i), BQ->width);
    if (pv[i] < 0 || ispiv[Q[pv[i]]] >= 0) { fprintf(stderr, "internal: bad pivot\n"); exit(3); }
  }
  mzd_t *Bn = mzd_submatrix(NULL, BQ, 0, 0, rk, q);
  mzd_free(BQ);
  if (r > 0) { /* back-reduce old rows on the new pivots */
    mzd_t *G = mzd_init(r, rk);
    for (int j = 0; j < r; j++) {
      word const *wr = mzd_row(WQ, j);
      for (int i = 0; i < rk; i++)
        if (wr[pv[i] >> 6] >> (pv[i] & 63) & 1) mzd_write_bit(G, j, i, 1);
    }
    mzd_addmul(WQ, G, Bn, 0);
    mzd_free(G);
  }
  /* register new pivots, queue low ones (snapshot in full low coordinates) */
  for (int i = 0; i < rk; i++) {
    int col = Q[pv[i]];
    piv[r + i] = col;
    ispiv[col] = r + i;
    if (col >= nhigh) queue_push_low(col, mzd_row(Bn, i), Q, q);
  }
  /* stack and drop the new pivot columns */
  mzd_t *S = r > 0 ? mzd_stack(NULL, WQ, Bn) : mzd_copy(NULL, Bn);
  if (WQ) mzd_free(WQ);
  mzd_free(Bn);
  char *dead = calloc(q, 1);
  for (int i = 0; i < rk; i++) dead[pv[i]] = 1;
  int *keep = malloc(sizeof(int) * q), nq = 0;
  for (int c = 0; c < q; c++)
    if (!dead[c]) keep[nq++] = c;
  r += rk;
  if (nq > 0) {
    mzd_t *St = mzd_transpose(NULL, S); mzd_free(S); /* q x r */
    mzd_t *Kt = gather_rows(St, keep, nq); mzd_free(St);
    WQ = mzd_transpose(NULL, Kt); mzd_free(Kt);
  } else {
    mzd_free(S);
    WQ = mzd_init(r, 1); /* placeholder, q = 0 */
  }
  int *Qn = malloc(sizeof(int) * (nq > 0 ? nq : 1));
  for (int c = 0; c < nq; c++) Qn[c] = Q[keep[c]];
  free(Q); Q = Qn; q = nq;
  free(keep); free(dead); free(pv);
}

static void dims(int *cum) {
  int cnt[12] = {0};
  for (int j = 0; j < r; j++) cnt[__builtin_popcountll(mons[piv[j]])]++;
  int s = 0;
  for (int e = 0; e <= D; e++) { s += cnt[e]; cum[e] = s; }
}

int main(int argc, char **argv) {
  if (argc < 2) { fprintf(stderr, "usage: fallclose D [BATCH] [FULL]\n"); return 2; }
  D = atoi(argv[1]);
  int BS = argc > 2 ? atoi(argv[2]) : 8192;
  int full = argc > 3 ? atoi(argv[3]) : 0;
  unsigned long long t;
  if (scanf("%d %d %llu %llu", &n, &k, &t, (unsigned long long *)&a6) != 4) return 2;
  poly = t;
  if (scanf("%llu", &t) != 1) return 2;
  z = t;
  for (int i = 0; i < k; i++) {
    if (scanf("%llu", &t) != 1) return 2;
    V[i] = t;
  }
  N = 2 * k;
  if (N > 62 || D > 10 || D < 1) { fprintf(stderr, "size\n"); return 2; }
  for (int a = 0; a < 70; a++)
    for (int b = 0; b < 12; b++) Bin[a][b] = b == 0 ? 1 : (a == 0 ? 0 : Bin[a - 1][b - 1] + Bin[a - 1][b]);
  ncols = 0;
  for (int d = D; d >= 0; d--) { off[d] = ncols; ncols += (int)Bin[N][d]; }
  nhigh = (int)Bin[N][D];
  nlow = ncols - nhigh;
  lw = (nlow + 63) / 64;
  mons = malloc(sizeof(uint64_t) * ncols);
  for (int d = 0; d <= D; d++) {
    if (d == 0) { mons[off[0]] = 0; continue; }
    uint64_t m = (1ULL << d) - 1;
    for (long long q = 0; q < Bin[N][d]; q++) {
      if (colidx(m) != off[d] + q) { fprintf(stderr, "rank bug\n"); return 3; }
      mons[off[d] + q] = m;
      uint64_t c = m & -m, rr = m + c; /* Gosper */
      m = (((rr ^ m) >> 2) / c) | rr;
    }
  }
  multab = malloc(sizeof(int32_t) * (size_t)nlow * N);
  for (int c = 0; c < nlow; c++)
    for (int i = 0; i < N; i++) multab[(size_t)c * N + i] = colidx(mons[nhigh + c] | (1ULL << i));

  double t0 = now();
  Q = malloc(sizeof(int) * ncols);
  for (int c = 0; c < ncols; c++) Q[c] = c;
  q = ncols;
  piv = malloc(sizeof(int) * ncols);
  ispiv = malloc(sizeof(int) * ncols);
  for (int c = 0; c < ncols; c++) ispiv[c] = -1;

  /* generators */
  uint64_t vsq[64], z2 = gmul(z, z);
  for (int i = 0; i < k; i++) vsq[i] = gmul(V[i], V[i]);
  mzd_t *B = mzd_init(n, ncols);
  for (int i = 0; i < k; i++)
    for (int j = 0; j < k; j++) {
      uint64_t c = gmul(vsq[i], vsq[j]) ^ gmul(z, gmul(V[i], V[j]));
      int col = colidx((1ULL << i) | (1ULL << (k + j)));
      if (D >= 2)
        for (int b = 0; b < n; b++)
          if (c >> b & 1) mzd_write_bit(B, b, col, 1);
    }
  for (int i = 0; i < k; i++) {
    uint64_t c = gmul(z2, vsq[i]);
    for (int b = 0; b < n; b++)
      if (c >> b & 1) {
        mzd_write_bit(B, b, colidx(1ULL << i), 1);
        mzd_write_bit(B, b, colidx(1ULL << (k + i)), 1);
      }
  }
  for (int b = 0; b < n; b++)
    if (a6 >> b & 1) mzd_write_bit(B, b, colidx(0), 1);
  if (D < 2) { fprintf(stderr, "D>=2 required (generators are quadratic)\n"); return 2; }
  insert_batch(B);
  mzd_free(B);
  int gen_rank = r;
  if (full == 2) { /* generator span only: cumulative dims of span(F) cap B_{<=e} */
    int g[12];
    dims(g);
    printf("{\"n\": %d, \"k\": %d, \"gen_rank\": %d, \"span_dims\": [", n, k, gen_rank);
    for (int e = 0; e <= D; e++) printf("%s%d", e ? ", " : "", g[e]);
    printf("]}\n");
    return 0;
  }
  long long products = 0;
  int batches = 0;
  int one_col = off[0];
  int per = BS / N;
  if (per < 1) per = 1;
  while (qhead < qn && (full || ispiv[one_col] < 0)) {
    int take = qn - qhead < per ? qn - qhead : per;
    mzd_t *Bt = mzd_init(take * N, ncols);
    for (int q = 0; q < take; q++) {
      word *f = queue[qhead + q];
      for (int w = 0; w < lw; w++) {
        word x = f[w];
        while (x) {
          int c = w * 64 + __builtin_ctzll(x);
          x &= x - 1;
          int32_t *mt = multab + (size_t)c * N;
          for (int i = 0; i < N; i++) {
            word *row = mzd_row(Bt, q * N + i);
            row[mt[i] >> 6] ^= (word)1 << (mt[i] & 63);
          }
        }
      }
      free(f);
      queue[qhead + q] = NULL;
    }
    qhead += take;
    products += (long long)take * N;
    insert_batch(Bt);
    mzd_free(Bt);
    batches++;
    int cum[12];
    dims(cum);
    fprintf(stderr, "[%.1fs] batch %d products %lld rank %d queue %d low %d one %d\n", now() - t0, batches,
            products, r, qn - qhead, cum[D - 1], ispiv[one_col] >= 0);
  }
  int cum[12];
  dims(cum);
  struct rusage ru;
  getrusage(RUSAGE_SELF, &ru);
  printf("{\"n\": %d, \"k\": %d, \"N\": %d, \"D\": %d, \"one_in\": %d, \"W_dims\": [", n, k, N, D, ispiv[one_col] >= 0);
  for (int e = 0; e <= D; e++) printf("%s%d", e ? ", " : "", cum[e]);
  printf("], \"ncols\": %d, \"codim\": %d, \"gen_rank\": %d, \"products\": %lld, \"batches\": %d, \"stopped_early\": %d, "
         "\"sec\": %.1f, \"maxrss_kb\": %ld}\n",
         ncols, ncols - r, gen_rank, products, batches, (!full && ispiv[one_col] >= 0 && qhead < qn), now() - t0, ru.ru_maxrss);
  return 0;
}
