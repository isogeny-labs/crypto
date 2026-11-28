/* FLINT nmod_poly root finder for matched native comparison.
 *
 * stdin lines:  p deg c0 c1 ... c_deg
 * stdout lines: ok ns root0 root1 ...   OR   err <message>
 *
 * Roots are the distinct F_p zeros of the dense polynomial, sorted.
 * Wall time is clock_gettime(CLOCK_MONOTONIC) around nmod_poly_factor +
 * extraction of linear factors only (rational roots).
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <time.h>

#include <flint/flint.h>
#include <flint/nmod_poly.h>
#include <flint/nmod_poly_factor.h>

static uint64_t mono_ns(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

static int cmp_u64(const void *a, const void *b) {
    uint64_t x = *(const uint64_t *)a;
    uint64_t y = *(const uint64_t *)b;
    return (x > y) - (x < y);
}

int main(void) {
    char *line = NULL;
    size_t cap = 0;
    while (getline(&line, &cap, stdin) > 0) {
        unsigned long p = 0;
        int deg = -1;
        char *save = NULL;
        char *tok = strtok_r(line, " \t\n", &save);
        if (!tok) {
            printf("err empty\n");
            fflush(stdout);
            continue;
        }
        p = strtoul(tok, NULL, 10);
        tok = strtok_r(NULL, " \t\n", &save);
        if (!tok) {
            printf("err missing_deg\n");
            fflush(stdout);
            continue;
        }
        deg = atoi(tok);
        if (p < 2 || deg < 0 || deg > 64) {
            printf("err bad_header\n");
            fflush(stdout);
            continue;
        }
        nmod_poly_t f;
        nmod_poly_init(f, p);
        nmod_poly_fit_length(f, deg + 1);
        for (int i = 0; i <= deg; i++) {
            tok = strtok_r(NULL, " \t\n", &save);
            if (!tok) {
                nmod_poly_clear(f);
                printf("err missing_coeff\n");
                fflush(stdout);
                goto next;
            }
            nmod_poly_set_coeff_ui(f, i, strtoul(tok, NULL, 10) % p);
        }
        nmod_poly_factor_t fac;
        nmod_poly_factor_init(fac);
        uint64_t t0 = mono_ns();
        nmod_poly_factor(fac, f);
        uint64_t t1 = mono_ns();

        uint64_t roots[128];
        int nroots = 0;
        for (slong i = 0; i < fac->num; i++) {
            if (nmod_poly_degree(fac->p + i) == 1 && fac->exp[i] > 0) {
                /* ax + b = 0 => x = -b * a^{-1} */
                ulong a = nmod_poly_get_coeff_ui(fac->p + i, 1);
                ulong b = nmod_poly_get_coeff_ui(fac->p + i, 0);
                ulong inv = n_invmod(a, p);
                ulong x = n_mulmod2_preinv((p - (b % p)) % p, inv, p,
                                          n_preinvert_limb(p));
                if (nroots < 128) {
                    roots[nroots++] = (uint64_t)x;
                }
            }
        }
        qsort(roots, (size_t)nroots, sizeof(uint64_t), cmp_u64);
        /* dedup */
        int w = 0;
        for (int i = 0; i < nroots; i++) {
            if (w == 0 || roots[i] != roots[w - 1]) {
                roots[w++] = roots[i];
            }
        }
        nroots = w;

        printf("ok %llu", (unsigned long long)(t1 - t0));
        for (int i = 0; i < nroots; i++) {
            printf(" %llu", (unsigned long long)roots[i]);
        }
        printf("\n");
        fflush(stdout);

        nmod_poly_factor_clear(fac);
        nmod_poly_clear(f);
    next:;
    }
    free(line);
    return 0;
}
