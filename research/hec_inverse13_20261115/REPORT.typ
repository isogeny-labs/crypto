#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Verified algebraic Kummer inverse
F127 · genus 3 · 15 November 2026

#image("flow.svg", width: 100%)

The registered public target reconstructs one oriented Jacobian image of B. One positive and one target solver cell pass exact verification of all 12 input equations. The target elimination polynomial has degree 12 and factors into four irreducible cubics. All four factors descend to F127 divisors and yield one common image after public adjacency selects the sign. Corrupted adjacency is rejected. Recovery source: 569ce35e4511; pre-run S3 checkpoint: 20261115T220449Z.

== Why orientation is independent
Let Y be the image of B+D1 and G the known image of D1. A lift supplies X=±Y. Compare the two candidate Kummer coordinates of X+G and -X+G with the public image of B+2D1. Both signs could pass only if 2Y=0 or 2G=0, excluded by the checked nonzero odd-prime subgroup. Subtract G, then compare to the earlier B certificate. That certificate is never used to choose the sign.

== Reproducible record and retained failures
Source curve: y²=f(x) over F127 with ascending coefficients (31,89,73,48,91,41,123,1). Subgroup order 1028893. D1=2(0,44), B=2(3,21). The complete target quartic and public coordinate inputs are recorded in RESULTS.md and PUBLIC-KUMMER.txt / PUBLIC-NEXT-KUMMER.txt.

Generator control: 12 zero residuals, independent expansion and corruption checks. Solver: private msolve v0.11.0, 71 upstream tests passed. Two collector failures occurred before any solver cell because of manifest paths; their raw failures and frozen amendments remain. The corrected collector launched each scientific cell once. Exact verifiers check squarefreeness, invertible denominator, all polynomial identities and the known positive control. Raw recovery is RECOVER.txt, exit status is EXIT-RECOVER.txt, and stderr is empty.

== Scope and next step
Classification: engineering implementation of an established inverse route; see Tian §4.2.3, https://arxiv.org/html/2007.03172. This local macOS run is correctness evidence. Sampled RSS missed the short child peak and is only a lower bound. No timing gain follows. Existing performance graphs remain unchanged. The previous cold-cost gate remains failed; the full 10× goal remains open. Next: integrate the inverse with charged cold construction/calibration and sparse destination linear algebra, then evaluate genus 2 and 4 separately.

Source-linked details: RESULTS.md; PROTOCOL.md; PROTOCOL-SOLVE.md; PROTOCOL-RECOVER.md; generator.rs; verify.rs; recover.rs; run-target.res; RECOVER.txt, in research/hec_inverse13_20261115. PR: https://github.com/isogeny-labs/crypto/pull/4274.
