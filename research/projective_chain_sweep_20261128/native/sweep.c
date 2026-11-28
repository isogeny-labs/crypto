/* Finite projective Semaev-chain sweep. Protocol: ../PROTOCOL.md.
 * This is a root-propagation diagnostic, not a Groebner solver. */
#include <inttypes.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define PMAX 257
#define BMAX 16
#define POINTMAX (2 * BMAX)
#define STATEMAX 1024

typedef struct { int x, y, inf; } Point;
typedef struct {
    int all_x[BMAX], x[BMAX], n_x, n_points;
    Point points[POINTMAX];
    uint64_t candidate_checks, lift_tests;
} Base;
typedef struct {
    bool hit;
    int state_counts[7];
    unsigned infinity_mask;
    uint64_t s3_evals, identity_edges, inverse_edges;
    uint64_t candidate_checks, lift_tests;
} ChainResult;
typedef struct {
    bool hit;
    uint64_t group_adds, candidate_checks, lift_tests;
} ReferenceResult;

static int modp(int64_t a, int p) {
    int r = (int)(a % p);
    return r < 0 ? r + p : r;
}
static int power(int x, int e, int p) {
    int r = 1;
    while (e) {
        if (e & 1) r = modp((int64_t)r * x, p);
        x = modp((int64_t)x * x, p);
        e >>= 1;
    }
    return r;
}
static int curve_rhs(int x, int p) {
    return modp((int64_t)x*x*x + 2LL*x + 3, p);
}
static bool has_lift(int x, int p) {
    int rhs = curve_rhs(x, p);
    for (int y = 0; y < p; ++y)
        if (modp((int64_t)y*y, p) == rhs) return true;
    return false;
}
static void fail(const char *message) {
    fprintf(stderr, "ERROR: %s\n", message);
    exit(2);
}
static void build_base(int p, int t, Base *base) {
    memset(base, 0, sizeof(*base));
    const int n = 1 << t;
    if (n > BMAX || (p-1) % n) fail("invalid Kummer depth");
    int z = 0;
    for (int h = 2; h < p; ++h) {
        int candidate = power(h, (p-1)/n, p);
        if (power(candidate, n/2, p) != 1) { z = candidate; break; }
    }
    if (!z) fail("no primitive Kummer root");
    int cur = 1;
    for (int k = 0; k < n; ++k) {
        base->all_x[k] = modp(2LL*cur, p);
        cur = modp((int64_t)cur*z, p);
    }
    if (cur != 1) fail("Kummer root order mismatch");
    for (int i = 1; i < n; ++i) {
        int v = base->all_x[i], j = i;
        while (j && base->all_x[j-1] > v) {
            base->all_x[j] = base->all_x[j-1]; --j;
        }
        base->all_x[j] = v;
    }
    for (int i = 0; i < n; ++i) {
        int x = base->all_x[i];
        if (i && x == base->all_x[i-1]) fail("duplicate base abscissa");
        ++base->candidate_checks;
        bool lifted = false;
        int rhs = curve_rhs(x, p);
        for (int y = 0; y < p; ++y) {
            ++base->lift_tests;
            if (modp((int64_t)y*y, p) == rhs) {
                if (base->n_points >= POINTMAX) fail("point capacity");
                base->points[base->n_points++] = (Point){x, y, 0};
                lifted = true;
            }
        }
        if (lifted) base->x[base->n_x++] = x;
    }
}
static bool in_v(const Base *base, int x, int t) {
    for (int i = 0; i < (1 << t); ++i)
        if (base->all_x[i] == x) return true;
    return false;
}
static Point group_add(Point a, Point b, int p) {
    if (a.inf) return b;
    if (b.inf) return a;
    if (a.x == b.x && modp(a.y+b.y, p) == 0) return (Point){0,0,1};
    int numerator, denominator;
    if (a.x == b.x) {
        numerator = modp(3LL*a.x*a.x+2, p);
        denominator = modp(2LL*a.y, p);
    } else {
        numerator = modp(b.y-a.y, p);
        denominator = modp(b.x-a.x, p);
    }
    if (!denominator) fail("zero group-law denominator");
    int lambda = modp((int64_t)numerator*power(denominator,p-2,p), p);
    int x = modp((int64_t)lambda*lambda-a.x-b.x, p);
    int y = modp((int64_t)lambda*(a.x-x)-a.y, p);
    return (Point){x,y,0};
}
static Point scalar_multiple(Point a, int n, int p) {
    Point r = {0,0,1};
    for (int i = 0; i < n; ++i) r = group_add(r,a,p);
    return r;
}
static bool s3_zero(int u, int x, int v, int p) {
    int64_t ux = (int64_t)u*x;
    int64_t c2 = (int64_t)(u-x)*(u-x);
    int64_t c1 = -2*((u+x)*(ux+2)+6);
    int64_t c0 = (ux-2)*(ux-2)-12LL*(u+x);
    return modp(c2*v*v+c1*v+c0,p) == 0;
}
static ChainResult chain_query(int p, int t, int m, int target, bool projective) {
    Base base;
    build_base(p,t,&base);
    ChainResult result = {0};
    result.candidate_checks = base.candidate_checks;
    result.lift_tests = base.lift_tests;
    unsigned char cur[PMAX+1] = {0}, next[PMAX+1];
    cur[p] = 1; /* initial empty sum */
    for (int depth = 0; depth < m; ++depth) {
        memset(next,0,sizeof(next));
        for (int u = 0; u <= p; ++u) {
            if (!cur[u]) continue;
            for (int j = 0; j < base.n_x; ++j) {
                int x = base.x[j];
                if (u == p) {
                    next[x] = 1;
                    ++result.identity_edges;
                } else {
                    if (projective && u == x) {
                        next[p] = 1;
                        ++result.inverse_edges;
                    }
                    for (int v = 0; v < p; ++v) {
                        ++result.s3_evals;
                        if (s3_zero(u,x,v,p)) next[v] = 1;
                    }
                }
            }
        }
        memcpy(cur,next,sizeof(cur));
        int count = 0;
        for (int u = 0; u <= p; ++u) count += cur[u] != 0;
        result.state_counts[depth] = count;
        if (cur[p]) result.infinity_mask |= 1U << depth;
    }
    result.hit = target >= 0 && cur[target] != 0;
    return result;
}
static int point_id(Point a, int p) { return a.inf ? p*p : a.x*p+a.y; }
static Point id_point(int id, int p) {
    return id == p*p ? (Point){0,0,1} : (Point){id/p,id%p,0};
}
static ReferenceResult reference_query(int p, int t, int m, int target) {
    Base base;
    build_base(p,t,&base);
    ReferenceResult result = {0};
    result.candidate_checks = base.candidate_checks;
    result.lift_tests = base.lift_tests;
    int cur[STATEMAX], next[STATEMAX];
    unsigned char seen[PMAX*PMAX+1] = {0};
    int ncur = 1;
    cur[0] = p*p;
    for (int depth = 0; depth < m; ++depth) {
        memset(seen,0,sizeof(seen));
        int nnext = 0;
        for (int i = 0; i < ncur; ++i) {
            Point a = id_point(cur[i],p);
            for (int j = 0; j < base.n_points; ++j) {
                ++result.group_adds;
                Point sum = group_add(a,base.points[j],p);
                int id = point_id(sum,p);
                if (!seen[id]) {
                    seen[id] = 1;
                    if (nnext >= STATEMAX) fail("group state capacity");
                    next[nnext++] = id;
                }
            }
        }
        ncur = nnext;
        memcpy(cur,next,(size_t)ncur*sizeof(*cur));
    }
    for (int i = 0; i < ncur; ++i) {
        Point a = id_point(cur[i],p);
        if (!a.inf && a.x == target) result.hit = true;
    }
    return result;
}
static void print_counts(const int *counts, int m) {
    for (int i = 0; i < m; ++i) printf("%s%d", i?":":"",counts[i]);
}
static void row(int p, int t, int m, const char *label, int target,
                int base_x, int base_points) {
    if (target < 0) {
        printf("%d,2,3,%d,%d,%s,-1,%d,%d,censored,,,,,,,,,,,,,\n",
               p,t,m,label,base_x,base_points);
        return;
    }
    ChainResult projective = chain_query(p,t,m,target,true);
    ChainResult affine = chain_query(p,t,m,target,false);
    ReferenceResult reference = reference_query(p,t,m,target);
    if (projective.hit != reference.hit) {
        fprintf(stderr,"mismatch p=%d t=%d m=%d target=%s:%d projective=%d reference=%d\n",
                p,t,m,label,target,projective.hit,reference.hit);
        exit(3);
    }
    if (strcmp(label,"cancel") == 0 && !reference.hit)
        fail("planted cancellation target was not reached");
    printf("%d,2,3,%d,%d,%s,%d,%d,%d,ok,%d,%d,%d,",
           p,t,m,label,target,base_x,base_points,
           projective.hit,affine.hit,reference.hit);
    print_counts(projective.state_counts,m);
    printf(",%u,",projective.infinity_mask);
    print_counts(affine.state_counts,m);
    printf(",%u,%" PRIu64 ",%" PRIu64 ",%" PRIu64 ",%" PRIu64
           ",%" PRIu64 ",%" PRIu64 ",%" PRIu64 ",%" PRIu64
           ",%" PRIu64 ",%" PRIu64 ",%" PRIu64 "\n",
           affine.infinity_mask,projective.s3_evals,affine.s3_evals,
           projective.identity_edges,projective.inverse_edges,
           projective.candidate_checks,projective.lift_tests,
           affine.candidate_checks,affine.lift_tests,
           reference.group_adds,reference.candidate_checks,
           reference.lift_tests);
}
int main(int argc, char **argv) {
    bool smoke = argc == 2 && strcmp(argv[1],"--smoke") == 0;
    if (argc > 2 || (argc == 2 && !smoke)) fail("usage: sweep [--smoke]");
    puts("p,a,b,t,m,target_label,target_x,base_x,base_points,status,projective_hit,affine_hit,reference_hit,projective_states,projective_infinity_mask,affine_states,affine_infinity_mask,projective_s3_evals,affine_s3_evals,projective_identity_edges,projective_inverse_edges,projective_base_checks,projective_lift_tests,affine_base_checks,affine_lift_tests,reference_group_adds,reference_base_checks,reference_lift_tests");
    const int primes[] = {97,193,257};
    for (int pi = 0; pi < 3; ++pi) {
        int p = primes[pi];
        if (smoke && pi) break;
        for (int t = 2; t <= 4; ++t) {
            if (smoke && t > 2) break;
            Base evaluator_base;
            build_base(p,t,&evaluator_base);
            int scans[3], nscan = 0;
            for (int x = 0; x < p && nscan < 3; ++x)
                if (!in_v(&evaluator_base,x,t) && has_lift(x,p))
                    scans[nscan++] = x;
            if (nscan != 3) fail("not enough scan targets");
            for (int m = 4; m <= 7; ++m) {
                if (smoke && m > 4) break;
                for (int i = 0; i < 3; ++i) {
                    char label[8];
                    snprintf(label,sizeof(label),"scan%d",i);
                    row(p,t,m,label,scans[i],evaluator_base.n_x,
                        evaluator_base.n_points);
                }
                int cancel = -1;
                for (int i = 0; i < evaluator_base.n_points; ++i) {
                    Point q = scalar_multiple(evaluator_base.points[i],m-2,p);
                    if (!q.inf) { cancel = q.x; break; }
                }
                row(p,t,m,"cancel",cancel,evaluator_base.n_x,
                    evaluator_base.n_points);
            }
        }
    }
    return 0;
}
