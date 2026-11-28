# The missing relation locator

This note states the remaining algorithmic interface for the selected
`E/F_p: y²=x³−3x+40`, `p=2^61−1` experiment. It is a research target,
not an algorithm or a claim that a solution exists. The multiplicative
root x-base has `B=2447` columns and the proposed cube-root-scale budget is
`Õ(B²)`; here `B²=5,987,809` is a scale marker, not a measured runtime.

## What the arithmetic already supplies

- The native census certifies `#E=7r` with prime `r=329406144237565159`
  and exactly 2,447 lift-valid x-values in `μ_4950`.
- Cofactor projection accepts any signed row whose full-group sum lies in
  rational 7-torsion. Exact tuple-class counts give nearly seven times as
  many eligible signed configurations as full-group equality. Converting
  that to expected actual relations uses an unverified uniformity model.
- Under that model there are about 57.58 six-term decompositions for a fixed
  prime-subgroup target and about 20,078 seven-term homogeneous rows after
  identifying global sign. Availability does not provide a witness search.
- Given verified rows of rank `B−1` modulo `r` and verified decompositions
  of both `G` and `Q`, the native checker recovers and verifies `log_G(Q)`.
  The included dense nullspace routine is only a toy verifier; a large run
  needs sparse linear algebra and an end-to-end cost ledger.
  An [enumerated toy rank probe](toy_relation_rank.md) reaches `B−1` on
  several small bases, but does not establish M61 row independence or
  change the collector's quartic cost.
  A [sparse Wiedemann component](sparse_solve.md) now compresses tall
  verified rows and certifies rank `B−1` plus a null vector on success
  within quadratic field work per bounded attempt. M61 relation
  collection, retry bounds, and target descents are still missing.

## Where direct approaches spend the budget

| Object or method | Selected-instance size | Asymptotic work or state | Consequence |
|---|---:|---:|---|
| Pair-sum table | `B²=5,987,809` | `Θ(B²)` | Fits the proposed scale |
| Distinct-index signed triple inventory | `8·C(B,3)=19,512,280,120` | `Θ(B³)` | 3,259 times `B²` here |
| Multiset triple-sum eliminant degree, lower bound | `2B³−B²=29,298,349,437` | `Ω(B³)` dense coefficients | 4,893 times `B²` here |
| Direct three-versus-three target matching | triple inventory scale | `Θ(B³)` time and space | Prime-field exponent `1/2` |
| Direct three-versus-four homogeneous matching | quartet inventory scale | `Θ(B⁴)` time | Prime-field exponent `2/3` |

The quotient-algebra and target-intersection notes explain why the compact
`O(B²)` description of the eliminant does not locate its sparse target
zeros. The native toy probes measure support, target coverage and witness
replay; they do not shrink the triple inventory. Torsion translations,
fixed-degree isogeny transport, scalar returns, and pair collisions have
only scoped negative or constant-factor results on this base.
The [exact M61 pair census](pair_sum_census.md) establishes that all `B²`
pair-sum x-classes are distinct, so there is no selected-instance
pair-collision compression of the quotient algebra; the same is true after
cofactor projection. The [three-pair ledger](three_pair_gap.md) makes the
remaining structured batch-intersection objective explicit.
The [one-large-prime recombination analysis](large_prime_recombination.md)
shows that cheap partial six-sum rows also miss the `B²` target under a
uniform birthday model; a structured large-prime map would be a new
algorithmic ingredient, not a consequence of the tuple count.
The [ratio-fiber census](ratio_fibers.md) checks one way to use the smooth
multiplicative x-subgroup: every nonidentity ratio still has `Θ(B)`
lift-valid ordered pairs, with no dense selected-instance fiber.
The [ratio-cover calculation](ratio_cover.md) shows that a fixed
nonidentity ratio creates a genus-four correspondence with a genus-two
quotient. It gives an explicit quartic for **pair-sum queries** with
`O(|H|·polylog p)` expected work per target, now exercised by a
[native M61 locator](native/examples/pair_locator.rs). Pair hits are far
too rare to furnish six-term relations by
themselves; a higher-arity locator and full cost ledger are still needed.
The [CM positive control](cm_positive_control.md) shows why abundant
short relations from an automorphism cannot substitute for independent
cross-orbit equations or target descents.
The [Frobenius-discriminant check](cm_isogeny_obstruction.md) rules out
transporting those extra CM automorphisms into the selected isogeny
class over `F_p`.
The [Dickson trace-base probe](trace_dickson.md) instead gives exact
three-point horizontal-line rows on the selected curve: `μ_8190` yields
698 independent rows among 2,092 columns. They are disjoint-orbit rows,
so 1,393 rank dimensions still need cross-orbit equations. The uniform
projected seven-term estimate exceeds that gap, but no actual extra rank
or target witness has been measured, and the row locator remains missing.
The smaller `μ_7137` candidate has 1,703 columns, 568 free rows, and
`B²=2,900,209`; its orbit-distinct projected seven-row uniform estimate
is 1,543 versus 1,134 remaining dimensions, with no found witnesses.
Tuning `b` to make a trace fixed-ratio quotient rational leaves at most
six such ratios for any nonsingular curve. In the intended
`|H|=Θ(B)=Θ(r^(1/6))` regime, three special-ratio pair sums yield only
`O(r^−1/2)` expected fixed-target hits and, with one extra base point,
`O(r^−1/3)` expected seven-point rows under uniform prime components.
The [engineered-curve census](trace_dickson.md#fixed-parameter-ratio-quotient)
confirms the restricted fiber's small size; any viable locator must
also exploit generic ratios or a new nonuniform correlation.
The single-parameter trace curve has a genus-three dihedral model whose
Jacobian is `F_p`-isogenous to `E_b²×T_b`, with
`T_b: W²=u³+bu²+u`. The two `E_b` components recover the known
horizontal rows, while `T_40` has a different M61 point count from
the selected DLP curve. This structural splitting supplies no
cross-orbit row or higher-arity locator by itself; see the
[differential proof](trace_dickson.md#dihedral-cover-and-its-jacobian).
An engineered nonzero `b` makes `T_b≅E_b` with a 55-bit prime
subgroup, but the natural pullback through the parameter cover followed
by trace pushforward is the zero homomorphism: the two points in each
inversion fiber always sum to fixed two-torsion on `T_b`. The explicit
isomorphism therefore supplies no new DLP endomorphism or relation
locator by that route. The equal-`j` curve's exhaustive `μ_7137`
three-point and signed pair-sum scans likewise find no cross-orbit
triple or distinct-column four-point collision before or after
prime-subgroup projection; higher-arity rows remain untested there.
The explicit equal-`j` identification also gives a rational
mixed-sum x-map `P(t)±R_E(t)` that can be queried in `O(|H|)` group
work. On `μ_7137`, neither the second image nor either mixed sum
hits the trace base for any of 3,549 lifted parameters; its only
two repeated sum x-values are the inversion pair at `t³=1`.
This is a cheap query mechanism with no observed usable rows,
not a higher-arity locator.
After `[104]` projection it has 3,550 repeated sum entries, but every
collision stays within one inversion parameter orbit. Its six base
hits are exactly the `t³=1` cases where the second image projects to
zero; they add no rank. Collision counts must therefore be interpreted
with their parameter witnesses, not as independent relations.

### Local-map supply ledger

Suppose an explicit rational map produces `Θ(B)` point queries by
visiting the `B` trace parameters, and each non-forced projected query
is uniformly distributed among the `r/2` signed x-classes of a prime
subgroup. A `B`-column base then catches an expected `Θ(B²/r)`
outputs per map. Evaluating `K` such maps point by point costs
`Ω(KB)` and yields `Θ(KB²/r)` expected base hits. Within the
`Õ(B²)` budget, `K=Õ(B)`, leaving only
`Õ(B³/r)=Õ(r^−1/2)` hits when `B=Θ(r^(1/6))`.
Obtaining `Θ(B)` hits by independent local images requires
`K=Θ(r/B)` maps and `Θ(r)` point queries, far above the target
budget. For the equal-`j` fixture, `B=1775`, `r=22171567371951581`,
and `B²=3150625`; the leading `B³/r` value is about
`2.52×10^−7` before sign and map multiplicities. The observed zero
non-forced base hits are consistent with this model.

This is a conditional supply calculation, not a lower bound for all
algebraic algorithms. It assumes uniform non-forced images and charges
each local point query. A viable cube-root route would need a proved
algebraic incidence that concentrates map images on the base, or a
batch witness extractor whose amortized cost escapes this ledger;
it must still produce rank-sufficient cross-orbit rows and both target
descents. Forced inversion and torsion hits are excluded because they
add no rank on the measured fixture.
The equal-`j` equation leaves only `b=0` and `±1913898821915072477`
over M61. The zero case has 2-power order; the opposite sign has a
44-bit largest prime factor and `B²` already near `√r`. Tuning `b`
within this exact-isomorphism family therefore offers no further
large-prime M61 fixture beyond the measured positive sign.
For the selected b40 generic trace-ratio quotient, the exact
`j`-invariant is a nonconstant degree-six rational function of
`s=α+α^−1`. The native scan finds no nonidentity
`α∈μ_2379` with `j(Dα)=j(E_40)`. A fixed cyclic isogeny degree
`d` can affect at most `12ψ(d)` α-values over the algebraic closure
when `p∤d` (at most three rational q cube roots each), so bounded-degree
quotient transport cannot organize a positive fraction of generic
ratio fibers. This does not rule out high-degree isogenies or a
different batch intersection algorithm.
Likewise, the [direct common-parameter cover](ratio_cover.md#genus-calculation)
for `n` distinct fixed ratios has genus
`1+3(n−1)2^(n−2)`: six simultaneous elliptic lifts already give
genus 241. Its all-product quotient has genus eight but loses the
individual lift signs. A low-genus quotient alone therefore does not
replay a six-point relation; multi-parameter batch methods remain open.
The diagonal-sign quotient that actually retains `x(ΣP_i)` has
genus 113 for six distinct ratios. A native M61 fiber check finds
all 32 sign-orbit sum x-values distinct in every one of 75 fully
lifted root-subgroup fibers, and a sign-flip argument proves that
this sum coordinate generates the quotient field over the common
parameter. Its degree-32 minimal polynomial over that parameter
still gives no cheap target inversion or factor-base witness. The
seventh summation polynomial becomes degree at most 192 in the
common parameter for a **fixed** six-ratio tuple; there are roughly
`m⁵` relative tuples, while testing only `O(B²)` tuples over
`O(B)` allowed parameters gives `O(B³/r)` uniform expected target
hits. Root degree does not shrink the tuple search.
The native fixed-tuple search on selected M61 recovers one planted
six-column projected target after 4,950 parameter checks and 2,400
signed sum evaluations. Its 2,400 distinct projected sum x-classes
cover exactly 4,800 subgroup target points after global sign,
`1.46×10^−14` of `r`; an unrelated subgroup target misses.
This validates one descent component and quantifies its very small
coverage. It does not locate a useful tuple for an arbitrary target
within the `Õ(B²)` total budget.
An exact intersection of those 2,400 projected six-sum x-classes
with the 2,447 projected root-base x-classes is empty. This tuple
therefore yields no seven-point row by adjoining a seventh base
column. An independent-uniform model predicts only
`2·2400·2447/r≈3.57×10^−11` such x-class matches, so this is a
scoped negative observation, not evidence against a structured
cross-tuple generator.
For the symmetric ratio tuple `μ₆`, parameter rotation permutes the
six lifts. An orbit-representative scan reconstructs the entire
signed-sum image using 825 parameters and 448 sum evaluations,
versus 4,950 parameters and 2,688 evaluations in the redundant
full scan. The exact image has 448 x-classes and covers only 896
signed targets (`2.72×10^−15` of `r`); it has zero base-column
matches. The sixfold saving is matched by sixfold lower coverage,
so this symmetry by itself does not change the global locator budget.
All nontrivially stabilized six-x subsets have stabilizer order
`2,3`, or `6`. The exact M61 liftable-orbit counts are respectively
616, 194, and 14. An order-three exhaustive scan evaluates 599,072
projected six-sums and finds no projected base match. For order two,
three antipodal-pair sums plus a base point form a four-sum problem:
hashing two pair sums and probing the third plus a base point costs
3,030,720 and 6,029,408 group additions respectively, with a
1,515,360-entry x-class table. It finds zero matches. Thus the
entire nontrivially rotation-stabilized six-set family gives no
seven-point row on the selected base, despite admitting an exact
`O(B²)` meet-in-the-middle check. Under a uniform model the
order-two family's signed-sum capacity gives only `O(B⁴/p)`
expected base matches when `B≈p^(1/6)`; that is not an algebraic
impossibility result for other curves or asymmetric ratio families.

### Fixed-ratio dimension budget

For each nonidentity `t∈μ_4950`, let `E_t` count directed base-x
pairs `(x,tx)` with both endpoints in the 2,447-column root base.
The native census verifies `Σ_{t≠1}E_t=B(B−1)=5,985,362`;
`E_t` ranges from 1,157 to 1,257, with the maximum at
`t=779821647332969385`. Seven-point supports containing one such
pair number at most `E_t·C(B−2,5)` before duplicate supports.
Counting 64 signs up to global negation and assigning uniform
projected subgroup sums gives an expected row **upper bound** of
`64·E_t·C(B−2,5)/r`. Even the busiest selected ratio gives only
**177.10** expected rows, against the `B−1=2,446` rank target.
This is a conditional expectation, not a deterministic row bound.

In a growing family with `B=Θ(p^(1/6))` and `E_t=Θ(B)`, one fixed
ratio exposes `O(B⁶)` signed candidate supports and thus `O(1)`
uniform expected rows. Full rank needs `Θ(B)` independent rows, so
a fixed finite collection of ratios cannot supply it under this
model. A viable ratio-batched route could process `Θ(B)` ratios in
`Õ(B²)` total work, provided each ratio's rare seven-row witnesses
can be extracted in `Õ(B)` amortized work and the recovered rows
have independent rank. No such witness operator is known here;
the arithmetic count alone does not construct one. It also leaves
general-target descent and sparse-solve costs to be charged.
This rank budget is specific to the **fixed root-base log system**.
[Amadori, Pintore, and Sala (2018)](https://eprint.iacr.org/2017/609.pdf)
give a different prime-field index-calculus variant: choose points
`R_i=u_iG+v_iQ` with known coefficients and recover the target log
from one nontrivial relation among them. That avoids a `B−1`-row
linear system, but their randomly generated base points do not retain
the `x∈μ_m` structure used by our ratio and cover probes. Our
fixed-ratio row deficit therefore does not rule out a one-relation
method; transferring the root-structured locator to a coefficient-
known base would require a new coordinate-input construction and
its full cost ledger. The [2020 analysis of naive Semaev
index calculus](https://doi.org/10.1515/jmc-2019-0029) gives a
conditional Gröbner-basis cost obstruction for that naive method,
not a lower bound on every structured batch locator.
The simplest hybrid fails by exact sampling arithmetic: the signed
projected root base has `2B=4,894` distinct subgroup points. If `u`
is uniform modulo `r`, then `R=uG+vQ` is uniform for every fixed `v`
and every target `Q`, so `Pr[R∈±[7]F]=4894/r`. Rejection sampling
needs `r/4894≈6.73×10^13` point evaluations per accepted base point;
even `B²=5,987,809` trials give only `8.90×10^−8` expected hits.
Choosing a root-base point first leaves its `G,Q` coefficients unknown.
This rules out the elementary sampling bridge, not a targeted
coordinate map or a different one-relation construction.
The same limit holds without independent random trials for a fixed
schedule of `K` pairs `(u_j,v_j)` with every `v_j≠0 mod r`: each map
`Q↦u_jG+v_jQ` is a permutation of the subgroup, so its inverse image
of `±[7]F` has exactly 4,894 targets. The union bound gives success
probability at most `4894K/r` for uniform `Q`, and covering every
target requires `K≥⌈r/4894⌉=67,308,161,879,356`. This statement
does not apply when coefficients are selected from target coordinates
or previous coordinate observations change the next map.
The [coordinate selector toy probe](native/examples/coordinate_selector.rs)
([frozen output](coordinate_selector.out)) tests that open case on
three certified control curves, enumerating **every nonzero target**
in each prime subgroup. It tries `u=x(Q), y(Q), x(Q)+y(Q), x(Q)²`
modulo `r` in `R=Q+[u]G`, using only target coordinates to select
`u`. The four-rule union covers 54 versus 58 targets for four fixed
shifts at `r=59`, 397 versus 409 at `r=3541`, and 615 versus 621
at `r=16421`. Individual rule counts and signed projected base
sizes are in the frozen output. These predeclared elementary rules
show no consistent enrichment over the matched fixed-shift control.
The exhaustive screen precomputes all `r` multiples of `G`, already
beyond an `Õ(B²)` setup budget in a growing `B≈r^(1/6)` family;
evaluating each coefficient by scalar multiplication instead costs
`O(log r)` group operations per trial. A base hit is not yet a log
relation or a target descent. The toy negative result does not test
more complex adaptive maps.

### Seven-point evaluation determinant

On `E_40`, the Riemann–Roch space `L(7O)` has basis
`1,x,y,x²,xy,x³,x²y` (pole orders `0,2,3,4,5,6,7`).
For seven distinct affine points `P_i`, form the `7×7` evaluation
matrix with one row per point. Its determinant vanishes **if and only
if** `ΣP_i=O`: a null vector is a function with those seven zeros
and at most a `7O` pole, so its principal divisor gives the group
relation; conversely a zero sum makes that divisor principal.
This is an exact algebraic relation predicate, not a method to find
the rare singular seven-subsets.

### Norm-divisor batch target

The same predicate has a form that exposes the multiplicative root
set. For a full zero-sum seven-row, a function in `L(7O)` is
`F=A(x)+yB(x)` with `deg A≤3`, `deg B≤2`. Its norm under
`y↦−y` is `N_F=A(x)²−(x³−3x+40)B(x)²`, of degree at most seven.
For seven distinct abscissae, the row condition is equivalent to a
nonzero such `F` whose degree-seven norm is a scalar times their
product `∏(x−x_i)`. Thus a genuine root-base row gives a norm
polynomial dividing `x^4950−1`, with seven lift-valid roots and
recoverable signs. The division condition by itself must still be
checked against the curve and the signs.

The six nonzero rational 7-torsion sum classes also have this form.
For a chosen `τ≠O`, take `F=A(x)+yB(x)∈L(8O)` with
`deg A≤4`, `deg B≤2` and impose `F(−τ)=0`. Then `N_F` has
degree at most eight and contains the fixed factor `x−x(τ)`.
The quotient `D_τ=N_F/(x−x(τ))` has degree at most seven; when it
has seven distinct lift-valid root-base abscissae, the signs selected
by `F=0` give points summing to `τ`. Conversely a distinct-x seven-row summing to
`τ` supplies this `F` by Riemann–Roch. The [native M61 norm
replay](native/examples/norm_divisor.rs) ([frozen output](norm_divisor.out))
constructs both full-zero and nonzero-torsion witnesses from six
root-base points plus a planted seventh point, solves the linear
evaluation nullspaces, and verifies the norm roots and quotient.
Both planted tails lie outside the root base, so this is a
representation check, not a newly found row.

For target descent, let `T` be a prime-subgroup point and compute
`R=[7^−1 mod r]T`. Six original base points project to `T` exactly
when their full-group sum is `R+τ` for some `τ∈E[7]`.
For each of the seven `τ`, append the fixed point `−(R+τ)` and
seek an `L(7O)` norm with that one prescribed abscissa and six
distinct roots in the factor base. This makes the missing target
primitive as explicit as the homogeneous row primitive, but still
does not locate its rare six roots. The native replay plants `T=[7]S`
from six base points, computes the prime lift without using a target
log, recovers a nonzero torsion shift equal to five times the
example's verified order-seven point, and verifies that
the appended point is exactly `−S`. Its norm has the six base roots
and that prescribed seventh root outside the base. This certifies
the algebraic interface, not general-target coverage.

A proposed batch primitive would enumerate all degree-seven divisors
of `x^m−1` admitting this norm representation and extract their
point witnesses within `Õ(B²+output)` work, including every torsion
class. Testing one candidate norm by polynomial gcd is cheap; finding
the rare candidates is the unsolved step. Enumerating seven-element
root subsets costs `Θ(B⁷)`, and sampling arbitrary `A,B` is too
sparse. No complexity claim follows from the norm identity alone.
The [mixed orbit seven-row screen](native/examples/mixed_orbit_seven.rs)
([frozen output](mixed_orbit_seven.out)) tests one asymmetric norm
factor shape, `1+1+2+3`, in quadratic work. It hashes 5,985,362
signed singleton-pair x-classes and probes 3,824,128 sums of one
liftable antipodal pair and one liftable cube-root triple. **Zero**
x-classes match, so the complete restricted family yields no
seven-row on selected M61. A uniform model predicts only about
`1.39×10^−4` raw x matches, making zero unsurprising. This
falsifies the restricted orbit family, not the general norm search.

The [native determinant replay](native/examples/seven_eval_rank.rs)
([frozen output](seven_eval_rank.out)) constructs the `μ_7137`
trace base, obtains a nonzero determinant
`869836213412560046` on its first seven columns, verified to lie
in distinct dihedral orbits, and gets zero
after replacing the seventh point by the negative sum of the first
six. The constructed tail is not claimed to be a base point. Because
those seven evaluation vectors are independent, their 35 triple
exterior products form a basis of `Λ³(F_p^7)`. Thus even the
orbit-distinct triple Plücker vectors from this trace base have
**full linear span 35**:
a simple lower-dimensional *linear* wedge compression is absent.
This does not rule out nonlinear structure or fast specialized
determinant search.
An independent [two-parameter trace template](trace_dickson.md#two-parameter-trace-identity-template)
does generate 25 verified independent cross-orbit seven-point rows
on a toy curve in `O(m²)` candidate-support work, but its count
matches equal-size random controls. Even if every selected M61
`μ_8190` parameter pair were admissible, the uniform expected row
count is only `1.30×10^−8`, far below the residual rank.

For projected rows, `Σ[7]P_i=O` allows the full sum to be any
rational 7-torsion point `τ`. One can test each nonzero `τ` by
appending `−τ` to the seven points and evaluating `L(8O)` at the
eight distinct affine points; the `τ=O` case uses the determinant
above. Repeated points require confluent evaluation rather than a
naive duplicate row. This increases the constant number of predicates,
but supplies no `Õ(B²)` witness extractor.

## Generalized-birthday candidate and its filtration gap

A Wagner-style tree is tempting for longer, constant-weight rows. If an
additively compatible filtration were available, `2^h` disjoint signed
lists of length roughly `r^(1/(h+1))` could be merged in roughly linear
list work, with the usual carries handled in the numerical group model.
For the selected `μ_8190` trace base, partitioning its `B=2092`
columns gives an average `2B/32=130.75` signed choices per 32-list bucket,
versus `r^(1/6)≈831.04`; with 64 lists it gives `2B/64=65.375`
versus `r^(1/7)≈318.08`. These finite lists are below even the ideal
numeric threshold. Larger bases or an asymptotic family would still
need the following missing operation.

The standard merge filters partial sums by numerical low bits or nested
additive buckets so earlier constraints survive later additions. An
elliptic point's x-coordinate bits do **not** have that property:
`x(P+Q)` is not `x(P)+x(Q)` modulo a bit range. Filtering point x-values
at one merge merely returns another list of apparently general points;
it accumulates no constraints toward `O`. In the prime-order projected
subgroup, every homomorphism to a group of order below `r` is trivial.
A nontrivial additive label into a group of order `r`, normalized on `G`,
would itself reveal discrete logs. This excludes the ordinary quotient
filtration, not every possible algebraic batch locator. Any proposed
coordinate-based replacement must prove a conditional distribution or
composition law that survives all merge levels, then charge tuple
distinctness, row rank and both descents. Increasing relation length
alone does not give the claimed `Õ(B²)` ECDLP algorithm.

## A result that would cross the boundary

An explicit candidate should take the certified curve, its ordered signed
root base, and a target point `T`, then return a **verified six-column
decomposition** of `[7]T` or a justified failure. Shared setup must be
charged once and be independent of `T`. The same construction must produce
at least `B−1` independent verified seven-column homogeneous rows modulo
`r`, or another rank-sufficient relation system, and decompose both the
known subgroup generator and the ECDLP target. Every row must pass group
addition; the recovered scalar must pass `[d]G=Q`.

For the desired `Õ(B²)` total budget, shared preprocessing and memory must
stay within `Õ(B²)`, and roughly `B` useful rows leave only `Õ(B)`
amortized work per row. The complete ledger must include factor-base
construction, candidate generation, witness recovery, failed attempts,
matrix rank and sparse solve, target descents, and final verification.
An asymptotic claim additionally needs a growing family with a suitable
divisor of `p−1`, enough lift-valid points, non-negligible target coverage,
and bounded constants. A finite M61 result by itself would not establish
that family-level exponent.

The next mathematical step is therefore a **circuit-aware target-zero
locator** that uses the pair-sum representation without expanding its
`Θ(B³)` triple image. No such locator has been constructed in this branch.

## Boundary with the existing pencil collector

The repository's `research/pencil_large_prime_20261029/` pipeline collects
symbolic large-prime relations in a **genus-2 Jacobian** on synthetic
instances. Its own source assessment states that the cited double-large-prime
complexity theorem (Gaudry, Thériault and Thomé, 2005,
https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf) assumes genus
at least three and is not transferred to
that genus-2 stream. Applying the pencil pipeline to this selected elliptic
ECDLP would additionally require an explicit, efficiently evaluated map of
the `r`-order subgroup and target into its Jacobian relation setting, plus
the full relation, rank, and descent costs there. Neither that map nor an
`Õ(B²)` bound follows from the existing pencil producer. The useful
reusable piece is its disciplined row verification and cost accounting,
not a ready elliptic relation generator.
