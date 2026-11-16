---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Actual point Frobenius descent

For every finite field F, every field extension L/F and every general
Weierstrass model, Lean proves the following equivalence on actual
Mathlib nonsingular curve points. Here $q=|F|$. [pcc:unverified]

$$\operatorname{Frob}_q(P)=P\iff
\exists Q\in W(F):\ \operatorname{baseChange}(Q)=P.$$

The map is Mathlib's actual additive Point.map applied to the finite-field
Frobenius algebra homomorphism. It preserves every r-torsion set. All
actual r-torsion points are rational iff this actual Frobenius map is
identity on that set. No torsion basis or assumed descent criterion is
used. [pcc:unverified]

![Actual descent proof and the BK excluded-case control. [pcc:unverified]](proof.pdf){width=94%}

\newpage

# A checked disproof, with its exact scope

For E/F7:$y^2=x^3+3x+2$, Lean checks nonzero discriminant and
$\#E(F7)=9$. The existing algebraic-closure construction gives an actual
point killed by3 outside the rational-point image. The new theorem
proves that actual7-Frobenius does not fix it. Lean directly negates
identity on the entire actual3-torsion set, although $3\mid7-1$ and
$9\mid\#E$. [pcc:unverified]

This refutes removal of BK's exclusion, including an attempted repair
using $r^2\mid\#E$. The original BK1998 Theorem1 and the corpus node
retain $r\nmid q-1$; the original paper already warns about this boundary.
The qualified theorem is not refuted. [pcc:unverified]

The scalar proof uses the actual splitting and complete roots of
$X^q-X$: any extension root lies in the base-field image. For actual
affine points, descend both coordinates and use baseChange_nonsingular
to construct a base-field point. The point at zero is handled separately.
The rational-torsion criterion applies this theorem point by point;
torsion stability follows from the actual additive map. [pcc:unverified]

Main binding: `FrobeniusPointDescentAudit.frobenius_descent_audit_certificate`.
Proof map: `../../FROBENIUS_DESCENT.md`; source hashes, gate and actual
packet standing are in `../../frobenius-point-descent-sources.json` and
the separate verification record. Whole census gate479passes, zero failures/zero trusted-native exceptions.
Local source commit `12cb97ae` precedes packet `PCC-20261110-f1abe2`;
actual local verdict and signed standing are recorded separately.
Lean4.35.0/Mathlib56a00585 are pinned.
Standard axioms only: propext, Classical.choice and Quot.sound. No hole,
custom axiom, native decision exception or linter waiver. Failed builds
are preserved; fresh lean4checker is unavailable. [pcc:unverified]

Read original BK1998 Theorem1/proof/Remark1, pp142–143, DOI
10.1007/s001459900040, retained PDF SHA256 ac837b8e…c987499. Reviewed
Mathlib finite-field Frobenius/roots, Polynomial.Splits, actual point map
and nonsingularity descent, and the existing excluded-case point proof.
KB/Conductor unavailable; bounded repository/primary/Mathlib fallback.
No novelty claim or broad source-status promotion. [pcc:unverified]

Actual geometric torsion rank two, determinant q, Weil necessity and
q-Frobenius iterate/extension identification remain open for full BK.
The requested all-p<500 census is also unresolved. This component closes
actual point and torsion descent over any given finite base field.
No native experiment, timing or paid compute. Signed PCC standing
remains unverified pending a trusted signed verdict. [pcc:unverified]
