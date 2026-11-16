# Diem's size hypothesis: checked arithmetic and an actual small-field control

Lean proves the arithmetic implication used at the end of Diem2011
Proposition4.11. For natural n,q,N, assume n≥2,2^(7n)≤q and the explicit
geometric estimate N≥q+1−n2^(n+2)(sqrt(q)+1). Then q+1≤2N. The
Hasse–Weil/genus/singular-point estimate is an explicit premise; this
component does not prove that geometric premise. [pcc:unverified]

Lean also checks an actual small-field model and directly refutes an
unrestricted point/chart factor-base bound. Let F=F2[alpha] with
alpha²+alpha+1=0. The irreducible quotient is an actual field of4elements
and degree2 overF2. The actual general Weierstrass curve
y²+y=x³+alpha has discriminant1 and exactlyone F-rational point,O.
For the projective-chart homography phi=alpha+1/x, phi(O)=alpha, which
is outside the F2 projective-chart image. Thus its actual factor base is
empty:0<(2+1)/2. The homography has an explicitly checked inverse.
[pcc:unverified]

The actual source proposition requires n≥2 and log2(q)≥7n, as well as
Condition2.7. The original PDFpage22/printed95 was read and visually
inspected. This small-field control has q=2,n=2 and fails the size
hypothesis. Its rational point/chart statement is fully formal; the
scheme-theoretic degree/ramification and Condition2.7 identification are
not formalized here. It does not refute the qualified Diem proposition.
[pcc:unverified]

## Arithmetic proof

For n≥2, n≤2^(n−1). Therefore the square of n2^(n+4) is at most
2^(4n+6)≤2^(7n)≤q. Put s=sqrt(q) and a=n2^(n+2). Then s≥4a and
s≥1. Hence a(s+1)≤2as≤s²/2=q/2≤(q+1)/2. Substituting this into the
explicit geometric lower estimate yields the required half-size bound.
The integer form q+1≤2N avoids rounding ambiguity. [pcc:unverified]

A separate synthetic arithmetic control takes n=2,q=4,N=0. The geometric
estimate alone is satisfied, while q+1≤2N is false. Lean directly
negates the universal inference without its size premise. N=0 here is
an arithmetic test value, not an asserted curve/factor-base count.
It is separate from the actual F4 model with base q=2. [pcc:unverified]

## Actual field, curve and map

The polynomial X²+X+1 overF2 has no root and degree2, giving a checked
irreducible quotient and field structure. Its power basis has dimension2,
so the actual field cardinality is4. The quotient relation implies
alpha²≠alpha and alpha is outside the base-field image. [pcc:unverified]

For every field element z, z⁴=z. In characteristic2, y²+y and x³ are
fixed by squaring: the former follows from z⁴=z and add-square, the
latter from x=0 or x³=1. If y²+y=x³+alpha held, squaring and subtracting
would force alpha²=alpha. Thus there are no affine F-points. The actual
Mathlib point group consists only of zero, so its cardinality is1;
discriminant1 is independently checked. [pcc:unverified]

The projective chart is represented by Option F, with none=infinity.
The actual base chart is the range of Option.map(algebraMap F2 F),
including infinity. The homography sends infinity to c,0 to infinity,
and nonzero x to c+1/x. Its inverse sends infinity to0,c to infinity,
and other y to1/(y−c); both compositions are proved over every field.
The factor base is the actual point preimage after composing this
homography with SemaevS3.pointX. At c=alpha its sole possible rational
point maps outside the base chart, giving exact cardinality0.
[pcc:unverified]

## Proof map

| Module and declaration | Exact obligation |
|:--|:--|
| DiemThreshold.small_power_bound / squared_error_bound | Actual integer power estimates under n≥2 and2^(7n)≤q |
| DiemThreshold.error_absorbed | Universal real square-root error estimate |
| DiemThreshold.half_count_nat_of_geometric_estimate | Conditional count bound with geometric estimate explicit |
| DiemThreshold.size_free_arithmetic_inference_false | Direct negation using the synthetic n2/q4/N0 control |
| DiemSmallField.extensionPolynomial_irreducible / field_cardinality / field_finrank | Actual irreducible quotient, field size4 and extension degree2 |
| DiemSmallField.alpha_not_base / pow_four | Actual root outsideF2 and finite-field fourth-power law |
| DiemSmallField.curve_nonsingular / curve_cardinality | Actual discriminant1 and point count1 |
| DiemSmallField.mobiusChart_bijective | Explicit homography/inverse compositions over every field |
| DiemSmallField.factorBase_cardinality | Actual point preimage of the base projective chart has cardinality0 |
| DiemSmallField.unrestricted_factor_base_bound_false | Direct universal negation of the unrestricted nonsingular-curve/chart bound |
| DiemSmallField.small_size_excluded | Actual q2/n2 violates2^(7n)≤q |
| DiemSmallField.diem_domain_audit_certificate | Fully quantified arithmetic implication/disproof and actual field/curve/map data |

Names above belong to the corresponding namespace ending in Audit.
The component uses only propext, Classical.choice and Quot.sound. No
hole, custom axiom, native-decision exception, elaboration option or
linter waiver. Failed attempts are retained losslessly. Lean4.35.0 and
Mathlib56a00585 are pinned; fresh lean4checker is unavailable. Exact
statement/source/gate and actual packet standing are recorded separately
in diem-domain-sources.json and diem-domain-verification.json.
[pcc:unverified]

## Sources and limits

Read Diem2011 Condition2.7, Proposition4.9/4.10 and Proposition4.11;
the last proposition and proof were visually checked on printedp95,
[DOI](https://doi.org/10.1112/S0010437X10005075). Retained PDF SHA256:
74902b5816cd550aa57e180c60fb5bc60b64d93c8ce9f2019a53ca30a934a58a.
Reviewed Mathlib AdjoinRoot's actual irreducible quotient/power basis,
finite-field cardinality, characteristic transport, real sqrt and the
existing actual elliptic point coordinates/constructors. KB and Conductor
remain unavailable; bounded primary/repository/Mathlib fallback. No
novelty claim or blanket source-status promotion. [pcc:unverified]

The earlier stable-branch family fails the orbit form of Condition2.7.
The new actual point/chart control omits that admissibility premise and
the size threshold. Neither is a refutation of the qualified source
bound. The actual scheme ramification/cover identification and the
geometric Hasse–Weil/genus estimate remain open. This result checks the
arithmetic reduction and an unrestricted strengthening at their stated
scope. It does not establish an admissible Diem counterexample.
[pcc:unverified]

The statements concern inequalities, a characteristic-two quotient and
actual point-image sets. IDCv1's fixed odd-modulus polynomial format does
not encode them; no new IDC result is claimed. The kernel proofs carry
this component. No native experiment, timing or paid compute. Leaderboard,
benchmark and ecdb graphs have no new measured value; the proof-dependency
diagram accompanies this component. Signed PCC standing remains
unverified pending a trusted signed verdict. [pcc:unverified]

Local source commit `6e8973a3` precedes packet `PCC-20261110-dda065`.
The full producer-local verification passes at06:31:34UTC on762f5fc120c9;
signed standing remains unverified. Whole census gate535passes with zero
failures/zero trusted-native exceptions. See [actual bindings](diem-domain-verification.json),
[inspected two-page report](reports/proof-20261110-diem-domain/report.pdf),
and [editable report](reports/proof-20261110-diem-domain/report.md).
[pcc:unverified]
