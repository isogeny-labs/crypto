#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Bidirectional nonsingular character columns
2026-11-05 · T-1640

The cubic power builder now uses disjoint forward and reciprocal prefixes. It supplies the reflection-odd solver with the same coefficient columns while removing the characteristic-r anchor convolution. This is a supplied derivation and recorded local execution, with no signed PCC standing. [pcc:unverified]

#image("pipeline.svg", width: 100%)

== Endpoint construction
Let h=(r-1)/2 and D=degree(g) h after removing a possible zero root. Generate coefficients 0 through min(D,r-1) using the ordinary Miller recurrence. Reverse the monic polynomial g and generate coefficients 0 through D-r when D reaches r. Reversing the latter coefficients supplies original degrees D down to r. For cubic g, D-r is at most (r-3)/2, so both prefixes stay strictly below the singular recurrence index. Restore the zero-root power and reduce modulo Z^r-Z only after ordinary polynomial construction. [pcc:unverified]

== Recorded checks
The optimized overflow-checked native run reports 102 passing tests: retained reference tests, 1,037 full-column comparisons over seven prime fields, 36 span controls, the q15 inconsistency control, all 479 target identities and eight source integrations. Additional tests bind the packet to the saved reflection result, reject a singular prefix, and check empty and zero-root columns. Twenty-two source/data locks bind the run. [pcc:unverified]

The separately implemented polynomial checker is binary-hash checked and freshly accepts the new packet. The previous partner-sign corruption record is preserved but was not rerun. Eight compiler warnings remain in build.log. [pcc:unverified]

== Cost, scope, and next construction
The derived fixed-degree bound remains O(r + log r) field work after shared O(r) inverse setup. The singular convolution is removed; no timing or resident-memory comparison is reported. The solve retains 239 rows, 344 orbit columns, 479 output terms and 16 source signs in the existing r479 fixture. The original 4,563-case cohort is preserved. [pcc:unverified]

The endpoint argument also covers quartic powers, where D=2r-2. Next implement quartic feature synthesis and examine whether the expanded odd span needs fewer source shifts. The current API accepts cubic labels only. Fast structured elimination still needs a suitable low-length displacement generator. [pcc:unverified]

== Provenance
Theorem PCC-20261105-f52ca7 and execution PCC-20261105-c46a77 have local verdict not_covered. No signed verification is asserted. See DERIVATION.md, claims.json, receipts, logs and source-lock.json. Prior work: #link("https://flintlib.org/doc/fmpz_poly/index.html")[FLINT polynomial documentation] (revision notice read), and #link("https://arxiv.org/html/1703.03734")[Bostan et al., displacement-structured algorithms]. No novelty claim.
