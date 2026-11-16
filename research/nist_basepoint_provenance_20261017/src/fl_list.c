// Same FreeLIP search as freelip_brute3.c, over seeds listed in a file.
#define main brute_main
#include "freelip_brute3.c"
#undef main
int main(int argc, char **argv) {
    mpz_init(Q); mpz_ui_pow_ui(Q, 2, 107); mpz_sub_ui(Q, Q, 1);
    mpz_init_set_ui(A, 3); mpz_powm_ui(A, A, 121, Q);
    for (size_t c = 0; c < NC; c++) {
        mpz_init_set_str(C[c].p, C[c].p_hex, 16); C[c].pbits = mpz_sizeinbase(C[c].p, 2); mpz_init(C[c].gx);
        if (C[c].gx_hex) mpz_set_str(C[c].gx, C[c].gx_hex, 16);
    }
    unsigned long *S = malloc(sizeof(unsigned long) * 400000); long n = 0;
    FILE *f = fopen(argv[1], "r"); while (n < 400000 && fscanf(f, "%lu", &S[n]) == 1) n++; fclose(f);
    // plant: synthetic P-256 w62 point from the date seed 19970816, offset 9
    { uint64_t l[512]; int w = 62; stream(19970816UL, w, l, 512); form_x(C[6].gx, l + 9, nlimbs(C[6].pbits, w), w, C[6].p); }
    { uint64_t l[512]; int w = 30; stream(980403UL, w, l, 512); form_x(C[7].gx, l + 30, nlimbs(C[7].pbits, w), w, C[7].p); }
    long hits = 0;
    #pragma omp parallel for schedule(dynamic, 64) reduction(+:hits)
    for (long k = 0; k < n; k++) {
        uint64_t l[OFFMAX + NMAXLIMB]; mpz_t x; mpz_init(x);
        for (int wi = 0; wi < 3; wi++) {
            int w = W[wi]; stream(S[k], w, l, OFFMAX + NMAXLIMB);
            for (size_t c = 0; c < NC; c++) { int nl = nlimbs(C[c].pbits, w);
                for (int off = 0; off < OFFMAX; off++) { form_x(x, l + off, nl, w, C[c].p);
                    if (mpz_cmp(x, C[c].gx) == 0) {
                        #pragma omp critical
                        printf("HIT %-40s seed=%lu NBITS=%d offset=%d\n", C[c].name, S[k], w, off);
                        hits++; } } } }
        mpz_clear(x);
    }
    printf("done: %ld listed seeds, NBITS {26,30,62}, offsets 0..%d, hits %ld\n", n, OFFMAX - 1, hits);
    return 0;
}
