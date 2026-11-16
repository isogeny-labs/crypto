# Native public-target GHS replay on the mask-167 source

16 November 2026. The exact source is `E167⁻/F243: V²=U³+U²+(a⁴+2a+1)` with `a⁵+2a+1=0`, base-changed to `L=F_(3^35)` using the saved field model. The public `P167,Q167` in [construct.json](../ghs_twist_isogenous2_20261116/construct.json) have prime order `r=185302017921931` and `Q167=12345P167`. This replay begins with those two points, evaluates the reverse degree-two isogeny, recomputes the genus-five cover transfer and target descent, and checks the recovered scalar on **P167,Q167**. [pcc:unverified]

The independent [audit](audit_native.json) verified every input receipt hash, reconstructed a degree-two isogeny with Sage's constructor rather than the attack's rational formula, checked its action on both source points, matched the previous trace images and descent row, and checked the cost sum. The attack [receipt](native_ghs167_result.json) records 250,000 relation attempts, 2,069 accepted relations, a full-rank 1,092-column system, all 1,092 factor-base logs checked as group equalities, target descent after 124 offsets, and scalar 12,345 verified on both elliptic and Jacobian targets.

| Charged GHS phase | CPU seconds |
|---|---:|
| Source setup, including isogeny evaluation | 0.218665 |
| Pullback and five-term trace | 0.182640 |
| Factor-base construction | 6.407235 |
| Saved relation collection on the same carrier | 222.494982 |
| Saved linear algebra | 1.863915 |
| New target descent and verification | 0.126936 |
| **Full one-target GHS cost with saved shared precomputation charged** | **231.294373** |

The attack replay reuses the exact relation checkpoint and modular logs from the earlier completed mask-30 run. The new run independently performs the online steps from the public mask-167 points and verifies the old factor-base logs at 53.145711 CPU seconds, excluded from the attack cost as a validation pass. The relation collection and linear algebra costs are charged from their saved measurements, so 231.294373 is a **stitched phase total**, not one uninterrupted process measurement. It excludes carrier discovery and other failed search arms.

The previously completed Frobenius-aware rho suffix, after the exact mask-167 preprocessing, costs 3268.080314 CPU seconds under the same transported target. Against that saved baseline the GHS total is **14.1295 times lower**. An independent native rho process on P167,Q167 is still running; its terminal result will be reported separately and not pooled with this transported baseline. [The paired-orbit report](../ghs_twist_isogenous2_20261116/RESULT.md) describes the cover, degree bounds and bounded prior-work audit. This replay adds a direct source-to-solve measurement, not a new attack method or a new curve-family claim.
