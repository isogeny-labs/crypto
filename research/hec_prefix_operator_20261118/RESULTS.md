# Original-model prefix operator results [pcc:unverified]

Freeze295f2645769 pushed before sole execution; input hashes pass, exit0. All11model traces each2048steps produced smooth rows and passed native signed divisor reconstruction. Eight vector pairs per model match both explicit products and bilinear adjoint identities:88forward,88transpose,88adjoint comparisons. Every literal product count matches the asserted sparse-term or canonical-nnz count. Independent post-run bookkeeping verifies all22addition-count formulas without rerunning scientific code. COUNTS.json retains exact integer budgets and rational ratios.

|g|p|rows|explicit M=A|implicit M|forward A|
|--:|--:|--:|--:|--:|--:|
|2|101|922|42417|1809|4780|
|2|251|1000|62701|1986|5035|
|2|503|1085|68636|2163|5297|
|2|1009|956|61106|1909|4914|
|2|2003|1049|67653|2096|5194|
|2|4001|1008|64596|2015|5072|
|3|61|376|12892|1073|3498|
|3|211|298|18939|883|3230|
|3|401|318|20451|950|3317|
|4|31|100|1750|351|2500|
|4|61|131|4894|497|2677|

Transpose A is forward A+1. Explicit counts are the canonical sparse multiply reference, not existing elimination/forest costs. No CPU/runtime/allocation pricing is established. No rank coverage or same-target solve asserted for these formal original-model traces.

For nonnegative primitive prices mu,alpha (not both zero), operator cost ratio is nnz*(mu+alpha)/(s*mu+a*alpha). It is a weighted average of nnz/s and nnz/a. Consequently both genus4traces exclude10x for every such price pair: p31endpoints1750/351 and7/10; p61endpoints4894/497 and4894/2677. This is a statement about these two literal operators/traces only, not a lower bound on other circuits or solvers. For five genus2traces (p>=251), both endpoints exceed10; p101depends on price. None of these operator comparisons establishes an integrated speedup against the stronger forest pipeline.

Decision: qualify prefix/suffix operators as exact reusable primitives. Do not implement a full campaign on a genus2multiplication ratio alone. Genus4needs fewer traversed events, a different relation representation or another source of gain; black-box iteration count and minimal-polynomial/reconstruction costs are still missing. The general10xobjective remains active. Existing dense-block and stronger forest baselines remain intact.

Next admission can use the exact dual certificate in TARGET-CERTIFICATE.md to specify target-only solving without assuming a nonsingular square matrix. A cost budget must account for obtaining that certificate, not merely checking it.
