#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= IndependentControls correction proposal
[pcc:unverified] Source-only changes address the native V3 diagnostics. Original source and receipts remain immutable in archive commit a51006010d.
#image("correction-flow.svg",width:100%)
- Line 19: qualify ActualNonbranchNorm.residual.
- Divisibility and Bezout proofs: normalize Polynomial.C numerals before ring.
- Line 74: indent theorem argument continuation.
- Lines 120–121: use AddValuation.map_mul for additive valuation.
Theorem statements, hypotheses, fixture constants and controls are retained. The candidate is uncompiled. Other diagnostics may become visible after these errors are resolved.
Sources: archived native stdout/source; pinned Mathlib Polynomial.Basic C_ofNat (line 451), Valuation.Basic AddValuation.map_mul (line 1160). Exact diff, hashes and machine-readable diagnostic mapping accompany this report.
No compiler, test or replay was invoked. The three-attempt ceiling remains exhausted. A separate prospective amendment and independent source review are necessary before execution. A future protocol must preserve all 30 readback names and the separate negative control. This packet renews no clock or attempt allowance.
