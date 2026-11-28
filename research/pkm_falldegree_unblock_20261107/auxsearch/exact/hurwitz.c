/* Hurwitz class number H(N), N > 0, N = 0,3 mod 4: weighted count of all
   (not necessarily primitive) reduced forms (a,b,c), b^2 - 4ac = -N,
   |b| <= a <= c, b >= 0 if |b| = a or a = c; weight 1/2 for a = c, b = 0;
   1/3 for a = b = c.  Returned as 6*H (an integer).
   Modes:
     hurwitz H N...                 print 6H(N) per N
     hurwitz selftest p             compare the accepted-curve count of every t
                                    with a full enumeration of E(a,b)/F_p
     hurwitz predict p t_lo t_hi    exact P_accept(t) for the pilot's search  */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <math.h>
typedef int64_t i64; typedef unsigned __int128 u128;
static i64 H6(i64 N) {
  if (N <= 0 || (N % 4 != 0 && N % 4 != 3)) return 0;
  i64 tot = 0, amax = (i64)sqrtl((long double)N / 3.0) + 2;
  for (i64 a = 1; a <= amax; a++) {
    i64 b0 = (N % 2) ? 1 : 0;               /* b = N mod 2 */
    for (i64 b = -a + ((a + b0) % 2 ? 1 : 0); b <= a; b += 2) {
      if (((b % 2) + 2) % 2 != b0) continue;
      i64 num = b * b + N;
      if (num % (4 * a)) continue;
      i64 c = num / (4 * a);
      if (c < a) continue;
      if ((b < 0) && (-b == a || a == c)) continue;
      if (a == c && b == 0) tot += 3;        /* 1/2 */
      else if (a == b && b == c) tot += 2;   /* 1/3 */
      else tot += 6;
    }
  }
  return tot;
}
static i64 mulmod(i64 a, i64 b, i64 p) { return (i64)((u128)((uint64_t)a) * (uint64_t)b % (uint64_t)p); }
static int v2(i64 n) { if (n == 0) return 63; int v = 0; while (!(n & 1)) { n >>= 1; v++; } return v; }
int main(int argc, char **argv) {
  if (argc >= 3 && !strcmp(argv[1], "H")) {
    for (int i = 2; i < argc; i++) printf("%lld %lld\n", atoll(argv[i]), (long long)H6(atoll(argv[i])));
    return 0;
  }
  if (argc == 3 && !strcmp(argv[1], "selftest")) {
    i64 p = atoll(argv[2]);
    /* Legendre table */
    int *chi = calloc(p, sizeof(int));
    for (i64 x = 1; x < p; x++) chi[mulmod(x, x, p)] = 1;
    int tmax = 0; while ((1LL << (tmax + 1)) <= 4 * p) tmax++;
    i64 *acc = calloc(64, sizeof(i64)); i64 ncurves = 0;
    for (i64 A = 0; A < p; A++) for (i64 B = 0; B < p; B++) {
      i64 disc = (4 * mulmod(mulmod(A, A, p), A, p) + 27 * mulmod(B, B, p)) % p;
      if (disc == 0) continue;
      ncurves++;
      i64 n = 1; int roots = 0;
      for (i64 x = 0; x < p; x++) {
        i64 r = (mulmod(mulmod(x, x, p), x, p) + mulmod(A, x, p) + B) % p;
        if (r == 0) { n += 1; roots++; } else if (chi[r]) n += 2;
      }
      int full2 = (roots == 3);
      for (int t = 1; t < 40; t++) if (v2(n) >= t && !full2) acc[t]++;
    }
    /* formula: accepted(t) = (p-1)/2 * sum_{s = p+1 mod 2^t, |s|<=2sqrt p} [H(4p-s^2) - H((4p-s^2)/4)]
       where the second term only for s even (full 2-torsion needs 2 | #E, i.e. s = p+1 = 0 mod 2) */
    i64 r = (i64)sqrtl((long double)4 * p); while (r * r > 4 * p) r--;
    int ok = 1;
    for (int t = 1; (1LL << t) <= 2 * r + 1 && t < 40; t++) {
      i64 M = 1LL << t, sum6 = 0;
      for (i64 s = -r; s <= r; s++) {
        if ((((p + 1 - s) % M) + M) % M) continue;
        i64 D = 4 * p - s * s;
        sum6 += H6(D) - ((s % 2 == 0 && D % 4 == 0) ? H6(D / 4) : 0);
      }
      /* accepted = (p-1)/2 * sum6/6 */
      i64 pred = (p - 1) * sum6; /* = 12 * accepted if exact */
      int good = (pred == 12 * acc[t]);
      ok &= good;
      printf("p=%lld t=%d enumerated=%lld formula=%.3f %s\n", (long long)p, t, (long long)acc[t], pred / 12.0, good ? "ok" : "MISMATCH");
    }
    printf("curves=%lld %s\n", (long long)ncurves, ok ? "SELFTEST OK" : "SELFTEST FAIL");
    return ok ? 0 : 1;
  }
  if (argc == 5 && !strcmp(argv[1], "predict")) {
    i64 p = atoll(argv[2]); int tlo = atoi(argv[3]), thi = atoi(argv[4]);
    i64 r = (i64)sqrtl((long double)4 * p); while (r * r > 4 * p) r--;
    for (int t = tlo; t <= thi; t++) {
      i64 M = 1LL << t; long double sum = 0; int ntr = 0;
      const char *sh = getenv("SHARD"); int k = 0, K = 1, idx = -1;
      if (sh) sscanf(sh, "%d/%d", &k, &K);
      for (i64 s = -r; s <= r; s++) {
        if ((((p + 1 - s) % M) + M) % M) continue;
        if (++idx % K != k) continue;
        i64 D = 4 * p - s * s;
        i64 h = H6(D), h4 = (s % 2 == 0 && D % 4 == 0) ? H6(D / 4) : 0;
        sum += (long double)(h - h4); ntr++;
        fprintf(stderr, "t=%d s=%lld 6H(D)=%lld 6H(D/4)=%lld\n", t, (long long)s, (long long)h, (long long)h4);
      }
      /* fraction of all (a,b) in F_p^2: (p-1)/2 * sum/6 / p^2 */
      long double P = (long double)(p - 1) / 2.0L * sum / 6.0L / ((long double)p * p);
      printf("{\"t\": %d, \"traces\": %d, \"p_accept\": %.10Le, \"expected_curves_tried\": %.4Lf, \"ratio_to_2t\": %.6Lf}\n",
             t, ntr, P, 1.0L / P, (1.0L / P) / M);
      fflush(stdout);
    }
    return 0;
  }
  fprintf(stderr, "usage\n"); return 2;
}
