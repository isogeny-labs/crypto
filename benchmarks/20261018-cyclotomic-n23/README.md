# Original n23 cyclotomic solver package

The [combined report](report.pdf), [SVG](a2-solver-status.svg) and 150 dpi
[PNG](a2-solver-status.png) copy the
[source run](../../research/ideas_execution_20261018/runs/a2-n23-20261018/REPORT.md).
Its raw native output, manifest and original argv are unchanged.

Five cells each requested sixteen targets. Two cells timed out at their
150-second cell limit before target records; two ONB cells emitted 32 records,
all with unresolved solving degree and zero lifted decompositions; one encoding
reported sixteen unsupported targets. K0 ONB recorded four unlifted Boolean
roots; K1 ONB recorded none. Missing timings are not replaced by zeros.

The graph plots requested slots by recorded cell status, and recorded F4
mean/median time only for completed ONB cells. Shared-host L0/Tier 0 diagnostic
notation, concurrent builds and trials, no qualified isolation or speed claim.
Reached degree three is not a measured solving degree. WDSat was unavailable
to this original launch and receives no retrospective credit.
