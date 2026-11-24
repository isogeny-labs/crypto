# Direct duplication bound for the fixed xedni curve

For `E/ℚ: y²=x³−47x²+751x−3997`, the duplication forms and an exact
interval certificate give the uniform comparison

`|h(2P)−4h(P)| ≤ 53 log(2)` and `|H(P)−h(P)| ≤ (53/3) log(2) < 14`,

where `h(P)` is the logarithmic projective height of the reduced `x` fraction,
`h(0)=0`, and `H(P)=lim h(2^nP)/4^n`. The second inequality follows from the
checked geometric-tail theorem in the [canonical-limit packet](../xedni_canonical_height_limit_20261122/README.md).
This derives the numerical comparison needed by the existing 130-bit
fixed-curve exclusion without invoking the general Silverman estimate.

## Exact argument

For reduced `x=A/B`, put `M=max(|A|,|B|)` and define the homogeneous
duplication forms

```
F = A⁴ − 1502A²B² + 31976AB³ − 187435B⁴
G = 4A³B − 188A²B² + 3004AB³ − 15988B⁴.
```

The tangent formula gives `x(2P)=F/G` as a projective fraction, including
`G=0` for a doubled identity point. When `B=0`, `P` is the identity and its
doubling defect is zero. The coefficient sums of both forms are below `2¹⁸`,
so `max(|F|,|G|)<2¹⁸M⁴`.

The exact archimedean certificate proves
`max(|F|,|G|)≥M⁴/1024`. For `|A/B|≤128`, its 91 closed rational intervals
cover `[-128,128]`; each lists a signed form whose Bernstein coefficients
are all at least `max(1,|left|,|right|)⁴/1024`. Bernstein basis functions
are nonnegative and sum to one on each interval. Outside this range,
`|F(A,B)|≥(3/4)|A|⁴` follows from the exact tail inequality checked by Lean.

The homogeneous Bézout identity in `Bounds.lean` is

`U(A,B)F(A,B)+V(A,B)G(A,B)=R B⁷`, with `R=8038359040000<2⁴³`.

For coprime `A,B`, every common divisor `d` of `F,G` is coprime to `B`:
modulo `B`, `F≡A⁴`, and any prime dividing both `d` and `B` would divide
`A`. Since `d|R B⁷`, Euclid's lemma gives `d|R`. Thus reducing `[F:G]`
removes at most a factor `R`; combining the upper and lower bounds yields
`−53 log(2)≤h(2P)−4h(P)≤18 log(2)`.

## Verification and proof map

| Result | Evidence | Status |
| --- | --- | --- |
| Forms match fixed-curve doubling | `naiveHeight_double_ne` and `naiveHeight_double_eq` in `PointBridge.lean`; `check_sage.py` replays 30 signed multiples | Lean checked for all nonsingular affine points, including doubling to the identity; finite replay agrees |
| Rational x-coordinate normalized to the integer forms | `primitive_x`, `duplication_fraction_normalized`, `denominator_ne_of_double_ne`, and `naiveHeight_double_normalized` in `RationalBridge.lean` | Lean checked: the denominator form is nonzero for every nonsingular affine point whose double is not the identity |
| Homogeneous Bézout identity, coefficient and tail inequalities | `Bounds.lean`, pinned build and `#print axioms` | Lean checked |
| Quartic Bernstein coefficient lower-bound rule | `bernstein_quartic_lower` in `Bounds.lean` | Lean checked for arbitrary coefficients and `s∈[0,1]` |
| Certificate cell 26 (`191/16≤A/B≤383/32`) | `interval_26_positive_F` in `Bounds.lean` | Lean checked with its exact rational coefficients |
| Finite-range archimedean lower bound on `[-128,128]` | `finite_archimedean_lower` in generated `Cells.lean`; generator agrees with `certificate.json` | All 91 cells and coverage Lean checked |
| Global real and homogeneous archimedean lower bound | `global_univariate_lower` and `global_homogeneous_lower` in `Global.lean` | Lean checked for all real inputs, including both outer tails and `B=0` |
| Primitive integer input and a common divisor of both forms | `integer_homogeneous_lower` and `reduced_forms_lower` in `IntegerBridge.lean` | Lean checked: if `gcd(A,B)=1`, `d>0`, `F=d f`, and `G=d g`, then `max(|A|,|B|)^4 ≤ 1024·8038359040000·max(|f|,|g|)` |
| Canonical reduced output pair | `canonical_output_lower` and `point_canonical_output_lower` in `OutputBridge.lean` | Lean checked: the same lower bound holds with `f=q.num`, `g=q.den`, `q=F(A,B)/G(A,B)`; the point theorem discharges both primitive-input and nonzero-denominator hypotheses |
| Point-level lower logarithmic defect | `point_lower_height_defect` in `HeightBridge.lean`, using Mathlib's `Rat.logHeight₁_eq_log_max` and the canonical output lower bound | Lean checked for every nonsingular affine point whose double is affine: `4·h(x(P))−h(x(2P))≤53 log 2` |
| Homogeneous and canonical output upper bound | `homogeneous_upper`, `canonical_output_upper`, and `point_canonical_output_upper` in `UpperBridge.lean` | Lean checked: the raw forms and reduced output coordinates satisfy the `2¹⁸M⁴` bound for every nonsingular affine point whose double is affine |
| Point-level upper and absolute defect | `point_upper_height_defect` and `point_affine_abs_height_defect` in `HeightBridge.lean` | Lean checked for every nonsingular affine point whose double is affine: `h(x(2P))−4h(x(P))≤18 log 2` and `|h(x(2P))−4h(x(P))|≤53 log 2` |
| Rational root and two-torsion exclusion | `no_integer_cubic_root`, `no_rational_cubic_root`, and `no_affine_two_torsion` in `TorsionBridge.lean` | Lean checked: the cubic has no root modulo 3 after the monic rational-root reduction, so every nonsingular affine rational point doubles to an affine point |
| All-point duplication defect | `all_point_abs_height_defect` in `TorsionBridge.lean` | Lean checked for every rational point on the fixed curve, including the identity: `|h(2P)−4h(P)|≤53 log 2` |
| Explicit canonical-height comparison | `explicit_fixed_curve_canonical_height` and `explicit_fixed_curve_height_gap_lt14` in `CanonicalBridge.lean`, importing the pinned `CanonicalLimit.lean` | Lean checked: there is a quadratic limit function `H` on all rational points with `|H(P)−h(P)|≤(53/3)log 2<14` |
| Fixed-curve height-cap bridge | `no_bounded_target_class_lift` in `HeightExclusionBridge.lean`, importing the pinned `HeightExclusion.lean` | Lean checked: a point `P=nQ` with `n mod 43=22`, the stated `65Q` witness lower bound, and `h(P)≤260 log 2` leads to contradiction; generator, residue class, witness, and cap are explicit hypotheses |
| Common-divisor bound `d|F,G ⇒ d|R` for primitive `A,B` | `common_divisor_dvd_resultant` in `Bounds.lean` | Lean checked, including coprimality bridge |
| Uniform doubling defect and numerical comparison | Above inequalities and the canonical-limit geometric tail; Lean checks `(53/3)log 2<14` | Both the all-point defect and explicit canonical-height comparison Lean checked |

The polynomial, divisibility, global archimedean, all-point defect, and
explicit canonical-height declarations use only standard foundations in their
`#print axioms` output. The build pins and recompiles the separate
canonical-height module before checking the bridge. The numerical comparison
is now formalized.
The defining cubic's rational-root exclusion is formalized using Mathlib's
[integral root theorem](https://github.com/leanprover-community/mathlib4/blob/021c46bdd042f67b538b9f9871d556a93713a9a9/Mathlib/RingTheory/Polynomial/RationalRoot.lean)
and a three-residue certificate.
Mordell–Weil generation, rational-to-finite reduction, and a Lean certificate
for the exact `65Q` coordinate remain separate obligations for a complete
fixed-curve exclusion theorem. The cap in the bridge is a bound on Mathlib's
naive logarithmic height; converting the candidate's `max(|a|,b)` rule to this
bound is another explicit step.

## Literature and library check

Jacobson, Koblitz, Silverman, Stein and Teske, [*Analysis of the Xedni Calculus Attack*](https://pages.cpsc.ucalgary.ca/~jacobs/PDF/xedni.pdf), §4 and Lemma 4.1, analyze a family of rational lifts under an explicit conditional height premise. The bound here applies to the one displayed rational curve and supplies its numerical comparison directly. Mathlib's [affine point doubling lemmas](https://github.com/leanprover-community/mathlib4/blob/021c46bdd042f67b538b9f9871d556a93713a9a9/Mathlib/AlgebraicGeometry/EllipticCurve/Affine/Point.lean), [naive-height definition](https://github.com/leanprover-community/mathlib4/blob/021c46bdd042f67b538b9f9871d556a93713a9a9/Mathlib/NumberTheory/Height/EllipticCurve.lean), and [rational height formula](https://github.com/leanprover-community/mathlib4/blob/021c46bdd042f67b538b9f9871d556a93713a9a9/Mathlib/NumberTheory/Height/NumberField.lean) supply the formal coordinate and height bridge. The existing [canonical-limit packet](../xedni_canonical_height_limit_20261122/README.md) supplies the geometric tail and quadraticity once the explicit duplication defect is available.
The [rank-one generator certificate](../xedni_rank1_generator_certificate_20261121/README.md) and [height-exclusion packet](../xedni_explicit_height_exclusion_20261121/README.md) supply the exact computational inputs and conditional arithmetic theorem reused here. Mathlib's [curve reduction module](https://github.com/leanprover-community/mathlib4/blob/021c46bdd042f67b538b9f9871d556a93713a9a9/Mathlib/AlgebraicGeometry/EllipticCurve/Reduction.lean) defines reduction of a Weierstrass equation; its affine point `map` requires a field embedding, so the rational-to-finite point homomorphism remains to be built.

Replay from this directory:

```sh
python3 generate_certificate.py
python3 audit_certificate.py
python3 generate_lean_cells.py --check
sh build.sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_duplication_height_bound_20261122/check_sage.py
```

The recorded runtime and result files correspond to the exact `check_sage.py`
in this packet. The float scan in `protocol.json` was only a pre-freeze pilot;
the 91-interval certificate is the promoted arithmetic evidence.
