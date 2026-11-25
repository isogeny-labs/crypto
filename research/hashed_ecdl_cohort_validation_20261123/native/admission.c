/* Independent bounded fixture admission. No discrete logarithm is supplied.
 * Established method: prime-order point witness + Hasse unique multiple.
 * OpenSSL is used only for curve/point addition and doubling, never for
 * configured generator-order scalar reduction. Raw counters are diagnostic.
 */
#define _POSIX_C_SOURCE 200809L
#include <errno.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <openssl/bn.h>
#include <openssl/crypto.h>
#include <openssl/ec.h>
#include <openssl/err.h>

typedef __uint128_t Wide;
_Static_assert(sizeof(BN_ULONG) >= sizeof(uint64_t), "64-bit OpenSSL word required");
#define LIMIT (UINT64_C(1) << 48)
#define REAL_ROWS 12U
#define CONTROL_ROWS 199U

typedef enum {
    CERTIFIED, RESOURCE, FIELD_RANGE, FIELD_PRIME, CURVE_CANON, SINGULAR,
    ORDER_RANGE, ORDER_PRIME, G_IDENTITY, G_CANON, G_POINT, Q_IDENTITY,
    Q_CANON, Q_POINT, G_ORDER, Q_ORDER, INTERVAL
} Code;

static const char *code_name(Code c) {
    static const char *const names[] = {
        "certified", "unknown_resource", "rejected_field_range",
        "rejected_field_composite", "rejected_curve_noncanonical",
        "rejected_singular", "rejected_order_range", "rejected_order_composite",
        "rejected_generator_identity", "rejected_generator_noncanonical",
        "rejected_generator_off_curve", "rejected_target_identity",
        "rejected_target_noncanonical", "rejected_target_off_curve",
        "rejected_generator_order", "rejected_target_order", "unknown_interval"
    };
    return names[(unsigned)c];
}

typedef struct {
    char id[64];
    int domain;
    uint64_t seed, p, a, b, r, gx, gy, qx, qy;
    bool g_infinity, q_infinity;
} Input;

typedef struct {
    Code code;
    uint64_t divisions, factor, adds, doubles, radius, lower, upper;
    int64_t trace;
    unsigned embedding;
    bool anomalous, supersingular, j0, j1728, eligible;
} Result;

static struct {
    FILE *out;
    unsigned rows, controls, small_cells, real_attempted, real_certified;
    unsigned domains_certified, eligible_domains;
    uint64_t divisions, adds, doubles;
} state;

static void fatal(const char *message) {
    fprintf(stderr, "admission failure: %s\n", message);
    ERR_print_errors_fp(stderr);
    if (state.out != NULL) {
        fprintf(state.out, "{\"kind\":\"failure\",\"message\":\"%s\","
                "\"rows_before_failure\":%u,\"real_attempted\":%u,"
                "\"real_certified\":%u,\"real_unattempted\":%u}\n",
                message, state.rows, state.real_attempted,
                state.real_certified, REAL_ROWS - state.real_attempted);
        (void)fflush(state.out);
    }
    exit(2);
}

static void require(bool ok, const char *message) {
    if (!ok) fatal(message);
}

static uint64_t mulmod(uint64_t a, uint64_t b, uint64_t p) {
    return (uint64_t)((Wide)a * b % p);
}

static uint64_t rhs(uint64_t x, uint64_t a, uint64_t b, uint64_t p) {
    return (uint64_t)(((Wide)mulmod(mulmod(x, x, p), x, p)
                      + mulmod(a, x, p) + b) % p);
}

static bool singular(uint64_t p, uint64_t a, uint64_t b) {
    Wide disc = (Wide)4 * mulmod(mulmod(a, a, p), a, p)
              + (Wide)27 * mulmod(b, b, p);
    return disc % p == 0;
}

/* Exact trial division, deliberately independent of the Rust MR routine. */
static bool prime_trial(uint64_t n, uint64_t *factor, uint64_t *divisions) {
    *factor = 0;
    if (n < 2) return false;
    if (n == 2) return true;
    ++*divisions;
    if (n % 2 == 0) { *factor = 2; return false; }
    for (uint64_t d = 3; d <= n / d; d += 2) {
        ++*divisions;
        if (n % d == 0) { *factor = d; return false; }
    }
    return true;
}

/* Integer Newton, not the source implementation's binary-search routine. */
static uint64_t root_floor(uint64_t n) {
    if (n == 0) return 0;
    uint64_t x = n, y = (x + 1) / 2;
    while (y < x) { x = y; y = (x + n / x) / 2; }
    return x;
}

static BIGNUM *word_bn(uint64_t n) {
    BIGNUM *v = BN_new();
    require(v != NULL && BN_set_word(v, (BN_ULONG)n) == 1, "BIGNUM allocation/set");
    return v;
}

static bool killed(const EC_GROUP *group, const EC_POINT *point,
                   uint64_t scalar, BN_CTX *ctx, Result *result) {
    EC_POINT *sum = EC_POINT_new(group), *base = EC_POINT_dup(point, group);
    require(sum != NULL && base != NULL, "point workspace allocation");
    require(EC_POINT_set_to_infinity(group, sum) == 1, "infinity initialization");
    while (scalar != 0) {
        if ((scalar & 1U) != 0) {
            require(EC_POINT_add(group, sum, sum, base, ctx) == 1, "point addition");
            ++result->adds;
        }
        require(EC_POINT_dbl(group, base, base, ctx) == 1, "point doubling");
        ++result->doubles;
        scalar >>= 1;
    }
    int inf = EC_POINT_is_at_infinity(group, sum);
    EC_POINT_free(base);
    EC_POINT_free(sum);
    return inf == 1;
}

static Result validate(const Input *in) {
    Result z = { .code = RESOURCE };
    if (in->p >= LIMIT || in->r >= LIMIT) return z;
    z.code = FIELD_RANGE;
    if (in->p < 5) return z;
    z.code = FIELD_PRIME;
    if (!prime_trial(in->p, &z.factor, &z.divisions)) return z;
    z.code = CURVE_CANON;
    if (in->a >= in->p || in->b >= in->p) return z;
    z.code = SINGULAR;
    if (singular(in->p, in->a, in->b)) return z;
    z.code = ORDER_RANGE;
    if (in->r < 3) return z;
    z.code = ORDER_PRIME;
    if (!prime_trial(in->r, &z.factor, &z.divisions)) return z;
    z.code = G_IDENTITY;
    if (in->g_infinity) return z;
    z.code = G_CANON;
    if (in->gx >= in->p || in->gy >= in->p) return z;
    z.code = G_POINT;
    if (mulmod(in->gy, in->gy, in->p) != rhs(in->gx, in->a, in->b, in->p)) return z;
    z.code = Q_IDENTITY;
    if (in->q_infinity) return z;
    z.code = Q_CANON;
    if (in->qx >= in->p || in->qy >= in->p) return z;
    z.code = Q_POINT;
    if (mulmod(in->qy, in->qy, in->p) != rhs(in->qx, in->a, in->b, in->p)) return z;

    BN_CTX *ctx = BN_CTX_new();
    BIGNUM *p = word_bn(in->p), *a = word_bn(in->a), *b = word_bn(in->b);
    BIGNUM *gx = word_bn(in->gx), *gy = word_bn(in->gy);
    BIGNUM *qx = word_bn(in->qx), *qy = word_bn(in->qy);
    require(ctx != NULL, "BN context allocation");
    EC_GROUP *group = EC_GROUP_new_curve_GFp(p, a, b, ctx);
    require(group != NULL, "curve construction after arithmetic checks");
    const BIGNUM *order = EC_GROUP_get0_order(group);
    const EC_POINT *generator = EC_GROUP_get0_generator(group);
    require(order == NULL || BN_is_zero(order), "unexpected supplied group order");
    require(generator == NULL || EC_POINT_is_at_infinity(group, generator) == 1,
            "unexpected finite library generator");
    EC_POINT *g = EC_POINT_new(group), *q = EC_POINT_new(group);
    require(g != NULL && q != NULL, "input point allocation");
    require(EC_POINT_set_affine_coordinates(group, g, gx, gy, ctx) == 1,
            "generator import after equation check");
    require(EC_POINT_set_affine_coordinates(group, q, qx, qy, ctx) == 1,
            "target import after equation check");
    z.code = G_ORDER;
    if (!killed(group, g, in->r, ctx, &z)) goto cleanup;
    z.code = Q_ORDER;
    if (!killed(group, q, in->r, ctx, &z)) goto cleanup;
    z.radius = root_floor(4 * in->p);
    require((Wide)z.radius * z.radius <= (Wide)4 * in->p
            && (Wide)(z.radius + 1) * (z.radius + 1) > (Wide)4 * in->p,
            "square-root certificate");
    require(in->p + 1 >= z.radius, "Hasse lower-bound arithmetic");
    z.lower = in->p + 1 - z.radius;
    z.upper = in->p + 1 + z.radius;
    z.code = INTERVAL;
    if (z.upper >= 2 * in->r) goto cleanup;
    require(z.lower <= in->r && in->r <= z.upper, "inconsistent Hasse/order facts");
    z.code = CERTIFIED;
    z.trace = (int64_t)(in->p + 1) - (int64_t)in->r;
    z.anomalous = in->r == in->p;
    z.supersingular = z.trace == 0; /* prime p>3 */
    z.j0 = in->a == 0;
    z.j1728 = in->b == 0;
    uint64_t v = 1;
    for (unsigned k = 1; k <= 99; ++k) {
        v = mulmod(v, in->p % in->r, in->r);
        if (v == 1) { z.embedding = k; break; }
    }
    z.eligible = !z.anomalous && !z.supersingular && z.embedding == 0;
cleanup:
    EC_POINT_free(q); EC_POINT_free(g); EC_GROUP_free(group);
    BN_free(qy); BN_free(qx); BN_free(gy); BN_free(gx);
    BN_free(b); BN_free(a); BN_free(p); BN_CTX_free(ctx);
    return z;
}

static const char *boolean(bool x) { return x ? "true" : "false"; }

static void emit(const char *kind, const Input *in, const Result *z,
                 Code expected, uint64_t brute_count, bool real) {
    fprintf(state.out, "{\"kind\":\"%s\",\"id\":\"%s\",\"domain\":%d,"
            "\"seed\":%" PRIu64 ",\"p\":%" PRIu64 ",\"a\":%" PRIu64
            ",\"b\":%" PRIu64 ",\"r\":%" PRIu64 ",\"status\":\"%s\","
            "\"expected_status\":\"%s\",\"checks_passed\":%s,"
            "\"trial_divisions\":%" PRIu64 ",\"factor\":%" PRIu64
            ",\"point_adds\":%" PRIu64 ",\"point_doubles\":%" PRIu64
            ",\"radius\":%" PRIu64 ",\"lower\":%" PRIu64 ",\"upper\":%" PRIu64
            ",\"enumerated_affine_plus_infinity\":%" PRIu64,
            kind, in->id, in->domain, in->seed, in->p, in->a, in->b, in->r,
            code_name(z->code), code_name(expected), boolean(z->code == expected),
            z->divisions, z->factor, z->adds, z->doubles, z->radius, z->lower,
            z->upper, brute_count);
    if (z->code == CERTIFIED) {
        fprintf(state.out, ",\"certified_order\":%" PRIu64 ",\"trace\":%" PRId64
                ",\"anomalous\":%s,\"supersingular\":%s,\"j0\":%s,"
                "\"j1728\":%s,\"embedding_k_le_99\":%u,\"eligible\":%s",
                in->r, z->trace, boolean(z->anomalous), boolean(z->supersingular),
                boolean(z->j0), boolean(z->j1728), z->embedding, boolean(z->eligible));
    }
    fprintf(state.out, "}\n");
    require(fflush(state.out) == 0, "raw result flush");
    ++state.rows;
    if (!real) ++state.controls;
    state.divisions += z->divisions;
    state.adds += z->adds;
    state.doubles += z->doubles;
}

static Result expect(const char *kind, const Input *in, Code code, uint64_t n) {
    Result z = validate(in);
    emit(kind, in, &z, code, n, false);
    require(z.code == code, "unexpected control classification");
    return z;
}

static bool small_prime_table(uint64_t n) {
    static const unsigned primes[] = {
        2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97
    };
    for (size_t k = 0; k < sizeof(primes) / sizeof(primes[0]); ++k)
        if (n == primes[k]) return true;
    return false;
}

static uint64_t enumerate(Input *in) {
    uint64_t count = 1;
    bool found = false;
    for (uint64_t x = 0; x < in->p; ++x) {
        for (uint64_t y = 0; y < in->p; ++y) {
            if (y * y % in->p == (x * x * x + in->a * x + in->b) % in->p) {
                ++count;
                if (!found) { in->gx = x; in->gy = y; found = true; }
            }
        }
    }
    in->qx = in->gx; in->qy = in->gy;
    in->g_infinity = !found; in->q_infinity = !found;
    return count;
}

static void controls(const Input real[REAL_ROWS]) {
    for (uint64_t n = 0; n <= 65535; ++n) {
        uint64_t s = root_floor(n);
        require(s * s <= n && (s + 1) * (s + 1) > n, "root-range postcondition");
    }
    fprintf(state.out, "{\"kind\":\"root_control\",\"cases\":65536,\"checks_passed\":true}\n");
    ++state.controls; ++state.rows;
    static const uint64_t composites[][2] = {{561,3},{2047,23},{3215031751,151}};
    for (size_t k = 0; k < sizeof(composites) / sizeof(composites[0]); ++k) {
        uint64_t factor = 0, divisions = 0;
        bool prime = prime_trial(composites[k][0], &factor, &divisions);
        require(composites[k][0] % composites[k][1] == 0 && !prime
                && factor > 1 && composites[k][0] % factor == 0, "composite control");
        fprintf(state.out, "{\"kind\":\"prime_control\",\"n\":%" PRIu64
                ",\"factor\":%" PRIu64 ",\"trial_divisions\":%" PRIu64
                ",\"checks_passed\":true}\n", composites[k][0], factor, divisions);
        state.divisions += divisions;
        ++state.controls; ++state.rows;
    }
    static const uint64_t fields[] = {5,7,11,13,17,19,23,29,31,37,41,43};
    for (size_t pidx = 0; pidx < sizeof(fields) / sizeof(fields[0]); ++pidx) {
        uint64_t p = fields[pidx], avals[] = {0,1,p-1};
        for (size_t ai = 0; ai < 3; ++ai) for (uint64_t b = 0; b <= 3; ++b) {
            Input in = {.domain=-1, .p=p, .a=avals[ai], .b=b};
            (void)snprintf(in.id, sizeof(in.id), "small-%" PRIu64 "-%" PRIu64 "-%" PRIu64,
                           p, in.a, b);
            uint64_t n = enumerate(&in);
            require(n < 100, "small-prime table coverage");
            in.r = n;
            Code wanted;
            if (singular(p, in.a, b)) wanted = SINGULAR;
            else if (n < 3) wanted = ORDER_RANGE;
            else if (!small_prime_table(n)) wanted = ORDER_PRIME;
            else wanted = p + 1 + root_floor(4*p) < 2*n ? CERTIFIED : INTERVAL;
            Result z = expect("small_curve", &in, wanted, n);
            if (z.code == CERTIFIED) {
                require(z.trace == (int64_t)(p+1) - (int64_t)n
                        && z.anomalous == (n == p)
                        && z.supersingular == (n == p+1), "enumerated-order flags");
            }
            ++state.small_cells;
        }
    }
    Input special = {.id="anomalous-7", .domain=-1, .p=7, .a=0, .b=5,
                     .r=7, .gx=3, .gy=2, .qx=3, .qy=2};
    Result anomaly = expect("literal_control", &special, CERTIFIED, 7);
    require(anomaly.anomalous && !anomaly.eligible, "anomaly eligibility control");
    special = (Input){.id="singular-7", .domain=-1, .p=7, .r=7,
                      .gx=1, .gy=1, .qx=1, .qy=1};
    (void)expect("literal_control", &special, SINGULAR, 0);
    for (unsigned d = 0; d < 6; ++d) {
        const Input *base = &real[2*d];
        require(base->b != 0 && base->p < LIMIT/9 && base->r < LIMIT/9,
                "mutant construction preconditions");
        static const Code codes[] = {FIELD_PRIME,ORDER_PRIME,G_ORDER,G_POINT,
                                      Q_POINT,G_CANON,G_IDENTITY,Q_IDENTITY};
        static const char *const labels[] = {"field9p","order9r","wrong-prime3",
            "off-generator","off-target","generator-alias","identity-generator","identity-target"};
        for (unsigned m = 0; m < 8; ++m) {
            Input in = *base;
            (void)snprintf(in.id, sizeof(in.id), "domain%u-%s", d, labels[m]);
            switch (m) {
            case 0: in.p *= 9; break;
            case 1: in.r *= 9; break;
            case 2: in.r = 3; break;
            case 3: in.gx = 0; in.gy = 0; break;
            case 4: in.qx = 0; in.qy = 0; break;
            case 5: in.gx += in.p; break;
            case 6: in.g_infinity = true; break;
            case 7: in.q_infinity = true; break;
            default: fatal("unreachable mutant");
            }
            (void)expect("mutant_control", &in, codes[m], 0);
        }
    }
    Input over = real[0];
    (void)snprintf(over.id, sizeof(over.id), "field-resource-bound");
    over.p = UINT64_C(1) << 49;
    Result limited = expect("resource_control", &over, RESOURCE, 0);
    require(limited.divisions == 0 && limited.adds == 0 && limited.doubles == 0,
            "resource error misclassified or arithmetic attempted");
    require(state.controls == CONTROL_ROWS && state.small_cells == 144,
            "control census differs from prospective plan");
}

static uint64_t parse_uint(const char *s) {
    require(s != NULL && *s >= '0' && *s <= '9' && !(s[0]=='0' && s[1]!='\0'),
            "noncanonical unsigned input");
    errno = 0; char *end = NULL;
    unsigned long long value = strtoull(s, &end, 10);
    require(errno == 0 && end != NULL && *end == '\0', "invalid numeric input");
    require(value <= UINT64_MAX, "numeric input outside u64");
    return (uint64_t)value;
}

static void load_inputs(const char *path, Input rows[REAL_ROWS]) {
    FILE *f = fopen(path, "r");
    require(f != NULL, "input file open");
    char line[1024]; unsigned count = 0;
    while (fgets(line, sizeof(line), f) != NULL) {
        require(strchr(line, '\n') != NULL && count < REAL_ROWS, "input row length/count");
        char *save = NULL, *tokens[11];
        for (unsigned k = 0; k < 11; ++k) {
            tokens[k] = strtok_r(k == 0 ? line : NULL, "\t\r\n", &save);
            require(tokens[k] != NULL, "missing input field");
        }
        require(strtok_r(NULL, "\t\r\n", &save) == NULL, "extra input field");
        uint64_t domain = parse_uint(tokens[1]), seed = parse_uint(tokens[2]);
        require(domain == count/2 && seed == (count%2 == 0 ? 101U : 202U),
                "input identity/order contract");
        Input in = {.domain=(int)domain, .seed=seed};
        (void)snprintf(in.id, sizeof(in.id), "case%u-seed%" PRIu64, count/2, seed);
        require(strcmp(in.id, tokens[0]) == 0, "input id contract");
        uint64_t *fields[] = {&in.p,&in.a,&in.b,&in.r,&in.gx,&in.gy,&in.qx,&in.qy};
        for (unsigned k = 0; k < 8; ++k) *fields[k] = parse_uint(tokens[k+3]);
        rows[count++] = in;
    }
    require(ferror(f) == 0 && fclose(f) == 0 && count == REAL_ROWS, "input completion");
    for (unsigned d = 0; d < 6; ++d) {
        const Input *x = &rows[2*d], *y = &rows[2*d+1];
        require(x->p==y->p && x->a==y->a && x->b==y->b && x->r==y->r
                && x->gx==y->gx && x->gy==y->gy, "paired domain mismatch");
        require(x->qx!=y->qx || x->qy!=y->qy, "duplicate paired public target");
    }
}

int main(int argc, char **argv) {
    if (argc != 3) { fprintf(stderr, "usage: admission INPUT.tsv NEW_OUTPUT.jsonl\n"); return 2; }
    state.out = fopen(argv[2], "wx");
    if (state.out == NULL) { perror("exclusive output creation"); return 2; }
    Input real[REAL_ROWS];
    load_inputs(argv[1], real);
    controls(real);
    Result domain_results[6] = {{0}};
    for (unsigned i = 0; i < REAL_ROWS; ++i) {
        ++state.real_attempted;
        Result z = validate(&real[i]);
        emit("real_fixture", &real[i], &z, CERTIFIED, 0, true);
        require(z.code == CERTIFIED, "real fixture failed predicted order/point certification");
        ++state.real_certified;
        unsigned d = i/2;
        if (i%2 == 0) {
            domain_results[d] = z;
            ++state.domains_certified;
            if (z.eligible) ++state.eligible_domains;
        } else {
            const Result *a = &domain_results[d];
            require(z.trace == a->trace && z.embedding == a->embedding
                    && z.eligible == a->eligible, "paired domain census mismatch");
        }
    }
    require(state.rows == CONTROL_ROWS + REAL_ROWS && state.real_certified == REAL_ROWS,
            "final census mismatch");
    fprintf(state.out, "{\"kind\":\"summary\",\"checks_passed\":true,"
            "\"control_rows\":%u,\"small_parameter_cells\":%u,"
            "\"real_attempted\":%u,\"real_certified\":%u,\"real_unattempted\":0,"
            "\"domains_certified\":%u,\"eligible_domains\":%u,"
            "\"trial_divisions_diagnostic\":%" PRIu64 ",\"point_adds_diagnostic\":%" PRIu64
            ",\"point_doubles_diagnostic\":%" PRIu64 ",\"jsonl_rows\":%u,"
            "\"scope\":\"existing-method independent admission replay; no solver or timing comparison\"}\n",
            state.controls, state.small_cells, state.real_attempted, state.real_certified,
            state.domains_certified, state.eligible_domains, state.divisions, state.adds,
            state.doubles, state.rows+1);
    FILE *closing = state.out;
    state.out = NULL;
    require(fclose(closing) == 0, "final raw close");
    printf("admission complete: %u real points, %u domains, %u controls\n",
           state.real_certified, state.domains_certified, state.controls);
    return 0;
}
