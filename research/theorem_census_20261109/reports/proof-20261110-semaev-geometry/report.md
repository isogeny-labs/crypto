---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Canonical Semaev: every arity, actual curve points

For every algebraically closed field, a general Weierstrass curve with
nonzero discriminant, and every tuple of $m\ge3$ inputs, Lean proves the
following equivalence for the actual canonical resultant family.
[pcc:unverified]

$$S_m(x_1,\ldots,x_m)=0\iff
\exists P_1,\ldots,P_m:\ x(P_i)=x_i,\quad\sum_iP_i=0.$$

The points and sums are actual Mathlib elliptic-curve objects. The proof
covers every characteristic, repeated inputs and zero intermediate sums.
Explicit ring homomorphisms evaluate the existing canonical polynomials;
no geometric property is assumed in their definition. [pcc:unverified]

![Checked proof dependencies and the finite-field domain control. [pcc:unverified]](proof.pdf){width=94%}

\newpage

# Why the converse needs its hypotheses

Both the polynomial and actual point-family recurrences have the same
two alternatives: a finite intermediate coordinate, or zero old sum and
equal last coordinates. The latter corresponds to opposite last points.
Preserving the declared Sylvester sizes is essential when evaluation
lowers degrees. The leading coefficients are the preceding evaluation
squared and $(z-t)^2$. The exact declared-root criterion, actual S3/S4
bases, and strong induction complete the proof. [pcc:unverified]

On the nonsingular F5 curve $y^2=x^3+1$, the actual canonical polynomial
has $S_3(3,1,4)=0$, yet $y^2=3$ has no F5 solution. Lean directly refutes
the universal rational-only converse. An arity6 point-list control uses
the actual prefix $P,-P,P,-P$ with x=4 followed by unliftable x=3,3.
It refutes the unrestricted-coordinate recurrence without closure;
it does not refute a version supplying rational point lifts.
[pcc:unverified]

The combined binding is
`RecursiveSemaevGeometryAudit.geometric_domain_audit_certificate`.
Proof map: `../../SEMAEV_GEOMETRY.md`. Exact hypotheses, source hashes,
toolchain, whole-project gate and actual packet result are recorded in
`../../semaev-geometry-sources.json` and the separate verification record.
Whole census gate471passes, zero failures/zero trusted-native exceptions.
Local source commit `2d252c2b` precedes packet `PCC-20261110-af2329`;
actual local verdict and signed standing are recorded separately.
Lean4.35.0/Mathlib56a00585 are pinned. Standard axioms only:
propext, Classical.choice and Quot.sound. No hole, custom axiom,
native decision exception or linter waiver. Failed source attempts and
raw diagnostics are retained. Fresh lean4checker is unavailable.
[pcc:unverified]

Reviewed existing GeneralS3/GeneralS4 actual coordinate/gluing proofs,
Mathlib resultant map/root-product/degree-padding lemmas, IsAlgClosed
splitting and Fin tuple/sum identities. KB and Conductor remain unavailable;
bounded repository/Mathlib fallback, no novelty claim. Original Semaev2004
remains unread after403; retained primary read scopes are unchanged.
The previous scalar IDC1h0872017c01480c3d supplement keeps its fixed-D=8
scope. This universal geometric theorem rests on the Lean proof.
[pcc:unverified]

The all-arity geometric bridge is now closed. Earlier all-input formal
degree proofs are unchanged. Full polynomial permutation invariance and
irreducibility remain separate open obligations. No new native experiment,
performance measurement or paid compute. Signed PCC standing remains
unverified pending a trusted signed verdict. [pcc:unverified]
