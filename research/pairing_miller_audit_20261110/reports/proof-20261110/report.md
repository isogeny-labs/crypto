---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Actual Miller computations and quotient-domain disproofs

For E/F5: y²=x³+1 and the actual field L=F25=F5[β], β²=2,
Lean verifies both 9×9 order-three Miller tables and every used nonzero
evaluation. It proves bilinearity and nondegeneracy on the actual finite
quotient domains, plus direct base-field domain counterexamples.
[pcc:unverified]

The functions are prescribed Miller expressions with actual curve/field
operations. Their canonical divisor-defined pairing identification remains
a separate formal obligation. Tangent triple-intersection certificates
are included; no divisor/valuation theorem is assumed. [pcc:unverified]

![Actual computations, quotient domains, counterexamples and geometric boundary. [pcc:unverified]](proof.pdf){width=91%}

\newpage

# Concrete formulas and actual proof map

P=(0,1), Q=(1,β), S=(4,0), ω=2+3β. The actual subgroup A=E(L)[3]
has nine points, indexed by aP+bQ with a,b in Z/3. The index map uses
actual Mathlib point addition; injectivity and exhaustive torsion coverage
are proved. P/Q have order3; S is nonzero order2. [pcc:unverified]

For nonzero U=(x,y), use f_U(R)=y_R−y−(3x²/(2y))(x_R−x),
with f_O=1. The checked expressions are
W(U,V)=f_V(U+S)/f_V(S) · f_U(−S)/f_U(V−S) and
T(U,V)=(f_U(V+S)/f_U(S))⁸. All used values are nonzero. Every
ordered pair of indexed torsion points satisfies W=ω^(ad−bc),
T=ω^(2(ad−bc)). In particular W(P,Q)=ω and T(P,Q)=ω².
Orientation follows Galbraith(26.1). [pcc:unverified]

| Module | Exact obligation |
|:--|:--|
| MillerCurve | Actual field, primitive root, curve and P/Q/S orders |
| TorsionIndex | Actual index homomorphism, injectivity, exhaustive coverage/cardinality9 |
| MillerExpressions | Formulas, generator values, auxiliary support separation |
| MillerTable | Both full81-entry tables and nonzero evaluations |
| MillerProperties | Finite-expression bilinearity/nondegeneracy and cyclic restriction failure |
| BaseDescent | Actual Frobenius/base image, Q nonrational, base target trivial and direct negations |
| TateQuotient | Every actual point killed by6; actual quotient representatives/cardinality9 |
| UnitsTarget | Final-exponent kernel/surjectivity, target isomorphism and cardinalities3 |
| TateClassPairing | Raw/reduced maps on actual quotient domains and their group properties |
| WeilTarget | Root-valued alternating Weil expression and base restriction disproof |
| TangentCertificate | Triple-intersection polynomial identity, actual coefficient checks and universal ring proof |
| MillerCertificate | Combined audit theorem and exact generator roots |

Every actual E(L) point is killed by6. The actual inclusion A→E(L)/3E(L)
is bijective; each class has its unique 3-torsion representative, equivalently
4R for a point representative R. The raw Miller ratio therefore defines
an actual unit class in C=L*/(L*)³ on the correct right quotient.
Final exponent8 has kernel exactly the cubes and induces C≃μ3(L).
Both raw and reduced maps are bilinear and nondegenerate in both variables.
The Weil expression is also alternating and bilaterally nondegenerate.
[pcc:unverified]

\newpage

# Skeptical controls, provenance and exact assurance

Actual Frobenius sends (a,b) to (a,−b). The F5-rational image in A is
exactly the cyclic P-line, while Q lies outside it. W restricted to that
line is identically1 despite a nonzero input; Lean directly negates the
resulting nondegeneracy assertion. The actual base target F5*/(F5*)³ is
trivial because cubing is surjective. An actual nonzero order3 base point
therefore precludes every left-nondegenerate map to that target.
The checks are at extension degrees1/2; gcd(3,5)=1, 3 not dividing4,
and3 dividing24 are proved. Correct root-of-unity hypotheses remain intact.
[pcc:unverified]

The tangent polynomial substitution is exactly (X−x)³ at every nonzero
actual 3-torsion point, with nonzero y. Finite field coefficient identities
and a universal polynomial ring proof establish it. A projective local-chart
factor and a nonzero numerator at infinity are also checked. Turning these
identities into a complete principal-divisor/valuation identification remains
open. The packet does not import a source nondegeneracy theorem as an axiom.
[pcc:unverified]

Full project gate:126declarations, zero failures and zero trusted-native
exceptions. All51attempt/source files and five whole-gate files have
byte-equal archive roundtrip checks. Thirty-two formal/config dependency
files are pinned in sources.json. Lean4.35.0 and Mathlib56a00585;
only propext, Classical.choice and Quot.sound. No holes, custom axioms,
native_decide, waived linters or increased elaboration limits. New source
imports serialize heavy branches for local memory; issued parents are
unchanged. Actual local replay and independent signed standing are recorded
separately in verification.json. Signed standing remains unverified.
[pcc:unverified]

Primary reading: Galbraith26.2 alternative Weil definition(26.1),
26.3 Tate(26.2), MillerAlgorithm29 and reduced Tate26.3.2, including
support, quotient, orientation and field hypotheses. Frey2009 selected
3.1.1–3.4/5.7–5.10 has different local/cohomological domains and is not
substituted for the finite-field construction. Original Frey–Rück1994
remains unread: correct DOI suffix−6 is confirmed by Crossref, but the
AMS PDF still returns403; old sealed suffix−3 remains historical.
Exact hashes, URLs, read scopes and metadata receipts are retained.
[pcc:unverified]

The finite checks do not prove a generic all-curve nondegeneracy, geometric
torsion-rank, MOV/Frey–Rück reduction or complexity theorem. Base-domain
controls do not refute correctly qualified source results. No native experiment,
sampling, timing, benchmark, independent-host or paid-compute claim.
KB MCP absent/wrapper127; bounded primary/repository/Mathlib fallback.
Proof map, exact statement and dependencies: README.md and sources.json.
Actual packet/verdict/standing: verification.json. [pcc:unverified]
