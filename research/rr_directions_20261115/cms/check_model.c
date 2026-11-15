// check_model CNF SOLVER_OUTPUT : verifies the `v` line of the solver output against the DIMACS+XOR file.
// Clause: some literal true. `x l1..lk 0`: XOR of literals (as truth values) = 1. Empty clause: unsatisfied.
// exit 0 = model satisfies every line; 1 = violated; 2 = usage/parse problem.
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(int argc, char** argv) {
    if (argc < 3) return 2;
    FILE* o = fopen(argv[2], "r"); if (!o) return 2;
    long cap = 1 << 20; signed char* val = calloc(cap, 1); long maxv = 0;
    char* line = NULL; size_t n = 0;
    while (getline(&line, &n, o) > 0) {
        if (line[0] != 'v') continue;
        char* p = line + 1; char* e;
        for (;;) { long l = strtol(p, &e, 10); if (e == p) break; p = e; if (l == 0) continue;
            long v = labs(l); if (v >= cap) { signed char* nv = calloc(v * 2, 1); memcpy(nv, val, cap); free(val); val = nv; cap = v * 2; }
            val[v] = l > 0 ? 1 : -1; if (v > maxv) maxv = v; }
    }
    fclose(o);
    FILE* f = fopen(argv[1], "r"); if (!f) return 2;
    long lineno = 0, bad = 0, nlines = 0;
    while (getline(&line, &n, f) > 0) {
        lineno++;
        if (line[0] == 'p' || line[0] == 'c' || line[0] == '\n') continue;
        int isx = line[0] == 'x'; char* p = line + isx; char* e;
        int sat = 0, par = 0, cnt = 0;
        for (;;) { long l = strtol(p, &e, 10); if (e == p) break; p = e; if (l == 0) break; cnt++;
            long v = labs(l); int t = (v <= maxv && val[v] != 0) ? (val[v] > 0) : 0;  // unassigned = false
            if (l < 0) t = !t; sat |= t; par ^= t; }
        nlines++;
        int ok = isx ? par : sat;
        if (!ok) { bad++; if (bad <= 3) fprintf(stderr, "violated line %ld\n", lineno); }
    }
    printf("lines=%ld violated=%ld\n", nlines, bad);
    return bad ? 1 : 0;
}
