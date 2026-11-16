---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# An exact Mumford count differs from one half

For $f=X^5-X$ over $\mathbf F_3$, Lean counts the complete space of
polynomial pairs with $u$ monic, $\deg u\le2$, $\deg v<\deg u$ and
$u\mid v^2-f$. There are eight such reduced pairs; seven have completely
split $u$. Identity $(1,0)$ is included. [pcc:unverified]

$$\text{split fraction}=\frac78,\qquad \frac78-\frac12=\frac38.$$

This disproves an exact finite-field reading of $1/g!$ at genus two.
Gaudry's fixed-genus, $q\to\infty$ limit is preserved. The counted object
is the complete reduced-Mumford polynomial-pair model; formal geometric
identification with $\operatorname{Pic}^0(C)$ remains open. [pcc:unverified]

The curve polynomial is proved monic, degree five and separable/squarefree.
An explicit bijection proves that every admissible polynomial pair has a
canonical coefficient shape and that distinct shapes give distinct pairs.
The enumeration does not assume a selected list is complete. [pcc:unverified]

![Complete pair model, exact splitting count and separate open obligations.](proof.pdf){width=100%}

\newpage

# Proof mechanics, scope and checks

![Exact proof counts; all eight representations, without statistical sampling.](counts.pdf){width=100%}

Of 91 canonical coefficient candidates, eight satisfy the actual reduced
pair conditions. Seven have split u; the sole nonsplit case is u=X²+1,v=0.
[pcc:unverified]

Mathlib's polynomial division is noncomputable. Lean first proves a scalar
coefficient test equivalent to actual monic division, using an exact
quotient/remainder identity and a degree bound. Kernel `decide` enumerates
the scalar test, and proved equivalences transfer its count to the full
polynomial-pair space. Splitting is the actual `Polynomial.Splits` predicate:
$X^2+1$ has no root over F3, and the other seven cases split. [pcc:unverified]

Source: ../../lean/MumfordSmoothness.lean. The combined binding is
`MumfordSmoothnessAudit.mumford_count_audit_certificate`. The whole tracked
project gate passes 80 declarations, 0 failing and 0 trusted-native
exceptions; counts include generated declarations. Only propext,
Classical.choice and Quot.sound are accepted. [pcc:unverified]

Source commit 6fee3178 precedes issue, including the standalone issuer's
dependency lock. Packet PCC-20261109-07eb0d pins the combined theorem;
its full local check passes20:00:58Z. The receipt is
../../evidence/pcc-mumford-count-check.json.
Standing remains unsigned until a separate signed verdict. [pcc:unverified]

Supplement IDC1h7d12c937b3b1482b checks only the scalar quotient/remainder
rewrite: 5 variables, total degree bound 5, 32 derived and 11 extra points
over $2^{61}-1$. The unchanged Rust checker accepts, and the source pointer
matches. These finite one-host samples establish neither F3 cardinalities
nor splitting. The pair counts and splitting rest on Lean; geometry and
the subgroup law remain open. [pcc:unverified]

Primary read scopes: Gaudry2000 Definitions 2--4, Section 2.3 and
Proposition 4; MMT2002 Sections 4.2--4.3, printed pp133--134/PDF7--8.
The MMT PDF page8 was visually inspected: eq12 begins at degree1 and omits
identity; equality of subgroup and full-group proportions is explicitly a
heuristic before eq13. This odd-characteristic F3 count does not test
MMT's binary large-prime regime. URLs and exact hashes are in
../../sources.json. [pcc:unverified]

Cantor arithmetic, cyclic-subgroup proportions, geometric Picard
identification, a scaling family and a native multi-field census remain open.
No new native population, statistical study, benchmark, paid compute or
separate-host run occurred; the original fourteen-item request remains partial.
[pcc:unverified]

All nine attempt logs remain: four passing stages/five failed builds, then
the passing gate. No performance or curve-comparison metric changed.
[pcc:unverified]
