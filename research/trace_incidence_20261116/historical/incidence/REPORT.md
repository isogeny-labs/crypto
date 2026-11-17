# Implicit three-pool incidence GCD: implementation and results

The exact GCD candidate works, but it does not beat the strong streamed-pair
control on this panel. Avoiding an explicit pair table does not by itself
reduce the amount of algebra below pair enumeration. No new ECDLP exponent
or cryptographic-size speedup is established here.

## Exact construction

For three globally disjoint abscissa pools, let `F2` and `F3` be their monic
root polynomials and `R=Fp[Z]/F3`. For each first-pool coordinate `x`, write

```
S3(x,Y,Z) = (x-Z)^2 (Y^2 + beta Y + gamma).
```

The leading coefficient is a unit in `R`. Reduce `F2(Y)` to `u+vY` in the
quadratic algebra and compute its norm `u^2-beta*u*v+gamma*v^2`. The GCD of
`F3` with the product of these norms has exactly the third-pool coordinates
participating in signed zero-sum triples. This follows pointwise at every
distinct root of `F3`; the omitted leading-coefficient factors are units.
`F1` is split and monic, so its outer resultant is the product over the first
pool. No pair table, pair-root list, or hidden scan over the second pool is
used in this computation.

`timed_candidate_primary.py` preserves the exact source used for the primary
measurements. `candidate.py` additionally continues searching after an
uninformative label relation (`sum sign*beta=0`). All 90 original fixture
results, witnesses, and recovered scalars are unchanged by that correction.
A public-valid regression input demonstrates that the old early return loses
a later informative witness and the corrected implementation finds it.

## Work and memory charged

With equal pool size `B` and quotient-multiplication cost `M(B)`, the Horner
arm makes exactly `2B^2+10B` explicit quotient multiplications, plus `B`
quotient inversions. It has an upper bound of
`O(B^2 M(B) + B M(B) log B)` before witness recovery. This is not a new
subquadratic field-operation bound.

The blocked arm uses `m=floor(sqrt(B))+1` and
`c=ceil((B+1)/m)`. It makes exactly `B*(8+2m+5c)` explicit quotient
multiplications, plus `B` quotient inversions. Its block matrix products
have a classical scalar-product count `2B^2*m*c`, of order `B^3`.
Thus a reduction in quotient-product count does not establish a reduction
of total field work below cubic. A valid classical upper bound is
`O(B^(3/2) M(B) + B^3 + B M(B) log B)`. Native FLINT may use faster internal
multiplication; the reported schoolbook coefficient-pair and classical
matrix counts are proxies, not measured backend field-operation counts.

Largest explicit stored block matrices have `O(B^(3/2))` entries; the
Horner arm streams degree-`B` polynomials. No degree-`2B^2` resultant is
expanded. Native allocation and startup are included in measured process
peak RSS. At `B=128`, Horner makes 34,048 quotient products, blocked 11,136;
blocked additionally has 4,325,376 classical matrix scalar products and a
largest explicit matrix of 3,072 entries.

Witness recovery evaluates the GCD at known third-pool coordinates
(`O(B deg(G))` scalar work), then uses a signed second-pool hash map and
group additions to find a first witness. The worst-case group-search work
can be `O(B deg(G))`; it is not assumed constant. Label sums, modular scalar
recovery, and verification `kP=Q` are charged. Uninformative matches are
skipped by the corrected source. Public mixed-label pool construction is
separately charged from fixture generation; no structured factor-base
labels are assumed available for free.

## Primary panel

Each size has 12 natural random fixtures: three independently generated
prime-order curves and four public labelled pools per curve. Subgroup
orders satisfy `r/B^3` approximately 8. These are small prime fields using
FLINT word-sized `nmod` arithmetic, not 256-bit prime-field timings. The 15
planted controls and 15 negative aliases are stored separately and do not
increase the natural sample count. All 180 fixture/arm runs finished;
there are no censored primary rows. Misses remain charged attempts.

| B | natural relation/miss | Horner complete oracle, ms | blocked complete oracle, ms | median public pool generation, ms |
|---:|:---:|---:|---:|---:|
| 8 | 4/8 | 0.387 | 0.723 | 1.611 |
| 16 | 7/5 | 1.361 | 2.829 | 3.772 |
| 32 | 4/8 | 6.283 | 11.492 | 9.259 |
| 64 | 6/6 | 41.963 | 59.497 | 21.548 |
| 128 | 7/5 | 428.240 | 353.280 | 52.469 |

These are medians of complete calls, including polynomial preparation,
normalization/inversion, norm evaluation, GCD, misses, and witness/scalar
recovery. They are exploratory shared-host measurements with CPU 1 pinned
and one native thread. Each fixture/arm starts in a fresh process and repeats
fresh complete calls for at least 0.08 seconds, allowing short calls to be
measured by an outer CPU bracket. Repeated calls do not reuse polynomial
preprocessing. The saved inner phase CPU intervals can undersum badly and
are diagnostic only; comparisons use outer batch CPU divided by repetitions.

Public generation plus complete-oracle median costs at `B=128` are 480.809 ms
(Horner) and 408.228 ms (blocked); medians of sums need not equal sums of
medians. Fresh-process/library/serialization CPU startup is retained
separately (about 58.9 ms median across the panel). Add it for literal cold
one-shot cost, or amortize it explicitly for a persistent solver. Peak fresh
process RSS, including Python/backend startup, is approximately 26--27 MiB.
Do not infer an asymptotic exponent from five small sizes or treat different
implementation languages as a controlled algorithm-only comparison.

The sibling portable C++ streamed-pair baseline has complete-oracle medians
of 0.00609, 0.01546, 0.04695, 0.16668, and 0.72155 ms for these five sizes.
At `B=128`, its median public generation plus oracle cost is 52.720 ms,
versus 408.228 ms for blocked incidence; the generation cost masks part of
the much larger oracle gap. This control includes parsing, indexing,
search, recovery, and teardown inside its batch bracket, and records process
startup separately. It is a faithful portable C++ control, not a replay of
the upstream Rust executable. Its results are in
`../repo_controls/pair_benchmark.json`. The existing repository m2
quotient/GCD target-scan port takes about 404.364 ms at `B=128`; beating a
weaker elimination control would not constitute beating pair enumeration.

`analysis_results.json` derives paired costs and exact four-seed-order runs
for each curve, charging each failed pool before the first informative
relation. An all-miss four-pool sequence is censored, with its spent work
retained; it is not assigned a finite success cost or labelled unsatisfiable.

## Native modular-composition supplement

`compose_candidate.py` interpolates the known third-pool ordinate polynomial
`Y3(Z)` and forms `x(P1+P3)` and `x(P1-P3)` in `R`. It evaluates `F2` at these
two rational maps using native `FLINT compose_mod` and multiplies the results.
This is an exact split-coordinate version of the same elimination; it uses
no pair table. It costs `2B` native degree-`B` modular compositions, `5B`
explicit quotient products, and charged ordinate interpolation. Internal
modular-composition work is not incorrectly counted as one field operation.
No new complexity bound is claimed for the backend.

The separate bounded supplement tests two natural seeds on one curve per
size and the uninformative-first regression. All 11 root sets agree with
the independently enumerated group-law oracle, and all informative witnesses
verify. It is faster than these Python/FLINT Horner calls at several small
sizes, but does not show a scaling improvement at `B=128` (about 380--386 ms
for its two cases) and does not beat the native streamed-pair baseline.
This supplement is not pooled with the primary scaling panel.

## Exceptional inputs and checks

The caller must supply three finite, valid, globally disjoint coordinate
pools. Duplicate coordinates, including a point and its negation across
pools, violate the normalization precondition and are rejected explicitly.
Points at infinity are outside this representation. Prime subgroup fixtures
have odd order, hence no nonidentity two-torsion points. Tangent/repeated
roots in the quadratic norm are valid algebraically: the final squarefree
`F3` GCD only selects coordinates, so multiplicities are not counted as
extra relations. No failed inversion is silently interpreted as a miss.

`audit_results.json` records equality of both primary GCD root sets with
separately written control group arithmetic on all 90 fixtures, independent
witness verification, 144 random small-field quotient-algebra checks,
an erased-positive mutation check, and the label-recovery regression.
`edge_check_results.json` additionally exercises coordinate-collision and
nonunit-denominator rejection and 12 repeated-quadratic-root norm cases,
including factors whose norm vanishes.
The parent additionally checks the saved results using Jacobian arithmetic
and validation-only exact fixture truth. This is executable evidence, not a
formal proof or an independent third-party certificate.

## Reproduction

From `/workspace/ecdlp_research` with the supplied `math_vendor` install:

```
python incidence/audit.py
python incidence/edge_checks.py
python incidence/benchmark.py --candidate incidence/timed_candidate_primary.py
python incidence/compose_panel.py
python incidence/summarize.py
```

The benchmark caps each subprocess at 30 CPU seconds and retains censored
statuses. `benchmark_results.json` retains all primary rows, actual fixture
hashes, public generation, complete CPU, startup, recovery, counted algebra
dimensions, and RSS; `benchmark_summary.json` preserves the timed source
hash. `compose_panel_results.json` retains the separate supplement.

The missing contribution remains a better total-complexity bound or a
reproducible improvement over the strong pair control, with pool creation
and failed searches included. These implementations provide neither at
present.
