#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= Relation oversampling in the factor-base log precompute — frozen run run-20261111T0602Z

*Protocol:* `research/logs_oversampling_20261111/protocol.json`, frozen 2026-11-11T05:54Z before this data, sha256 `aa928f27f9be1dff19e1d62ee79a148ac14c06d56c0dd80193bef62ae1153076`. All numbers `[pcc:unverified]`.

== Outcome and checks
- 4 cells × 30 seeds × 6 arms = 720 rows requested, 720 completed, *0 failed verification* (each arm re-checked `[log]G == point` for every column).
- *Decision (protocol rule): H0 for every f in every cell.* Every f ∈ {0.1, 0.25, 0.5} has a mean paired ΔC > 0 with the 95% bootstrap interval entirely above 0 at λ̂, and ΔC > 0 at λ = 0, λ̂/10 and 10λ̂. The `ic logs` reference arm (`logs_default`) is also costlier than f = 0 in every cell.
- Mechanism: the extra relations cost trials that the filter/LA cannot win back. LA is 0.8–15.7% of f0 wall time. In only two cell–arm pairs did LA work fall on average (C3 f0.1: −17.9 row_ops; C4 f0.1: −4.2); for oversampling to pay there, a row operation would have to cost 0.61 and 2.6 trials respectively, about 98× and 14,000× the measured λ̂ (`full/break_even.json`).
- Consistent with the 5-seed pilot (run-20261111T0451Z, disjoint seeds).

== What ran
- `research/logs_oversampling_20261111/full/run_full.sh` under bench record `benchmarks/records/20261111T0602Z-logs-oversampling-full.json`; release `ic` rebuilt at the frozen commit (branch `aburan28/kind-volta-4zo2ul`, after merging main); `ic oversample --seeds 30 --seed 0x4b6f626c69747a64 --fractions 0,0.1,0.25,0.5 --aa-control --logs-arm`, `RAYON_NUM_THREADS=1`. Wall 2 min 45 s.
- Host: shared cloud container, Intel Xeon, 4 CPUs, isolation *unstated*. Owner-requested local route (isolab-eks unavailable: no AWS identity); S3 resume checkpoint waived by the owner. The primary metric is in counted units (trials, row_ops), exact; wall values are indicative only.
- Cells: C1 deg 23 m=2 factor-index 0 (44 cols); C2 deg 37 m=3 `ic search` selection (16 cols); C3 deg 37 m=3 subgroup_orbits 7400, CADO filter (104 cols); C4 = C3 with legacy filter.
- Protocol gaps recorded at freeze: no research-kb receipt (not reachable), no independent reviewer.

== Results (derived by `full/analyze.py` from `full/rows/*.jsonl`)
C(f) = trials + λ·row_ops; λ̂ = median over f0 arms of (LA s / row_op) ÷ (collection s / trial). ΔX = arm − f0 on the same seed's pool. CI: paired bootstrap over 30 seeds, 10,000 resamples.

#include "table.typ"

#figure(image("full_deltas.svg", width: 100%), caption: [Paired differences vs f0 per cell, 30 seeds each, run-20261111T0602Z. Left: total counted cost at λ̂ (mean, 95% bootstrap CI). Right: LA row operations (mean). Derived values.])

== Scope
This measures `ic logs` precompute on these four small Koblitz cells, where filtering leaves a Krylov core of 0–14. It says nothing about cells with large cores (hundreds of columns or more), where LA could dominate. No improvement over the index-calculus baseline was found; no leaderboard row.

== Raw records
`research/logs_oversampling_20261111/full/rows/C{1..4}.jsonl` and `.out.json`; summary `full/full_summary.json`. Hashed in the bench record; not uploaded to S3 (no AWS identity).
