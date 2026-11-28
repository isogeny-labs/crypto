# M61 root-base relation pieces

This is a native Rust, public synthetic ECDLP research fixture. It certifies
one curve instance, enumerates a multiplicative-root x-factor base, counts
cofactor classes of signed tuples exactly, and verifies supplied relations.
It does **not** collect the predicted relations or solve the 61-bit ECDLP.
The [locator contract](locator_contract.md) puts the exact missing
witness-finding step and its full cost budget in one place.
It also records an exact seven-point evaluation-determinant predicate
and the measured full exterior rank of the selected trace base; neither
turns the predicate into an `Õ(B²)` witness search.

## Exact instance and census

The curve is `E/F_p: y²=x³−3x+40`, with `p=2^61−1`. The native executable
checks primality of `p` and `r` using the deterministic seven-base Miller–Rabin
test for `u64`, then verifies that

`#E(F_p)=N=2305843009662956113=7·329406144237565159=7r`.

Its point `R=(0,313662643971843112)` satisfies `[N]R=O`, `[r]R≠O`, and
`[7]R≠O`. Since `N` lies in the Hasse interval and the interval width is
smaller than `N`, these checks certify the exact group order. The program
also verifies that `z=636328216386749903` has order 4,950 in `F_p*`, and
finds exactly **2,447** of its powers whose x-coordinates lift to curve points.

For the chosen sign of each lift, the seven cofactor-class counts relative to
`T=[r]R` are `[324,369,343,339,352,365,355]`. A descending `u128` dynamic
program then counts unordered, distinct-column signed `k`-tuples, taking
both point signs for every selected column:

| k | Exact signed total | Exact class-0 count | Total/class-0 |
|---:|---:|---:|---:|
| 6 | 18,966,495,451,758,766,912 | 2,709,499,351,595,025,010 | 6.999999996528 |
| 7 | 13,227,775,827,926,614,294,912 | 1,889,682,261,129,892,671,314 | 7.000000000009 |

These are **exact combinatorial counts of cofactor classes**, not actual
zero-sum relation counts. If prime-subgroup components were uniform within
each cofactor class, accepting a row or descent after `[7]` projection would
give 57.58 instead of 8.23 expected six-term decompositions of a fixed
subgroup target, and 20,078 instead of 2,868 expected seven-term homogeneous
rows. The seven-term values identify a row and its global negative. The
near-sevenfold gain is a constant factor; the uniformity and independent-rank
assumptions are unverified on this 61-bit base.

## Prior work and boundary

[The existing prime-field program, §4.6](../../RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md)
already examined multiplicative-subgroup x-bases `x^B=1`. It found that the
sparse membership polynomial and cyclic-norm folding do not change the
relation-search exponent in the analyzed Semaev setting. This fixture adds
an exact M61 instance and cofactor-projected count; it makes no novelty claim
for the factor-base idea and does not overturn that earlier obstruction.
[Petit et al., PKC 2016, §3.3](https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf)
also explicitly studies smooth multiplicative-subgroup and coset
x-coordinate factor bases over prime fields. Its overall relation-system
cost is left unresolved; our fixed-ratio quartic is a specialization of
[Semaev's third summation polynomial](https://eprint.iacr.org/2004/031.pdf),
not a new summation identity.

The verifier accepts signed rows whose full-group sum lies in `E[7]`, checks
rank `B−1` modulo `r`, verifies projected decompositions of generator and
target, and checks the recovered scalar equation exactly. Its included toy
fixture has 20 columns and 19 independent full-zero rows, plus a
projection-only row and a corrupt-row rejection control. The current nullspace routine is
dense Gaussian elimination, `O(B³)` field work. A 61-bit end-to-end result
still needs a relation generator, target descents, and sparse linear algebra
with complete costs at the proposed `O~(B²)` scale.

A [separate sparse Wiedemann component](sparse_solve.md) now compresses
verified tall rows, certifies rank `B−1` on success, and recovers a
checked null vector in `Õ(B²)` field work per bounded attempt when
`m=Õ(B)` and rows stay sparse. It succeeds on the toy fixture; M61
relations, retry bounds, and both target descents remain open.
A [synthetic selected-dimension probe](sparse_solve.md#selected-dimension-synthetic-cost-probe)
certified a planted 19,568-row sparse system in about seven seconds in
one local release run; it is a solver cost check, not a relation result.

## Reproduce

From the repository root:

```sh
cargo test --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --offline
cargo run --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --bin m61_census --offline
```

The crate has no external dependencies and does not use Python. Its
parameter-specific implementation is isolated from the repository's existing
`src/ecc` and `src/cryptanalysis/prime_fast.rs` production paths.

## Next relation-generation probe

[Pair-sum quotient algebra](pair_sum_quotient.md) describes a compact
`O(B²)`-state determinant for signed triple sums, then identifies the
`Θ(B³)` target-intersection and witness-extraction barrier. The [native
pair-energy probe](pair_energy.md) counts nontrivial signed pair-sum collisions
on three toy fields against deterministic equal-size random-x controls. The
root bases show no clear collision enrichment. This is a bounded negative
test of one possible compression mechanism, not a hardness result.
An [exact M61 pair census](pair_sum_census.md) now checks all `B²=5,987,809`
pair-sum x-classes: every one is distinct and none returns to the base.
The projected pair sums are also all distinct. This proves the selected
`D₂` is squarefree of degree `B²` and rules out cofactor-induced pair
support compression on this instance. The [three-pair gap](three_pair_gap.md)
quantifies the next batch-intersection target and its random-set baseline.

[Target-intersection analysis](target_intersection.md) gives the exact
triple-sum operator and shows that its ordinary Sylvester displacement rank
is small but its explicit degree remains cubic in `B`. A [second native
probe](triple_intersection.md) replays target intersections and a signed
six-term witness on certified toy curves. Its sparse `B=8` cases retain
nearly all triple-sum support; larger toy bases saturate their small groups
and cannot forecast M61 costs.
The native probe also measures exact signed six-term target coverage at
`B=8`, with 128 equal-size random controls, every cyclic root window, and
the full tuple-enumeration cost. The first root window is unusually favorable;
the broader comparison shows no consistent coverage gain.
An [actual seven-term toy-rank scan](toy_relation_rank.md) collects and
replays genuine projected rows by a 3+4 meet in the middle. On the
`p=131071` toy field it reaches rank `B−1` at root-base sizes 12, 14,
and 20; cyclic root windows and random-x controls show comparable
rank onset. Its `Θ(B⁴)` collector does not supply the missing M61 locator.
The [one-large-prime ledger](large_prime_recombination.md) uses the same
native six-sum counts to show why cheap partial rows still need too many
collisions or target matches under a uniform model.
The [ratio-fiber probe](ratio_fibers.md) groups root-base pairs by the
multiplicative ratio of their x-coordinates. Its exact M61 counts show no
dense ratio class; the finite lift-mask diagnostic does not test elliptic
target-sum extraction.
The [ratio-cover analysis](ratio_cover.md) identifies the fixed-ratio
correspondence as a genus-four biquadratic cover with a genus-two quotient.
The quotient gives an explicit quartic pair-sum query for a target
x-coordinate. A [native locator](native/examples/pair_locator.rs) factors
those quartics over M61, recovers two planted signed pairs, and verifies
their sums; the six-term relation search remains unresolved.
For the exceptional opposite-x ratio, [exact point counts of both elliptic
quotients](exceptional_quotient_order.md) differ from the selected
curve's count, so neither is `F_p`-isogenous to the ECDLP curve.
An [order-three automorphism positive control](cm_positive_control.md)
on `y²=x³+40` produces 811 exact short relation rows, then measures
their limited rank and known scalar-action dependence. That comparison
curve has a much smaller largest prime subgroup and does not supply a
hard comparable ECDLP instance.
The [selected Frobenius discriminant](cm_isogeny_obstruction.md) rules
out `j=0` and `j=1728` curves throughout the selected `F_p`-isogeny
class, closing an isogeny transport of that automorphism shortcut.

[Torsion and isogeny analysis](torsion_isogeny.md) checks the selected
curve's rational 7-torsion against the root base. The [native probe](torsion_translate.md)
finds no nonzero torsion translate back in the base and verifies the
normalized degree-7 quotient images. Torsion-orbit closure would add
projection-equivalent columns, so it cannot add independent log equations.
The quotient images occupy 2,447 distinct multiplicative root cosets, so
no global coordinate scaling puts more than one back into `μ_4950`.

[Short scalar-return screening](scalar_returns.md) tests whether small
multiples of root-base points give directly verifiable relations. The
exhaustive `m=2,…,128` M61 scan finds none; its uniform expected count is
far below one, so the result is a selected-instance exclusion only.

[Dickson trace-base orbit relations](trace_dickson.md) use
`x=t+t^−1` and the selected `x³−3x` cubic to obtain exact same-y
three-point chord rows. A 43-divisor sweep found `μ_7137` as a leaner
candidate with 1,703 liftable columns, 568 free rows, and a 2.90 million
entry `B²` budget; all row-supply figures remain heuristic. The
`μ_8190` variant has 2,092 liftable columns
and 698 disjoint, verified rows of exact rank 698. They reduce the
remaining rank target to 1,393 but do not connect orbits or locate a
target decomposition. Exact signed-tuple totals and uniform projected
estimates are recorded separately from measured relation rank. The
exhaustive signed three-point scan finds no cross-orbit rows, even after
cofactor projection. Its 4,374,372 signed pair-sum x-values are also
all distinct, before and after projection, excluding distinct-column
four-point zero rows on this finite base. The trace-base premise and
chord identity have clear prior art.
An [exact trace-base toy rank control](toy_relation_rank.md) finds 44
seven-distinct-orbit rows on a 21-column `p=131071` base; they alone
reach rank 20. Two checked six-column descents then recover and verify
a synthetic toy log `1777`. Exhaustive six-orbit enumeration covers all
3,541 prime-subgroup toy points, with 67–117 decompositions each. The
collector still costs `Θ(B⁴)` and cannot serve as the missing M61
relation locator.
The [trace-ratio quotient probe](trace_dickson.md#fixed-parameter-ratio-quotient)
specializes a fixed ratio of underlying torus parameters to an elliptic
quartic. It checks lift failures and the three-valued pair-sum map on
the selected curve; it has not yielded cross-orbit rows.
An engineered comparison curve with the quotient degenerated to a
conic retains a 56-bit prime subgroup and 1,775 trace-base columns,
but its 888 liftable pairs at the special ratio yield no new
three-point relation. The selected `b=40` curve cannot realize that
degeneration for any rational parameter ratio.
