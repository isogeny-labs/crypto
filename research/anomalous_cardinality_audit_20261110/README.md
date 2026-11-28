# The entire corrected 160-bit curve has exactly p points

Lean proves `Nat.card formulaCurve.Point = p` for the actual corrected curve,
with the same prime, curve and points used by the existing point-order and
concrete Smart arithmetic certificates. The argument uses a certified missing
x-fibre, an elementary injection bound and Lagrange; it assumes no Hasse or
CM point-count theorem. Whole gate: 17 declarations, zero failures and zero
trusted-native exceptions. [pcc:unverified]

```text
p = 730750818665451459112596905638433048232067471723
a = 425706413842211054102700238164133538302169176474
b = 203362936548826936673264444982866339953265530166
E: y² = x³ + ax + b over ZMod p
```

A separate actual F₅ curve, y²=x³+3x, has ten points and a point of order five.
Lean directly disproves the universal inference from an order-characteristic
point to total curve order equal to the characteristic. This control has no
missing x-fibre; the extra counting argument in the large certificate is
essential to the proof, rather than silently identifying subgroup and curve
orders. [pcc:unverified]

## The argument

1. The frozen parent certificate proves p prime and an actual point G of exact
   order p. Lagrange gives p divides the actual point count N. The actual set
   is finite and contains infinity, so N>0. [pcc:unverified]
2. The kernel checks b^(p/2)=−1 and b≠0 in ZMod p. Euler's criterion proves
   b is not a square; therefore x=0 has no affine point. [pcc:unverified]
3. For any short curve over a finite field, two square roots differ by sign.
   Choosing one root per nonempty fibre gives an injective encoding of actual
   affine nonsingular points into (K excluding x₀)×Bool. With infinity this
   gives N<2#K whenever one x-fibre is empty. [pcc:unverified]
4. On the actual curve, 0<N<2p and p divides N force N=p. [pcc:unverified]

Every claim above is formalized. The chosen-root encoding is a mathematical
noncomputable injection, not a claimed enumeration or algorithmic cost.
The generic bound does not assume a curve cardinality. [pcc:unverified]

## Proof map and controls

| Source / declaration | Exact obligation |
|:--|:--|
| `ShortPointCount.square_equation` | Actual nonsingular points satisfy the short equation |
| `ShortPointCount.cardinality_lt_twice_field` | Generic actual point-count bound from one empty x-fibre |
| `ShortPointCount.unrestricted_twice_field_bound_false` | Actual F₃ curve with seven points refutes dropping that bound's premise |
| `ShortPointCount.multiple_below_twice_eq` | Positive multiple below twice p equals p |
| `MissingFiber.constantB_euler` / `missing_zero_fiber` | Closed nonresidue check and actual missing x=0 fibre |
| `CardinalityControls.E5_cardinality` / `E5_order_five_point` | Actual curve with ten points and an actual order-five point |
| `CardinalityControls.E5_no_missing_fiber` | Every x has an affine square root on the control |
| `CardinalityControls.unrestricted_point_order_forces_cardinality_false` | Direct negation of order-characteristic-alone inference |
| `AnomalousCardinality.actual_point_cardinality` | Actual corrected large curve has N=p |
| `AnomalousCardinality.anomalous_curve_cardinality_certificate` | Prime/bit bounds/nonsingularity/N=p/actual order/known answer plus direct negative control |

Namespaces end in `Audit` as written in the source. Main binding:
`AnomalousCardinalityAudit.anomalous_curve_cardinality_certificate`.
Source commit `7783f692` precedes `PCC-20261110-541e6f`; actual full local
verification passes at 10:39:11Z. Signed standing remains unverified.
The 17-declaration count includes generated equations; private chosen-root
proofs are checked transitively. [pcc:unverified]

Lean 4.35.0 / Mathlib 56a00585 are pinned. Only propext, Classical.choice and
Quot.sound; no holes, custom axioms, native_decide exceptions, linter waivers
or added elaboration settings. Legacy import advisories remain: changing only
the new headers to modern modules was incompatible with pinned legacy
parents, so it was reverted and the rejected setup retained. All proof
attempts/gate receipts are archived with byte-equal roundtrip checks.
[pcc:unverified]

## Reproduction and remaining scope

```sh
cd research/anomalous_cardinality_audit_20261110/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/anomalous_cardinality_audit_20261110/lean
```

See `sources.json`, `verification.json` and the
[inspected report](reports/proof-20261110/report.pdf). This closes the previous
whole-curve-cardinality obligation in item 11. The generic Smart p-adic
transfer theorem, complexity/native compiler refinement and matched
SafeCurves benchmark remain open. The native Smart measurement packet stays
`not_covered`; its standing is separate from mathematical correctness.
NIST's screened recommendation is not refuted. [pcc:unverified]

Sources checked: corrected anomalous-curve construction and author erratum,
Smart HPL-97-128, and actual Mathlib nonsingular-point equivalence, finite
cardinality/injection, Lagrange, Euler criterion and square-root declarations.
The parent source plan retains paper hashes/read scopes. KB/Conductor
unavailable; bounded primary/repository/Mathlib fallback. No novelty, native
experiment, sampling, timing, security-cost or paid-compute claim.
[pcc:unverified]
