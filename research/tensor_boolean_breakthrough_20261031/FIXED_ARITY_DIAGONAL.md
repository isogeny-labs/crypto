# Fixed-arity diagonal route to subexponential prime-field ECDLP

Date: 2026-11-18

Status: the scheduler implication is locally kernel-checked; its specialization
to ECDLP depends on the paper-level fixed-arity coverage and recovery theorem
described below. [pcc:unverified]

## Result isolated by the audit

Let `A_m` be a Las Vegas ECDLP algorithm for each fixed integer `m >= 2`, and
let

```text
mu_m(p) = worst-input expected running time of A_m at field size p.
```

Assume that for one fixed numerator `c`, every fixed `m`, and every
`delta > 0`,

```text
mu_m(p) <= C_(m,delta) p^(c/m + delta).
```

Kaarelson's unrestricted dense Theorem 5 supplies `c = 3`; the sparse planted
Theorem 1 improves this to `c = 2` but is unnecessary for the qualitative
diagonal. [pcc:unverified]

Assume also a universal interpreter whose cost for simulating all descriptions
of length at most `h` for `2^h` steps, including parsing and verifying a scalar
output by checking `[d]P = Q`, is at most

```text
poly(log p) 2^(C h)
```

for one absolute constant `C`. Then one Las Vegas algorithm handles all of
these ECDLP instances in expected time `p^o(1)`. [pcc:unverified]

This implication does not require a computable map from `m` to a preferred
program. The universal interpreter enumerates every finite program
description. For any fixed `m`, the description of `A_m` eventually occurs.
Only outputs that parse as bounded-length scalars and satisfy `[d]P = Q` are
accepted.

## Capped-repetition calculation

At stage `h`, run `R` fresh independent copies of every admitted description
for `2^h` simulated steps. Fix a candidate `A_m` with description length `d_m`
and mean runtime `mu_m`. Let `h0` be the least integer with

```text
h0 >= d_m,                 2^h0 >= 2 mu_m.
```

Markov's inequality gives, for each fresh copy,

```text
Pr[T_m > 2^h] <= mu_m / 2^h <= 1/2       for h >= h0.
```

Thus all `R` copies fail at a post-threshold stage with probability at most
`q = 2^(-R)`. Fresh random tapes between stages give a reach probability at
most `q^j` for stage `h0+j`. The expected post-threshold work is bounded by

```text
poly(log p) 2^(C h0) sum_(j>=0) (q 2^C)^j
  = poly(log p) 2^(C h0) / (1 - 2^(C-R)).
```

Choose the fixed repetition count `R > C`. Since `d_m` is constant in `p` and
`2^h0 < 4 max(1,mu_m,2^d_m)`, the scheduler has, for every fixed `m`,

```text
E[T_universal] <= K_m poly(log p) max(1,mu_m)^C.
```

Substituting the fixed-arity bound gives

```text
E[T_universal] <= K_(m,delta) p^(cC/m + C delta + o(1)).
```

For a requested `epsilon > 0`, first choose a fixed
`m > 2cC/epsilon`, then a fixed `delta < epsilon/(2C)`. The same universal algorithm therefore has
expected time at most `p^epsilon` for all sufficiently large `p`. Since this
holds for every positive `epsilon`, its expected time is `p^o(1)`. When the
prime subgroup order has `log r = Omega(log p)`, this is also `r^o(1)`.

The geometric series in this calculation is theorem
`TensorBoundaryBounds.geometricSchedulerTailBound`. The asymptotic transfer
from polynomial simulation of a vanishing-rate candidate family is theorem
`TensorBoundaryBounds.polynomialUniversalSchedulerIsSubexponential`. The
new theorem `fixedNumeratorCandidateFamilyHasVanishingRate` checks that every
fixed numerator `c` is absorbed after a fixed polynomial simulation degree;
its local claim is `PCC-20261119-76300b`. Consequently numerator `3` from the
dense algorithm is enough. The
project proof gate now reports 44 theorems and zero trusted declarations; a fresh
`leanchecker` replay exits successfully. These theorems formalize the generic
implication, not the ECDLP premises. Claims `PCC-20261118-546652` and
`PCC-20261118-ed2042` pass their local Lean verifiers and retain unverified
signed standing. [pcc:unverified]

The dense numerator is checked in exact natural arithmetic too. [pcc:unverified]
`denseCubicRowBoundAtFixedArity` proves

```text
X^m <= p, rows <= 3X  ->  rows^(3m) <= 3^(3m) p^3.
```

Thus cubic dense elimination on at most `3X` rows has fixed numerator `3`
after cross-multiplying by the arity, without real roots or asymptotic exponent
manipulation. Claim `PCC-20261119-77d40e` passes its local Lean verifier and
retains unverified signed standing. The theorem checks exponent arithmetic,
not an executable elimination implementation or its bit costs.
[pcc:unverified]

## Bird planted-dense fallback

There is now a checked algebraic alternative to relying immediately on a
general Howell implementation. Mathlib's flat-array Bird implementation is
wrapped as `birdMatrixDet`, and `det_eq_birdMatrixDet` proves value equality
with the ordinary determinant. Applying it to every row-updated cofactor gives
`birdAdjugate`; `birdAdjugate_eq_adjugate` proves that this is exactly the
ordinary adjugate. Finally, `birdZModInverse` scales that computed adjugate by
the concrete, executable `ZMod` inverse of the computed determinant, and
`birdZModInverse_eq_inv` proves equality with Mathlib's nonsingular matrix
inverse whenever `IsUnit A.det`. Claims `PCC-20261119-7f673b`,
`PCC-20261119-de3426`, and `PCC-20261119-598c61` pass their local Lean
verifiers and retain unverified signed standing. [pcc:unverified]

The conservative exponent arithmetic is checked separately: [pcc:unverified]
`birdSixthPowerRowBoundAtFixedArity` proves

```text
X^m <= p, rows <= 3X  ->  rows^(6m) <= 3^(6m) p^6.
```

This is the fixed-numerator form that would result from `rows^2` cofactor
determinants at `rows^4` operations each. [pcc:unverified]
Claim `PCC-20261119-cfa7f9` passes
its local Lean verifier. The operation count is now checked independently of
Mathlib's source comment. `eagerBirdStep` materializes every complete recurrence
state as an array-backed length-`n^2` vector, and `eagerBirdDet_eq_det` proves
that its result is the determinant. `RingCounted` instruments scalar addition,
multiplication, and negation. Lean proves an exact `3(n-i-1)+3` count for entry
`(i,j)`, then derives

```text
eagerBirdDetRingOps n      <= 4 (n+1)^4,
eagerBirdAdjugateRingOps n <= 4 (n+1)^6.
```

Claims `PCC-20261119-2b2b04`, `PCC-20261119-8d6681`,
`PCC-20261119-fa515d`, and `PCC-20261119-84b10b` pass their local Lean
verifiers. The whole inverse now has the checked conditional bit-work bound

```text
(4(n+1)^6 + 4(n+1)^4 + n^2) C (bits(q)+1)^d
  + D (bits(q)+1)^e,
```

provided a modular ring operation costs at most `C(bits(q)+1)^d` and the one
extended-GCD inverse costs at most `D(bits(q)+1)^e`. The hypotheses are
intentional: the pinned Mathlib implements `ZMod.inv` through `Nat.gcdA` but
contains no proved machine-time theorem for that implementation. Array
allocation and integer-indexing costs remain open as well. Claim
`PCC-20261119-e1a1a1` passes its local Lean verifier. The algebraic
block binding itself is checked: `executableBirdPlantedKernel` substitutes the
Bird inverse into `(-R B⁻¹ | I)`, and
`executableBirdPlantedKernel_spec` proves both annihilation of `[B;R]` and the
identity right-block law under `IsUnit B.det`. That is the exact interface used
by upstream `planted_kernel`, `kerMat_mul`, and `kerMat_rightInv`. Claim
`PCC-20261119-d0d20c` passes its local Lean verifier. The exact source-shaped
adapter is checked locally too: `executableBirdKerMat` accepts upstream
`Ttop`, `Rint`, `nu`, and `q`, and `executableBirdKerMat_spec` proves the same
two laws after integer reduction into `ZMod q`. Claim `PCC-20261119-0d4820`
passes its local verifier. Only integration and rebuilding in the external
pinned source tree remain; the fallback no longer depends on the unformalized
sparse Wiedemann--Dixon cost.
[pcc:unverified]

The Mathlib pin is commit
`cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`. The exact source hashes are
`fa690f7f31bdb3108c61c7c4aca62f56182732728a8928e793c2a5b743a4ba54`
for `Bird/Defs.lean` and
`e36d1901d2279de39d4cb702625ac1c6defaae30eaaba372260c8a1551369fc1`
for `Bird/Correctness.lean`. The local project gate reports 84 theorems and
zero trusted declarations at the adapter checkpoint; after the conditional
bit-work theorem it reports 87 theorems and zero trusted declarations. The
independent fresh checker exits 0.
[pcc:unverified]

## What the pinned primary source supplies

At commit `d1deb5abe008e981206ed58cbd83012b6afc3740`, Kaarelson's Theorem 5
states the sufficient fixed-`m` expected bound `p^(3/m+o(1))` for every
nonsingular prime-field curve, arbitrary group order and structure, and every
`P,Q` with `Q` in the subgroup generated by `P`. It uses deterministic dense
elimination over the prime-power relation modules. Theorem 1's planted sparse
solver improves the numerator to `2`, but that optimization is not required
for `p^o(1)`. [pcc:unverified] Theorem 2 supplies the decomposition oracle. The exact Lean
decomposition capstone is

```text
ECDLP.L104R.subexponentialRate_all :
  forall m, ECDLP.SubexponentialRateStmt m
```

and `SubexponentialRateStmt m` contains an actual finite Mathlib TM2 computing
the lexicographically least generic decomposition within `C_k 2^(L/k)` for
every fixed `k`. Tracing `L61_Machine.tm_of_source` confirms that the machine
and its per-input step proof are constructed, rather than postulated as a
numeric work function.

The decomposition capstone stops at decomposition search, but the same pinned
v2 tree contains a separate checked index-calculus reduction. Its modules
`L114_Trans`, `L114_Plant`, and `L114_Main` supply:

- Sections 6.2--6.5: checked full-group relation-lattice and exact-recovery
  algebra, parameterized by its stated round hypotheses.
- Section 6.6: checked adaptive-transcript independence, integer
  characteristic-polynomial and union bounds, prime-power block invertibility,
  planted kernel construction, a finite round-failure budget, and successful-
  round correctness.
- A final scalar multiplication check, which makes any returned output Las
  Vegas-correct even when a round's generation or linear-algebra events fail.

These are not conclusions of `subexponentialRate_all`, but they are genuine
Lean theorems in the separately declared `v2/lean` project. A local build at
the pinned commit completed for `ECDLPv2.L114_Plant` (9,050 jobs) and
`ECDLPv2.L114_Main` (9,057 jobs). Explicit axiom output for
`det_sub_scalar`, `bad_nu_le`, `bad_nu_union`, `block_isUnit`,
`planted_kernel`, `double_count`, `trans_count_indep`, `round_failure_le`, and
`round_output` is exactly `[propext, Classical.choice, Quot.sound]`.
[pcc:unverified]

The concrete follow-up modules close several of those round hypotheses.
`L116_Alg` builds the actual first-accepted-row collector and proves its
`RoundSpec.row_rel`; `L116_Chernoff` proves the finite adaptive-oracle cap bound;
and `L116_Main` proves tag-evaluation surjectivity from a generating pair,
discharges the prime-power `GenBound`, and gives an explicit lower bound for two
uniform generators of a rank-two finite abelian group. A pinned build of
`ECDLPv2.L116_Alg`, `ECDLPv2.L116_Chernoff`, and `ECDLPv2.L116_Main` completed
all 9,067 jobs. Explicit axiom output for `algSpec_row_rel`, `cap_bound`,
`gen_two_prod`, `gen_two_explicit`, `evG_surjective_of_closure`,
`hsurj_of_pair`, and `genBound_primePow` is exactly `[propext,
Classical.choice, Quot.sound]`. [pcc:unverified]

`L118_Tot` also checks the uniform-generator restart factor. Its
`gen_two_trials` theorem bounds the reciprocal two-generator probability by
`(16/5) * (1 + log₂ |G|)`, using a proved elementary totient bound. The pinned
target built successfully with 9,065 jobs, and axiom output for
`prod_ratio_le`, `card_primeFactors_le_log`, `totient_ge`, `gen_two_log`, and
`gen_two_trials` is exactly `[propext, Classical.choice, Quot.sound]`. This is
an expectation bound for two uniform elements. The paper's concrete affine
point sampler is only near-uniform, so its constant-factor transfer remains a
separate interface. [pcc:unverified]

The generic transfer is now locally kernel-checked. `weightedPairEventLower`
proves that a common sampler-fibre floor `lower` gives weighted pair-event
count at least `lower^2` times the uniform pair-event count. [pcc:unverified]
`nearUniformPairEventTransfer` combines this with
`total <= distortion * |G| * lower` to give the exact cross-multiplied loss
bound

```text
total^2 * goodPairs <= distortion^2 * |G|^2 * weightedGoodPairs.
```

Claims `PCC-20261119-da3506` and `PCC-20261119-a2fecb` pass their local Lean
verifiers and retain unverified signed standing. The project proof gate reports
36 theorems and zero trusted declarations, and a fresh checker replay exits 0.
The remaining sampler obligation is concrete: the paper's affine sampler omits
the identity, so it must either expose an identity fibre or add an explicit
identity branch before the checked transfer applies. [pcc:unverified]

## Corrections and remaining obligations

The paper's Lemma 5.5 overstates its constant-torsion conclusion: a torsion
point over a transcendence-degree-one extension need only be defined over an
algebraic extension of the constant field, not necessarily over `F_p` itself.
The proof that follows needs only that its coordinates are algebraic over
`F_p`, which still contradicts the coordinate `s t_i` being transcendental.
The coverage argument should be published with that corrected statement.

The finite arithmetic of the first-moment repair is now kernel-checked.
`firstMomentForcesLargeValueSeeds` turns the scaled inequalities

```text
value(u) <= 2X,
3X #seeds <= 4 sum_u value(u)
```

into `#seeds <= 6 #{u : X <= 2 value(u)}`. The theorem
`oneSixthEventSurvivesOneTwelfthBadSet` then removes an energy-bad event of
size at most `1/12` and leaves at least `1/12` of all seeds. Claims
`PCC-20261118-07b50f` and `PCC-20261118-aee405` pass their local Lean
verifiers and retain unverified signed standing. The character-sum estimate
and the total-energy estimate that instantiate these generic hypotheses remain
paper-to-curve obligations. [pcc:unverified]

The energy accounting is now composed rather than merely listed.
`twelveMarkovBadSetBound` derives the factor-12 bad-set cardinality directly
from a supplied total-energy bound, and
`firstMomentAndEnergyGiveOneTwelfthGoodSeeds` combines both supplied totals
into the exact joint `1/12` event. Claims `PCC-20261119-1fd609` and
`PCC-20261119-5bbc92` pass locally and retain unverified signed standing.
The remaining curve-specific inputs are precisely the two total-sum estimates,
not an additional probabilistic inference. [pcc:unverified]

The generic planted-diagonal root count is now kernel-checked as well.
`charpolySingularShiftCardLeDimension` proves that an `f`-dimensional square
matrix has at most `f` candidate scalars `nu` for which `nu I - M` is
singular. `finiteBadShiftUnionCardBound` and `sevenEighthsShiftsSurvive` then
prove that at most `wf` shifts are bad for `w` supplied residual blocks and
that a candidate pool of size at least `8wf` leaves at least `7/8` of its
shifts good. The underlying polynomial lemma is
`polynomialBadValuesCardLeDegree`. Claims `PCC-20261119-d2e9ce`,
`PCC-20261119-b68c62`, `PCC-20261119-698c76`, and
`PCC-20261119-5e863b` pass locally and retain unverified signed standing.
The residue injection is also checked:
`zmodNatCastInjectiveOnFin` proves that the first `nu0` natural shifts are
distinct modulo `ell` from `nu0 <= ell`, and
`zmodCharpolySingularShiftCardLeDimension` composes this with the determinant
root count over a prime residue field. Claims `PCC-20261119-5adae9`,
`PCC-20261119-7817f3`, and `PCC-20261119-b9e430` pass locally and retain
unverified signed standing. The stronger pinned-source theorems close the
source-specific bridge too: `trans_count_indep` proves transcript-law
independence for an arbitrary history-dependent shift and stateful oracle;
`round_failure_le` composes that fact with the integer determinant and union
bound; and `round_output` proves correctness of a successful round. The local
generic theorems remain useful as small independently checked replicas, but
they are not the only Lean evidence for the planting step. [pcc:unverified]

Before the ECDLP specialization is promoted from unverified paper deduction,
the remaining independent obligations are:

1. formalize or independently reconstruct the curve-specific character-sum
   and energy estimates that instantiate the checked finite coverage lemmas,
   together with all constants used by the capped relation collector
   [pcc:unverified];
2. bind the factor-base labels returned by the Lean oracle to the rows used by
   the arbitrary-group relation lattice;
3. implement and cost either the Howell/dense-elimination procedure using the
   checked cubic row-to-`p^3` conversion or the eager Bird planted-block
   fallback using the checked sixth-power conversion; bind the checked finite round to the concrete
   coverage oracle, instantiate the checked
   near-uniform transfer with an identity-inclusive curve sampler, assemble the
   remaining restart events, and derive Theorem 5's bit bound
   [pcc:unverified];
4. implement or formalize the universal interpreter, independent random tapes,
   scalar-output length cap and polynomial simulation exponent `C`.

Sparse Wiedemann--Dixon remains an obligation only for preserving Theorem 1's
sharper numerator `2`; it is no longer on the qualitative subexponential path.

The scheduler removes the need for a bound uniform in growing `m`; it does not
remove any of these fixed-`m` correctness and cost obligations.

## Sources and exact pins

- Kaarelson, *A fast algorithm for elliptic-curve discrete logarithms*, v2,
  commit `d1deb5abe008e981206ed58cbd83012b6afc3740`, Theorems 1--3 and Sections
  5--8:
  <https://github.com/kaarelkaarelson/fast-ecdlp/blob/d1deb5abe008e981206ed58cbd83012b6afc3740/v2/paper.pdf>
- Lean capstone at the same commit:
  <https://github.com/kaarelkaarelson/fast-ecdlp/blob/d1deb5abe008e981206ed58cbd83012b6afc3740/lean/ECDLP/L104_Final.lean>
- Pinned planted-diagonal sources:
  <https://github.com/kaarelkaarelson/fast-ecdlp/blob/d1deb5abe008e981206ed58cbd83012b6afc3740/v2/lean/ECDLPv2/L114_Plant.lean>,
  <https://github.com/kaarelkaarelson/fast-ecdlp/blob/d1deb5abe008e981206ed58cbd83012b6afc3740/v2/lean/ECDLPv2/L114_Trans.lean>, and
  <https://github.com/kaarelkaarelson/fast-ecdlp/blob/d1deb5abe008e981206ed58cbd83012b6afc3740/v2/lean/ECDLPv2/L114_Main.lean>.
- Pinned concrete row, cap and generation sources:
  <https://github.com/kaarelkaarelson/fast-ecdlp/blob/d1deb5abe008e981206ed58cbd83012b6afc3740/v2/lean/ECDLPv2/L116_Alg.lean>,
  <https://github.com/kaarelkaarelson/fast-ecdlp/blob/d1deb5abe008e981206ed58cbd83012b6afc3740/v2/lean/ECDLPv2/L116_Chernoff.lean>, and
  <https://github.com/kaarelkaarelson/fast-ecdlp/blob/d1deb5abe008e981206ed58cbd83012b6afc3740/v2/lean/ECDLPv2/L116_Main.lean>.
- Pinned uniform-generator expectation source:
  <https://github.com/kaarelkaarelson/fast-ecdlp/blob/d1deb5abe008e981206ed58cbd83012b6afc3740/v2/lean/ECDLPv2/L118_Tot.lean>.
- Local source hashes: `v2/paper.pdf`
  `eacf5acb66cedbd7eb656580322c9770d72f47ad6d95412e3d4e5c87309f696d`;
  `lean/ECDLP/L104_Final.lean`
  `fbe402947e16b04d13d96f90ebd3ba0e75503b66c264e35635f3db8d68665738`;
  `v2/lean/ECDLPv2/L114_Plant.lean`
  `9e98de7e7347505427643dd57dc2be513a58cb1dd11c0a24fbceec3b6c74ac3f`;
  `v2/lean/ECDLPv2/L114_Trans.lean`
  `378057e83147933551ff574be80e485220f867201a1db357f682fccc4b169109`;
  `v2/lean/ECDLPv2/L114_Main.lean`
  `f247462fc4a23d8e2f4134d768b3e7072e2aba2a263c249c2c9180c173a34b39`;
  `v2/lean/ECDLPv2/L116_Alg.lean`
  `6fd6e5267b98ff3c730888835c3020528713b75e3db283e74b063a2f422d6667`;
  `v2/lean/ECDLPv2/L116_Chernoff.lean`
  `d95c55f402b699f3e2a093d24b9d20190117dc458c6dc5567a457c5dc536ec70`;
  `v2/lean/ECDLPv2/L116_Main.lean`
  `e4d970303f43f75701347e50eef9f042dd9a337df7c95d1c2892fb8709f398bf`;
  `v2/lean/ECDLPv2/L118_Tot.lean`
  `f7a1c4817f76a5f7a463a0f6f3e84798c05f6132cef53b904d41f0112cead4de`.

The repository research-KB wrapper was unavailable (`research-kb: no
executable found`). The review therefore used this exact primary-source pin,
the repository's earlier proof audit, and Mathlib's pinned machine definition.
