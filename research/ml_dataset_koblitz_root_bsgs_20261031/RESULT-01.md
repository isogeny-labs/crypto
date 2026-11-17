# Completed Koblitz n23 root and BSGS pilot

The frozen 16-cell native population completed on local XFS. It crossed one
Koblitz n23 curve seed, m2, two generators, two keys, both twist choices, and
root/BSGS methods. The official frozen-protocol checker returned zero findings
before generation; native plan and preflight agreed on 16 cells and pinned
source, binary, and protocol hashes. The controller's terminal marker was
`{"planned":16,"custody_receipts":16,"complete":true}` with exit status 0.
`TERMINAL-01.json` retains a later read-only terminal observation of the same
marker, with captured stdout, stderr, and process exit status. It made zero
new generation attempts.

Every index 0..15 has one attempt marker, one native cell, successful native
verification, report, and strict export, and a restricted full-run archive
plus custody pointer with producer fresh S3 GET comparison. The archives total
2,780,728 bytes, ranging from 94,034 to 382,292 bytes. `AUDIT-01.json`
recomputes the axis census, native labels, strict export row counts, and local
archive hashes and sizes. The strict exports contain 16 general public feature
rows, 14 cyclic-subgroup ECDLP public feature rows, and 16 each of restricted
target and ECDLP provenance records. The two timed-out BSGS cells have no
public ECDLP feature row or recovered-scalar label; their bounded outcomes
remain in the native artifacts and restricted records.

All eight root cells have independently replayed, non-censored certificates
for their completed binary S3 root queries; none found a PDP witness. Their
native solver outcomes are `partial` because the ECDLP pipeline did not
complete, and none is counted as an ECDLP scalar solve. Six of eight BSGS
cells independently recovered and replayed the scalar. The other two reached
the fixed 30-second inner worker cap and are recorded as timeouts. Root
certificates establish their stated bounded query scope, not complete native
execution traces or whole-worker instruction counts.

The first wave stopped between cells after index 10 when the 4 GiB XFS
admission floor failed. `STOP-01.md` records the exact boundary; index 11 had
no attempt marker and was later attempted once under a fresh scoped
Conductor lease. No marked shard was rerun. All 16 cells were generated
locally; no cloud worker ran. The 16 cells all share n23 and therefore remain
one whole-field-size split component for leakage-safe training. This pilot
adds native Koblitz root traces and paired BSGS outcomes to the preceding
prime n23 slice; larger field sizes, arities and method arms remain separate
planned coverage. Exact first/last fall-degree certificates and whole-worker
retired instruction counts are null in these records.
