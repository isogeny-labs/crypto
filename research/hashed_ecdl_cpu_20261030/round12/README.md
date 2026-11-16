# Checked projective witness-sum screen

Observations only: shared-host L0 diagnostics, not a certified speedup or scientific verdict.
Baseline u/x is `2bd4588fba60`; previous density-aware candidate y is
`d0ad5a617f59`; candidate v and batch1 control w use `308bf1a6a5bc`.
The candidate validates every point and sums oracle witnesses projectively,
with one final affine conversion. Solver checks and both complete certificate
replays remain unchanged.

| Fixture | Baseline u ms | Previous y ms | Candidate v ms | u/v | y/v |
|---|---:|---:|---:|---:|---:|
| 8-bit, 3 slots | 0.286241 | 0.121970 | 0.119681 | 2.392 | 1.019 |
| 16-bit, 3 slots | 3.985123 | 0.666682 | 0.657712 | 6.059 | 1.014 |
| 16-bit, 4 slots | 4.174184 | 0.809032 | 0.796733 | 5.239 | 1.015 |

All 150 executions completed: 135 measured and 15 warmups. All full certificate
bytes agree across arms; scalar/replay checks and corrupted-scalar rejection pass.
Nine measured repetitions follow one warmup per cell, on AMD EPYC9R45,
one software thread, 2 vCPUs/4 GiB reserved. No failed or excluded cells.
Medians above are measured; ratios are derived as the named numerator divided by v.
The small and middle fixtures have overlapping candidate/previous ranges;
the largest fixture has v 0.794022–0.802362 ms versus y 0.806052–0.814513 ms.
Retain the candidate as a bounded engineering choice; these shared-host timings
do not certify improvement. The further 10x target remains unmet.

The complete interval includes setup, discarded trials, relation collection,
elimination, both certificate checks and serialization. Process launch, archive
I/O and the negative control are excluded. No phase or fixture was removed.
The export passes check-export --verify-files with zero findings. The source
validation recorded 31 release tests and strict Clippy passing.

[Protocol](protocol.json), [runner](run.sh), [rows](results/rows.csv),
[summary](results/summary.csv), [graph](results/latency.svg),
[export](results/export.json), [build identities](results/build.json),
[terminal receipt](status.json), [archive receipt](results-receipt.json),
[report](../reports/round12/report.pdf), [candidate notes](../checked-sum-candidate.md).

Earlier provenance corrections and the rejected round10 candidate remain intact.
Strict isolation, independent scientific review, matched external/rho comparisons
and larger-scale recovery remain outstanding. Shared-source ownership is still
a constraint on the next reduction optimization; this change is in the CPU crate.

## Delivery handoff

- What was done: reconciled the successful EKS run, preserved its S3 receipt and
  150 raw executions, verified the export, and refreshed existing PR #2946.
- Decisions: preserve the original optimized baseline and all five controls;
  retain the checked-sum candidate without claiming certified performance.
- Open threads: further 10x target, strict isolation, external comparisons,
  and larger-scale recovery remain incomplete.
- Errors: no run failures or export findings. The dated shared session log is
  absent in this checkout; this scoped handoff preserves the delivery record.
