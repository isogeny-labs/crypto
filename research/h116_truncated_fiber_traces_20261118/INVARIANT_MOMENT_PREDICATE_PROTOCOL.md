# H116 invariant-moment predicate exploration protocol

Protocol ID: `H116-INVARIANT-MOMENT-PREDICATE-001`

Filed before any full-row moment generation or predicate scoring for this
experiment. This is an excluded-fixture hypothesis-generation run, not the
frozen H116 out-of-sample campaign.

## Question

Does any one discriminant-normalized low-fiber moment, quantized into six
equal base-field bins, determine the scalar modulo six under one fixed bin
permutation across two curves and three generator relabelings?

## Fixed fixtures and rows

| curve | order | trace | torsion extension |
| --- | ---: | ---: | ---: |
| `y^2=x^3+x+19` over `F_31` | 37 | -5 | 4 |
| `y^2=x^3+x+38` over `F_47` | 61 | -13 | 3 |

Both exact models are predecessor fixtures excluded from the H116 discovery
cohort. For each curve, generator labels 1, 2 and 3 and every scalar
`1..r-1` are included. The moment producer computes both the native model and
the `lambda=2` isomorphic model, and must reproduce every moment by direct
fiber enumeration before scoring is admitted.

## Frozen invariant channels

Let `D=4*A^3+27*B^2`. Under the coordinate isomorphism
`(x,y,A,B)->(lambda^2*x,lambda^3*y,lambda^4*A,lambda^6*B)`, the moments have
weights

```text
weight(X_j)=2*(j+1),   weight(Y_j)=2*j+3,   weight(D)=12.
```

The sixteen fixed channels are

```text
IX_j = X_j^6 / D^(j+1),       j=0..7,
IY_j = Y_j^12 / D^(2*j+3),    j=0..7.
```

All divisions are in the base field; nonsingularity makes `D` a unit. The
scorer must verify byte-for-byte equality of every invariant between the two
coordinate models before using it.

For canonical representative `v in 0..p-1`, define
`bin(v)=min(5,floor(6*v/p))`. Each channel is crossed with all 720
permutations of digits `0..5`, yielding exactly 11,520 frozen predicates.
No coefficients, channel combinations, per-curve maps or post-run predicates
may be introduced.

## Decisions and controls

1. **Moment identity:** zero candidate/direct/A-A mismatches across all 576
   model-target records and all sixteen coordinates.
2. **Invariant identity:** zero native/`lambda=2` channel mismatches.
3. **Uniform positive:** one identical channel/permutation predicate is exact
   on all six curve/generator rows. Report all ties.
4. **Curve holdout diagnostic:** report exact predicates on `p=31` alone and
   whether each survives unchanged on `p=47`; this cannot promote a claim.
5. **Null control:** within each curve/generator row, apply a deterministic
   SplitMix64 Fisher--Yates shuffle to the scalar digits while retaining the
   moments. No predicate should remain exact on all six shuffled rows. Report
   the best target and shuffled-control match counts, rates and all tied
   predicate identifiers, plus the best target count in every channel.
6. **Positive scorer control:** a synthetic channel whose bin is exactly the
   digit must be detected with its identity permutation, and a one-label
   mutation must reject that exact predicate.

Any failed identity/control invalidates predicate scoring. Even a uniform
positive result is only a candidate for a separately frozen cohort and does
not establish a decoder, generic-prime-field applicability, or subexponential
ECDLP runtime. Timings are diagnostic and no performance comparison is in
scope. [pcc:unverified]
