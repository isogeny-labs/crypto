#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Value-only projected torsion validation
2026-11-07 | PCC-20261107-b5cddb

#image("flow.svg", width: 100%)

*Construction.* After seed cancellation, the projected path needs subgroup validation but no phase value. The new predicate checks the original bounds, gcd, curve/point/nonzero-y conditions and vanishing of the order-r division value. Prime p and r remain caller assumptions. Separability implies the old derivative is nonzero on this domain. [pcc:unverified]

*Observed development checks.* Five selected native methods pass. The predicate agrees with both the old phase routine and affine multiplication on 588 point/order cases: 12 accepted, 576 rejected. Seven malformed/unsupported cases are rejected. The paired source retains 499 subgroup rows, 100 splits, 1375 advances and 450 skips, plus factor and closure checks. These are unsigned producer results; each invocation filters 60 other methods. [pcc:unverified]

*Cost boundary.* No projected derivative propagation, derivative inverse or phase exponentiation remains. Value-only torsion evaluation and its 2y inverse remain; the mandatory reference still computes full phases. No scalar recovery, timing or cloud run; original cohort intact.

*Next.* Reuse the torsion-check cache for direct factors, then address projected residuals and complete costs.

Evidence: receipt.json, source-lock.json, subgroup.log and existing test logs. Full domain and fixture panel: DERIVATION.md. Sources: native modules and Sutherland MIT18.783 Lecture6 (2017), https://math.mit.edu/classes/18.783/2017/LectureNotes6.pdf. KB unavailable. No novelty claim or independent-review standing.
