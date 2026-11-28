# Independent prior-work and mathematical scope review

Date: 2026-11-23 UTC. Reviewer: separate `/root/map_cost_audit` agent in the same host/session team. Parent tracking PR: #4952. This review precedes the producer's frozen qualifier output. No new mathematical program, benchmark, root solver, or certificate checker was run for this review. It is an unsigned written audit, not a Lean proof, signed execution record, or novelty certification. [pcc:unverified]

The initial source reads observed checkout HEAD `62357e0977c708f0e216ad162b356dd0dc4d0baa`, following the task base `f0ca664eca0081a30adfcc79dcf62a32de40d2af`. The protocol was read after its freeze and matched SHA-256 `3eafeab5c2dc73c0add918bb23c9e8cdac1cf82d567d18a49792fd84aee0b20f`. No old coupled evidence, arithmetic source, active ledger, or concurrent extractor was changed.

## 1. Accepted theorem and its exact domain

Let the characteristic satisfy `p > max(3,2d)`, let `V = mu_d` split in the working field, and fix a nonempty subset `W` of `V`, of size `k`. Treat `A,B,X` as independent parameters. Put

    H(x,t) = product_(z in V) S3(x,z,t) mod (x^d-1),
    R_W(t) = product_(z in W) S3(t,z,X).

The claimed generic coefficient rank of `H mod monic(R_W)` is `r = min(d,2k)`. The written proof is sound with the following domain and representation qualifications.

**Monic chart.** The coefficient of `t^2` in `S3(t,z,X)` is `(z-X)^2`. Consequently the raw product has leading coefficient `lambda_W(X) = product_(z in W) (z-X)^2`, and has degree `2k` only when `X` is not in `W`. Monic normalization divides by `lambda_W`. Targets in `W` require a degree-drop branch. The protocol explicitly includes such targets.

**Specialization.** At `A=B=X=0`, every `z` in `W` is nonzero, `S3(t,z,0)=z^2 t^2`, and the normalized product is `t^(2k)`. The prior first-norm identity supplies

    [t^j] H_(0,0)(x,t) = (-1)^j binom(2d,j) x^(-j),  0 <= j < d.

Reduction modulo `t^(2k)` preserves columns `0,...,r-1`. In the increasing row order `0,d-r+1,...,d-1`, they form a scaled permutation matrix. Reversing the last `r-1` rows contributes sign `(-1)^((r-1)(r-2)/2)`; the column signs contribute `(-1)^(r(r-1)/2)`. The selected minor is therefore

    (-1)^(r-1) product_(j=0)^(r-1) binom(2d,j),

which is nonzero because `p>2d`. The rank is at least `r`, and the `d` by `2k` matrix dimensions give the matching upper bound. This proves generic rank over the parameter function field.

**Nonsingular curves.** The nonzero minor, the normalization denominator, and `4A^3+27B^2` define nonempty intersecting Zariski-open conditions: their product is a nonzero polynomial after denominators are cleared. This is a geometric generic statement. The singular specialization is only a nonvanishing witness. It is not a cryptographic curve or a group-relation fixture.

**Limits of specialization and sampling.** Remainder coefficients are rational functions of the parameters after normalization. A finite-field failure bound requires a new explicit degree bound for a cleared minor and a specified sampling distribution; the old coupled `2d^2/p` bound cannot simply be copied. There is no individual guarantee for the fixed Prime31 curve, no automatic statement conditioned on prime group order, and no automatic statement when `W` itself is selected from curve-dependent liftability data. If roots are not all in the base field, an arbitrary subset may require a coefficient extension; the frozen native protocol avoids this by using split `V`.

**Empty subset.** For `W` empty, `R_W=1` and the remainder is zero. This is a separate trivial quotient, not a positive-dimensional minor case.

## 2. Full right polynomial versus rational-root quotient

For `W=V` and `X` outside `V`, the right polynomial has degree `2d`. The already published first-norm identity gives `deg_t H <= 2d-1`, so reduction by that full polynomial leaves `H` literally unchanged. Thus the full nonreduced quotient case is a direct corollary of the prior first-norm rank result. It does not establish additional compression or a new extraction algorithm.

The actual rational-root quotient is different:

    G_X(t) = gcd(R_V(t), t^p-t), made monic.

It is split and squarefree, and its degree is the number of distinct rational right roots. At the cusp specialization, the full normalized polynomial is `t^(2d)`, but its rational-root part is only `t`. More generally, the witness `t^(2k)` for a fixed subset loses its multiplicities under this gcd. Therefore the proposed minor proof does not transfer to `H mod G_X`. A measurement of rank `min(d,deg G_X)` on specified fixtures is valid evidence for those fixtures, not a consequence of the nonreduced generic proof.

For a fixed non-2-torsion rational target, the discriminant of the quadratic `S3(t,z,X)` is `16 f(z) f(X)`, where `f(u)=u^3+Au+B`. This helps explain why restricting to rational roots can remove factors associated with nonliftable selector coordinates. It is still necessary to treat `z=X` degree drops, 2-torsion, coincident roots, and the point at infinity explicitly. The native protocol tests finite algebraic charts; it does not certify signed group relations or all infinity branches.

When `X=z`, the factor's linear coefficient is `-4f(X)`. For an ordinary rational target this is a genuine linear factor. For a nonsingular 2-torsion target it becomes the nonzero constant `(3X^2+A)^2`. These are concrete reasons to record actual degrees, rather than force degree `2d` in every cell.

## 3. Rank versus zero support

For `e=deg G_X>0`, let `C` be the `d` by `e` coefficient matrix of the canonical remainder in the monomial bases. Evaluation at the distinct roots of `x^d-1` and `G_X` multiplies `C` by invertible Vandermonde matrices on its two sides. Hence coefficient rank equals evaluation-matrix rank on the full split grid. This equality is a suitable native correctness check. A restriction to only liftable left roots has different dimensions and must be labeled separately.

The zero-indicator matrix is a nonlinear transformation of that evaluation matrix. In the split product algebra,

    E = 1 - H^(p-1)

evaluates to one exactly on zero entries and to zero elsewhere. Its rank may be small because its support is sparse; it is bounded by the number of nonzero entries. This does not conflict with maximal rank of `H`. For example, a full-rank matrix with no zero entries has a zero zero-indicator matrix.

Neither statement gives a cheap way to find the support. A dense coefficient representation of the quotient has `d*e` slots. Forming the projector, finding its zero/nonzero positions, recovering eliminated coordinates, and validating signs require algorithms with charged setup, arithmetic, memory, and failed-query costs. A sparse evaluation support can interpolate to dense coefficients. Knowing after enumeration that the indicator has small rank is not an input representation from which an extraction bound follows.

If a trace is used to count zero cells, it initially gives the integer count only modulo `p`; interpreting it as an exact nonnegative integer needs a count bound below `p` or separate reconstruction. The frozen qualifier can simply count its finite grid and avoid making a general trace-count claim.

## 4. Exact prior overlap and what remains distinct

The shared KB was unavailable in the parent runtime, with no exposed KB tools and no executable behind the repository wrapper. The following direct published-source fallback was reviewed. This limited search supports attribution, not an exhaustive absence or priority claim.

| Published source | Finding relevant to this task |
|---|---|
| `research/fass_prime31_20261122/coupled/REPORT.md`, sections 3 and 5 | Proves generic first-norm rank `d`; explicitly defines the target-dependent right resultant and its squarefree rational-root quotient; formulates the `d*e`-dimensional nonunit/common-root query; gives streamed-norm and pair-table baselines; leaves target-specific quotients and alternative zero-set representatives open. |
| `research/prime31_summation_compression_20261122/NEXT_IMPLICIT_INDEX.md`, section A | Already derives the Fermat idempotent and exact trace count for a split squarefree algebra with dimension below `p`; supplies the complete powering/trace recurrence; establishes the linear state floor only for the explicitly materialized dense implementation. It explicitly does not exclude all symbolic projector compression. |
| `research/prime31_summation_compression_20261122/NEXT_ROOT_INTERSECTION.md`, sections 4–5 | Establishes the dimension-charge limitation of explicit quotient-vector descent and states that a small circuit or bounded-rank intermediate tensor is insufficient without contraction, emptiness, and extraction costs. |
| `research/prime31_summation_compression_20261122/NEXT_IMPLICIT_INDEX.md`, sections B and F | Already charges complete signed tuple indexing, uniform-target row collection, and the conditional setup/query exponent budget. These are architecture-specific bounds, not general rootfinding lower bounds. |
| `research/prime31_summation_compression_20261122/iteration8/EXACT_CONTRACTION_AND_YIELD.md`, sections 1–3 | Already supplies exact signed group-semiring contraction and uniform-target yield/rank bounds. Its pair-table scan is a baseline, with actual support dimensions and empty queries charged, not a nonenumerative algorithm. |

The distinct proposed contribution is the fixed-subset nonreduced quotient minor argument and exact, source-bound observations of actual rational quotient ranks and zero support. The Fermat projector, the target quotient/nonunit formulation, and the requirement to charge state size were already present. Full separation rank excludes small exact separated linear representations of that canonical polynomial; it is not an arithmetic-circuit, zero-set, general rootfinding, or ECDLP lower bound.

## 5. Source binding and next review step

SHA-256 values of the reviewed published notes:

| Source | SHA-256 |
|---|---|
| `coupled/REPORT.md` | `caad5711bb0c4094c2f6f2de9460779816d911281fe8558957eaaa82ebdb0735` |
| `NEXT_IMPLICIT_INDEX.md` | `e5d6b7182453c4cc5a33b1f2d78c7f464aee58ca34771c827214ea93ed69b54d` |
| `NEXT_ROOT_INTERSECTION.md` | `7bf17f0b34efd2f43a838cfa03ec8ea99844537a5ef57754f66a996809c07a98` |
| `iteration8/EXACT_CONTRACTION_AND_YIELD.md` | `35a97211f5c0ef84878847203ce850343a465eb369c2900a534b5769697f9f20` |

The next assigned step is independent source inspection and bounded replay after the producer freezes the native source, immutable executable, and raw receipts. That outcome will be recorded separately, with pre/post hashes and raw stdout/exit records. No native replay has occurred as part of this written review.
