// GMP 2.0.2 mpz_random base-point test. Limbs least significant first; 32-bit limb = random() ^ (random() << 1),
// 64-bit limb = random() ^ (random() << 31) ^ (random() << 62) (urandom.h). random() = glibc TYPE_3 or 4.3BSD
// (generators from b163_brute.c). x = mpz_random(n) then reduced mod p and possibly stepped by a small j, so the
// test compares limbs 1..K of Gx/Gy (the lowest limb is skipped). Seeds: 0..2^24, every 1997-98 time t and t/2.
// Search depth: the first M random() outputs after seeding, every start offset.
#define main b163_main
#include "b163_brute.c"
#undef main

#define DEPTH 2048
#define K32 5
#define K64 3

typedef struct { const char *name; uint32_t l32[8]; uint64_t l64[K64 + 1]; } tgt_t;
static tgt_t TG[16]; static int NTG = 0;

static void add_target(const char *name, const char *hex) {
    tgt_t *t = &TG[NTG++]; t->name = name;
    int L = strlen(hex);
    for (int i = 0; i < 8; i++) {           // 32-bit limb i = hex chars [L-8(i+1), L-8i)
        char buf[9] = {0}; int s = L - 8 * (i + 1); if (s < 0) s = 0;
        strncpy(buf, hex + s, L - 8 * i - s); t->l32[i] = (uint32_t)strtoul(buf, 0, 16);
    }
    for (int i = 0; i <= K64; i++) t->l64[i] = ((uint64_t)t->l32[2 * i + 1] << 32) | t->l32[2 * i];
}

int main(void) {
    static const char *G[][2] = {
        {"P-224 Gx", "b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21"},
        {"P-224 Gy", "bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34"},
        {"P-256 Gx", "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296"},
        {"P-256 Gy", "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5"},
        {"P-384 Gx", "aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab7"},
        {"P-384 Gy", "3617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f"},
        {"P-521 Gx", "c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66"},
        {"P-521 Gy", "11839296a789a3bc0045c8a5fb42c7d1bd998f54449579b446817afbd17273e662c97ee72995ef42640c550b9013fad0761353c7086a272c24088be94769fd16650"},
    };
    for (int i = 0; i < 8; i++) add_target(G[i][0], G[i][1]);
    // planted controls: limbs from glibc srandom(892000000) offset 100 (32-bit) and 4.3BSD srandom(5000) offset 37 (64-bit)
    { uint32_t o[400]; gen(0, 892000000u, o, 400); tgt_t *t = &TG[NTG++]; t->name = "CONTROL glibc 32-bit limbs seed 892000000 off 100";
      for (int i = 0; i <= K32; i++) t->l32[i] = o[100 + 2 * i] ^ (o[101 + 2 * i] << 1); for (int i = 0; i <= K64; i++) t->l64[i] = 0; }
    { uint32_t o[400]; gen(1, 5000u, o, 400); tgt_t *t = &TG[NTG++]; t->name = "CONTROL 4.3BSD 64-bit limbs seed 5000 off 37";
      for (int i = 0; i <= K64; i++) { int b = 37 + 3 * i; t->l64[i] = (uint64_t)o[b] ^ ((uint64_t)o[b + 1] << 31) ^ ((uint64_t)o[b + 2] << 62); }
      for (int i = 0; i <= K32; i++) t->l32[i] = 0; }
    const unsigned long r1[3][2] = {{0, 1UL << 24}, {426038400UL, 457574400UL}, {852076800UL, 915148800UL}};
    long hits = 0;
    for (int g = 0; g < 2; g++) {
        for (int rr = 0; rr < 3; rr++) {
            #pragma omp parallel for schedule(dynamic, 65536) reduction(+:hits)
            for (unsigned long s = r1[rr][0]; s < r1[rr][1]; s++) {
                uint32_t o[DEPTH + 16];
                gen(g, (uint32_t)s, o, DEPTH + 16);
                for (int i = 0; i + 2 * (K32 + 1) + 1 < DEPTH; i++) {
                    uint32_t u = o[i] ^ (o[i + 1] << 1);
                    uint64_t v = (uint64_t)o[i] ^ ((uint64_t)o[i + 1] << 31) ^ ((uint64_t)o[i + 2] << 62);
                    for (int t = 0; t < NTG; t++) {
                        if (TG[t].l32[1] && u == TG[t].l32[1]) {          // candidate limb 1 at offset i
                            int k = 2; while (k <= K32 && (o[i + 2 * (k - 1)] ^ (o[i + 2 * (k - 1) + 1] << 1)) == TG[t].l32[k]) k++;
                            if (k > K32) {
                                #pragma omp critical
                                printf("HIT %-50s 32-bit limbs gen=%s seed=%lu offset(limb1)=%d\n", TG[t].name, GN[g], s, i);
                                hits++; }
                        }
                        if (TG[t].l64[1] && v == TG[t].l64[1]) {
                            int k = 2; while (k <= K64) { int b = i + 3 * (k - 1);
                                uint64_t w = (uint64_t)o[b] ^ ((uint64_t)o[b + 1] << 31) ^ ((uint64_t)o[b + 2] << 62);
                                if (w != TG[t].l64[k]) break; k++; }
                            if (k > K64) {
                                #pragma omp critical
                                printf("HIT %-50s 64-bit limbs gen=%s seed=%lu offset(limb1)=%d\n", TG[t].name, GN[g], s, i);
                                hits++; }
                        }
                    }
                }
            }
            fprintf(stderr, "done %s range %d\n", GN[g], rr);
        }
    }
    printf("done: glibc and 4.3BSD random(), seeds 0..2^24 and 1997-98 t, t/2; depth %d; hits %ld\n", DEPTH, hits);
    return 0;
}
