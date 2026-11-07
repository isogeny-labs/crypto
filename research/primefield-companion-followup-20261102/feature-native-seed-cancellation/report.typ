#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Projected monomial seed cancellation
2026-11-07 | PCC-20261107-c45d3c / PCC-20261107-c64866

#image("flow.svg", width: 100%)

*Derived construction.* The combined seed exponent is divisible by four in both partition-parity cases. For image order two or four, raising to the odd part of p-1 therefore cancels the seed. Direct monomial initialization uses raw division values. A constructor guard checks modular exponent closure. [pcc:unverified]

*Observed development checks.* Four selected native methods pass, including closure on 3479 algebraic partitions and rejection of 3479 changed seed exponents. Existing paired-source agreement covers 499 subgroup rows, 100 splits, 1375 advances and 450 skips; limits and factor-request checks also pass. These are unsigned producer results. The algebraic cases are not a curve cohort. [pcc:unverified]

*Scope and cost.* Phase/subgroup calculation and the full-value reference remain. This removes per-factor seed powers and twice-base seed construction, not all phase or residual work. No scalar recovery, comparative timing or cloud job ran. Original cohort intact.

*Failure retained.* First compile failed on an ambiguous integer type in the new test. The initial-build snapshot preserves it. A u64 annotation fixed compilation; no native test failed.

*Next.* Replace unused projected-path phase-value computation with sufficient subgroup validation. Investigate projected residual work, then complete-cost accounting.

Full parity proof: DERIVATION.md. Exact commands/hashes: receipt.json and source-lock.json. Sources: predecessor factor/projection derivations and phase implementation, with the previously reviewed Sutherland cyclic-group and Sage division-polynomial references. KB unavailable. No novelty-priority claim.
