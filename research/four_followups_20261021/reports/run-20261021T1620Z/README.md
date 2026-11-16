# Point-driven pencil: completed four-model pilot

- Run ID: `20261021T1620Z-pencil-pilot`.
- Cohort: one 16-bit and three 20-bit original exact-order models, 60 frozen public targets per model. The full 21-model, 1,260-target requirement remains open.
- Input: `research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst`, SHA-256 `550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339`.
- Implementation: `src/bin/pencil_full.rs` extends the source-checked F739 point-driven sieve with a sparse modular row collector. Its initial F739 port reproduced all 61 earlier setup/target records after excluding wall-clock fields; the sparse pilot then ran the original cohort inputs.
- Raw: `results/pencil-pilot/f{739,3371,3527,3931}-cohort.jsonl`, one setup row and 60 target rows per curve. All 240 candidate and 240 same-process source-negation-rho scalars were independently verified by source multiplication.
- Replay: the four matching files under `results/pencil-pilot/replay/` agree on every non-wall field. `validate-pencil` checked the fresh scalars against the committed input archive, paired rho counts with the frozen controls, and recomputed 12 representative cost values.
- Pricing: `results/pencil-pilot/price-grid.csv` has 36 cells per curve. Full setup is charged to each cold one-target solve. At `M+0.5S+10I` and 0.1 GAE per native operation, pencil/rho is 12.931178, 246.117110, 267.374941 and 332.187349 in ascending p order.
- Host: Intel Xeon 6975P-C, 32 visible CPUs, L0 shared host. Wall time is diagnostic and excluded from the ratios. One algorithm seed per target; no uncertainty interval.

The [PDF](report.pdf) and [graph](pencil-pilot-cost.svg) render the recorded costs and row-update volume. The validator and replay certificate are in `results/pencil-pilot/`; the [data dictionary](../../DATA_DICTIONARY.md) defines their fields and the pricing treatment.
