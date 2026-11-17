# IndependentControls source correction proposal

[pcc:unverified] This candidate changes only proof-script name resolution, polynomial constant normalization and continuation indentation. The original source and native diagnostics remain in nonbranch-v3-execution-20261105, commit a51006010d. No theorem statement, fixture coefficient, hypothesis or control is removed. The diff is correction.patch; exact original/candidate hashes are in result.json.

Four diagnostic groups are mapped in diagnostics.csv: qualify ActualNonbranchNorm.residual; normalize Polynomial.C numerals before ring in the two divisibility goals and Bezout certificate; indent selected_order arguments; use AddValuation.map_mul for the additive valuation. These are proposed remedies, not compiler-checked repairs. Other errors may become visible after these errors are resolved.

Sources checked: exact native IndependentControls.stdout and IndependentControls.lean; pinned Mathlib/Algebra/Polynomial/Basic.lean C_ofNat at line 451; pinned Mathlib/RingTheory/Valuation/Basic.lean AddValuation.map_mul at line 1160. KB not loaded; this is an exact-source diagnostic correction, with no new mechanism claim.

No compiler, test or replay was invoked. The existing three-attempt ceiling remains exhausted. A distinct prospective amendment and independent source review are required before any new replay; this packet grants neither execution nor a renewed clock. All 30 readback names and the separate negative control must be retained by any future protocol. T-1671 owns this new proposal path only.
