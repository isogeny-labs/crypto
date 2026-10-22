# Skipped NIST seed candidates: DiSSECT continuation

The [factorization study](factorization/README.md) continues the timeout
entries from the earlier external DiSSECT run: **eight of nine distinct
integers are fully factored**, resolving **thirteen of fourteen timeout-valued
fields**. Only the P-256 block-876 extension-twist order retains a composite
cofactor, of 444 bits. [R02 results and replay](factorization/rounds/R02-20261022/README.md)
preserve the earlier R01 evidence unchanged.

The [23-job runner](../../backlog/XP-20261021-nist-skipped-curves/README.md)
and manual workflow remain available for a fresh upstream run. This study
imports the earlier run as unchanged evidence and supplies new native
factorization results; it does not claim that the manual workflow ran.

See the [R02 PDF](factorization/rounds/R02-20261022/reports/run-20261022T0724Z/report.pdf),
[latest exact factors](factorization/rounds/R02-20261022/results.json), and
[remaining obstruction](../../problems/nist-extension-twist-composite-cofactors.md).
These results do not establish why the historical candidates were skipped.
