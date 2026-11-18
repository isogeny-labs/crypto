# Construction and bounded-tail work diagnostics

Same n17/n23 generator systems; baseline, prepared layout/top-degree/linear-tail/direct-packing package, and bounded quadratic-tail package. Every original root set verifies equal. Shared-host native wall values are retained raw but not admitted or quoted.

The n23 complete-root tree uses1022 matrices before,511 with prepared policies and63 with the quadratic tail. The tail also performs4194304 Gray assignments. Those are different work units; fewer matrices alone is not a speed result. Both matrix engines show the same count pattern here.

These diagnostics motivated the frozen dedicated-host runs; they do not substitute for five paired native timings or full DLP recovery. Sources: diagnostic-*.jsonl, frozen baseline and candidate manifests.

![local-matrix-work.svg](../../local-matrix-work.svg)

## Recorded work, including enumeration

|Arm|n|Policy|Matrices|Word XORs|Gray assignments|
|---|---|---|---|---|---|
|solve-f4|17|baseline|126|2705998|0|
|solve-f4|17|prepared|63|754446|0|
|solve-f4|17|quadratic-tail|1|39685|65536|
|solve-f4|23|baseline|1022|112863703|0|
|solve-f4|23|prepared|511|26680925|0|
|solve-f4|23|quadratic-tail|63|6779923|4194304|
|solve-f5|17|baseline|126|2705998|0|
|solve-f5|17|prepared|63|754446|0|
|solve-f5|17|quadratic-tail|1|39685|65536|
|solve-f5|23|baseline|1022|112863703|0|
|solve-f5|23|prepared|511|26680925|0|
|solve-f5|23|quadratic-tail|63|6779923|4194304|
