/* chaincheck: independent replay checker for chainsd-cert/v1 certificates.
 *
 * A certificate lists, in order, the closure steps "s v r": the new basis row is the reduction of
 * (variable v) * (basis row r).  The checker replays the steps with its own exact F_p linear
 * algebra (lazily reduced 64-bit rows, weight-descending column order, canonical echelon
 * reduction), and enforces on every step that the shifted row has weight <= D-1 (the degree
 * budget of the pinned computation) and that the reduced result is nonzero.
 *
 * Upper certificate (end one_in_V 1): after the replay, the constant monomial must be in the span.
 * Lower certificate (end one_in_V 0): the constant must not be in the span, AND the full
 * closure sweep must pass: for every basis row of weight <= D-1 and every variable, the shifted
 * row reduces to zero.  Only then is the span the full iterated degree-bounded closure V_D.
 *
 * Build: cc -O2 -fopenmp -o chaincheck chaincheck.c   (OpenMP only parallelises the final sweep)
 * Usage: chaincheck CERTFILE
 * Output: one JSON line; exit status 0 iff the certificate verifies.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>

typedef uint64_t u64;
typedef uint32_t u32;

static u64 P;            /* prime, < 2^23 */
static u32 K, KSHIFT, D; /* exponent width, shift width, degree bound */
static int UNI;
static u64 NMASK;

static u64 *masks;       /* columns: masks with popcount <= D, weight-descending, mask ascending */
static u32 ncols;
static u64 *htab_keys;   /* open addressing hash: mask -> column */
static u32 *htab_vals;
static u64 hsize;

static u64 **rows;       /* basis rows (lazy, entries < 2^63) */
static u32 *pivcol;      /* pivot column of each row */
static int32_t *pivot_row; /* column -> row index or -1 */
static u32 nrows, rowcap;

static int popc(u64 m) { return __builtin_popcountll(m); }

static void die(const char *msg) {
    printf("{\"verified\":false,\"error\":\"%s\"}\n", msg);
    exit(1);
}

static u64 hmix(u64 x) { x ^= x >> 33; x *= 0xff51afd7ed558ccdULL; x ^= x >> 33; x *= 0xc4ceb9fe1a85ec53ULL; x ^= x >> 33; return x; }

static void hput(u64 key, u32 val) {
    u64 h = hmix(key) & (hsize - 1);
    while (htab_vals[h] != 0xffffffffu) h = (h + 1) & (hsize - 1);
    htab_keys[h] = key; htab_vals[h] = val;
}
static int64_t hget(u64 key) {
    u64 h = hmix(key) & (hsize - 1);
    while (htab_vals[h] != 0xffffffffu) {
        if (htab_keys[h] == key) return htab_vals[h];
        h = (h + 1) & (hsize - 1);
    }
    return -1;
}

static int cmp_desc_weight(const void *a, const void *b) {
    u64 x = *(const u64 *)a, y = *(const u64 *)b;
    int wx = popc(x), wy = popc(y);
    if (wx != wy) return wy - wx;        /* weight descending */
    return (x < y) ? -1 : (x > y);       /* mask ascending */
}

static void build_layout(void) {
    u32 nv = 2 * K;
    u64 cap = 0;
    /* count: sum_{w<=D} C(nv, w) */
    for (u32 w = 0; w <= D && w <= nv; w++) {
        double c = 1; for (u32 i = 0; i < w; i++) c = c * (nv - i) / (i + 1);
        cap += (u64)(c + 0.5);
    }
    masks = malloc(cap * sizeof(u64));
    if (!masks) die("alloc masks");
    u64 n = 0;
    u64 limit = 1ULL << nv;
    for (u32 w = 0; w <= D && w <= nv; w++) {
        if (w == 0) { masks[n++] = 0; continue; }
        u64 m = (1ULL << w) - 1;
        while (m < limit) {
            masks[n++] = m;
            u64 c = m & (~m + 1);
            u64 r = m + c;
            if (r >= limit) break;
            m = (((r ^ m) >> 2) / c) | r;
        }
    }
    if (n != cap) die("layout count mismatch");
    ncols = (u32)n;
    qsort(masks, ncols, sizeof(u64), cmp_desc_weight);
    hsize = 1; while (hsize < 2 * (u64)ncols) hsize <<= 1;
    htab_keys = malloc(hsize * sizeof(u64)); htab_vals = malloc(hsize * sizeof(u32));
    if (!htab_keys || !htab_vals) die("alloc hash");
    memset(htab_vals, 0xff, hsize * sizeof(u32));
    for (u32 i = 0; i < ncols; i++) hput(masks[i], i);
}

static u64 shift_mask(u64 mask, u32 v) {
    u64 a = mask & NMASK, b = mask >> K;
    if (UNI || v < KSHIFT) { a = (a + (1ULL << v)) & NMASK; }
    else { u32 j = v - KSHIFT; b = (b + (1ULL << j)) & NMASK; }
    return a | (b << K);
}

static void normalize(u64 *row, u32 from) { for (u32 j = from; j < ncols; j++) row[j] %= P; }

/* reduce a row against the basis; returns pivot column or -1 if zero. Row becomes fully reduced. */
static int64_t reduce(u64 *row) {
    u32 ops = 0; int64_t first = -1;
    for (u32 col = 0; col < ncols; col++) {
        u64 v = row[col] % P;
        if (v == 0) { row[col] = 0; continue; }
        if (pivot_row[col] >= 0) {
            const u64 *b = rows[pivot_row[col]];
            u64 c = P - v;
            for (u32 j = col; j < ncols; j++) row[j] += c * b[j];
            row[col] = 0;
            if (++ops >= 65536) { normalize(row, col); ops = 0; }
        } else {
            row[col] = v;
            if (first < 0) first = col;
        }
    }
    return first;
}

static u64 modpow(u64 a, u64 e) { u64 r = 1; a %= P; while (e) { if (e & 1) r = r * a % P; a = a * a % P; e >>= 1; } return r; }

static void insert_row(u64 *row, u32 piv) {
    u64 inv = modpow(row[piv] % P, P - 2);
    for (u32 j = piv; j < ncols; j++) row[j] = (row[j] % P) * inv % P;
    if (nrows == rowcap) { rowcap = rowcap ? rowcap * 2 : 1024; rows = realloc(rows, rowcap * sizeof(u64 *)); pivcol = realloc(pivcol, rowcap * sizeof(u32)); if (!rows || !pivcol) die("alloc rows"); }
    rows[nrows] = row; pivcol[nrows] = piv; pivot_row[piv] = (int32_t)nrows; nrows++;
}

/* weight of a (reduced) row = max popcount over its nonzero columns */
static int row_weight(const u64 *row) {
    int w = -1;
    for (u32 j = 0; j < ncols; j++) if (row[j] % P) { int pw = popc(masks[j]); if (pw > w) w = pw; }
    return w;
}

static u64 *shifted_row(const u64 *src, u32 v, int *okweight) {
    u64 *out = calloc(ncols, sizeof(u64));
    if (!out) die("alloc shifted");
    int wmax = -1;
    for (u32 j = 0; j < ncols; j++) {
        u64 c = src[j] % P;
        if (!c) continue;
        int pw = popc(masks[j]); if (pw > wmax) wmax = pw;
        u64 m2 = shift_mask(masks[j], v);
        int64_t idx = hget(m2);
        if (idx < 0) die("shift left the weight window");
        out[idx] = (out[idx] + c) % P;
    }
    *okweight = (wmax <= (int)D - 1);
    return out;
}

int main(int argc, char **argv) {
    if (argc != 2) { fprintf(stderr, "usage: chaincheck CERT\n"); return 2; }
    FILE *f = fopen(argv[1], "r");
    if (!f) die("cannot open certificate");
    char line[1 << 16];
    if (!fgets(line, sizeof line, f) || strncmp(line, "chainsd-cert/v1", 15)) die("bad magic");
    unsigned uni;
    if (fscanf(f, " p %" SCNu64 " k %u kshift %u uni %u D %u", &P, &K, &KSHIFT, &uni, &D) != 5) die("bad header");
    UNI = (int)uni;
    if (P >= (1ULL << 23) || K == 0 || K > 31 || KSHIFT > K || D > 2 * K) die("unsupported parameters");
    NMASK = (1ULL << K) - 1;
    build_layout();
    pivot_row = malloc(ncols * sizeof(int32_t)); for (u32 i = 0; i < ncols; i++) pivot_row[i] = -1;
    /* generator */
    if (fscanf(f, " generator") != 0) die("bad generator line");
    u64 *g = calloc(ncols, sizeof(u64));
    int c; int nterms = 0; int gw = 0;
    while ((c = fgetc(f)) == ' ') {
        u64 a, b, coef;
        if (fscanf(f, "%" SCNu64 ":%" SCNu64 ":%" SCNu64, &a, &b, &coef) != 3) die("bad generator term");
        u64 m = (a & NMASK) | ((b & NMASK) << K);
        if (popc(m) > (int)D) die("generator weight exceeds D");
        if (popc(m) > gw) gw = popc(m);
        int64_t idx = hget(m); if (idx < 0) die("generator monomial not in layout");
        g[idx] = (g[idx] + coef) % P; nterms++;
    }
    if (nterms == 0) die("empty generator");
    int64_t gp = reduce(g);
    if (gp < 0) die("generator is zero");
    insert_row(g, (u32)gp);
    /* steps */
    u64 nsteps = 0, nweightfail = 0;
    for (;;) {
        char tag[16];
        if (fscanf(f, " %15s", tag) != 1) die("truncated certificate");
        if (!strcmp(tag, "end")) break;
        if (strcmp(tag, "s")) die("bad step tag");
        u32 v; u64 r;
        if (fscanf(f, " %u %" SCNu64, &v, &r) != 2) die("bad step");
        u32 nv = UNI ? KSHIFT : 2 * KSHIFT;
        if (v >= nv) die("variable out of range");
        if (r >= nrows) die("source row out of range");
        int okw;
        u64 *row = shifted_row(rows[r], v, &okw);
        if (!okw) { nweightfail++; die("step violates the degree budget: source row has weight > D-1"); }
        int64_t piv = reduce(row);
        if (piv < 0) die("step reduced to zero (not a new basis element)");
        insert_row(row, (u32)piv);
        nsteps++;
    }
    u64 rank_claim; unsigned one_claim;
    if (fscanf(f, " rank %" SCNu64 " one_in_V %u", &rank_claim, &one_claim) != 2) die("bad end line");
    fclose(f);
    int one_in_V = pivot_row[ncols - 1] >= 0;
    if (masks[ncols - 1] != 0) die("layout: constant not last");
    int rank_ok = (rank_claim == nrows);
    int one_ok = ((unsigned)one_in_V == one_claim);
    /* lower certificate: full closure sweep */
    u64 sweep_rows = 0, sweep_bad = 0; int swept = 0;
    if (!one_in_V) {
        swept = 1;
        u32 nv = UNI ? KSHIFT : 2 * KSHIFT;
        u32 nr = nrows;
        /* the basis is read-only from here on; every (row, variable) check is independent */
        #pragma omp parallel for schedule(dynamic, 1) reduction(+:sweep_rows,sweep_bad)
        for (u32 r = 0; r < nr; r++) {
            if (row_weight(rows[r]) > (int)D - 1) continue;
            sweep_rows++;
            for (u32 v = 0; v < nv; v++) {
                int okw; u64 *row = shifted_row(rows[r], v, &okw);
                if (reduce(row) >= 0) sweep_bad++;
                free(row);
            }
        }
    }
    int verified = rank_ok && one_ok && (one_in_V || sweep_bad == 0);
    printf("{\"verified\":%s,\"p\":%" PRIu64 ",\"k\":%u,\"D\":%u,\"columns\":%u,\"rank\":%u,\"rank_claimed\":%" PRIu64
           ",\"steps\":%" PRIu64 ",\"generator_weight\":%d,\"one_in_V\":%d,\"one_claimed\":%u,\"closure_sweep\":\"%s\",\"sweep_rows\":%" PRIu64
           ",\"sweep_bad\":%" PRIu64 ",\"kind\":\"%s\"}\n",
           verified ? "true" : "false", P, K, D, ncols, nrows, rank_claim, nsteps, gw, one_in_V, one_claim,
           swept ? "done" : "not_needed", sweep_rows, sweep_bad, one_in_V ? "upper" : "lower");
    return verified ? 0 : 1;
}
