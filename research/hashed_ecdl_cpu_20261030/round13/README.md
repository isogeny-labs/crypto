# Direct projective witness comparison screen

Observations only: shared-host L0 diagnostics, not a certified speedup or scientific verdict.
Baseline u/x is `2bd4588fba60`; previous checked-sum candidate y is
`308bf1a6a5bc`; direct-comparison candidate v and batch1 control w use
`92463171363c`. Every tuple point and target is validated. The oracle uses
cross multiplication to compare its projective sum with the target, removing
the final inversion. Solver checks and both full certificate replays are unchanged.

| Fixture | Baseline u ms | Previous y ms | Candidate v ms | u/v | y/v |
|---|---:|---:|---:|---:|---:|
| 8-bit, 3 slots | 0.287681 | 0.122430 | 0.119581 | 2.406 | 1.024 |
| 16-bit, 3 slots | 3.972973 | 0.659942 | 0.657142 | 6.046 | 1.004 |
| 16-bit, 4 slots | 4.180033 | 0.800253 | 0.792513 | 5.274 | 1.010 |

All 150 executions completed: 135 measured and 15 warmups. All full certificate
bytes agree across arms; scalar/replay checks and corrupted-scalar rejection pass.
Nine measured repetitions follow one warmup per cell, on AMD EPYC9R45,
one software thread, 2 vCPUs/4 GiB reserved. No failed or excluded cells.
Medians above are measured; ratios are derived as the named numerator divided by v.
All three candidate/previous ranges overlap. Candidate v ranges are
0.118280–0.125210, 0.652072–0.668372 and 0.790512–0.799413 ms; previous y ranges
are 0.120040–0.132980, 0.658352–0.671782 and 0.795922–0.806052 ms.
Retain the candidate for the removed inversion and passing exact checks;
these timings do not certify improvement. The further 10x target remains unmet.

The complete interval includes setup, discarded trials, relation collection,
elimination, both certificate checks and serialization. Process launch, archive
I/O and the negative control are excluded. No phase or fixture was removed.
The export passes check-export --verify-files with zero findings. The source
validation recorded 31 release tests and strict Clippy passing.

[Protocol](protocol.json), [runner](run.sh), [rows](results/rows.csv),
[summary](results/summary.csv), [graph](results/latency.svg),
[export](results/export.json), [build identities](results/build.json),
[terminal receipt](status.json), [archive receipt](results-receipt.json),
[report](../reports/round13/report.pdf), [candidate notes](../projective-compare-candidate.md).

Earlier provenance corrections and the rejected round10 candidate remain intact.
Strict isolation, independent scientific review, matched external/rho comparisons
and larger-scale recovery remain outstanding. Shared-source ownership is still
a constraint on the next reduction optimization; this change is in the CPU crate.

## Delivery handoff

- What was done: reconciled the successful EKS run, preserved its S3 receipt and
  150 raw executions, verified the export, and refreshed existing PR #2946.
- Decisions: preserve the original optimized baseline and all five controls;
  retain the direct-comparison candidate without claiming certified performance.
- Open threads: further 10x target, strict isolation, external comparisons,
  and larger-scale recovery remain incomplete.
- Errors: no run failures or export findings. The dated shared session log is
  absent in this checkout; this scoped handoff preserves the delivery record.
