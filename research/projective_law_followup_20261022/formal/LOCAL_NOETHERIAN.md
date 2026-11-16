# Local Noetherianity of the actual weighted-Proj curve

**Status:** Pinned Lean 4.34.1 checks three declarations in `WeightedProjLocallyNoetherian.lean`. The whole-curve theorem has hypotheses `Field K` and an arbitrary coefficient tuple; it does not require the quintic to be separable. This is a proof-only packet, with no DLP trial or cost comparison.

The actual infinity and affine chart rings are quotients of `MvPolynomial (Fin 2) K`, hence Noetherian. The checked chart scheme isomorphisms carry local Noetherianity to the actual `X`- and `Z`-basic opens. The existing two-basic-open cover then yields `IsLocallyNoetherian (Proj (degreeSubmodule a3 a2 a1 a0))`. The earlier whole-curve `IsIntegral` theorem separately requires quintic separability.

This closes one prerequisite for Mathlib's order-of-vanishing API. It does not identify every closed support point with a coheight-one point, transport the source `N/B-y` function to all such stalks, prove finite support or the infinity pole, construct a global principal divisor, establish the Pic⁰ class equation, or connect the proof to compiled Rust.

The [PDF report](LOCAL_NOETHERIAN.pdf), [dependency graph](local-noetherian.svg), [standardized status CSV](local-noetherian.csv), [Lean source](WeightedProjLocallyNoetherian.lean), and [compiler receipt](local-noetherian-lean.log) are the review packet. The source and receipt hashes are recorded in the dated session log.
