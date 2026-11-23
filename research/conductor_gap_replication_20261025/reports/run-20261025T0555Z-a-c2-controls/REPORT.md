# Recorded run: a-c2-controls

Session `ECBS1hf5b424776547`; spec `ECS1h282a3e20106d`. Source commit `68e6c8b12d9185824bc071d7bc29ce2c2b5a2372`. Public synthetic, independent one-target solves.

Requested 1024; recorded 1024. Statuses: `{'verified': 1024}`. Required isolation L0; wall time is descriptive only.

| Arm | Attempts | Verified | Charged GAE sum (lower bound) | Peak RSS KiB |
|---|---:|---:|---:|---:|
| rho-neg-aa | 512 | 512 | 3912873.0 | 128064 |
| rho-plain | 512 | 512 | 5496831.0 | 127916 |

![Native recorded means; no timing or speed ranking.](recorded-costs.pdf)

Raw records: `research/conductor_gap_replication_20261025/sessions/a-c2-controls/records.jsonl`; spec and seeds: that session's `spec.json`. Host: `host.json`; exact replay: `research/conductor_gap_replication_20261025/sessions/a-c2-controls.audit.json`. All per-curve 0/0.1/1 GAE bands, native counters, resource and failure totals: `analysis/diagnostics-A.json`. The plotted means are native ecbench summaries of verified records. Missing prices remain unknown. These observations alone admit no full-cost method ranking. Calibration sessions are excluded from primary inference.
