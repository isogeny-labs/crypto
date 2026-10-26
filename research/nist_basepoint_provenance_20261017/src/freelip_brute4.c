// Small-seed FreeLIP brute force for base-point x coordinates.
// FreeLIP (lip.c): zrstarts(s) sets seed = s; zrandom(b): seed <- seed*3^121 mod (2^107-1), return seed mod b.
// zrandomb(p): (#NBITS-digits of p)+1 calls zrandom(2^NBITS), most significant first, result mod p.
// For every seed 1..SMAX, NBITS in {26, 30, 62}, and every start offset 0..OFFMAX-1 into the output stream,
// form x = zrandomb(p) and compare with the published Gx. Controls: P-192 and prime239v1 (known FreeLIP
// points) and synthetic points built here with NBITS 30 and 62.
#include <gmp.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#ifndef OFFMAX
#define OFFMAX 256
#endif
#define NMAXLIMB 32

typedef struct { const char *name; const char *p_hex; const char *gx_hex; mpz_t p, gx; int pbits; } curve_t;

static curve_t C[] = {
 {"P-192 (control, seed 20000 w26)", "fffffffffffffffffffffffffffffffeffffffffffffffff", "188da80eb03090f67cbf20eb43a18800f4ff0afd82ff1012"},
 {"prime239v1 (control, seed 20420 w26)", "7fffffffffffffffffffffff7fffffffffff8000000000007fffffffffff", "0ffa963cdca8816ccc33b8642bedf905c3d358573d3f27fbbd3b3cb9aaaf"},
 {"P-224", "ffffffffffffffffffffffffffffffff000000000000000000000001", "b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21"},
 {"P-256", "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff", "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296"},
 {"P-384", "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff", "aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab7"},
 {"P-521", "1ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66"},
 {"SYNTH P-256 w62 seed 700000000 off 9", "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff", NULL},
 {"SYNTH P-384 w30 seed 800000000 off 30", "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff", NULL},
};
#define NC (sizeof(C)/sizeof(C[0]))
static const int W[3] = {26, 30, 62};
static mpz_t Q, A;

static void stream(unsigned long seed, int w, uint64_t *out, int n) {
    mpz_t s; mpz_init_set_ui(s, seed);
    for (int i = 0; i < n; i++) {
        mpz_mul(s, s, A); mpz_mod(s, s, Q);
        out[i] = mpz_getlimbn(s, 0) & ((w == 64) ? ~0ULL : ((1ULL << w) - 1));
    }
    mpz_clear(s);
}

static int nlimbs(int pbits, int w) { return (pbits + w - 1) / w + 1; }

static void form_x(mpz_t x, const uint64_t *l, int nl, int w, const mpz_t p) {
    mpz_set_ui(x, 0);
    for (int i = 0; i < nl; i++) { mpz_mul_2exp(x, x, w); mpz_add_ui(x, x, l[i]); }
    mpz_mod(x, x, p);
}

int main(int argc, char **argv) {
    unsigned long SMAX = argc > 1 ? strtoul(argv[1], 0, 10) : (1UL << 20);
    unsigned long SMIN = argc > 2 ? strtoul(argv[2], 0, 10) : 1;
    mpz_init(Q); mpz_ui_pow_ui(Q, 2, 107); mpz_sub_ui(Q, Q, 1);
    mpz_init_set_ui(A, 3); mpz_powm_ui(A, A, 121, Q);
    for (size_t c = 0; c < NC; c++) {
        mpz_init_set_str(C[c].p, C[c].p_hex, 16); C[c].pbits = mpz_sizeinbase(C[c].p, 2); mpz_init(C[c].gx);
        if (C[c].gx_hex) mpz_set_str(C[c].gx, C[c].gx_hex, 16);
    }
    // build synthetic controls
    { uint64_t l[512]; int w = 62; stream(700000000UL, w, l, 512); form_x(C[6].gx, l + 9, nlimbs(C[6].pbits, w), w, C[6].p); }
    { uint64_t l[512]; int w = 30; stream(800000000UL, w, l, 512); form_x(C[7].gx, l + 30, nlimbs(C[7].pbits, w), w, C[7].p); }
    long hits = 0;
    #pragma omp parallel reduction(+:hits)
    {
        uint64_t l[OFFMAX + NMAXLIMB];
        mpz_t x; mpz_init(x);
        #pragma omp for schedule(dynamic, 1024)
        for (unsigned long seed = SMIN; seed <= SMAX; seed++) {
            for (int wi = 0; wi < 3; wi++) {
                int w = W[wi];
                stream(seed, w, l, OFFMAX + NMAXLIMB);
                for (size_t c = 0; c < NC; c++) {
                    int nl = nlimbs(C[c].pbits, w);
                    for (int off = 0; off < OFFMAX; off++) {
                        form_x(x, l + off, nl, w, C[c].p);
                        if (mpz_cmp(x, C[c].gx) == 0) {
                            #pragma omp critical
                            { printf("HIT %-40s seed=%lu NBITS=%d offset=%d\n", C[c].name, seed, w, off); fflush(stdout); }
                            hits++;
                        }
                    }
                }
            }
        }
        mpz_clear(x);
    }
    printf("done: seeds %lu..%lu, NBITS {26,30,62}, offsets 0..%d, hits %ld\n", SMIN, SMAX, OFFMAX - 1, hits);
    return 0;
}
