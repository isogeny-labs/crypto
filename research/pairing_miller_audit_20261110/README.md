# Actual Miller computations and quotient-domain disproofs

For E/F5: y²=x³+1, Lean computes prescribed order-three Miller
expressions over the actual field L=F25=F5[β], β²=2. It verifies both
9×9 tables, every used numerator/denominator, and the full actual
L-rational 3-torsion subgroup. It proves bilinearity and nondegeneracy
on the actual finite quotient domains, and directly disproves
unrestricted base-field domain readings. [pcc:unverified]

These are prescribed Miller expressions with explicit point operations.
The formal identification with canonical divisor-defined Weil/Tate
pairings remains open. The tangent triple-intersection certificates are
included; they do not substitute for a valuation/divisor proof.
[pcc:unverified]

## Concrete data and actual domains

P=(0,1), Q=(1,β), S=(4,0), ω=2+3β. P and Q have order3;
S is nonzero order2. The actual 3-torsion subgroup
A=E(L)[3] is additively isomorphic to (Z/3)² by actual point addition,
with (a,b) mapped to aP+bQ. Its cardinality is9; no assumed torsion
basis or generic rank-two theorem is used. [pcc:unverified]

For nonzero U=(x,y) in A, define the tangent expression
f_U(R)=y_R−y−(3x²/(2y))(x_R−x), with f_O=1. The fixed auxiliary
S is checked to avoid all supports. The concrete values are

- W(U,V)=f_V(U+S)/f_V(S) · f_U(−S)/f_U(V−S).
- T(U,V)=(f_U(V+S)/f_U(S))⁸.

Lean proves W(aP+bQ,cP+dQ)=ω^(ad−bc) and
T(aP+bQ,cP+dQ)=ω^(2(ad−bc)), with exponents in Z/3. Thus
W(P,Q)=ω and T(P,Q)=ω², both nontrivial primitive cube roots.
Orientation follows Galbraith(26.1); inverse conventions exchange
these roots. Every value is checked from actual point/field operations
before the determinant formulas prove the group properties.
[pcc:unverified]

The Tate domains are B=E(L)/3E(L) and C=L*/(L*)³. Lean checks
sixfold annihilation for every actual E(L) point, then proves that the
inclusion A→B is an isomorphism, with each class represented by actual
3-torsion (equivalently4R for a point representative R). It proves
|B|=9. The actual final exponent8 has kernel exactly the cubes and
induces C≃μ3(L); both targets have cardinality3. [pcc:unverified]

The raw Miller ratio defines t:A×B→C using that unique 3-torsion
representative. The reduced map lands in actual field-unit cube roots.
Both maps are bilinear and nondegenerate in both variables, proved
from the exact tables, actual quotient maps and final-exponent
isomorphism. W:A×A→μ3(L) is also bilinear, alternating and
nondegenerate in both variables. [pcc:unverified]

## Skeptical base-field controls

Actual Frobenius sends (a,b) to (a,−b). The F5-rational point image
in A is exactly the cyclic P-line, while Q is not base-rational.
W restricted to that rational line is identically1 despite a nonzero
input. Lean directly negates the resulting nondegeneracy assertion.
[pcc:unverified]

The actual base-field target F5*/(F5*)³ is trivial, since cubing is
surjective. An actual nonzero order3 point of E(F5) therefore prevents
*any* map to that target from being left nondegenerate. The arithmetic
conditions gcd(3,5)=1, 3 not dividing5−1, and3 dividing25−1 are all
discharged. These are extension-degree1/2 controls; the qualified
root-of-unity hypotheses are retained. [pcc:unverified]

| Module | Checked obligation |
|:--|:--|
| MillerCurve | Actual field, primitive root, nonsingular curve, actual P/Q/S orders |
| TorsionIndex | Actual additive index homomorphism, injectivity, exhaustive 3-torsion coverage/cardinality9 |
| MillerExpressions | Prescribed formulas, generator values and auxiliary support separation |
| MillerTable | Both full81-entry tables and all used nonzero evaluations |
| MillerProperties | Finite-expression bilinearity/nondegeneracy and cyclic restriction counterexample |
| BaseDescent | Actual Frobenius/rational image, Q nonrational, base unit quotient trivial and direct universal disproofs |
| TateQuotient | Actual6P=0 for all points, unique 3-torsion class representative and quotient cardinality9 |
| UnitsTarget | Actual final-exponent kernel/surjectivity, unit-quotient/root isomorphism and cardinalities3 |
| TateClassPairing | Raw and reduced maps on actual quotient domains, bilinearity and nondegeneracy |
| WeilTarget | Actual root-valued Weil expression, alternating/nondegenerate, rational restriction disproof |
| TangentCertificate | Actual cubic polynomial substitution is (X−x)³, finite coefficient checks and universal ring proof |
| MillerCertificate | Combined main declaration and exact generator root values |

## Scope and sources

The formal geometric divisor/valuation identification of these Miller
functions remains open, along with general MOV/Frey–Rück theorem and
complexity statements. The positive finite computations do not prove
the general nondegeneracy theorem, and the base-domain controls do not
refute its correctly qualified version. L-rational torsion coverage is
proved; a generic all-curve geometric torsion-rank theorem is not assumed.
[pcc:unverified]

Primary reading: Galbraith26.2(26.1),26.3(26.2), MillerAlgorithm29 and
reduced Tate26.3.2, including exact support, quotient and orientation
conditions. Frey2009 selected3.1.1–3.4/5.7–5.10 concerns different
local/cohomological domains and is not substituted for the finite-field
pairing. Original Frey–Rück1994 remains unread: correct DOI suffix−6
is confirmed by Crossref, but its AMS PDF still returns403. The old
sealed catalog suffix−3 is preserved historically. Exact primary hashes,
URLs, read scopes and metadata receipts are in evidence/.
[pcc:unverified]

Lean4.35.0 / Mathlib56a00585 are pinned. No holes, custom axioms,
native_decide, waived linters or increased elaboration settings.
Only propext, Classical.choice and Quot.sound. Heavy source branches
are serialized by imports to bound local build memory. All failed builds,
source snapshots and full gate receipts are retained byte-for-byte.
Local replay and independent signed standing are separate.
No native experiment, sampling, benchmark or paid compute.
KB unavailable (MCP absent/wrapper127); bounded primary/repository/
Mathlib fallback. [pcc:unverified]

```sh
cd research/pairing_miller_audit_20261110/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/pairing_miller_audit_20261110/lean
```

Exact source plan and actual local verdict/standing: sources.json and
verification.json. See the inspected [report](reports/proof-20261110/report.pdf)
and editable proof.dot. [pcc:unverified]

Source4db7a35b precedes PCC-20261110-5b33a1; full local replay
passes at18:43:39Z. All32 formal/config and source-review/artifact
pins match. Fullgate126/0/0; all51attempt/source files and five gate
files have byte-equal archive roundtrips. The final three-page58960byte
report was inspected on every page. Signed standing remains unverified.
[pcc:unverified]
