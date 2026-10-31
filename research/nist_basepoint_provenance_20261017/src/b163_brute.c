// B-163 seed search: is 85e25bfe5c86226cdb12016f7553f9d0e693a268 output of a
// classic C library generator seeded with a 1997-98 timestamp (t or t/2)?
// Generators: glibc random(), 4.3BSD random(), SysV rand() (15-bit),
// BSD rand() (31-bit), MSVC rand() (15-bit). Layouts: one byte per call
// (low 8 or top 8 bits), one 16-bit half per call (low or top 16 bits, 31-bit
// generators only); seed read big-endian or byte-reversed. Search depth: the
// first M outputs after seeding. Planted positive controls for every generator.
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define M 2048
#define NG 5
static const char *GN[NG] = {"glibc random()", "4.3BSD random()", "SysV rand()", "BSD rand()", "MSVC rand()"};
static const int GW[NG] = {31, 31, 15, 31, 15};

typedef struct { int32_t r[34 + M + 400]; } rs_t;

static void gen(int g, uint32_t seed, uint32_t *out, int n) {
    if (g == 0) {               // glibc TYPE_3
        int32_t r[344 + M + 64];
        r[0] = seed ? (int32_t)seed : 1;
        for (int i = 1; i < 31; i++) {
            int32_t hi = r[i - 1] / 127773, lo = r[i - 1] % 127773;
            int32_t w = 16807 * lo - 2836 * hi;
            if (w < 0) w += 2147483647;
            r[i] = w;
        }
        for (int i = 31; i < 34; i++) r[i] = r[i - 31];
        for (int i = 34; i < 344 + n; i++) r[i] = (int32_t)((uint32_t)r[i - 31] + (uint32_t)r[i - 3]);
        for (int k = 0; k < n; k++) out[k] = ((uint32_t)r[344 + k]) >> 1;
    } else if (g == 1) {        // 4.3BSD: LCG fill, fptr=3 rptr=0, 310 discards
        uint32_t st[31];
        st[0] = seed;
        for (int i = 1; i < 31; i++) st[i] = 1103515245u * st[i - 1] + 12345u;
        int f = 3, r = 0;
        for (int k = 0; k < 310 + n; k++) {
            st[f] += st[r];
            uint32_t o = (st[f] >> 1) & 0x7fffffff;
            if (++f >= 31) f = 0;
            if (++r >= 31) r = 0;
            if (k >= 310) out[k - 310] = o;
        }
    } else if (g == 2) {
        uint32_t x = seed;
        for (int k = 0; k < n; k++) { x = x * 1103515245u + 12345u; out[k] = (x >> 16) & 0x7fff; }
    } else if (g == 3) {
        uint32_t x = seed;
        for (int k = 0; k < n; k++) { x = (x * 1103515245u + 12345u) & 0x7fffffff; out[k] = x; }
    } else {
        uint32_t x = seed;
        for (int k = 0; k < n; k++) { x = x * 214013u + 2531011u; out[k] = (x >> 16) & 0x7fff; }
    }
}

// modes: 0 low8, 1 top8, 2 low16, 3 top16
static inline uint32_t sym(uint32_t o, int mode, int w) {
    switch (mode) {
        case 0: return o & 0xff;
        case 1: return (o >> (w - 8)) & 0xff;
        case 2: return o & 0xffff;
        default: return (o >> (w - 16)) & 0xffff;
    }
}

typedef struct { char name[64]; int len; uint32_t s[20]; int bits; int only_g; } target_t;
#define MAXT 64
static target_t T[MAXT]; static int NT = 0;

static void add_bytes(const char *name, const uint8_t *b, int only_g) {
    target_t *t = &T[NT++]; snprintf(t->name, 64, "%s", name); t->len = 20; t->bits = 8; t->only_g = only_g;
    for (int i = 0; i < 20; i++) t->s[i] = b[i];
}
static void add_words(const char *name, const uint8_t *b, int le, int only_g) {
    target_t *t = &T[NT++]; snprintf(t->name, 64, "%s", name); t->len = 10; t->bits = 16; t->only_g = only_g;
    for (int i = 0; i < 10; i++) t->s[i] = le ? (b[2 * i + 1] << 8 | b[2 * i]) : (b[2 * i] << 8 | b[2 * i + 1]);
}

int main(void) {
    const uint8_t B163[20] = {0x85,0xe2,0x5b,0xfe,0x5c,0x86,0x22,0x6c,0xdb,0x12,0x01,0x6f,0x75,0x53,0xf9,0xd0,0xe6,0x93,0xa2,0x68};
    uint8_t rev[20]; for (int i = 0; i < 20; i++) rev[i] = B163[19 - i];
    add_bytes("B-163 bytes", B163, -1);
    add_bytes("B-163 bytes reversed", rev, -1);
    add_words("B-163 16-bit BE", B163, 0, -1);
    add_words("B-163 16-bit LE", B163, 1, -1);
    // positive controls: one planted target per generator and layout family
    uint32_t cseeds[NG] = {891607927u, 446413329u, 863711343u, 446413500u, 872952608u};
    uint32_t buf[M + 32];
    for (int g = 0; g < NG; g++) {
        gen(g, cseeds[g], buf, M);
        uint8_t cb[20]; int off = 777 + 111 * g, mode = (g % 2) ? 1 : 0;
        for (int i = 0; i < 20; i++) cb[i] = (uint8_t)sym(buf[off + i], mode, GW[g]);
        char nm[64]; snprintf(nm, 64, "CONTROL %s mode%d seed %u off %d", GN[g], mode, cseeds[g], off);
        add_bytes(nm, cb, g);
        if (GW[g] == 31) {
            target_t *t = &T[NT++]; snprintf(t->name, 64, "CONTROL %s top16 seed %u off %d", GN[g], cseeds[g], off + 5);
            t->len = 10; t->bits = 16; t->only_g = g;
            for (int i = 0; i < 10; i++) t->s[i] = sym(buf[off + 5 + i], 3, 31);
        }
    }
    const uint32_t lo[2] = {426038400u, 852076800u}, hi[2] = {457574400u, 915148800u};
    long hits = 0;
    for (int g = 0; g < NG; g++) {
        #pragma omp parallel for schedule(dynamic, 65536) reduction(+:hits)
        for (long q = 0; q < (long)(hi[0] - lo[0]) + (long)(hi[1] - lo[1]); q++) {
            uint32_t seed = q < (long)(hi[0] - lo[0]) ? lo[0] + (uint32_t)q : lo[1] + (uint32_t)(q - (hi[0] - lo[0]));
            uint32_t o[M + 32];
            gen(g, seed, o, M + 20);
            for (int ti = 0; ti < NT; ti++) {
                target_t *t = &T[ti];
                if (t->only_g >= 0 && t->only_g != g) continue;
                int modes[2], nm = 0;
                if (t->bits == 8) { modes[nm++] = 0; modes[nm++] = 1; }
                else if (GW[g] == 31) { modes[nm++] = 2; modes[nm++] = 3; }
                for (int mi = 0; mi < nm; mi++) {
                    int md = modes[mi];
                    for (int i = 0; i + t->len <= M + 20; i++) {
                        if (sym(o[i], md, GW[g]) != t->s[0]) continue;
                        int k = 1;
                        while (k < t->len && sym(o[i + k], md, GW[g]) == t->s[k]) k++;
                        if (k == t->len) {
                            #pragma omp critical
                            printf("HIT %-44s gen=%s seed=%u mode=%d offset=%d\n", t->name, GN[g], seed, md, i);
                            hits++;
                        }
                    }
                }
            }
        }
        fprintf(stderr, "done %s\n", GN[g]);
        fflush(stdout);
    }
    printf("total hits %ld (targets %d)\n", hits, NT);
    return 0;
}
