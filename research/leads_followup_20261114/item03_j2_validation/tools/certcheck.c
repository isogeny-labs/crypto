// Validator certificate checker for the N4v2 GHS-weak instances (own code, no solver code reused).
//
// Re-derives, from the construction written in research/quartic_n4_20261019/ghs.rs (read as a specification, not linked):
//   * K = F_q[x]/(x^3 - c), the elliptic curve E0: w^2 = V^3 + a2 V^2 + a4 V + a6 over K from the seeded stream,
//   * its group order n by Hasse-interval baby-step giant-step, n = 4 ell, ell prime, a point G of order ell,
//   * the planted exponent k from the seed (stream seed ^ 0x1234),
// and checks for every recovered log read on stdin that  kfound * G == k * G  on E0, kfound < ell.
// Unit of the stream: xorshift64* exactly as the producer's instance generator (needed to name the instance, not trusted
// for correctness: the recorded ell and order must be reproduced by the independent group computation).
//
// usage: certcheck <q> <seed> <ell_rec> <order_rec> <k_rec> <maxtries>   (kfound values, one per line, on stdin)
// build: gcc -O2 -o certcheck certcheck.c -lm
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdint.h>
typedef uint64_t u64;
typedef unsigned __int128 u128;

static u64 Q, CC, W, W2;
// ---------- rng (xorshift64*) ----------
typedef struct { u64 s; } Rng;
static u64 rng_next(Rng *r) {
    u64 x = r->s; x ^= x >> 12; x ^= x << 25; x ^= x >> 27; r->s = x; return x * 0x2545F4914F6CDD1DULL;
}
static Rng rng_new(u64 seed) { Rng r; r.s = seed ^ 0x9E3779B97F4A7C15ULL; for (int i = 0; i < 4; i++) rng_next(&r); return r; }
static u64 below(Rng *r, u64 n) { return rng_next(r) % n; }
// ---------- F_q ----------
static inline u64 fadd(u64 a, u64 b) { u64 s = a + b; return s >= Q ? s - Q : s; }
static inline u64 fsub(u64 a, u64 b) { return a >= b ? a - b : a + Q - b; }
static inline u64 fmul(u64 a, u64 b) { return a * b % Q; }
static u64 fpow(u64 a, u64 e) { u64 r = 1; a %= Q; while (e) { if (e & 1) r = fmul(r, a); a = fmul(a, a); e >>= 1; } return r; }
static u64 finv(u64 a) { return fpow(a, Q - 2); }
// ---------- K = F_q[x]/(x^3 - CC) ----------
typedef struct { u64 a[3]; } K;
static K KZ = {{0, 0, 0}}, K1 = {{1, 0, 0}};
static int keq(K x, K y) { return x.a[0] == y.a[0] && x.a[1] == y.a[1] && x.a[2] == y.a[2]; }
static K kadd(K x, K y) { K r; for (int i = 0; i < 3; i++) r.a[i] = fadd(x.a[i], y.a[i]); return r; }
static K ksub(K x, K y) { K r; for (int i = 0; i < 3; i++) r.a[i] = fsub(x.a[i], y.a[i]); return r; }
static K kneg(K x) { K r; for (int i = 0; i < 3; i++) r.a[i] = x.a[i] ? Q - x.a[i] : 0; return r; }
static K kmul(K x, K y) {
    u64 t0 = x.a[0] * y.a[0] % Q;
    u64 t1 = (x.a[0] * y.a[1] + x.a[1] * y.a[0]) % Q;
    u64 t2 = (x.a[0] * y.a[2] + x.a[1] * y.a[1] + x.a[2] * y.a[0]) % Q;
    u64 t3 = (x.a[1] * y.a[2] + x.a[2] * y.a[1]) % Q;
    u64 t4 = x.a[2] * y.a[2] % Q;
    K r; r.a[0] = (t0 + CC * t3) % Q; r.a[1] = (t1 + CC * t4) % Q; r.a[2] = t2; return r;
}
static K kscal(K x, u64 s) { K r; for (int i = 0; i < 3; i++) r.a[i] = x.a[i] * (s % Q) % Q; return r; }
static K kfrob(K x) { K r; r.a[0] = x.a[0]; r.a[1] = fmul(x.a[1], W); r.a[2] = fmul(x.a[2], W2); return r; }
static K kinv(K x) {
    K x1 = kfrob(x), x2 = kfrob(x1); K p = kmul(x1, x2); K n = kmul(x, p);
    if (n.a[1] || n.a[2]) { fprintf(stderr, "norm not in F_q\n"); exit(3); }
    return kscal(p, finv(n.a[0]));
}
static K kpow(K a, u128 e) { K r = K1, b = a; while (e) { if (e & 1) r = kmul(r, b); e >>= 1; if (e) b = kmul(b, b); } return r; }
static int ksqrt(K a, K *out) {
    if (keq(a, KZ)) { *out = KZ; return 1; }
    u128 q3 = (u128)Q * Q * Q;
    K s = kpow(a, (q3 + 1) / 4);
    if (keq(kmul(s, s), a)) { *out = s; return 1; }
    return 0;
}
static K krand(Rng *r) { K x; x.a[0] = below(r, Q); x.a[1] = below(r, Q); x.a[2] = below(r, Q); return x; }
// ---------- E0: w^2 = V^3 + A2 V^2 + A4 V + A6 ----------
static K A2, A4, A6;
typedef struct { int inf; K x, y; } P;
static P pinf(void) { P p; p.inf = 1; p.x = KZ; p.y = KZ; return p; }
static int peq(P a, P b) { if (a.inf || b.inf) return a.inf == b.inf; return keq(a.x, b.x) && keq(a.y, b.y); }
static P pneg(P a) { if (a.inf) return a; a.y = kneg(a.y); return a; }
static K rhs(K x) { K x2 = kmul(x, x); return kadd(kadd(kmul(x2, x), kmul(A2, x2)), kadd(kmul(A4, x), A6)); }
static int on_curve(P a) { if (a.inf) return 1; return keq(kmul(a.y, a.y), rhs(a.x)); }
static P padd(P p, P q) {
    if (p.inf) return q; if (q.inf) return p;
    K lam;
    if (keq(p.x, q.x)) {
        if (keq(kadd(p.y, q.y), KZ)) return pinf();
        K x2 = kmul(p.x, p.x);
        K num = kadd(kadd(kscal(x2, 3), kmul(kscal(A2, 2), p.x)), A4);
        lam = kmul(num, kinv(kscal(p.y, 2)));
    } else {
        lam = kmul(ksub(q.y, p.y), kinv(ksub(q.x, p.x)));
    }
    K l2 = kmul(lam, lam);
    K x3 = ksub(ksub(ksub(l2, A2), p.x), q.x);
    K y3 = ksub(kmul(lam, ksub(p.x, x3)), p.y);
    P r; r.inf = 0; r.x = x3; r.y = y3; return r;
}
static P pmul(P p, u128 k) { P r = pinf(), b = p; while (k) { if (k & 1) r = padd(r, b); k >>= 1; if (k) b = padd(b, b); } return r; }
static P prand(Rng *r) {
    for (;;) {
        K x = krand(r); K y;
        if (ksqrt(rhs(x), &y)) { P p; p.inf = 0; p.x = x; p.y = y; return p; }
    }
}
// ---------- primality (deterministic Miller-Rabin for u64) ----------
static u64 mulmod(u64 a, u64 b, u64 m) { return (u64)((u128)a * b % m); }
static u64 powmod(u64 a, u64 e, u64 m) { u64 r = 1; a %= m; while (e) { if (e & 1) r = mulmod(r, a, m); a = mulmod(a, a, m); e >>= 1; } return r; }
static int is_prime(u64 n) {
    if (n < 2) return 0;
    static const u64 sp[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37};
    for (int i = 0; i < 12; i++) { if (n % sp[i] == 0) return n == sp[i]; }
    u64 d = n - 1; int s = 0; while (d % 2 == 0) { d /= 2; s++; }
    for (int i = 0; i < 12; i++) {
        u64 x = powmod(sp[i], d, n);
        if (x == 1 || x == n - 1) continue;
        int comp = 1;
        for (int j = 0; j < s - 1; j++) { x = mulmod(x, x, n); if (x == n - 1) { comp = 0; break; } }
        if (comp) return 0;
    }
    return 1;
}
// ---------- baby-step giant-step order (Hasse interval) ----------
typedef struct { uint32_t k[6]; u64 v; int used; } Slot;
static Slot *tab; static u64 tabmask;
static void keyof(P p, uint32_t *k) {
    if (p.inf) { for (int i = 0; i < 6; i++) k[i] = 0xFFFFFFFFu; return; }
    for (int i = 0; i < 3; i++) { k[i] = (uint32_t)p.x.a[i]; k[3 + i] = (uint32_t)p.y.a[i]; }
}
static u64 hk(const uint32_t *k) { u64 h = 1469598103934665603ULL; for (int i = 0; i < 6; i++) { h ^= k[i]; h *= 1099511628211ULL; h ^= h >> 29; } return h; }
static int tab_insert_first(P p, u64 v) {
    uint32_t k[6]; keyof(p, k); u64 i = hk(k) & tabmask;
    while (tab[i].used) { if (!memcmp(tab[i].k, k, 24)) return 0; i = (i + 1) & tabmask; }
    memcpy(tab[i].k, k, 24); tab[i].v = v; tab[i].used = 1; return 1;
}
static int tab_get(P p, u64 *v) {
    uint32_t k[6]; keyof(p, k); u64 i = hk(k) & tabmask;
    while (tab[i].used) { if (!memcmp(tab[i].k, k, 24)) { *v = tab[i].v; return 1; } i = (i + 1) & tabmask; }
    return 0;
}
static int cmp_u128(const void *a, const void *b) { u128 x = *(const u128 *)a, y = *(const u128 *)b; return x < y ? -1 : x > y; }
// returns 1 and n if the Hasse interval contains exactly one multiple of ord(p), consuming exactly one rand_point from rng
static int order_bsgs(Rng *rng, u128 *n_out) {
    u128 n0 = (u128)Q * Q * Q + 1;
    double hh = 2.0 * pow((double)Q, 1.5);
    u128 h = (u128)hh + 2;
    P p = prand(rng);
    if (p.inf) return 0;
    u128 s = (u128)ceil(sqrt((double)(2 * h + 1))) + 1;
    memset(tab, 0, (tabmask + 1) * sizeof(Slot));
    P cur = pinf();
    for (u128 b = 0; b < s; b++) { tab_insert_first(cur, (u64)b); cur = padd(cur, p); }
    P q0 = pmul(p, n0 - h);
    P target = pneg(q0);
    P sp = pmul(p, s);
    P nsp = pneg(sp);
    u128 hits[64]; int nh = 0;
    P g = target; u128 a = 0;
    while (a * s <= 2 * h + s) {
        u64 b;
        if (tab_get(g, &b)) { u128 m = a * s + b; if (m <= 2 * h && nh < 64) hits[nh++] = n0 - h + m; }
        g = padd(g, nsp); a++;
    }
    qsort(hits, nh, sizeof(u128), cmp_u128);
    int nd = 0; for (int i = 0; i < nh; i++) if (i == 0 || hits[i] != hits[i - 1]) hits[nd++] = hits[i];
    if (nd == 1) { *n_out = hits[0]; return 1; }
    return 0;
}
static int res_nonzero(K a0, K a1, K b0, K b1) {
    K c1 = a0, bb = a1; K d0 = ksub(a0, b0), d1 = ksub(a1, b1);
    K r = kadd(ksub(kmul(d0, d0), kmul(kmul(bb, d0), d1)), kmul(c1, kmul(d1, d1)));
    return !keq(r, KZ);
}
static K det3(K m[3][3]) {
    K a = kmul(m[0][0], ksub(kmul(m[1][1], m[2][2]), kmul(m[1][2], m[2][1])));
    K b = kmul(m[0][1], ksub(kmul(m[1][0], m[2][2]), kmul(m[1][2], m[2][0])));
    K c = kmul(m[0][2], ksub(kmul(m[1][0], m[2][1]), kmul(m[1][1], m[2][0])));
    return kadd(ksub(a, b), c);
}
static void pu128(u128 x, char *buf) { char t[64]; int n = 0; if (!x) t[n++] = '0'; while (x) { t[n++] = '0' + (int)(x % 10); x /= 10; } for (int i = 0; i < n; i++) buf[i] = t[n - 1 - i]; buf[n] = 0; }
static u128 parse_u128(const char *s) { u128 x = 0; for (; *s >= '0' && *s <= '9'; s++) x = x * 10 + (u128)(*s - '0'); return x; }
int main(int argc, char **argv) {
    if (argc < 7) { fprintf(stderr, "usage\n"); return 2; }
    Q = strtoull(argv[1], 0, 10); u64 seed = strtoull(argv[2], 0, 10);
    u64 ell_rec = strtoull(argv[3], 0, 10); u128 order_rec = parse_u128(argv[4]);   /* order = 4 ell exceeds 2^64 at k = 22 */ u64 k_rec = strtoull(argv[5], 0, 10);
    long maxtries = atol(argv[6]);
    tabmask = (1ULL << 21) - 1; tab = (Slot *)calloc(tabmask + 1, sizeof(Slot));
    // field constants
    CC = 2; for (;;) { if (powmod(CC, (Q - 1) / 3, Q) != 1) break; CC++; }
    W = powmod(CC, (Q - 1) / 3, Q); W2 = W * W % Q;
    Rng rng = rng_new(seed ^ (Q << 20));
    long matched = 0, prequartic_pass_before = 0; P G = pinf(); u128 n = 0; u64 ell = 0;
    for (long tries = 1; tries <= maxtries; tries++) {
        K u0 = krand(&rng), u1c = krand(&rng);               // u = t^2 + u1c t + u0 (u[0] = u0, u[1] = u1c)
        K v0 = kfrob(u0), v1 = kfrob(u1c);                    // frobenius conjugates
        K w0 = kfrob(v0), w1 = kfrob(v1);
        K disc = ksub(kmul(u1c, u1c), kscal(u0, 4));
        if (keq(disc, KZ) || !res_nonzero(u0, u1c, v0, v1) || !res_nonzero(u0, u1c, w0, w1) || !res_nonzero(v0, v1, w0, w1)) continue;
        K m[3][3] = {{u0, u1c, K1}, {v0, v1, K1}, {w0, w1, K1}};
        if (keq(det3(m), KZ)) continue;
        K qa = kadd(u1c, w1);
        K qb = kadd(kadd(u0, w0), kmul(u1c, w1));
        K qc = kadd(kmul(u1c, w0), kmul(w1, u0));
        K qd = kmul(u0, w0);
        A2 = qb;
        A4 = ksub(kmul(qa, qc), kscal(qd, 4));
        A6 = kadd(ksub(kmul(qc, qc), kscal(kmul(qb, qd), 4)), kmul(kmul(qa, qa), qd));
        u128 nn;
        if (!order_bsgs(&rng, &nn)) continue;
        if (nn % 4 != 0) continue;
        u128 el = nn / 4;
        if (el > (u128)UINT64_MAX || !is_prime((u64)el)) continue;
        P p2 = prand(&rng);
        if (!peq(pmul(p2, nn), pinf())) continue;
        P g = pinf();
        for (int i = 0; i < 50; i++) { P r = prand(&rng); g = pmul(r, 4); if (!g.inf) break; }
        if (g.inf || !peq(pmul(g, el), pinf())) continue;
        if ((u64)el == ell_rec && nn == order_rec) { matched = tries; G = g; n = nn; ell = (u64)el; break; }
        prequartic_pass_before++;                              // passed every check before the plane-quartic stage; the solver's instance is a later try
    }
    char b1[64], b2[64];
    if (!matched) { printf("{\"q\":%llu,\"seed\":%llu,\"matched\":false}\n", (unsigned long long)Q, (unsigned long long)seed); return 1; }
    // curve invariants: j in K, j not in {0,1728}, j not in F_q
    K b2c = kscal(A2, 4), b4 = kscal(A4, 2), b6 = kscal(A6, 4);
    K b8 = ksub(kscal(kmul(A2, A6), 4), kmul(A4, A4));
    K c4 = ksub(kmul(b2c, b2c), kscal(b4, 24));
    // c6 = -b2^3 + 36 b2 b4 - 216 b6
    K b2cub = kmul(b2c, kmul(b2c, b2c));
    K c6 = ksub(kadd(kneg(b2cub), kscal(kmul(b2c, b4), 36)), kscal(b6, 216));
    K b2sq = kmul(b2c, b2c);
    K delta = kadd(ksub(ksub(kneg(kmul(b2sq, b8)), kscal(kmul(b4, kmul(b4, b4)), 8)), kscal(kmul(b6, b6), 27)), kscal(kmul(b2c, kmul(b4, b6)), 9));
    int j_ok = !keq(delta, KZ);
    K j = kmul(kmul(c4, kmul(c4, c4)), kinv(delta));
    int j_not01728 = !keq(c4, KZ) && !keq(c6, KZ);
    int j_in_fq = keq(kfrob(j), j);
    // second-source check of the group-order claim: n * (fresh points) = O with a stream independent of the instance stream
    Rng rr = rng_new(0xABCDEF ^ seed);
    int ord_ok = 1; for (int i = 0; i < 3; i++) { P pt = prand(&rr); if (!peq(pmul(pt, n), pinf())) ord_ok = 0; }
    // planted k
    Rng rk = rng_new(seed ^ 0x1234);
    prand(&rk); prand(&rk);
    u64 kk = below(&rk, ell - 1) + 1;
    int k_match = (kk == k_rec);
    int g_ok = on_curve(G) && !G.inf && peq(pmul(G, ell), pinf()) && is_prime(ell) && n == (u128)4 * ell;
    P Q_pt = pmul(G, kk);
    int nlog = 0, nlog_ok = 0; char line[128];
    while (fgets(line, sizeof line, stdin)) {
        u64 kf = strtoull(line, 0, 10); nlog++;
        P A = pmul(G, kf);
        if (kf < ell && peq(A, Q_pt)) nlog_ok++;
    }
    pu128(n, b1);
    printf("{\"q\":%llu,\"seed\":%llu,\"matched\":true,\"tries_until_match\":%ld,\"prequartic_passes_before\":%ld,\"ell\":%llu,\"order\":%s,"
           "\"order_eq_4ell\":%s,\"ell_prime\":%s,\"G_on_curve_order_ell\":%s,\"order_kills_fresh_points\":%s,"
           "\"k_planted\":%llu,\"k_matches_record\":%s,\"j_delta_nonzero\":%s,\"j_not_0_1728\":%s,\"j_in_Fq\":%s,"
           "\"logs_checked\":%d,\"logs_ok\":%d}\n",
           (unsigned long long)Q, (unsigned long long)seed, matched, prequartic_pass_before, (unsigned long long)ell, b1,
           n == (u128)4 * ell ? "true" : "false", is_prime(ell) ? "true" : "false", g_ok ? "true" : "false", ord_ok ? "true" : "false",
           (unsigned long long)kk, k_match ? "true" : "false", j_ok ? "true" : "false", j_not01728 ? "true" : "false", j_in_fq ? "true" : "false",
           nlog, nlog_ok);
    (void)b2;
    return 0;
}
