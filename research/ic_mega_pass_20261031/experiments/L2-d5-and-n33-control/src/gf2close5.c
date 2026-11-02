/* gf2close: bit-packed GF(2) Boolean Macaulay closure W_d (M4RI-style 8-row tables),
 * static Nullstellensatz degree, and fast exhaustive search (quadratic systems).
 * Input (stdin): "MODE N d neqs" then per equation: "t m1 ... mt" (monomial bitmasks, decimal).
 * MODE: closure | static | sols | festest
 * Output: one JSON line.
 * Columns: degree-compatible order, index = off[deg] + colex rank; pivot = highest set bit.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <time.h>
typedef uint64_t u64;

static int N, D;
static long ncols, nlow, W;
static long off[8];
static u64 binom[70][8];
static u64 *mono;      /* column -> monomial mask */
static u64 *P;         /* triangular pivot rows: row p at P+roff[p], (p>>6)+1 words */
static size_t *roff;
#define PR(p) (P + roff[p])
static double cpucap = 0; static clock_t tstart;
static unsigned char *has;
static long npiv = 0, nlowpiv = 0;
static int one = 0;

static long col_of(u64 m) {
    int e = __builtin_popcountll(m); long r = 0; int i = 1;
    while (m) { int s = __builtin_ctzll(m); r += binom[s][i]; i++; m &= m - 1; }
    return off[e] + r;
}
static void setup(void) {
    for (int a = 0; a < 70; a++) for (int b = 0; b < 8; b++)
        binom[a][b] = (b == 0) ? 1 : (a == 0 ? 0 : binom[a-1][b-1] + binom[a-1][b]);
    off[0] = 0;
    for (int e = 0; e <= D; e++) off[e+1] = off[e] + binom[N][e];
    ncols = off[D+1]; nlow = off[D]; W = (ncols + 63) / 64;
    mono = malloc(sizeof(u64) * ncols);
    /* enumerate all subsets of size<=D */
    for (int e = 0; e <= D; e++) {
        if (e == 0) { mono[0] = 0; continue; }
        int idx[8]; for (int i = 0; i < e; i++) idx[i] = i;
        while (1) {
            u64 m = 0; for (int i = 0; i < e; i++) m |= 1ULL << idx[i];
            mono[col_of(m)] = m;
            int i = e - 1; while (i >= 0 && idx[i] == N - e + i) i--;
            if (i < 0) break; idx[i]++; for (int j = i+1; j < e; j++) idx[j] = idx[j-1] + 1;
        }
    }
    for (long c = 0; c < ncols; c++) if (col_of(mono[c]) != c) { fprintf(stderr, "colex bug\n"); exit(2); }
}
static inline long topbit(const u64 *r, long wmax) {
    for (long w = wmax; w >= 0; w--) if (r[w]) return w * 64 + 63 - __builtin_clzll(r[w]);
    return -1;
}
static inline int getb(const u64 *r, long c) { return (r[c >> 6] >> (c & 63)) & 1; }
static inline void xorw(u64 *restrict a, const u64 *restrict b, long nw) { for (long i = 0; i < nw; i++) a[i] ^= b[i]; }

/* queue of low pivot columns awaiting multiplication */
static long *queue; static long qh = 0, qt = 0;

static long *pcols; /* scratch: existing pivot cols descending */
static u64 *tab, *loc;

/* reduce chunk rows (S x W) against existing pivots with 8-row tables, then insert sequentially */
static void process_chunk(u64 *rows, long S) {
    if (S == 0 || one) return;
    long np = 0;
    for (long c = ncols - 1; c >= 0; c--) if (has[c]) pcols[np++] = c;
    long maxtop = -1;
    for (long s = 0; s < S; s++) { long t = topbit(rows + s*W, W-1); if (t > maxtop) maxtop = t; }
    for (long b = 0; b < np; b += 8) {
        int g = (np - b < 8) ? (int)(np - b) : 8;
        long c1 = pcols[b];
        if (c1 > maxtop) { /* rows have no bit there: still need block if any lower col <= maxtop */ }
        long nw = (c1 >> 6) + 1;
        /* local diagonalised copies */
        for (int i = g - 1; i >= 0; i--) {
            u64 *ri = loc + i*W; { long pw = (pcols[b+i] >> 6) + 1; memcpy(ri, PR(pcols[b+i]), pw * 8); if (pw < nw) memset(ri + pw, 0, (nw - pw) * 8); }
            for (int j = i + 1; j < g; j++) if (getb(ri, pcols[b+j])) xorw(ri, loc + j*W, nw);
        }
        int T = 1 << g;
        memset(tab, 0, nw * 8);
        for (int m = 1; m < T; m++) {
            u64 *t = tab + (long)m*W; const u64 *p = tab + (long)(m & (m-1))*W; const u64 *r = loc + (long)__builtin_ctz(m)*W;
            for (long i = 0; i < nw; i++) t[i] = p[i] ^ r[i];
        }
        for (long s = 0; s < S; s++) {
            u64 *r = rows + s*W; int m = 0;
            for (int i = 0; i < g; i++) m |= getb(r, pcols[b+i]) << i;
            if (m) xorw(r, tab + (long)m*W, nw);
        }
    }
    /* sequential insertion among chunk rows (they are zero on all old pivot columns) */
    for (long s = 0; s < S; s++) {
        u64 *r = rows + s*W; long wmax = W - 1;
        while (1) {
            long p = topbit(r, wmax);
            if (p < 0) break;
            if (has[p]) { xorw(r, PR(p), (p >> 6) + 1); wmax = p >> 6; continue; }
            memcpy(PR(p), r, ((p >> 6) + 1) * 8); has[p] = 1; npiv++;
            if (p < nlow) { nlowpiv++; queue[qt++] = p; }
            if (p == 0) one = 1;
            break;
        }
        if (one) return;
    }
}

static int neq; static u64 **eqm; static int *eqt;
static void read_eqs(void) {
    if (scanf("%d", &neq) != 1) exit(3);
    eqm = malloc(sizeof(u64*) * neq); eqt = malloc(sizeof(int) * neq);
    for (int i = 0; i < neq; i++) {
        if (scanf("%d", &eqt[i]) != 1) exit(3);
        eqm[i] = malloc(sizeof(u64) * (eqt[i] + 1));
        for (int j = 0; j < eqt[i]; j++) { unsigned long long x; if (scanf("%llu", &x) != 1) exit(3); eqm[i][j] = x; }
    }
}
static int eqdeg(int i) { int d = 0; for (int j = 0; j < eqt[i]; j++) { int e = __builtin_popcountll(eqm[i][j]); if (e > d) d = e; } return d; }

static void alloc_engine(long S) {
    roff = malloc(sizeof(size_t) * ncols); size_t tot = 0; for (long c = 0; c < ncols; c++) { roff[c] = tot; tot += (c >> 6) + 1; }
    P = calloc(tot, 8); fprintf(stderr, "P words %zu (%.2f GB)\n", tot, tot * 8.0 / 1e9); has = calloc(ncols, 1);
    queue = malloc(sizeof(long) * (ncols + 1)); pcols = malloc(sizeof(long) * ncols);
    tab = malloc(sizeof(u64) * 256 * W); loc = malloc(sizeof(u64) * 8 * W);
    if (!P || !has || !tab) { fprintf(stderr, "oom\n"); exit(4); }
    (void)S;
}
static void reset_engine(void) {
    memset(P, 0, (roff[ncols-1] + ((ncols-1) >> 6) + 1) * 8); memset(has, 0, ncols); npiv = nlowpiv = 0; one = 0; qh = qt = 0;
}

int main(int argc, char **argv) {
    char mode[32]; long S = 1024; if (argc > 1) S = atol(argv[1]);
    if (scanf("%31s %d %d", mode, &N, &D) != 3) return 3;
    read_eqs();
    clock_t t0 = clock(); tstart = t0; if (getenv("CPUCAP")) cpucap = atof(getenv("CPUCAP"));
    if (!strcmp(mode, "bsols")) {
        /* bilinear split: vars 0..K-1 = a, K..2K-1 = b; every monomial has <=1 b-variable and is affine in b.
           For each a: linear system in b, Gaussian elimination, enumerate solutions. */
        int K = N / 2; u64 nsol = 0; u64 sols[4096]; int ns = 0; u64 bmask = ((1ULL << K) - 1) << K;
        for (int i = 0; i < neq; i++) for (int j = 0; j < eqt[i]; j++) if (__builtin_popcountll(eqm[i][j] & bmask) > 1) { fprintf(stderr, "not bilinear\n"); return 5; }
        for (u64 a = 0; a < (1ULL << K); a++) {
            u64 rowv[64]; int nr = 0;   /* bit K = constant, bits 0..K-1 = b coefficients */
            for (int i = 0; i < neq; i++) {
                u64 r = 0;
                for (int j = 0; j < eqt[i]; j++) { u64 m = eqm[i][j]; u64 ma = m & ((1ULL << K) - 1);
                    if ((a & ma) != ma) continue; u64 mb = m & bmask;
                    if (mb) r ^= 1ULL << (__builtin_ctzll(mb) - K); else r ^= 1ULL << K; }
                rowv[nr++] = r;
            }
            /* eliminate */
            int pc[64]; u64 pr[64]; int rk = 0; int incons = 0;
            for (int i = 0; i < nr; i++) { u64 r = rowv[i];
                for (int q = 0; q < rk; q++) if ((r >> pc[q]) & 1) r ^= pr[q];
                u64 c = r & ((1ULL << K) - 1);
                if (!c) { if (r) incons = 1; continue; }
                int p = __builtin_ctzll(c);
                for (int q = 0; q < rk; q++) if ((pr[q] >> p) & 1) pr[q] ^= r;
                pc[rk] = p; pr[rk++] = r; }
            if (incons) continue;
            u64 piv = 0; for (int q = 0; q < rk; q++) piv |= 1ULL << pc[q];
            u64 fre = ((1ULL << K) - 1) & ~piv; int nf = __builtin_popcountll(fre);
            for (u64 f = 0; f < (1ULL << nf); f++) {
                u64 b = 0; u64 ff = fre; int bi = 0;
                while (ff) { int p = __builtin_ctzll(ff); ff &= ff - 1; if ((f >> bi) & 1) b |= 1ULL << p; bi++; }
                for (int q = 0; q < rk; q++) { u64 r = pr[q]; int v = (r >> K) & 1; v ^= __builtin_popcountll(r & b & ~(1ULL << pc[q])) & 1; if (v) b |= 1ULL << pc[q]; }
                nsol++; if (ns < 4096) sols[ns++] = a | (b << K);
            }
        }
        printf("{\"nsol\":%llu,\"sols\":[", (unsigned long long)nsol);
        for (int i = 0; i < ns; i++) printf("%s%llu", i ? "," : "", (unsigned long long)sols[i]);
        printf("],\"cpu\":%.3f}\n", (double)(clock() - t0) / CLOCKS_PER_SEC);
        return 0;
    }
    if (!strcmp(mode, "sols") || !strcmp(mode, "festest")) {
        /* fast exhaustive search, quadratic Boolean systems, <=32 equations, N<=34 */
        if (neq > 64 || N > 36) { fprintf(stderr, "fes size\n"); return 5; }
        u64 C0 = 0, L[64] = {0}; static u64 Q[64][64];
        memset(Q, 0, sizeof Q);
        for (int i = 0; i < neq; i++) for (int j = 0; j < eqt[i]; j++) {
            u64 m = eqm[i][j]; int e = __builtin_popcountll(m); u64 bit = 1ULL << i;
            if (e == 0) C0 ^= bit;
            else if (e == 1) L[__builtin_ctzll(m)] ^= bit;
            else if (e == 2) { int a = __builtin_ctzll(m); int b = 63 - __builtin_clzll(m); Q[a][b] ^= bit; Q[b][a] ^= bit; }
            else { fprintf(stderr, "deg>2\n"); return 5; }
        }
        u64 nsol = 0; u64 sols[4096]; int ns = 0;
        u64 F = C0, D1[64];
        D1[0] = L[0];
        for (int k = 1; k < N; k++) D1[k] = L[k] ^ Q[k][k-1];
        if (F == 0) { nsol++; sols[ns++] = 0; }
        u64 lim = 1ULL << N;
        for (u64 t = 1; t < lim; t++) {
            int k1 = __builtin_ctzll(t); u64 t2 = t & (t - 1);
            if (t2) { int k2 = __builtin_ctzll(t2); D1[k1] ^= Q[k1][k2]; }
            F ^= D1[k1];
            if (F == 0) { nsol++; if (ns < 4096) sols[ns++] = t ^ (t >> 1); }
        }
        if (!strcmp(mode, "festest")) {
            /* brute-force check */
            u64 bn = 0; int ok = 1; int si = 0;
            for (u64 x = 0; x < lim; x++) {
                u64 v = 0;
                for (int i = 0; i < neq; i++) { int b = 0; for (int j = 0; j < eqt[i]; j++) b ^= ((x & eqm[i][j]) == eqm[i][j]); v |= (u64)b << i; }
                if (v == 0) { bn++; int f = 0; for (int q = 0; q < ns; q++) if (sols[q] == x) f = 1; if (!f) ok = 0; (void)si; }
            }
            printf("{\"fes\":%llu,\"brute\":%llu,\"ok\":%d}\n", (unsigned long long)nsol, (unsigned long long)bn, ok && bn == nsol);
            return 0;
        }
        printf("{\"nsol\":%llu,\"sols\":[", (unsigned long long)nsol);
        for (int i = 0; i < ns; i++) printf("%s%llu", i ? "," : "", (unsigned long long)sols[i]);
        printf("],\"cpu\":%.3f}\n", (double)(clock() - t0) / CLOCKS_PER_SEC);
        return 0;
    }
    setup();
    alloc_engine(S);
    u64 *rows = malloc(sizeof(u64) * S * W);
    if (!rows) { fprintf(stderr, "oom rows\n"); return 4; }
    if (!strcmp(mode, "static")) {
        /* rows m*f, deg m <= Dd - deg f, Dd = 2..D ; report first Dd with 1 in span */
        int dns = -1; long dims[8] = {0};
        for (int Dd = 2; Dd <= D && dns < 0; Dd++) {
            reset_engine(); long s = 0;
            for (int i = 0; i < neq; i++) {
                int df = eqdeg(i); if (df > Dd) continue;
                for (long c = 0; c < off[Dd - df + 1]; c++) {
                    u64 m = mono[c]; u64 *r = rows + s*W; memset(r, 0, W*8);
                    for (int j = 0; j < eqt[i]; j++) { long cc = col_of(eqm[i][j] | m); r[cc>>6] ^= 1ULL << (cc & 63); }
                    if (++s == S) { process_chunk(rows, s); s = 0; }
                }
            }
            process_chunk(rows, s);
            dims[Dd] = npiv;
            if (one) dns = Dd;
        }
        printf("{\"dNS\":%d,\"Dmax\":%d,\"cpu\":%.3f}\n", dns, D, (double)(clock() - t0) / CLOCKS_PER_SEC);
        return 0;
    }
    /* closure */
    reset_engine();
    long s = 0;
    for (int i = 0; i < neq; i++) {
        if (eqt[i] == 0 || eqdeg(i) > D) continue;
        u64 *r = rows + s*W; memset(r, 0, W*8);
        for (int j = 0; j < eqt[i]; j++) { long cc = col_of(eqm[i][j]); r[cc>>6] ^= 1ULL << (cc & 63); }
        if (++s == S) { process_chunk(rows, s); s = 0; }
    }
    process_chunk(rows, s); s = 0;
    long nprod = 0;
    while (qh < qt && !one) {
        long p = queue[qh++];
        const u64 *g = PR(p); long nw = (p >> 6) + 1;
        for (int v = 0; v < N; v++) {
            u64 *r = rows + s*W; memset(r, 0, W*8);
            u64 bv = 1ULL << v;
            for (long w = 0; w < nw; w++) { u64 x = g[w]; while (x) { long c = w*64 + __builtin_ctzll(x); x &= x - 1;
                long cc = col_of(mono[c] | bv); r[cc>>6] ^= 1ULL << (cc & 63); } }
            nprod++;
            if (++s == S) { process_chunk(rows, s); s = 0; if (one) break; }
        }
        if (qh == qt && s) { process_chunk(rows, s); s = 0; }
        if (cpucap > 0 && (double)(clock() - tstart) / CLOCKS_PER_SEC > cpucap) {
            printf("{\"capped\":1,\"npiv\":%ld,\"nlowpiv\":%ld,\"qh\":%ld,\"qt\":%ld,\"ncols\":%ld,\"nlow\":%ld,\"nprod\":%ld,\"one\":%d,\"cpu\":%.3f}\n", npiv, nlowpiv, qh, qt, ncols, nlow, nprod, one, (double)(clock() - tstart) / CLOCKS_PER_SEC);
            return 0; }
        if (getenv("PROGRESS") && qh % 2000 == 0) fprintf(stderr, "prog qh=%ld qt=%ld npiv=%ld low=%ld cpu=%.0f\n", qh, qt, npiv, nlowpiv, (double)(clock() - tstart) / CLOCKS_PER_SEC);
    }
    if (s && !one) process_chunk(rows, s);
    long dim = npiv, low = nlowpiv;
    if (one) { dim = ncols; low = nlow; }
    printf("{\"dim\":%ld,\"low\":%ld,\"one\":%d,\"ncols\":%ld,\"nlow\":%ld,\"nprod\":%ld,\"cpu\":%.3f}\n",
           dim, low, one, ncols, nlow, nprod, (double)(clock() - t0) / CLOCKS_PER_SEC);
    return 0;
}
