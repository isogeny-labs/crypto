# Recorded run: b-c0

Session `ECBS1h9790b324db76`; spec `ECS1h4bc5fa67549d`. Source commit `68e6c8b12d9185824bc071d7bc29ce2c2b5a2372`. Public synthetic, independent one-target solves.

Requested 384; recorded 384. Statuses: `{'verified': 384}`. Required isolation L0; wall time is descriptive only.

| Arm | Attempts | Verified | Charged GAE sum (lower bound) | Peak RSS KiB |
|---|---:|---:|---:|---:|
| ic-recurrence | 128 | 128 | 8368988.0 | 147168 |
| ic-scalar | 128 | 128 | 8937308.0 | 147168 |
| rho-neg | 128 | 128 | 2243023.0 | 128040 |

![Native recorded means; no timing or speed ranking.](recorded-costs.pdf)

Raw records: `research/conductor_gap_replication_20261025/sessions/b-c0/records.jsonl`; spec and seeds: that session's `spec.json`. Host: `host.json`; exact replay: `research/conductor_gap_replication_20261025/sessions/b-c0.audit.json`. All per-curve 0/0.1/1 GAE bands, native counters, resource and failure totals: `analysis/diagnostics-B.json`. The plotted means are native ecbench summaries of verified records. Missing prices remain unknown. These observations alone admit no full-cost method ranking. Calibration sessions are excluded from primary inference.
