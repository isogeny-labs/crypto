#set page(paper: "a4", margin: 20mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: none)

= Nonbranch independent precheck
_2026-11-11 · source and elementary mathematics only_

The exact review plan and input hashes were published in crypto PR #3859 before the two separate reviewer sessions. The original campaign remains at three of three attempts. This packet contains no Lean invocation, new attempt, theorem verdict, or scientific promotion.

== Two independent findings

*Blind sign joint.* Given n(X) = -9 and b = -9, the local expression is g = -9 + 9y. Its residues at P+ = (1,1) and P- = (1,-1) are 0 and -18. Under the supplied local-ring unit criterion, g is a nonunit at P+ and a unit at P-. The negative equality n(1) = b(-1) reads -9 = 9 and is false. The blind reviewer read only the mathematical handout.

*Source fidelity.* The correction changes six proof-script or indentation sites. It leaves theorem statements, hypotheses, definitions, coefficients, fixtures, and controls unchanged. The archived readback names ten Nonbranch declarations and twenty IndependentControls declarations. The source reviewer read the archived and proposed Lean files, patch, result, and plan, with no sibling report.

#table(columns: (31%, 25%, 44%), inset: 6pt,
  [Joint], [Machine status], [Independent outcome],
  [Sign and false control], [not_verified], [Arithmetic holds under supplied local-ring assumptions],
  [Candidate source], [not_verified], [Statement and fixture preservation holds statically],
  [30 axiom readbacks], [not_verified], [Names retained; outputs absent],
  [Selected-order identity], [not_verified], [Factor definitions and norm/order premises still required],
)

== Open proof boundary

Hosted proof-gate batches 180 and 184 failed to resolve five archived pencil roots' cross-directory imports. The original native compiler result remains failed; the correction is proposed_uncompiled. A prospective one-attempt plan sets a 40-minute local limit and resource controls, but execution requires separate owner authorization. Passing compilation would still require an exact-name axiom audit, a failing negative control, and independent semantic review.

Reviewer isolation is self-attested; no applicable structured independence checker ran for this packet. The outcome is therefore a limited precheck, not validated_scoped.

#text(size: 8pt, fill: gray)[Artifacts: blind-handout.md, blind-review.md, source-review.md, flow.svg. The active user Spot-agent and experiment limit is USD 25/day; the repository's USD 200/day IsoLab allowance is separate.]
