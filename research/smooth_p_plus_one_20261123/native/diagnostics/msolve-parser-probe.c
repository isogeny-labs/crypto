/* Standalone diagnostic harness for the attributed upstream GPL parser excerpt.
 * Not linked into the research driver. No solver or algebra backend is called.
 * Build: cc -std=c11 -O2 -Wall -Wextra -o parser-probe msolve-parser-probe.c
 */
#define _POSIX_C_SOURCE 200809L
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef int32_t len_t;
typedef int32_t nelts_t;
typedef struct {
    int32_t nvars;
    char **vnames;
    int32_t *cfs;
    int32_t *exps;
} data_gens_ff_t;

#include "msolve-0.7.5-parser.inc"

int main(int argc, char **argv) {
    if (argc != 4 || strlen(argv[2]) > 512 || strlen(argv[3]) > 4096) return 2;
    char *end = NULL;
    long prime = strtol(argv[1], &end, 10);
    if (*end || (prime != 127 && prime != 8191)) return 2;
    for (const char *p = argv[3]; *p; ++p)
        if (!(isalnum((unsigned char)*p) || strchr("_^*+-", *p))) return 2;
    char *names = strdup(argv[2]);
    char *poly = strdup(argv[3]);
    char *variables[16], *save = NULL;
    int count = 0;
    if (!names || !poly) return 3;
    for (char *p = strtok_r(names, ",", &save); p; p = strtok_r(NULL, ",", &save)) {
        if (count == 16 || strlen(p) > 32) return 2;
        variables[count++] = p;
    }
    nelts_t terms = get_number_of_terms(poly);
    if (!count || !*poly || terms < 1 || terms > 256) return 2;
    data_gens_ff_t gens = {count, variables, calloc(terms, sizeof(int32_t)),
                          calloc(terms * count, sizeof(int32_t))};
    if (!gens.cfs || !gens.exps) return 3;
    if (get_coefficient_ff_and_term_from_line(poly, terms, prime, &gens, 0)) return 4;
    putchar('[');
    for (int i = 0; i < terms; ++i) {
        if (i) putchar(',');
        printf("{\"coefficient\":%d,\"exponents\":[", gens.cfs[i]);
        for (int j = 0; j < count; ++j) {
            if (j) putchar(',');
            printf("%d", gens.exps[i * count + j]);
        }
        printf("]}");
    }
    puts("]");
    free(gens.cfs);
    free(gens.exps);
    free(names);
    free(poly);
    return 0;
}
