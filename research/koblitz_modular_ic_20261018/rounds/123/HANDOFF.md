# R123 handoff

R123 combines the four published local splits into a normalized x=0 branch census. Protocol/source were committed at `a93a1ff42` before data. Result SHA-256: `1435f4fbc2d1b04d628b5adc326916ed1f3a1406c24271348aaffe857e0b3fe3`.

Each of the four F2-rational base places contributes two normalized F2 places, of residue degree 2 and relative ramification indices 1 and 7. Their x-divisor degrees total 112. The simple factor follows a length-one Hensel edge; the seven factor follows a slope with denominator 7 and residual relation `W^7=1`. Both F4 residue roots were checked. Next, calculate N51's local order on these eight normalized places, including leading coefficients where cancellation may occur, and reconcile the resulting divisor with the earlier source-order discriminant data.

Independent degree audit, source hash rejection, replay, finite Lean arithmetic and visual PDF inspection passed. No accepted-run failure.
