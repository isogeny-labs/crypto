# Ordered D18 external XOR solver control

The ordered-first-pair D18 positive XCNF has a complete original-constraint
model replay, but CryptoMiniSat 5.14.7 reached the frozen one-million-conflict
cap on that same satisfiable constructed control. It returned
`INDETERMINATE`, so the natural solver arm was withheld. [pcc:unverified]
Claim `PCC-20261114-907555` has `unverified` standing; this is a retained
local execution observation and raw-transcript audit.

| Item | Recorded result |
| --- | --- |
| Curve and base | primary N83 `a=0`; D18, 5,418,738 points and 32,643 folded columns |
| Formula | ordered-first-pair, phase-zero-first, shared-root, cadence-32; 329,808 variables, 2,807,999 CNF clauses, 3,901 XOR rows |
| Formula SHA-256 | `c632c152447cb09dbe7dd7cf7e29f713588c37bd56b67dfbf1fa7dac4cb15b74` |
| Positive model gate | prior native model passes all original XCNF rows; [replay evidence](../ordered-xcnf-20261114/RESULT.md) |
| Solver | `/opt/homebrew/bin/cryptominisat5` 5.14.7, SHA-256 `a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac` |
| Cap and outcome | one thread, 1,000,000 conflicts, 600-second solver cap; exit 15, `INDETERMINATE`, 1,000,002 reported conflicts |
| Resources | 431.659526792 s process wall; 425.033622 s user CPU, 1.850261 s system CPU; 367,376 KiB sampled peak RSS; no new swapouts; outer watchdog did not fire |
| Host gate | one-minute loads 7.43, 7.63, 7.03; swapouts held at 164,320; more than 2 GiB scratch |

The [protocol](PROTOCOL.md) and [export lock](EXPORT_LOCK.json) were committed
before solver execution. The [raw manifest](evidence/RAW_MANIFEST.json) retains
the command, UTC interval, stdout/stderr, process receipt, host samples,
independent status/conflict audit, formula hash, and source/binary bindings.
The 50 MB original XCNF remains at
`/Volumes/SSD990-2/tmp/n83-d18-ordered-xcnf-20261114/export-positive/result.xcnf`.

This arm measured solver capacity for one constructed control under an L0
shared-host envelope. It does not measure natural relation yield. The next
experiment must change a frozen solver or encoding policy and re-establish a
positive gate before natural D18 queries. Complete relation collection,
useful rank, subgroup logs, unseen-target descent, and matched same-point rho
remain required to rank end-to-end factor bases on both N83 curves.
