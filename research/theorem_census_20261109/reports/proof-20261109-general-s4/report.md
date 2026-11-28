---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Actual S4: the first resultant step

For any nonsingular general Weierstrass curve over an algebraically closed
field of any characteristic, Lean checks the first fixed-degree resultant
step: its vanishing is equivalent to four actual affine points with the
prescribed x-coordinates summing to zero. Repeated coordinates and partial
sums at infinity are included. [pcc:unverified]

Write the two S3 polynomials as $aT^2+bT+c$ and $dT^2+eT+f$.
The definition uses Mathlib's actual degree $2/2$ Sylvester determinant;
Lean computes its matrix and proves the following identity. [pcc:unverified]

$$\operatorname{Res}_{2,2}=(af-cd)^2-(ae-bd)(bf-ce).$$

With first-pair distinct x-coordinates, the checked S3 factorization gives
roots $r=x(P+Q)$ and $s=x(P-Q)$. The resultant becomes
$a^2S_3(x_3,x_4,r)S_3(x_3,x_4,s)$. A common root glues actual points equal
up to negation. The converse constructs the common coordinate from the
nonzero actual pair sum. [pcc:unverified]

![Proof dependencies; all scalar and point operations are Mathlib objects.](proof.pdf){width=100%}

If the first pair repeats but the second does not, swap the pairs. If both
repeat, both leading coefficients vanish and the fixed-size determinant
is zero. Actual points $P,-P,Q,-Q$ provide the four-point sum; no finite
intermediate coordinate is needed. This branch must not be replaced with
an assumed finite-common-root criterion. [pcc:unverified]

Main source: ../../lean/GeneralS4.lean, declaration
`GeneralS4Audit.generalS4_zero_iff_exists_points`. This establishes the
$m=4$ vanishing equivalence at every specialization; general recursion for
all $m$, irreducibility and symbolic individual-variable degrees remain
open. [pcc:unverified]

\newpage

# A geometric control for the degree-drop shortcut

Take the actual smooth curve $y^2=x^3-x$ over $\mathbf F_5$ and base change
to its algebraic closure. Lean checks the following identities for every
$t$ in that closure. [pcc:unverified]

| Quantity | Kernel statement |
|---|---|
| First specialized S3 | $S_3(0,0,t)=1$ |
| Second specialized S3 | $S_3(1,1,t)=4$ |
| Fixed degree $2/2$ resultant | $S_4(0,0,1,1)=0$ |
| Common finite root | Does not exist |
| Four actual affine points | A zero-sum quartet exists |

The nonzero constants have no finite common root. The zero fixed-size
resultant detects their common projective behavior at infinity. Recomputing
a resultant using their new actual degrees would change the operation.
This refutes the finite-root proof shortcut, while the qualified S4
point-sum equivalence is preserved. [pcc:unverified]

The determinant formula, factorization, group-law gluing, repeated-pair
construction and negative control are kernel proofs, not finite-sampling
inferences. The existing bounded S4/S5 census is unchanged. This increment
runs no new curve population, benchmark or paid compute. [pcc:unverified]

Primary source read: [Kosters–Yeo2015](https://arxiv.org/pdf/1503.08001),
Section 2/page 3 recursive definition and Proposition 2.1; PDF page visually
checked. Exact digests/read scope: ../../general-s4-sources.json.
Original Semaev 2004 remains unread: ePrint/CiteSeer returned 403 and the
cited RHUL copy returned 404. No full-paper or all-fourteen completion is
claimed. [pcc:unverified]

Whole census gate:197 declarations,0 failing,0 trusted-native exceptions.
Packets PCC-20261109-21757f and PCC-20261109-ce6722 bind the theorem and
control; full producer checks are retained separately and remain unsigned.
The count includes generated declarations, not 197 new results. [pcc:unverified]

Supplement IDC1hbf1f44625ee306dc checks the determinant coefficient rewrite
with the unchanged repository checker:32 derived plus11 extra points over
$2^{61}-1$,6 variables, degree bound 4. Source commit e6bcfc3a preceded issue;
the proof-file digest matches GeneralS4.lean. One-host finite checks do not
replace the universal Lean proof or establish separate-host standing.
[pcc:unverified]
