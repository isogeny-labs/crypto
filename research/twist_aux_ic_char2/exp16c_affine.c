// EXP16c: one-class lifting pools of affine char-2 factor-base lines x = c0 + u*w, w in F_Q, on
// E: y^2 + xy = x^3 + a x^2 + b over K = F_{2^N}, N = 4d, Q = 2^d.
// x lifts iff Tr(x) + Tr(a) + Tr(sqrt(b)/x) = 0; lifted x is in 2E iff Tr(x) = Tr(a) (Exp 13 criteria).
// pool(line) = max over the two classes of #{w in F_Q : x != 0 lifts, class c}.
// modes:
//   exh0  d ta seed        : every line with T(u) = 0 (all Q^2+Q+1 directions in ker T, all Q^3 cosets), table-driven
//   exhall d ta seed       : every line (all (2^N-1)/(Q-1) directions, all cosets), table-driven (d = 5 only)
//   samp d ta seed nlines  : random lines: nlines with T(u)=0 and Tr(c0)=Tr(a) (the best one-class case),
//                            nlines with T(u)=0 and random c0, nlines fully random; direct inversion.
// Build: cc -O3 -o exp16c_affine exp16c_affine.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
typedef uint64_t u64;
static int N, d, Q; static u64 POLY, MASKN, TRM;
static u64 rs = 88172645463325252ULL;
static u64 rnd(void){ rs ^= rs << 13; rs ^= rs >> 7; rs ^= rs << 17; return rs; }
static u64 rel(void){ return rnd() & MASKN; }
static u64 mul(u64 a, u64 b){ u64 r = 0; while (b){ if (b & 1) r ^= a; b >>= 1; a <<= 1; if (a >> N & 1) a ^= POLY; } return r; }
static u64 pw(u64 a, u64 e){ u64 r = 1; while (e){ if (e & 1) r = mul(r, a); a = mul(a, a); e >>= 1; } return r; }
static u64 frobk(u64 a, int k){ for (int i = 0; i < k; i++) a = mul(a, a); return a; }
static int deg(u64 a){ return a ? 63 - __builtin_clzll(a) : -1; }
static u64 inv(u64 a){ // extended Euclid in GF(2)[x]/(POLY)
  u64 r0 = POLY, r1 = a, s0 = 0, s1 = 1;
  while (r1 != 1){ int dd = deg(r0) - deg(r1);
    if (dd < 0){ u64 t = r0; r0 = r1; r1 = t; t = s0; s0 = s1; s1 = t; dd = -dd; }
    r0 ^= r1 << dd; s0 ^= s1 << dd;
    if (r0 == 1){ r1 = r0; s1 = s0; break; } }
  // reduce s1 mod POLY (s may exceed degree N-1)
  u64 s = s1; while (deg(s) >= N) s ^= POLY << (deg(s) - N); return s; }
static int Tr(u64 x){ return __builtin_parityll(x & TRM); }
static u64 TQ(u64 x){ u64 r = x, y = x; for (int i = 1; i < 4; i++){ y = frobk(y, d); r ^= y; } return r; }
static u64 pmod(u64 a, u64 m){ while (deg(a) >= deg(m)) a ^= m << (deg(a) - deg(m)); return a; }
static u64 pgcd(u64 a, u64 b){ while (b){ u64 t = pmod(a, b); a = b; b = t; } return a; }
static int irreducible(u64 f, int n){ // x^(2^n) = x mod f and gcd(x^(2^(n/p)) - x, f) = 1 for primes p | n
  POLY = f; N = n; u64 x = 2; if (frobk(x, n) != x) return 0;
  for (int p = 2; p <= n; p++){ if (n % p) continue; int pr = 1; for (int q = 2; q * q <= p; q++) if (p % q == 0) pr = 0;
    if (pr && pgcd(f, frobk(x, n / p) ^ x) != 1) return 0; }
  return 1; }
// F2 linear algebra: reduce v against echelon basis (pivot = top bit)
typedef struct { u64 v[64]; int n; } Basis;
static u64 bred(Basis *B, u64 v){ for (int i = 0; i < B->n; i++){ u64 b = B->v[i]; if (v >> deg(b) & 1) v ^= b; } return v; }
static int badd(Basis *B, u64 v){ v = bred(B, v); if (!v) return 0;
  for (int i = 0; i < B->n; i++) if (B->v[i] >> deg(v) & 1) B->v[i] ^= v;
  int i = B->n++; B->v[i] = v; // keep sorted by pivot descending
  while (i > 0 && deg(B->v[i]) > deg(B->v[i-1])){ u64 t = B->v[i]; B->v[i] = B->v[i-1]; B->v[i-1] = t; i--; }
  return 1; }
static u64 FQ[256]; static u64 hb[16]; // F_Q elements, F2-basis of F_Q
static u64 sb, aa; static int ta;
static unsigned char *TAB; // bit0 lift, bit1 class (Tr(x)+Tr(a)); x = 0 -> 0xff
static int lifts(u64 x){ return (Tr(x) ^ ta ^ Tr(mul(sb, inv(x)))) == 0; }
static void setup(int dd, int tta, u64 seed){
  d = dd; N = 4 * d; Q = 1 << d; MASKN = (N == 64) ? ~0ULL : ((1ULL << N) - 1); ta = tta; rs ^= seed * 0x9E3779B97F4A7C15ULL; for (int i = 0; i < 10; i++) rnd();
  u64 f; for (f = (1ULL << N) | 3; ; f += 2) if (irreducible(f, N)) break; POLY = f;
  TRM = 0; for (int i = 0; i < N; i++){ u64 y = 1ULL << i, s = 0; for (int k = 0; k < N; k++){ s ^= y; y = mul(y, y); } if (s & 1) TRM |= 1ULL << i; }
  // F_Q: h = g^((2^N-1)/(Q-1)) with F2-span of powers of full rank d
  for (;;){ u64 g = rel(); if (!g) continue; u64 h = pw(g, (MASKN) / (Q - 1)); Basis B = {{0}, 0}; u64 y = 1;
    for (int i = 0; i < d; i++){ badd(&B, y); hb[i] = y; y = mul(y, h); }
    if (B.n == d){ FQ[0] = 0; y = 1; for (int i = 1; i < Q; i++){ FQ[i] = y; y = mul(y, h); } break; } }
  u64 b; do b = rel(); while (!b); sb = frobk(b, N - 1);
  do aa = rel(); while (Tr(aa) != ta);
  fprintf(stdout, "# d=%d Q=%d N=%d poly=0x%llx Tr(a)=%d b=0x%llx\n", d, Q, N, (unsigned long long)POLY, ta, (unsigned long long)b);
}
static void buildtab(void){ TAB = malloc(1ULL << N); TAB[0] = 0xff;
  for (u64 x = 1; x <= MASKN; x++) TAB[x] = (unsigned char)(lifts(x) | (Tr(x) ^ ta) << 1); }
static int inP(u64 x){ u64 z = mul(x, inv(sb)); return frobk(z, 2 * d) == z; }
// enumerate all cosets of u*F_Q; hist0 = through-0 line pool, hist[pool]++ over affine cosets
static long long hist[257], nfullP, nfullNotP, nbeat, naff; static int histmax;
static void allcosets(u64 u){
  u64 ub[16]; for (int j = 0; j < d; j++) ub[j] = mul(u, hb[j]);
  Basis B = {{0}, 0}; for (int j = 0; j < d; j++) badd(&B, ub[j]);
  u64 cb[64]; int nc = 0; for (int i = 0; i < N; i++) if (badd(&B, 1ULL << i)) cb[nc++] = 1ULL << i;
  // inner Gray-code offsets of the line
  static u64 off[256]; off[0] = 0; for (int k = 1; k < Q; k++) off[k] = off[k-1] ^ ub[__builtin_ctz(k)];
  u64 c0 = 0;
  for (u64 k = 0; k < (1ULL << nc); k++){
    if (k) c0 ^= cb[__builtin_ctzll(k)];
    int n0 = 0, n1 = 0;
    for (int j = 0; j < Q; j++){ unsigned char t = TAB[c0 ^ off[j]]; if (t == 0xff) continue; if (t & 1){ if (t & 2) n1++; else n0++; } }
    int p = n0 > n1 ? n0 : n1;
    if (!k) continue; // through-0 line: skip (classified in Exp 13)
    hist[p]++; naff++; if (p > histmax) histmax = p;
    if (2 * p > Q) nbeat++;
    if (p == Q){ if (inP(c0)) nfullP++; else nfullNotP++; }
  }
}
static void exh(int zeroonly){
  buildtab();
  long long ndir = 0;
  if (zeroonly){
    // F2 basis of ker T, then an F_Q-basis e1,e2,e3
    Basis K0 = {{0}, 0}; u64 kb[64]; int nk = 0;
    // kernel by brute: T is F2-linear; solve via image matrix
    u64 img[64]; for (int i = 0; i < N; i++) img[i] = TQ(1ULL << i);
    // Gaussian elimination tracking combinations
    u64 rows[64], comb[64]; for (int i = 0; i < N; i++){ rows[i] = img[i]; comb[i] = 1ULL << i; }
    int r = 0; for (int bit = N - 1; bit >= 0; bit--){ int p = -1; for (int i = r; i < N; i++) if (rows[i] >> bit & 1){ p = i; break; }
      if (p < 0) continue; u64 t = rows[p]; rows[p] = rows[r]; rows[r] = t; t = comb[p]; comb[p] = comb[r]; comb[r] = t;
      for (int i = 0; i < N; i++) if (i != r && rows[i] >> bit & 1){ rows[i] ^= rows[r]; comb[i] ^= comb[r]; } r++; }
    for (int i = r; i < N; i++) kb[nk++] = comb[i];
    (void)K0;
    u64 e[3]; int ne = 0; Basis S = {{0}, 0};
    for (int i = 0; i < nk && ne < 3; i++){ if (bred(&S, kb[i]) == 0) continue; e[ne++] = kb[i]; for (int j = 0; j < d; j++) badd(&S, mul(kb[i], hb[j])); }
    for (int i = 0; i < Q; i++) for (int j = 0; j < Q; j++){ u64 u = e[0] ^ mul(FQ[i], e[1]) ^ mul(FQ[j], e[2]); if (TQ(u)) { printf("ERR T(u)!=0\n"); exit(1); } allcosets(u); ndir++; }
    for (int i = 0; i < Q; i++){ allcosets(e[1] ^ mul(FQ[i], e[2])); ndir++; }
    allcosets(e[2]); ndir++;
  } else {
    // all directions: canonical representative = u with u*F_Q*: mark visited
    unsigned char *seen = calloc((MASKN >> 3) + 1, 1);
    for (u64 u = 1; u <= MASKN; u++){ if (seen[u >> 3] >> (u & 7) & 1) continue;
      for (int i = 1; i < Q; i++){ u64 v = mul(u, FQ[i]); seen[v >> 3] |= 1 << (v & 7); }
      allcosets(u); ndir++; }
  }
  printf("directions %lld, affine lines %lld, max pool %d, lines with pool > Q/2: %lld, pool = Q: %lld in sqrt(b)F_{Q^2}, %lld outside\n",
    ndir, naff, histmax, nbeat, nfullP, nfullNotP);
  printf("pool histogram (pool:count):"); for (int p = 0; p <= Q; p++) if (hist[p]) printf(" %d:%lld", p, hist[p]); printf("\n");
}
static int poolof(u64 u, u64 c0){ int n0 = 0, n1 = 0; for (int j = 0; j < Q; j++){ u64 x = c0 ^ mul(u, FQ[j]); if (!x) continue;
    if (lifts(x)){ if (Tr(x) ^ ta) n1++; else n0++; } } return n0 > n1 ? n0 : n1; }
static void samp(long long nl){
  const char *nm[3] = {"T(u)=0,Tr(c0)=Tr(a)", "T(u)=0,random c0", "random u,c0"};
  for (int mode = 0; mode < 3; mode++){
    long long h[257] = {0}; int mx = 0; double s = 0, s2 = 0; long long nb = 0, cnt = 0;
    while (cnt < nl){ u64 u, c0;
      do u = rel(); while (!u || (mode < 2 && TQ(u) != 0));
      do c0 = rel(); while (mode == 0 && Tr(c0) != ta);
      // reject c0 in u*F_Q (through 0)
      { u64 z = mul(c0, inv(u)); if (frobk(z, d) == z) continue; }
      int p = poolof(u, c0); h[p]++; cnt++; if (p > mx) mx = p; s += p; s2 += (double)p * p; if (2 * p > Q) nb++; }
    double m = s / nl, sd = __builtin_sqrt(s2 / nl - m * m);
    printf("%-22s n=%lld mean %.3f sd %.3f max %d (max-Q/2)/sqrtQ %.3f  #pool>Q/2 %lld (%.4f)\n   hist:", nm[mode], nl, m, sd, mx, (mx - Q / 2.0) / __builtin_sqrt(Q), nb, (double)nb / nl);
    for (int p = 0; p <= Q; p++) if (h[p]) printf(" %d:%lld", p, h[p]); printf("\n"); fflush(stdout); }
}
int main(int argc, char **argv){
  if (argc < 5){ fprintf(stderr, "usage\n"); return 1; }
  setup(atoi(argv[2]), atoi(argv[3]), strtoull(argv[4], 0, 10));
  // self-check inverse
  for (int i = 0; i < 1000; i++){ u64 x = rel(); if (x && mul(x, inv(x)) != 1){ printf("inverse bug\n"); return 1; } }
  if (!strcmp(argv[1], "exh0")) exh(1);
  else if (!strcmp(argv[1], "exhall")) exh(0);
  else samp(atoll(argv[5]));
  return 0; }
