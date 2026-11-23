# Recorded run: a-c0-controls

Session `ECBS1hc7066408c7dd`; spec `ECS1h16432274c6a3`. Source commit `68e6c8b12d9185824bc071d7bc29ce2c2b5a2372`. Public synthetic, independent one-target solves.

Requested 1024; recorded 1024. Statuses: `{'verified': 1024}`. Required isolation L0; wall time is descriptive only.

| Arm | Attempts | Verified | Charged GAE sum (lower bound) | Peak RSS KiB |
|---|---:|---:|---:|---:|
| rho-neg-aa | 512 | 512 | 2277837.0 | 128044 |
| rho-plain | 512 | 512 | 3286503.0 | 127916 |

![Native recorded means; no timing or speed ranking.](recorded-costs.pdf)

Raw records: `research/conductor_gap_replication_20261025/sessions/a-c0-controls/records.jsonl`; spec and seeds: that session's `spec.json`. Host: `host.json`; exact replay: `research/conductor_gap_replication_20261025/sessions/a-c0-controls.audit.json`. All per-curve 0/0.1/1 GAE bands, native counters, resource and failure totals: `analysis/diagnostics-A.json`. The plotted means are native ecbench summaries of verified records. Missing prices remain unknown. These observations alone admit no full-cost method ranking. Calibration sessions are excluded from primary inference.
