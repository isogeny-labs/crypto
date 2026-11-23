/* rup_trim.c - small backward DRUP checker with core marking (own code, pilot).
 *
 * usage: rup_trim formula.cnf proof.drup [core_lemmas.out [core.cnf]]
 *
 * - Deletion lines ("d ...") in the proof are ignored (sound for RUP: more
 *   clauses stay available; it only weakens the "D" in DRUP).
 * - Only RUP is checked. A lemma that is not RUP makes the run FAIL.
 * - Backward pass from the first empty clause; every marked lemma is checked
 *   by a from-scratch unit propagation over clauses with a smaller index,
 *   with core-first propagation (already-marked clauses are tried first).
 * - Prints one line:
 *   RESULT status=VERIFIED n_orig= n_lemmas= core_orig= core_lemmas= core_lits= checks= props=
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct { int *a; int n, cap; } vec;
static void vpush(vec *v, int x) {
    if (v->n == v->cap) { v->cap = v->cap ? 2 * v->cap : 4; v->a = realloc(v->a, sizeof(int) * v->cap); }
    v->a[v->n++] = x;
}

static int nvars = 0, ncl = 0, norig = 0;
static vec pool;            /* literals of all clauses */
static vec cstart, clen;    /* per clause */
static char *mark;
static vec *watch;          /* per literal code */
static vec units;           /* ids of unit clauses, increasing */
static signed char *val;    /* per literal code: 1 true, -1 false, 0 unset */
static int *reason;         /* per var */
static int *trail, ntrail, qc, qn;
static char *seen;
static long long nprops = 0, nchecks = 0;

static inline int code(int lit) { return lit > 0 ? 2 * lit : 2 * (-lit) + 1; }
static inline int neg(int c) { return c ^ 1; }

static int cmpint(const void *x, const void *y) { return *(const int *)x - *(const int *)y; }

/* add clause given as DIMACS lits; returns id. Removes duplicate literals.
 * Tautologies are stored with length -1 (never used). */
static int add_clause(int *lits, int n) {
    qsort(lits, n, sizeof(int), cmpint);
    int m = 0, taut = 0;
    for (int i = 0; i < n; i++) {
        if (m && lits[m - 1] == lits[i]) continue;
        lits[m++] = lits[i];
    }
    for (int i = 0; i < m; i++) for (int j = i + 1; j < m; j++) if (lits[i] == -lits[j]) taut = 1;
    vpush(&cstart, pool.n);
    vpush(&clen, taut ? -1 : m);
    for (int i = 0; i < m; i++) {
        int v = abs(lits[i]);
        if (v > nvars) nvars = v;
        vpush(&pool, code(lits[i]));
    }
    return ncl++;
}

static void read_file(const char *path, int is_proof, int *empty_id) {
    FILE *f = fopen(path, "r");
    if (!f) { fprintf(stderr, "cannot open %s\n", path); exit(2); }
    char *line = NULL; size_t cap = 0;
    vec tmp = {0};
    while (getline(&line, &cap, f) > 0) {
        char *p = line;
        while (*p == ' ' || *p == '\t') p++;
        if (*p == 'c' || *p == 'p' || *p == '\n' || *p == 0) continue;
        if (*p == 'd') continue; /* ignore deletions */
        tmp.n = 0;
        char *end;
        for (;;) {
            long x = strtol(p, &end, 10);
            if (end == p) break;
            p = end;
            if (x == 0) break;
            vpush(&tmp, (int)x);
        }
        int id = add_clause(tmp.a, tmp.n);
        if (is_proof && tmp.n == 0 && *empty_id < 0) { *empty_id = id; break; }
    }
    free(line); free(tmp.a); fclose(f);
}

static inline void assign(int c, int r) {
    val[c] = 1; val[neg(c)] = -1; reason[c >> 1] = r; trail[ntrail++] = c;
}

/* scan watch list of the literal falsified by `lit`; coremode selects which
 * clauses (by mark) are examined. Returns conflicting clause id or -1. */
static int scan(int lit, int coremode, int cur) {
    int fl = neg(lit);
    vec *w = &watch[fl];
    int i = 0, j = 0, confl = -1;
    while (i < w->n) {
        int id = w->a[i++];
        if (id >= cur) continue;                 /* never needed again: drop */
        if ((mark[id] != 0) != coremode) { w->a[j++] = id; continue; }
        int *c = pool.a + cstart.a[id];
        int n = clen.a[id];
        if (c[0] == fl) { c[0] = c[1]; c[1] = fl; }
        if (val[c[0]] == 1) { w->a[j++] = id; continue; }
        int k, found = 0;
        for (k = 2; k < n; k++) if (val[c[k]] != -1) { found = 1; break; }
        if (found) {
            int t = c[k]; c[k] = c[1]; c[1] = t;
            vpush(&watch[t], id);
            continue;                            /* removed from this list */
        }
        w->a[j++] = id;
        nprops++;
        if (val[c[0]] == -1) { confl = id; break; }
        assign(c[0], id);
    }
    while (i < w->n) w->a[j++] = w->a[i++];
    w->n = j;
    return confl;
}

static int propagate(int cur) {
    for (;;) {
        int c;
        if (qc < ntrail) { c = scan(trail[qc++], 1, cur); if (c >= 0) return c; continue; }
        if (qn < ntrail) { c = scan(trail[qn++], 0, cur); if (c >= 0) return c; continue; }
        return -1;
    }
}

static vec stk;
static void analyze(int confl) {
    stk.n = 0;
    mark[confl] = 1;
    int *c = pool.a + cstart.a[confl];
    for (int k = 0; k < clen.a[confl]; k++) vpush(&stk, c[k] >> 1);
    while (stk.n) {
        int v = stk.a[--stk.n];
        if (seen[v]) continue;
        seen[v] = 1;
        int r = reason[v];
        if (r < 0) continue;
        mark[r] = 1;
        int *d = pool.a + cstart.a[r];
        for (int k = 0; k < clen.a[r]; k++) vpush(&stk, d[k] >> 1);
    }
}

/* RUP check of clause id against clauses with smaller index. 1 = ok. */
static int check(int id) {
    nchecks++;
    for (int i = 0; i < ntrail; i++) {
        int c = trail[i];
        val[c] = val[neg(c)] = 0; seen[c >> 1] = 0; reason[c >> 1] = -1;
    }
    ntrail = qc = qn = 0;
    int n = clen.a[id];
    if (n < 0) return 1; /* tautology */
    int *c = pool.a + cstart.a[id];
    for (int k = 0; k < n; k++) {
        int nl = neg(c[k]);
        if (val[nl] == 0) assign(nl, -1);
    }
    int confl = -1;
    for (int u = 0; u < units.n && units.a[u] < id; u++) {
        int uid = units.a[u];
        int l = pool.a[cstart.a[uid]];
        if (val[l] == 1) continue;
        if (val[l] == -1) { confl = uid; break; }
        assign(l, uid);
    }
    if (confl < 0) confl = propagate(id);
    if (confl < 0) return 0;
    analyze(confl);
    return 1;
}

int main(int argc, char **argv) {
    if (argc < 3) { fprintf(stderr, "usage: %s cnf proof [core_lemmas [core_cnf]]\n", argv[0]); return 2; }
    int empty_id = -1, dummy = -1;
    read_file(argv[1], 0, &dummy);
    norig = ncl;
    read_file(argv[2], 1, &empty_id);
    if (empty_id < 0) { int z = 0; empty_id = add_clause(&z, 0); }
    int total = empty_id + 1;
    mark = calloc(ncl + 1, 1);
    watch = calloc(2 * nvars + 4, sizeof(vec));
    val = calloc(2 * nvars + 4, 1);
    reason = malloc(sizeof(int) * (nvars + 2));
    seen = calloc(nvars + 2, 1);
    trail = malloc(sizeof(int) * (nvars + 2));
    for (int v = 0; v <= nvars + 1; v++) reason[v] = -1;
    for (int id = 0; id < total; id++) {
        int n = clen.a[id];
        if (n == 1) vpush(&units, id);
        else if (n >= 2) {
            vpush(&watch[pool.a[cstart.a[id]]], id);
            vpush(&watch[pool.a[cstart.a[id] + 1]], id);
        }
    }
    mark[empty_id] = 1;
    for (int id = empty_id; id >= norig; id--) {
        if (!mark[id]) continue;
        if (!check(id)) {
            printf("RESULT status=FAILED lemma_index=%d n_orig=%d n_lemmas=%d\n", id - norig, norig, empty_id - norig);
            return 1;
        }
    }
    long long core_orig = 0, core_lem = 0, core_lits = 0;
    for (int id = 0; id < norig; id++) core_orig += mark[id] != 0;
    for (int id = norig; id < empty_id; id++) if (mark[id]) { core_lem++; core_lits += clen.a[id] > 0 ? clen.a[id] : 0; }
    for (int pass = 0; pass < 2; pass++) {
        if (argc <= 3 + pass) break;
        FILE *o = fopen(argv[3 + pass], "w");
        int lo = pass ? 0 : norig, hi = pass ? norig : empty_id + 1;
        if (pass) fprintf(o, "p cnf %d %lld\n", nvars, core_orig);
        for (int id = lo; id < hi; id++) {
            if (!mark[id] || clen.a[id] < 0) continue;
            int *c = pool.a + cstart.a[id];
            for (int k = 0; k < clen.a[id]; k++) fprintf(o, "%d ", (c[k] & 1) ? -(c[k] >> 1) : (c[k] >> 1));
            fprintf(o, "0\n");
        }
        fclose(o);
    }
    printf("RESULT status=VERIFIED n_orig=%d n_lemmas=%d core_orig=%lld core_lemmas=%lld core_lits=%lld checks=%lld props=%lld\n",
           norig, empty_id - norig, core_orig, core_lem, core_lits, nchecks, nprops);
    return 0;
}
