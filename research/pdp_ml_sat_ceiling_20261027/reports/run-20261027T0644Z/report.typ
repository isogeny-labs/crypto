#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9pt)
#import "tables.typ": *
= Run report: ML-aided SAT ceiling for the PDP (run1)

*Study:* `research/pdp_ml_sat_ceiling_20261027/` · *Protocol:* PROTOCOL.md (commit 8a85584fd, amendment 1) · *Probe:* `examples/pdp_ml_sat_ceiling.rs`, seed 20261027.\
*Labelling run:* started 2026-10-27T06:44:54Z at commit 7dddd8099 (`run1/rows_*.jsonl`). *Analysis run:* 2026-10-27T08:46:02Z at commit d673109e7, `--from-rows run1` (`run1/analysis/`), wall 8 min 11 s.\
*Host:* local development container, 4 vCPU Intel Xeon @ 2.10 GHz, *no isolation* (not Iso Lab: no AWS identity). Cost unit: solver conflicts (machine-independent); wall times diagnostic only.\
*Arms:* symmetrised $S_4$, $m=3$, native CDCL+XOR, symmetry breaking on; families n15l5 (3000 targets) and n17l6 (1200); L3 on 60 / 30 held-out UNSAT targets, cap 10× a0 median.

*Runs requested and completed.* (1) First launch at commit d3d27b879: aborted by its own label cross-check — a solver soundness bug (fixed, 7dddd8099). (2) Labelling run: both families labelled; killed by the session's 2 h background limit during n17l6 L3; L1–L3 results were never written. (3) Analysis run from the saved rows: complete, exit 0. Every label (4200) agrees with exhaustive sorted-triple search.

*Population (measured).*
#table_pop

#figure(image("conflict_distribution.svg", width: 100%), caption: [Solver conflicts per target over sorted triples (run1 rows, all 4200 targets).])

*L1 triage (measured; gain derived as held-out relations/conflict ÷ solve-all, telemetry conflicts charged; CIs from 1000 bootstrap resamples).*
#table_l1

#figure(image("l1_auc.svg", width: 100%), caption: [Held-out AUC per feature group with positive and permuted-label controls (run1 analysis).])
#figure(image("gain_vs_ceiling.svg", width: 100%), caption: [Held-out gain against the exact-label ceiling, split even_odd (run1 analysis).])

*L2 fixed cutoff and the positive control.*
#table_l2

*Size transfer (telemetry model trained on n15l5, scored on n17l6).*
#table_transfer

*L3 branching order (capped arms: values are lower bounds).*
#table_l3

#figure(image("l3_branching.svg", width: 100%), caption: [Summed refutation conflicts relative to default VSIDS (run1 analysis).])

*Raw records (SHA-256), paths under `run1/`.*\
`rows_n15l5.jsonl` 40f5fee112f8dd271dddd3477fa2a02bc29a8bc250c6f975a0149b0ca8b81eec\
`rows_n17l6.jsonl` b959864d7064a3965a2e4255e51c0b40378f1a942adbf74f79cf54d4ce188673\
`analysis/l3_n15l5.json` f789f371a7e5e440b1204e69247811d029e93fa104fd305e126bfeb87f5dfc4f\
`analysis/l3_n17l6.json` 5e731b6bd568ab7e1c2ce521903872263f388b180c7de45e0634d0b3bf370403\
`analysis/summary_n15l5.json` 9df7be93e8317c742915a2e55da44ac1699ddb378ec95eaa6f1b459c41f1e31e\
`analysis/summary_n17l6.json` a273bacdd546c5938337ea1f529c0722cf13237e64f15f1f4667e52ff2cd33d1\
`analysis/transfer.json` 6ba8f3fdf66110f79a990a1cde8a7c32b7ff5ad4b627c2947f970aeb59705626\

Observations only; the interpretation and the verdict under the frozen rules are in the study README.
