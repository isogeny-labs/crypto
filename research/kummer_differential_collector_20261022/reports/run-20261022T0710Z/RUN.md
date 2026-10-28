# Recovered original paired Kummer run

Run `run-20261022T0710Z` began at a derived 07:10:45.527 UTC and ended
07:35:52.725 UTC on 2026-10-22. This is a retrospective run report made
after the Spot host migration. Its contemporaneous source is the
[recovery receipt](../../provenance/paired-run-recovery-20261022.json), which
pins the exact transcript event, command, working directory, exit status,
duration, executable hash, and source hashes. The original transcript is
preserved outside this repository on the retained source disk.

## Execution and observed result

The original command was
`research/kummer_differential_collector_20261022/target/release/paired_uncached`
from `/home/ec2-user/crypto-kummer-followup`. It exited 0 after
1,507.197243106 seconds and recorded only combined output:

```text
completed=1260 solved=1260
paired_rho=1260
```

The frozen inputs are the [theta models](../../../kummer_theta_20261022/results/theta-models.jsonl),
[coordinate target gates](../../../kummer_coordinates_20261022/results/),
and [source workload archive](../../../five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst).
The executable's SHA-256 is
`87d724fd89fa8d22c225eb0fae742c64d0c4d9378b7d4721752d0be9eb137b92`;
a locked offline rebuild at the saved source path reproduced the same bytes.
The original [seven candidate](../../results/paired/uncached/) and
[seven rho](../../results/paired/rho/) JSONL files are pinned by the study's
[SHA-256 manifest](../../SHA256SUMS). The independent Rust
[analyzer](../../src/bin/analyze.rs) checked 1,260 source scalar results,
recomputed the price grid, and wrote a replay certificate and validator
report. The repository [validation gate](../../VALIDATION.json) passes on
the materialized source inputs.

The original run did not capture separate stdout and stderr, a standalone
environment snapshot, user/system CPU time, or an earned isolation level.
The elapsed duration is execution context, not a comparative wall-time
measurement. The later [isolated full replay](../run-20261022T1354Z/RUN.md)
adds separate capture and reproduces all 14 raw files and five analysis
files byte for byte without changing the archived originals.

## Price-model result and visual

The [SVG graph](collector-cost-paired.svg) plots the archived
[whole-cohort price grid](../../results/paired/analysis/price-grid.csv)
over 1,260 public workloads. Its editable source is [plot.py](../../plot.py);
the adjacent PNG is a 150 dpi render of the sealed SVG. The full-cold
candidate/rho modeled GAE cost ratio is 8.959687–55.935424 across 36
pooled price cells. At `S/M=0.5`, `I/M=10`, and 0.1 GAE per native logical
operation, it is 14.356662. The candidate loses every pooled cell. The
original five-page [results PDF](../../KUMMER_PAIRED.pdf) gives the full
comparison; this report's [run PDF](run-report.pdf) embeds the graph and
execution provenance.

The later Lean theorem
[`KummerCost.pooled_loss_of_target_losses`](../../lean/CostAggregation.lean)
proves a conditional aggregation fact for natural-number price lists. It
does not import this run's floating-point CSV or prove the Kummer formulas.
The finite cohort and operation-price model remain the scope of the
empirical result. No practical-size cryptographic attack or wall-speed
advantage is established.
