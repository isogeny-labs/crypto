# Source-function order on the actual weighted-Proj curve

**Status:** Pinned Lean 4.34.1 checked the three declarations in `WeightedProjSourceOrder.lean` with exit code 0. The direct axiom prints list only `propext`, `Classical.choice`, and `Quot.sound`. This packet does not report a DLP trial or a new cost ratio.

The existing source-style common 2+2 function `N/B-y` lives in the function field of the actual weighted-Proj curve under the separability hypothesis. The existing whole-curve integrality and local Noetherianity theorems supply Mathlib's prerequisites for `Scheme.ord`. The new source specializes that order to the source function. It also records the API boundary: points not known to have coheight one get the defined zero value; at a coheight-one point, the value agrees with the stalk valuation.

This does **not** prove that any named closed support has coheight one, that the source function is nonzero, that its order is finite-supported, that its infinity pole has the expected value, or that its principal divisor gives the required `Pic⁰` equation. The compiled Rust correspondence is separate.

The [PDF](SOURCE_ORDER.pdf), [dependency graph](source-order.svg), [standardized gate CSV](source-order.csv), [Lean source](WeightedProjSourceOrder.lean), and [compiler receipt](source-order-lean.log) form the review packet. The compiler status and hashes are recorded in the dated session log.
