# Complete Lean proof of the ordinary guarded pair `(4,5)`

Date: 2026-10-29. Central declaration:
[`GuardedExactPair.exact_pair_prime`](GuardedExactPair.lean).

The full theorem is proved in Lean for the stated six-input prime-field
presentation. It is uniform in the modulus, curve coefficients, three base
abscissas and target, under the explicit hypotheses below. This supersedes
the central gaps in [the earlier partial report](exact-pair-progress.md);
that report and its dated verification receipts are preserved.

## Exact statement

Let `p` be prime with `p>3`, and work in `K=ZMod p`. The supplied scalars are
`α,β,a,b,c,r,Y` with:

- `a,b,c` pairwise distinct;
- `4*α^3+27*β^2 != 0`;
- `Y^2=r^3+α*r+β`;
- for every `x` in `{a,b,c}`, some `Z` in `K` satisfies `Z^2=x^3+α*x+β`;
- no assignment of the five field variables satisfies all six generators.

The generators, in variable order **u,x,y,z,v**, are exactly

```text
g(x), g(y), g(z), S3(x,y,u), S3(u,z,r), v*(x-y)-1,
g(T)=(T-a)(T-b)(T-c),
S3(x,y,u)=(x-y)^2*u^2-2*((x+y)*(x*y+α)+2*β)*u
           +(x*y-α)^2-4*β*(x+y).
```

`GuardedExactPair.generator_range` proves that the indexed generators used
by the first-fall theorem have exactly the input set used by last fall.

The conclusion is

```lean
WeightedFirstFall.IsFirstFallDegree (GuardedTops.generator α β a b c r) 4 ∧
GuardedLastFall.IsLastFallDegree (GuardedModel.inputs α β a b c r) 5
```

Both predicates state **attainment and minimality**, not just inequalities.
The actual first/last degrees, a unit ideal, a degree-five refutation and a
nonzero resultant are conclusions of the proof, not extra hypotheses.

`exact_pair` proves a stronger field statement with `6 != 0`, rational base
lifts and polynomial UNSAT. That statement needs neither nonsingularity nor
a target ordinate. `exact_pair_rational_curve` includes the original curve
and target hypotheses; `exact_pair_prime` also proves `6 != 0` from prime
`p>3`. The weaker requirements of the stronger theorem do not change the
original wrapper's presentation or quantifiers.

## Last fall: a constructed refutation and a separating space

The earlier checked `FilteredClosure.Derivable` is membership in the least
coefficient-linear space containing eligible inputs and closed under every
polynomial multiplication whose **actual product total degree** is within
the bound. `LastFallBound F d` means every `q` in the generated ideal is
derivable at degree `max(d,q.totalDegree)`.

The new upper proof constructs `1 ∈ V(F,5)`. It therefore constructs
`1 ∈ Ideal.span F`, and proves the full last-fall upper bound by multiplying
the derived one by each desired consequence at the appropriate degree.
The existing checked universal obstruction `1 ∉ V(F,4)` then excludes every
smaller last-fall bound. `GuardedLastFall.exact_last_five` composes these
implications without a unit-ideal premise.

The degree-five construction is explicit:

1. **Reduce the first pair.** The quadratic divided difference is already
   derived in degree four. Put `w=a+b+c-x-y`. `GuardedReduction` derives
   `g(w)` and the first quadratic family using exact identities. Its cubic
   correction is `g(w)*(2*u+w-(a+b+c))`, of degree at most four. The second
   input is already a quadratic family in `z,u`.
2. **Normalize quadratic leading coefficients.** `CubicArithmetic.rotate`
   implements one multiplication modulo the split monic cubic, with an
   explicit correction. `inverseCoeff` constructs an interpolation inverse.
   `CubicNormalization.monic_normalization` performs two successive
   multiplications and reductions. Every transient degree is at most five,
   and each reduced quadratic family returns to degree at most four.
3. **Extract a scalar resultant.** For monic quadratics `H1,H2`, form their
   linear difference and the affine-in-`u` cancellation `N`. The coefficient
   of `u` is separable in the two states. `AffineGrid` proves the precise
   missing-high-corner conditions that keep the first state shifts within
   degree four. The second shifts may reach five; they are not asserted to
   return to four. `StagedResultant.resultGrid_derivable` cancels `u` and
   derives a scalar grid polynomial with values `-2*quadRes`.
4. **Prove the scalar values nonzero.** Rational base lifts give explicit
   roots of the first specialized monic quadratic. If `m=(x+y)*(x*y+α)+2*β`
   and `d=(x-y)^2`, these roots are `(m-2*Zx*Zy)/d` and
   `(m+2*Zx*Zy)/d`. The proof uses the point-coordinate squares and an exact
   discriminant identity. It divides by no point ordinate, so repeated roots
   and two-torsion are included. Polynomial UNSAT excludes a zero of the
   second quadratic at either root. The identity `quadRes_split` expresses
   the scalar resultant as the product of those two nonzero values.
5. **Invert on the nine-point grid.** `inverseGrid` constructs the scalar
   interpolation inverse. `GridClosure.grid_refutation` derives its products
   by individual state shifts, reducing after each multiplication. Reduced
   scalar grids have degree at most four and each shift costs at most five.
   This yields the actual constant one.

The target-leading coefficient is invertible because UNSAT forces `r` out
of the base. If `r` were a base abscissa, use `x=r` and a different rational
base abscissa `y=z`; a rational root and S3 symmetry give a solution of all
six original inputs. `RationalFibers.target_outside_base` checks that argument.

This route proves the rationality/unit-ideal bridge directly. It does not
need a general algebraic-closure Nullstellensatz, a quotient-isomorphism
assumption, or identification with Mathlib's general resultant API.

## First fall: the actual graded top map modulo Koszul relations

For the actual six indexed polynomials `f_i`, let `d_i=totalDegree f_i` and
`t_i=homogeneousComponent d_i f_i`. `GuardedTops` proves the degree vector
`(3,3,3,4,4,2)` and every top form.

`TopRelation f d v` requires a homogeneous multiplier `v_i` of degree `d-d_i`,
with zero required when `d_i>d`, and the identity `sum_i v_i*t_i=0`.
`KoszulAt f d v` is the explicit homogeneous degree-`d` span of the classical
pair relations `t_j*e_i-t_i*e_j`, with homogeneous coefficient of degree
`d-d_i-d_j` and zero required when that degree would be negative. Nonmembership
in this span is the usual nonzero-class formulation of the quotient by
Koszul relations; no assumed syzygy uniqueness is used.

Every relation below degree four is proved zero by
`relation_below_four_zero`, using the actual cubic and guard tops.
At degree four the witness is

```text
(v, -v, 0, 0, 0, -(x^2+x*y+y^2)).
```

Every pair of distinct input degrees sums to at least five, so the degree-four
Koszul span is zero. The displayed witness is nonzero and a top relation.
Its lift to the original inhomogeneous generators is nonzero and has total
degree at most three (`witness_lift_ne_zero`, `witness_lift_degree`). Thus
`exact_first_four` proves attainment and minimality, and the exhibited class
is a genuine degree fall. The separately corrected divided-difference
derivation gives a quadratic consequence in degree four as well.

## Verification and reproduction

Lean 4.34.1, compiler commit `5045d0056413266e57c625dcd7c365b10e377c52`.
Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`; the existing manifest pins
all transitive packages. From this directory:

```sh
lake build
lake env lean --run ../../../tools/lean/AuditAxioms.lean -- CubicArithmetic CubicNormalization GridArithmetic GridClosure AffineGrid StagedResultant GuardedReduction RationalFibers GuardedFibers GuardedLastFall GuardedTops WeightedFirstFall GuardedExactPair
lake env leanchecker --fresh --verbose GuardedExactPair
```

The full receipt in `complete-exact-pair-validation/verification.json`
records the actual default build, transitive axiom audit, individual fresh
replays of every new module, exact source hashes and full-library outcome.
The default target includes the final theorem and all its dependencies.
The audit accepts only `propext`, `Classical.choice`, `Quot.sound` and checks
all new theorem declarations, including helper/equation theorems. No custom
axiom, placeholder, compiler-trusting evaluation or kernel bypass is used.

The [dependency diagram](complete-exact-pair.svg) and
[PDF report](complete-exact-pair.pdf) show the completed chain. Historical
partial receipts, the old identity-source hashes and all frozen numerical
evidence are unchanged.

## Scope retained

This is the explicit ordinary-ring three-abscissa inverse-guarded model.
It is not the operational Boolean/Weil-descent first-fall convention, and it
does not formalize the growing-base, characteristic-two or Boolean bounds.
It does not independently certify E12 closure runs, the four large-curve
numerical fixture witnesses, or group-law equivalence with signed point
decompositions. The latter is unnecessary for a theorem whose UNSAT premise
is explicitly about the six polynomial generators.

No scientific sweep, timing comparison, scalar-recovery, asymptotic
complexity or deployed-curve security result is claimed. The field-uniform
theorem includes the modulus sizes used by the large-curve applications
under its original hypotheses.

Primary sources/conventions: Huang--Kosters--Yang--Yeo, arXiv:1505.02532,
Definitions 2.1--2.2; Caminata--Gorla, arXiv:2112.05579v2, degree conventions;
Semaev, arXiv:1504.01175, S3; the study's symbolic/independent audits and
the pinned Mathlib degree, homogeneous-component and ZMod field sources.
No literature-priority claim is made.
