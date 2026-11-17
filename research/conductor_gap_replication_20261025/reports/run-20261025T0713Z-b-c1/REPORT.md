# Recorded run: b-c1

Session `ECBS1h0d3408524d88`; spec `ECS1h86118c087d71`. Source commit `68e6c8b12d9185824bc071d7bc29ce2c2b5a2372`. Public synthetic, independent one-target solves.

Requested 384; recorded 384. Statuses: `{'verified': 384}`. Required isolation L0; wall time is descriptive only.

| Arm | Attempts | Verified | Charged GAE sum (lower bound) | Peak RSS KiB |
|---|---:|---:|---:|---:|
| ic-recurrence | 128 | 128 | 8269945.0 | 147200 |
| ic-scalar | 128 | 128 | 8792185.0 | 147164 |
| rho-neg | 128 | 128 | 2183818.0 | 128044 |

![Native recorded means; no timing or speed ranking.](recorded-costs.pdf)

Raw records: `research/conductor_gap_replication_20261025/sessions/b-c1/records.jsonl`; spec and seeds: that session's `spec.json`. Host: `host.json`; exact replay: `research/conductor_gap_replication_20261025/sessions/b-c1.audit.json`. All per-curve 0/0.1/1 GAE bands, native counters, resource and failure totals: `analysis/diagnostics-B.json`. The plotted means are native ecbench summaries of verified records. Missing prices remain unknown. These observations alone admit no full-cost method ranking. Calibration sessions are excluded from primary inference.
