/* Fixed-leaf identity-chart stress panel, protocol addendum. */
#define main original_sweep_main
#include "sweep.c"
#undef main

static ChainResult fixed_chain(int p, int t, int m, int target,
                               const int *leaves, bool projective) {
    Base base;
    build_base(p,t,&base);
    ChainResult result = {0};
    result.candidate_checks = base.candidate_checks;
    result.lift_tests = base.lift_tests;
    unsigned char cur[PMAX+1] = {0}, next[PMAX+1];
    cur[p] = 1;
    for (int depth = 0; depth < m; ++depth) {
        int x = leaves[depth];
        bool in_base = false;
        for (int j = 0; j < base.n_x; ++j)
            if (base.x[j] == x) in_base = true;
        if (!in_base) fail("fixed leaf not in rebuilt base");
        memset(next,0,sizeof(next));
        for (int u = 0; u <= p; ++u) {
            if (!cur[u]) continue;
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
        memcpy(cur,next,sizeof(cur));
        int count = 0;
        for (int u = 0; u <= p; ++u) count += cur[u] != 0;
        result.state_counts[depth] = count;
        if (cur[p]) result.infinity_mask |= 1U << depth;
    }
    result.hit = cur[target] != 0;
    return result;
}
static ReferenceResult fixed_reference(int p, int t, int m, int target,
                                        const int *leaves) {
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
        int nnext = 0;
        memset(seen,0,sizeof(seen));
        for (int i = 0; i < ncur; ++i) {
            Point a = id_point(cur[i],p);
            for (int j = 0; j < base.n_points; ++j) {
                if (base.points[j].x != leaves[depth]) continue;
                ++result.group_adds;
                Point sum = group_add(a,base.points[j],p);
                int id = point_id(sum,p);
                if (!seen[id]) {
                    seen[id] = 1;
                    if (nnext >= STATEMAX) fail("fixed group state capacity");
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
int main(void) {
    puts("p,a,b,t,m,px,qx,target_x,status,projective_hit,affine_hit,reference_hit,projective_infinity_mask,affine_infinity_mask,projective_states,affine_states,projective_s3_evals,affine_s3_evals,projective_identity_edges,projective_inverse_edges,reference_group_adds,base_checks_per_arm,lift_tests_per_arm");
    const int primes[] = {97,193,257};
    for (int pi = 0; pi < 3; ++pi) {
        int p = primes[pi];
        for (int t = 2; t <= 4; ++t) {
            Base evaluator_base;
            build_base(p,t,&evaluator_base);
            Point P = {0,0,1}, Q = {0,0,1};
            if (evaluator_base.n_points) {
                P = evaluator_base.points[0];
                for (int j = 1; j < evaluator_base.n_points; ++j)
                    if (evaluator_base.points[j].x != P.x) {
                        Q = evaluator_base.points[j]; break;
                    }
            }
            for (int m = 4; m <= 7; ++m) {
                Point target = Q.inf ? Q : scalar_multiple(Q,m-2,p);
                if (P.inf || Q.inf || target.inf) {
                    printf("%d,2,3,%d,%d,%d,%d,-1,censored,,,,,,,,,,,,,,\n",
                           p,t,m,P.inf?-1:P.x,Q.inf?-1:Q.x);
                    continue;
                }
                int leaves[7];
                leaves[0] = leaves[1] = P.x;
                for (int j = 2; j < m; ++j) leaves[j] = Q.x;
                ChainResult projective = fixed_chain(p,t,m,target.x,leaves,true);
                ChainResult affine = fixed_chain(p,t,m,target.x,leaves,false);
                ReferenceResult reference = fixed_reference(p,t,m,target.x,leaves);
                if (projective.hit != reference.hit || !reference.hit) {
                    fprintf(stderr,"fixed mismatch p=%d t=%d m=%d\n",p,t,m);
                    exit(3);
                }
                printf("%d,2,3,%d,%d,%d,%d,%d,ok,%d,%d,%d,%u,%u,",
                       p,t,m,P.x,Q.x,target.x,projective.hit,affine.hit,
                       reference.hit,projective.infinity_mask,affine.infinity_mask);
                print_counts(projective.state_counts,m);
                printf(",");
                print_counts(affine.state_counts,m);
                printf(",%" PRIu64 ",%" PRIu64 ",%" PRIu64 ",%" PRIu64
                       ",%" PRIu64 ",%" PRIu64 ",%" PRIu64 "\n",
                       projective.s3_evals,affine.s3_evals,
                       projective.identity_edges,projective.inverse_edges,
                       reference.group_adds,projective.candidate_checks,
                       projective.lift_tests);
            }
        }
    }
    return 0;
}
