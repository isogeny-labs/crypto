#set page(paper: "a4", margin: 19mm)
#set text(size: 11pt)
= Native truncated polynomial products
Prime-field source arithmetic · 2026-11-03

The new mul_low operation computes only the requested low coefficients. Five Newton, reversed-quotient and remainder callsites use it; full product-tree construction remains unchanged.

#image("pipeline.svg", width: 100%)

== Construction
After splitting at k, the product is U plus X^k times (W-U-V) plus X^(2k)V. Compute U through the requested output and W,V through the cross-term precision. The high-half V must remain available for cross subtraction even if its final shifted contribution vanishes. Schoolbook leaves omit terms outside the requested range; full-output requests reuse the original multiplier.

== Checked behavior
One optimized native build and one bounded invocation pass all 66 methods with zero failures or filtered tests. The new grid includes 936 cases against full-product truncation and independent schoolbook convolution, plus sparse cancellation checks. All eight complete recovery receipts match the preserved reference. Seven prebuild source locks pass; two inherited warnings remain in the log.

== Scope and next step
No comparative count or timing panel ran. This separate candidate preserves the existing comparison binary and protocol. Next add matched instrumentation for the new loops and additions before a separately identified comparison. The original 4563-case cohort remains unchanged.

== Sources
The existing monic polynomial source and #link("https://flintlib.org/doc/nmod_poly/mul.html")[FLINT truncated-product documentation]. See DERIVATION.md for the algebraic argument and receipt.json, source-lock.json and raw logs for checks.
