#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Validated torsion-cache reuse
2026-11-07 | PCC-20261107-e63506

#image("flow.svg", width: 100%)

*Implementation.* Successful subgroup validation returns an owned cache bound to the field, curve, order and point. Direct projected initialization checks the binding and consumes the same engine for retained factors. The twice-base anchor keeps its existing fresh-engine path. Full-value validation remains mandatory.

*Observed development checks.* Six selected native methods pass. Each saved fixture uses one validation engine and no extra direct-factor engine; re-requesting the cached order-r value adds no entries. Context mismatch checks cover field, curve, order and point, including rejection by the consuming initializer. Existing agreement checks retain 499 subgroup rows, 100 splits, 1375 advances and 450 skips; the 588-case predicate panel, seven malformed cases and closure/factor checks also pass. Each invocation filters 61 methods. These are unsigned producer results. [pcc:unverified]

*Scope.* This establishes engine reuse, not a complete-cost or timing result. Additional factors may still add entries. Geometry residual, projection and reference work remain. No recovery or cloud panel ran; original cohort intact.

*Next.* Investigate computing projected geometry residuals directly, then account for complete work.

Evidence: README.md, receipt.json, source-lock.json and cache/test/subgroup logs. Source review: predecessor torsion predicate, transport direct paths and value-only cache. The previous subgroup/separability argument is unchanged. KB unavailable; no novelty or independent-review claim.
