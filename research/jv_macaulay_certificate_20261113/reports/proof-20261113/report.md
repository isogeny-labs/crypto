---
title: "Lean cutoff for the printed Joux–Vitse empty example"
fontsize: 10pt
geometry: margin=17mm
mainfont: DejaVu Serif
---

Lean proves a proper homogeneous ideal I in F101[x,y,h] contains every
monomial of total degree at least four. Its dehomogenized ideal at h=1
is the unit ideal: there is no affine solution in any characteristic101
field. The three literal quadratics are from the first F101³ worked
example on page7 of Joux–Vitse's author manuscript. Here x,y are the
elementary symmetric coordinates, h is the homogenizing variable.
No generic regularity premise is assumed. Whole gate: 39 declarations,
zero failures and zero trusted-native exceptions. [pcc:unverified]

| Equation | x² | xy | y² | xh | yh | h² |
|:--|--:|--:|--:|--:|--:|--:|
| f0 | 100 | 32 | 1 | 55 | 17 | 81 |
| f1 | 29 | 34 | 0 | 71 | 83 | 16 |
| f2 | 59 | 27 | 0 | 31 | 48 | 32 |

Universal characteristic101 identities give normalized quadratics
$g_y,g_{xy},g_x$ and exact ideal-membership certificates for

$$c_y=yh^2+82h^3,\qquad c_x=xh^2+66h^3,$$

$$h^4=77(h^2g_x-xc_x-30hc_x-64hc_y).$$

These are kernel-checked polynomial identities. No assumed Gröbner basis,
membership oracle or native execution result is used. [pcc:unverified]

All nine cubic normal forms reduce to specified scalars times h³.
The checked quartic relation and their products make every degree-four
monomial vanish in the actual quotient. Properness is proved by
evaluation at the origin before obtaining its characteristic101
structure. Explicit exponent factorization extends ideal membership to
every larger total degree. [pcc:unverified]

The closed declaration, in namespace JVMacaulayAudit, is
`literal_jv_macaulay_upper_certificate`. It proves the actual affine
unit ideal, homogeneous properness and all higher monomial membership.
Only propext, Classical.choice and Quot.sound occur. Lean4.35.0 /
Mathlib56a00585 are pinned; no holes, custom axioms, native_decide,
waivers or raised elaboration limits. Failed attempts and complete gate
receipts are retained losslessly. [pcc:unverified]

This proves an algebraic upper cutoff of four, not its minimality, exact
Hilbert function or quotient length8. Native execution, complete internal
F4degrees and formal F101³ curve/coefficient-descent identification remain
separate. The frozen native study returns GBdegree4 and then a degree5
selected-pair round with zero new polynomials/two zero reductions. That
log alone is not promoted to an Assumption2 refutation. Generic regularity,
larger m,n cases and the general computation-degree claim remain open.
[pcc:unverified]

Source: Joux–Vitse DLFq5.pdf pp7/9–10, SHA256 6f3268a0ef8c50b5…
(full digest in source-review.json);
exact read scope/source pins accompany this report. KB unavailable
(MCP absent/wrapper127); bounded primary/repository/Mathlib fallback.
No new native experiment, novelty claim, paid compute or agent storage
upload. Exact PCC/source receipts accompany the report; signed standing
is distinct from local kernel replay. [pcc:unverified]
