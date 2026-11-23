# VERIFY: emin-exact-large-n (independent skeptic), 2026-11-12

Every result statement here is `[pcc:unverified]`. No `pcc` claim exists for it.
Target: `RESULT.md` in this directory, which reports `e_min(31) = 5` (4/4) and `e_min(33) = 5` (3/3).
It also reports that `n = 35` at `e = 5` was not decided, because of an M4RI `int` overflow.

## Verdict

**Agrees** with the key numbers `[pcc:unverified]`. My own code re-derived both halves of
`e_min = 5`:

- **Feasible at e = 5.** All 7 lambda certificates satisfy (I) and (II) at every point of V, and each has ANF degree exactly 5.
- **Infeasible at e = 4.** I built the rows myself from the published point sets for n31_s2 and n33_s1. Both systems are inconsistent, and the ranks equal the pod's.

I found no refutation. The minor issues are listed at the end.

## What I did (own code, not the producer's scripts)

`skv.c` is the verifier, reproduced in the appendix (sha256 `ab0b7e42…f47b`). It does its own GF(2^n)
multiplication and trace, its own construction of the selector rows from Prop. S in
NOTE.md §6 (`Phi*_C(v) = v^4 + (C^2+z)v^2 + zCv`, `h_C = z^2C^2 + a6`), and its own Möbius/ANF
degree. For rank it uses either M4RI `mzd_echelonize`, a different routine from the producer's PLUQ verify,
or a plain word-wise Gaussian elimination written here. Build:
`gcc -O2 -march=native -o skv skv.c -lm4ri -lm`. It ran locally on one thread.

| check | result |
|:--|:--|
| `sha256sum -c SHA256SUMS` | all OK |
| `pod/inst_*.txt` equal `python3 emit_instance.py N SEED` for all 9 instances | match |
| field polynomials x^31+x^3+1, x^33+x^10+1, x^35+x^2+1 irreducible (own Ben-Or-style test) | yes |
| `skv count` (number of C with h_C in Phi*_C(V)), n31_s2, n33_s1, n35_s6 | 0, 0, 0, so unsatisfiable |
| emin.c sha256 = `c5ee59f6…eb22c`, last changed in `370e85e2`; HEAD `2003c4bc`; `git diff HEAD` on the research dir | as stated; no tracked-source diff |

### Feasibility certificates (e = 5), `skv lambda INST 5 lambda_e5.bin`

| instance | violated | bits above n | ANF degree | # degree-5 monomials | ok |
|:--|--:|--:|--:|--:|:--|
| n31 s2, s3, s5, s6 | 0 each | 0 | 5 | 4,368 (= C(16,5), all present) | true |
| n33 s1, s5, s8 | 0 each | 0 | 5 | 6,188 (= C(17,5)) | true |

Negative controls:

- Flipping one bit of n31_s2's lambda gives 10 violations and degree 16.
- n31_s3's lambda checked against the n31_s2 instance gives 556,908 violations.
- The same certificate with the bound set to E = 4 is rejected (degree 5 > 4).

### Infeasibility (e = 4, and e = 3), rows built from the published `points_e4.txt`

| instance | e | points | cols | rows | elimination | rank A | rank A\|b | result | pod `verify_e*.json` | time |
|:--|--:|--:|--:|--:|:--|--:|--:|:--|:--|--:|
| n31 s2 | 4 | 5,228 | 78,027 | 88,876 | M4RI echelonize | 77,451 | 77,452 | inconsistent | 77,451 / 77,452 | 106 s |
| n33 s1 | 4 | 6,690 | 106,062 | 120,420 | M4RI echelonize | 105,365 | 105,366 | inconsistent | 105,365 / 105,366 | 290 s |
| n31 s2 | 3 | 5,228 | 21,607 | 88,876 | **own Gauss** | 21,486 | 21,487 | inconsistent | 21,486 / 21,487 (1,495 pts) | 267 s |
| n31 s2 | 3 | same | same | same | M4RI echelonize | 21,486 | 21,487 | inconsistent | — | 16 s |
| n35 s6 | 3 | 8,448 | 34,580 | 160,512 | M4RI echelonize | 34,426 | 34,427 | inconsistent | 34,426 / 34,427 | 70 s |
| positive control: n31 s2, first 1,000 points | 4 | 1,000 | 78,027 | 17,000 | M4RI | 16,000 | 16,000 | **consistent** | — | — |

Infeasibility on a subset of points implies infeasibility on all of V. Degree-≤e selectors are nested,
so `e = 4` infeasible and `e = 5` feasible give `e_min = 5` for n31_s2 and n33_s1. For the other 5
instances the e = 4 infeasibility rests on the producer's pod logs only: the rank figures are identical,
and the files hash-check, but I did not recompute them.
I did not run n35 at e = 4: about 2.8 GB of matrix, at the 3 GB cap.

### Other numbers re-derived

- Columns `n·M(k,e)` and `rho_A = (k+1)M(k,e+1)/(n M(k,e))`: 78,027 / 213,435 / 106,062 / 310,266 /
  441,560 and 1.5001 / 1.1862 / 1.5956 / 1.2634 / 1.3417 all reproduce.
- The thresholds 1.2194 and 1.2698 match `out/lead_A2/RESULT.md`. The verdict table in §4 applies them correctly.
  For n = 31, 1.186 < 1.2194 and the prediction holds. n = 33 sits in the ambiguous band and is not scored. n = 35 is untested.
  The narrowing to theta_A in (1.2634, 1.2698) follows, given A's single-threshold form.
- Wall times and point counts from `line.json` (2,415 / 2,410 / 2,458 / 2,434 s; 7,408 / 7,421 / 7,245 s;
  15,819 and 23,877 pts) match RESULT.md. Peak RSS is 11.3 GB for n31 and 24.6 GB for n33 s1/s5.
- n = 35: both `solve.log` files end in `mzd.h:306: mzd_row: Assertion 'result == M->rows[row]' failed`,
  `Command terminated by signal 6`, after 5:20:26 and 5:14:41, with peak RSS 23.9 GB.
  - I read `/usr/include/m4ri/mzd.h`: `mzd_row` computes `wi_t big_vector = offset_vector + row * rowstride` with `wi_t = int` (misc.h:80).
  - 441,560 × 6,900 words ≈ 3.05e9 > 2^31, while n = 33 e = 5 is about 1.5e9. The diagnosis fits.
  - The producer labelled it untested, and I did not test it either.
  - e = 5 feasibility does not depend on M4RI's correctness, because the certificates were checked independently.

### RunPod

- At 22:48 UTC I listed the account's pods with GET `rest.runpod.io/v1/pods`; it returned 28 pods. None is named `bdr-*`, and the id `jozvsiy9t8avys` is absent. I created no pods.
- Created 16:07:58 and deleted 22:37:46 gives 6.497 h, × 1.04 USD/h = 6.76 USD, consistent with the report. These times come from the producer, and I could not check them after the deletion.

## Issues (none changes the verdict)

1. **Precedence of the preregistration cannot be verified.** The output directory is untracked
   (`git status: ??`), and RESULT.md's mtime is 22:38, after the runs. The thresholds do match lead_A2, and its
   n = 31 cell is not borderline (1.186 against 1.219).
2. **Independence of the e = 4 infeasibility proof.** At e = 4 the elimination uses M4RI, the same
   library as the producer though a different routine, with my own row construction. My own Gauss
   covered only e = 3, where it agreed with M4RI. An e = 4 run with my own Gauss would take an estimated 4e12 word operations, more than the 20-minute limit.
3. **Small reporting error.** RESULT.md gives n33_s8's peak RSS as "n.r.", but `cert/n33_s8/solve.log` records
   24,581,952 kB.
4. **Malformed JSON.** `cert/n35_s*/line.json` and `solve.json` are not valid JSON, because they were truncated by the abort.
   This is cosmetic, but any tool that parses every `line.json` will fail on them.
5. **e = 4 infeasibility was recomputed on 2 of 7 instances only.** The other 5 rely on the pod logs.

## Appendix: skv.c (as run)

```c
// Independent skeptic verifier for selector certificates (not derived from emin.c).
// Usage:
//   skv lambda INSTFILE E LAMBDA.bin        -> check (I),(II) at every C in V and ANF degree <= E
//   skv infeas INSTFILE E POINTS.txt [own]  -> build selector rows for the points at degree E,
//                                             rank(A) vs rank(A|b) (M4RI echelonize, or own Gauss with 'own')
//   skv count INSTFILE                      -> # C with h_C in Phi*_C(V) (solutions)
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <m4ri/m4ri.h>

static int n, k; static uint64_t poly, a6, z, V[64], trmask;
static uint64_t gmul(uint64_t a, uint64_t b) {
  uint64_t r = 0; uint64_t top = 1ULL << n;
  for (int i = 0; i < n; i++) { if (b >> i & 1) r ^= a; a <<= 1; if (a & top) a ^= poly; }
  return r;
}
static int tr(uint64_t x) { // Tr(x) = sum x^(2^i)
  uint64_t s = 0, y = x; for (int i = 0; i < n; i++) { s ^= y; y = gmul(y, y); }
  return (int)(s & 1); // trace lies in F2 = {0,1}
}
static int par(uint64_t x) { return __builtin_parityll(x); }
static void load(const char *f) {
  FILE *F = fopen(f, "r"); unsigned long long p, A, Z, v;
  if (fscanf(F, "%d %d %llu %llu %llu", &n, &k, &p, &A, &Z) != 5) exit(2);
  poly = p; a6 = A; z = Z;
  for (int i = 0; i < k; i++) { if (fscanf(F, "%llu", &v) != 1) exit(2); V[i] = v; }
  fclose(F);
  if (!((poly >> n) & 1) || (poly >> (n + 1))) { fprintf(stderr, "poly degree mismatch\n"); exit(3); }
  trmask = 0; for (int t = 0; t < n; t++) if (tr(1ULL << t)) trmask |= 1ULL << t;
}
static uint64_t spanv(uint64_t m) { uint64_t s = 0; for (int j = 0; j < k; j++) if (m >> j & 1) s ^= V[j]; return s; }
static uint64_t phis(uint64_t C, uint64_t v) { // v^4 + (C^2+z) v^2 + z C v
  uint64_t v2 = gmul(v, v); return gmul(v2, v2) ^ gmul(gmul(C, C) ^ z, v2) ^ gmul(gmul(z, C), v);
}
static uint64_t hC(uint64_t C) { return gmul(gmul(z, z), gmul(C, C)) ^ a6; }
// is h in F2-span of w[0..k-1] ?
static int inspan(uint64_t *w, uint64_t h) {
  uint64_t b[64]; int r = 0;
  for (int i = 0; i < k; i++) { uint64_t x = w[i]; for (int j = 0; j < r; j++) if ((x ^ b[j]) < x) x ^= b[j]; if (x) { b[r++] = x; for (int a = r - 1; a > 0 && b[a] > b[a - 1]; a--) { uint64_t t = b[a]; b[a] = b[a - 1]; b[a - 1] = t; } } }
  for (int j = 0; j < r; j++) if ((h ^ b[j]) < h) h ^= b[j];
  return h == 0;
}
int main(int argc, char **argv) {
  load(argv[2]);
  uint64_t P = 1ULL << k;
  if (!strcmp(argv[1], "count")) {
    long sol = 0;
    for (uint64_t ci = 0; ci < P; ci++) { uint64_t C = spanv(ci), w[64]; for (int j = 0; j < k; j++) w[j] = phis(C, V[j]); if (inspan(w, hC(C))) sol++; }
    printf("{\"n\":%d,\"k\":%d,\"C_with_solution\":%ld}\n", n, k, sol); return 0;
  }
  int E = atoi(argv[3]);
  if (!strcmp(argv[1], "lambda")) {
    FILE *F = fopen(argv[4], "rb"); fseek(F, 0, SEEK_END); long sz = ftell(F); rewind(F);
    if (sz != (long)(P * 8)) { printf("size mismatch %ld\n", sz); return 1; }
    uint64_t *L = malloc(P * 8); if (fread(L, 8, P, F) != P) return 2; fclose(F);
    long bad = 0, highbits = 0;
    for (uint64_t ci = 0; ci < P; ci++) {
      uint64_t C = spanv(ci), l = L[ci]; if (l >> n) highbits++;
      for (int j = 0; j < k; j++) if (par(gmul(l, phis(C, V[j])) & trmask)) bad++;
      if (par(gmul(l, hC(C)) & trmask) != 1) bad++;
    }
    for (int i = 0; i < k; i++) for (uint64_t c = 0; c < P; c++) if (c >> i & 1) L[c] ^= L[c ^ (1ULL << i)];
    int deg = -1; long topcnt = 0; for (uint64_t c = 0; c < P; c++) if (L[c]) { int d = __builtin_popcountll(c); if (d > deg) { deg = d; topcnt = 0; } if (d == deg) topcnt++; }
    printf("{\"n\":%d,\"k\":%d,\"violated\":%ld,\"bits_above_n\":%ld,\"anf_degree\":%d,\"top_degree_monomials\":%ld,\"ok\":%s}\n", n, k, bad, highbits, deg, topcnt, (bad == 0 && highbits == 0 && deg <= E) ? "true" : "false");
    return 0;
  }
  if (!strcmp(argv[1], "infeas")) {
    // points
    FILE *F = fopen(argv[4], "r"); uint64_t *pts = malloc(sizeof(uint64_t) * 200000); long np = 0; unsigned long long x;
    while (fscanf(F, "%llu", &x) == 1) { if (x >= P) { fprintf(stderr, "point out of range\n"); return 2; } pts[np++] = x; }
    fclose(F);
    // monomials of degree <= E over k vars
    uint64_t *mon = malloc(sizeof(uint64_t) * 2000000); long nm = 0;
    for (uint64_t m = 0; m < P; m++) if (__builtin_popcountll(m) <= E) mon[nm++] = m;
    long cols = nm * n, rows = np * (k + 1);
    int own = argc > 5 && !strcmp(argv[5], "own");
    fprintf(stderr, "points %ld monomials %ld cols %ld rows %ld\n", np, nm, cols, rows);
    mzd_t *M = mzd_init(rows, cols + 1);
    long r = 0;
    for (long pi = 0; pi < np; pi++) {
      uint64_t ci = pts[pi], C = spanv(ci);
      uint64_t w[65]; for (int j = 0; j < k; j++) w[j] = phis(C, V[j]); w[k] = hC(C);
      // coefficient of bit t of alpha_m in Tr(alpha_m * w) is Tr(x^t w)
      uint64_t tw[65]; // bitmask over t
      for (int j = 0; j <= k; j++) { uint64_t u = w[j], msk = 0; for (int t = 0; t < n; t++) { if (par(u & trmask)) msk |= 1ULL << t; u <<= 1; if (u >> n & 1) u ^= poly; } tw[j] = msk; }
      for (int j = 0; j <= k; j++, r++) {
        if (tw[j]) for (long mi = 0; mi < nm; mi++) if ((mon[mi] & ~ci) == 0) for (int t = 0; t < n; t++) if (tw[j] >> t & 1) mzd_write_bit(M, r, mi * n + t, 1);
        if (j == k) mzd_write_bit(M, r, cols, 1);
      }
    }
    rci_t rk;
    if (!own) rk = mzd_echelonize(M, 0);
    else { // plain Gaussian elimination on raw words
      wi_t W = M->width; rk = 0;
      for (long c = 0; c <= cols && rk < rows; c++) {
        long wc = c / 64; uint64_t bit = 1ULL << (c % 64); long piv = -1;
        for (long i = rk; i < rows; i++) if (M->rows[i][wc] & bit) { piv = i; break; }
        if (piv < 0) continue;
        if (piv != rk) { word *t = M->rows[piv]; M->rows[piv] = M->rows[rk]; M->rows[rk] = t; }
        word *pr = M->rows[rk];
        for (long i = rk + 1; i < rows; i++) if (M->rows[i][wc] & bit) { word *q = M->rows[i]; for (long a = wc; a < W; a++) q[a] ^= pr[a]; }
        rk++;
      }
    }
    // inconsistent iff the last nonzero row of the echelon form has its leading 1 in column 'cols'
    int incons = 0;
    if (rk > 0) { long lead = -1; for (long c = 0; c <= cols; c++) if (M->rows[rk - 1][c / 64] >> (c % 64) & 1) { lead = c; break; } incons = (lead == cols); }
    printf("{\"n\":%d,\"k\":%d,\"e\":%d,\"points\":%ld,\"cols\":%ld,\"rows\":%ld,\"rank_A\":%d,\"rank_A_b\":%d,\"inconsistent\":%s,\"elim\":\"%s\"}\n", n, k, E, np, cols, rows, rk - incons, rk, incons ? "true" : "false", own ? "own-gauss" : "m4ri-echelonize");
    return 0;
  }
  return 1;
}
```
