/* Independent #E(F_p) for y^2 = x^3 + a x + b by the naive count (Amendment R4 draft, edit E4.7):
 * #E = 1 + sum_x (1 + (x^3 + a x + b | p)), Jacobi symbol by binary reciprocity, x split over threads.
 * Shares no code with pkm_tower_pilot. Usage: naive_count P A B [THREADS]  -> prints the count. p < 2^62. */
#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
typedef unsigned __int128 u128;
static uint64_t P, A, B; static int T;
static int jacobi(uint64_t a, uint64_t n) {
  int r = 1; a %= n;
  while (a) {
    while (!(a & 1)) { a >>= 1; uint64_t m = n & 7; if (m == 3 || m == 5) r = -r; }
    uint64_t t = a; a = n; n = t;
    if ((a & 3) == 3 && (n & 3) == 3) r = -r;
    a %= n;
  }
  return n == 1 ? r : 0;
}
static void *work(void *arg) {
  long id = (long)arg; int64_t s = 0;
  for (uint64_t x = (uint64_t)id; x < P; x += (uint64_t)T) {
    uint64_t x2 = (uint64_t)((u128)x * x % P);
    uint64_t rhs = (uint64_t)(((u128)x2 * x + (u128)A * x + B) % P);
    s += 1 + jacobi(rhs, P);
  }
  int64_t *o = malloc(sizeof *o); *o = s; return o;
}
int main(int argc, char **argv) {
  if (argc < 4) { fprintf(stderr, "usage: naive_count P A B [THREADS]\n"); return 2; }
  P = strtoull(argv[1], 0, 10); A = strtoull(argv[2], 0, 10) % P; B = strtoull(argv[3], 0, 10) % P;
  T = argc > 4 ? atoi(argv[4]) : 4;
  pthread_t th[256]; int64_t total = 1;
  for (long i = 0; i < T; i++) pthread_create(&th[i], 0, work, (void *)i);
  for (int i = 0; i < T; i++) { void *r; pthread_join(th[i], &r); total += *(int64_t *)r; free(r); }
  printf("%lld\n", (long long)total);
  return 0;
}
