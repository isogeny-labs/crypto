#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= Relation-oversampling pilot — run-20261111T0451Z

*Status: pilot under a DRAFT protocol. Not the measurement; supports no claim.* All numbers are `[pcc:unverified]`.

== Outcome and checks
- 4 cells × 5 seeds × 6 arms = 120 rows requested, 120 completed, *0 failed verification* (every arm re-checked `[log]G == point` for every column).
- In every cell, every oversampled arm (f = 0.1, 0.25, 0.5) and the `ic logs` reference arm had a *higher* total counted cost than f = 0, at λ = 0, λ̂/10, λ̂ and 10λ̂. The extra collection trials dominate; LA row operations also rose on average in 15 of 16 cell–arm pairs (exception: C4 `logs_default`, −523 row_ops mean, which did not offset its +42.8 trials).
- C1–C3 (CADO filter) leave a Krylov core of 0–1; C4 (legacy filter) leaves 8–11.

== What ran
- Command: `research/logs_oversampling_20261111/pilot/run_pilot.sh` under bench record `benchmarks/records/20261111T0443Z-logs-oversampling-pilot.json` (record `commit` field 6df0f91 is from `init`; the run used checkout 9373a70, which differs only in the run script and factor-base spec files; Rust identical to a4867885, the commit CI tested).
- Binary: `target/release/ic` (`cargo build --release --bin ic`), `ic oversample --seeds 5 --fractions 0,0.1,0.25,0.5 --aa-control --logs-arm`, `RAYON_NUM_THREADS=1`.
- Host: shared cloud container, Intel Xeon @ 2.30 GHz, 4 CPUs, isolation *unstated* (no L2). Wall times are indicative only; counted units (trials, row_ops) are exact. Local route at the owner's request; S3 resume checkpoint waived by the owner (Git instead).
- Cells: C1 deg 23 m=2 factor-index 0; C2 deg 37 m=3 `ic search` selection (1184 pts); C3 deg 37 m=3 subgroup_orbits 7400 (CADO); C4 = C3 with legacy filter. Seeds: base `0x4b6f626c69747a00` + i, i = 0..4.

== Results (derived by `pilot/analyze.py` from `pilot/rows/*.jsonl`)
C(f) = trials + λ·row_ops. λ̂ = median over f0 arms of (LA s / row_op) ÷ (collection s / trial). ΔX = arm − f0 on the same seed's pool. CI: paired bootstrap over 5 seeds, 10,000 resamples.

#include "table.typ"

#figure(image("pilot_deltas.svg", width: 100%), caption: [Paired differences vs f0 per cell, 5 seeds each, run-20261111T0451Z. Left: total counted cost at λ̂ (mean, 95% bootstrap CI). Right: LA row operations (mean). Derived values.])

== Sizing the frozen run
`pilot_summary.json` gives, per arm and λ, the seeds needed for a 95% CI half-width of 1% of mean C(f0): ≤ 60 for all f arms except C2 f0.5 (207), where C(f0) is only 29 trials. For deciding the *sign* of ΔC the 5-seed intervals already exclude 0 in every cell. Proposed `seeds_per_cell`: 30 (unchanged) — a decision for the protocol reviewer at freeze.

== Raw records
`research/logs_oversampling_20261111/pilot/rows/C{1..4}.jsonl` (rows) and `C{1..4}.out.json` (command output); summary `pilot/pilot_summary.json`. Raw files are hashed in the bench record; S3 upload not possible from this container (no AWS identity).
