# Skipped NIST seed candidates: DiSSECT continuation

The [factorization study](factorization/README.md) continues the timeout
entries from the earlier external DiSSECT run: **eight of nine distinct
integers are fully factored**, resolving **thirteen of fourteen timeout-valued
fields**. Only the P-256 block-876 extension-twist order retains a composite
cofactor, of 444 bits. [R02 results and replay](factorization/rounds/R02-20261022/README.md)
preserve the earlier R01 evidence unchanged.

[R04](factorization/rounds/R04-20261022/README.md) preserves 329 validated
polynomial records and a native-selected job for the next P-256 sieving
allocation. Its bounded search returned no new factor; the arithmetic
counts above remain unchanged.

[R05](factorization/rounds/R05-20261022/README.md) ended with exit 139 after
3,164,757 main-file raw relations, plus a separate 32,684-line worker
output. Its failed native status, full checkpoint and fresh independent
arithmetic replay are recorded; no new factor was found and no worker is
running in that R05 allocation. The earlier interim observation remains unchanged.

The [2026-10-23 FactorDB check](factorization/factordb-20261023/README.md)
queries all nine original targets and the exact remaining cofactor. It
supplies no additional factors. Seven targets are more completely factored
locally; the other two partitions agree. All returned products and their
grouping into local pieces pass native PARI checks.

The [native timeout diagnostic](factorization/native-timeout-diagnostic-20261023/README.md)
reproduces the unpatched tool's exit 139 and validates its two-line printf
fix in bounded full-tool controls. The later resume card now requires the
checked executable and the R05 relation checkpoint; it remains unqueued.

[R06](factorization/rounds/R06-20261023/README.md) is a separate local
1800-second continuation with that validated binary and the saved relations.
Its dated 2026-10-23 01:43:40 UTC observation records two active native
workers. Final status, archive and post-run replay remain pending until
that actual bounded worker terminates.

The [23-job runner](../../backlog/XP-20261021-nist-skipped-curves/README.md)
and manual workflow remain available for a fresh upstream run. This study
imports the earlier run as unchanged evidence and supplies new native
factorization results; it does not claim that the manual workflow ran.

See the [R02 PDF](factorization/rounds/R02-20261022/reports/run-20261022T0724Z/report.pdf),
[latest exact factors](factorization/rounds/R02-20261022/results.json), and
[remaining obstruction](../../problems/nist-extension-twist-composite-cofactors.md).
These results do not establish why the historical candidates were skipped.
