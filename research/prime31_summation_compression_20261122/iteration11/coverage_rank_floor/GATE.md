# Prime31: unavoidable coverage floor for blind strict-four training

2026-11-24 Pacific; PR #5131; parent 9c15369cc90d7b237a33b8c724205d0f5f5f9fae. Written producer mathematics, [pcc:unverified]. No new field-dependent computation, experiment, performance sample, or budget approval.

## Model and exact finite bound

Retain the inherited cofactor-one ordinary source p=2147483647, r=2147446831, a=847514846, b=2087444724, G=(625037715,2100627251), B=31. Scalar rank is over F_r. As in ../rank_deficit_sharpening/GATE.md, assume the source-matched historical N4 injectivity (../historical_collision_reconciliation/GATE.md), exact strict distinct-index extraction, and iid uniform nonidentity public training targets with legitimately known RHS. Under these assumptions, each blind draw hits with probability q=16*C(B,4)/(r-1); conditional on a hit, its four distinct indices are a uniformly drawn 4-subset, and all signs are uniform. The successive hit rows are iid and independent of the Bernoulli hit times. A known-RHS training model is not a challenge-scalar oracle.

Full coefficient rank requires both at least B positive rows and every column to occur. Let C be the positive-row count at first full column coverage, and J=max(B,C). If R is the positive-row count at first full rank, R>=J pointwise. Let a_j=C(B-j,4)/C(B,4), with invalid binomials zero. Inclusion-exclusion over the j specified columns that are never touched gives

    Pr(C<=m)=sum_(j=0)^B (-1)^j C(B,j) a_j^m.

Summing the tail of J (m>=B) yields the exact rational mean

    E[J]=B+sum_(j=1)^B (-1)^(j+1) C(B,j) a_j^B/(1-a_j).

By independent Bernoulli thinning, the blind-draw time for J hits has expectation E[J]/q. It is a lower bound on the expected full-rank blind-query time.

For a fixed n blind queries, let M~Binomial(n,q). Then

    Pr(full rank by n) <= F(n)
       =sum_(m=B)^n Pr(M=m) Pr(C<=m).

F is monotone because the necessary event is monotone under adding rows. This is an upper bound on completion probability, not a prediction of rank. The same-coordinator deterministic certificate uses exact integer coverage recurrence: from c observed indices, there are C(B-c,k) C(c,4-k) possible next supports seeing k new indices. The total count after m hits is C(B,4)^m; the probability of coverage is the c=B count divided by this denominator. For the binomial terms, outward rounding on the integer scale 10^60 and a conservative upper tail for omitted m>256 enclose F(n). No floating approximation is needed for the two quantile crossing assertions.

## Conditional numerical result

| Metric | Necessary lower bound | Prior sufficient upper bound |
|---|---:|---:|
| Expected positive training rows | 34.026954 | 53.633 |
| Expected blind training queries | 145143.562601 | 228773.93 |
| 95% full-rank budget | 211709 | 453000 |
| 99% full-rank budget | 265406 | 560000 |

At exactly 31 positive hits, coverage probability is 0.6410775203545042 (so full-rank probability cannot exceed 64.11%); at 40 hits it is 0.8824592233536757. F(211708)<0.95<=F(211709); F(265405)<0.99<=F(265406), with directed arithmetic checking the strict sides. These are necessary budgets only: nonzero columns and enough rows do not imply independence. The expected lower bound is stronger than simply B/q. The prior upper bounds are retained from the parent and not remeasured. See BUDGET.svg for the conditional brackets.

## Growing-family consequence and challenge

If the same injective strict-hit and iid model applies along a growing B family, the full-independent-row idealization cannot mean O(B) total positive draws for complete rank at fixed arity four. After m positive rows, the unseen-column count Z has E[Z]=B(1-4/B)^m. For i!=j the probability that neither column is touched in one draw equals (B-4)(B-5)/(B(B-1)) <= (1-4/B)^2. Independence across draws implies nonpositive covariance of the unseen indicators, so Var Z<=E Z. Chebyshev yields Pr(Z=0)<=1/E Z. At m=(1-epsilon)B log B/4 for fixed 0<epsilon<1, E Z grows as B^epsilon; full rank has vanishing probability. At m=(1+epsilon)B log B/4, the union bound gives coverage with probability tending to one (not full rank). Hence complete training needs Omega(B log B) positive draws in expectation in this restricted model, or Omega(r log B/B^3) blind draws when q=Theta(B^4/r). The existing pivot-count sufficient bound gives the matching order under injectivity: its per-support pivot contribution is at least 8 for any support meeting the k pivot indices, and C(B,4)-C(B-k,4)>= (k/B) C(B,4); sum the resulting geometric waiting bounds. This identifies a logarithmic rank-collection overhead, not a changed power-law exponent. No existence of such injective growing elliptic families is asserted. For collisions, weighted/adaptive targets, preexisting rank, homogeneous rows, or partial rank goals, the lower bound need not hold. Matrix signs can also create extra dependencies, so no equality between coverage and rank is claimed. This is standard group-coupon reasoning applied to the conditional historical fixture, not a novel coupon theorem.

## Literature and scope

Betken and Thäle, Approaching the coupon collector's problem with group drawings via Stein's method (2023), https://doi.org/10.1017/jpr.2023.6 ; group-drawing coupon model. Coja-Oghlan et al., The rank of sparse random matrices, https://doi.org/10.1002/rsa.21085 ; sparse rank is a distinct question from coverage or matching. No imported asymptotic rank theorem is used. Shared KB/Conductor and native workers not exposed. Parallel independent repository reads only; original Rust candidate and current-root qualification remain separate (#5128). Historical seeds and replay have not been requalified as a reproducible A/B. No new randomized runs, native tests, paid jobs, signatures or Space mutation. The failed bounded native Space read required interactive authorization. No new extraction algorithm. [pcc:unverified]
