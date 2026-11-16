---
title: "General geometric 3-torsion on actual curves"
fontsize: 10pt
geometry: margin=17mm
mainfont: DejaVu Serif
---

Lean proves exactly nine actual points P with 3P=0 on every curve
y²=x³+ax+b over an algebraically closed field with 2≠0, 3≠0 and
4a³+27b²≠0. This includes the identity. No rank, basis, Hasse or pairing
premise is assumed. Whole gate: 20 declarations, zero failures, zero
trusted-native exceptions. [pcc:unverified]

The quartic is $\psi_3=3X^4+6aX^2+12bX-a^2$, with derivative
$12(X^3+aX+b)$. The identity

$$\psi_3(x)=12x(x^3+ax+b)-(3x^2+a)^2$$

shows that a common cubic/quartic root
would force a=−3x², b=2x³ and discriminant zero. Hence the quartic is
separable, of degree four, and has four distinct roots. The cubic value is
nonzero at each root; each y²=x³+ax+b fibre has two distinct roots.
[pcc:unverified]

| Actual proof step | Declaration in ThreeTorsionRankAudit |
|:--|:--|
| No common cubic/quartic root | quartic_cubic_no_common_root |
| Separable quartic and four roots | quartic_separable; quartic_root_card |
| Actual group torsion iff quartic zero | affine_three_torsion_iff |
| Construct actual nonsingular root-pair point | root_pair_nonsingular |
| Explicit inverse affine coordinate maps | affineRootEquiv |
| Include the actual group identity | geometricCoordinateEquiv |
| Cardinality nine | geometric_three_torsion_cardinality |

The repeated-S₃ equivalence is inherited from checked actual Mathlib point
arithmetic. If 3P=0, then 2P=−P has the same x-coordinate. Conversely the
same-x alternatives are 2P=P or 2P=−P; the first forces an affine point to
be the identity and is impossible. The coordinate equivalence precedes
the count: actual torsion equals identity plus four two-point fibres.
[pcc:unverified]

Source: Balasubramanian–Koblitz1998, Theorem1/proof/Remark1, printed142–143,
reread. PDF SHA256 ac837b8edc47b346dbb341f787c41de0402c95371a159f9786b747bb2c987499.
Its matrix argument uses geometric torsion rank before the determinant.
This check proves the r=3 cardinality component under explicit short-curve
characteristic conditions. The qualified source is not refuted. All-prime
rank, determinant, generic BK, p<500 census and canonical pairing/divisor
identification remain separate. [pcc:unverified]

Lean4.35.0 / Mathlib56a00585, only propext, Classical.choice and Quot.sound.
No holes, custom axioms, native_decide, waivers or raised elaboration limits.
All failed sources/logs/exits and the complete gate are retained with
byte-for-byte archive checks. KB unavailable (MCP absent/wrapper127);
bounded original-paper/repository/Mathlib fallback. Exact source and packet
receipts accompany this report. Independent signed standing remains
unverified. No native experiment or paid compute. [pcc:unverified]
