# Recorded run: pilot-b-c2

Session `ECBS1hc8a4f288bb5b`; spec `ECS1h356a6213e4f5`. Source commit `45def5ca7a5546113bc2fb47b44c43138323ac1d`. Public synthetic, independent one-target solves.

Requested 16; recorded 16. Statuses: `{'verified': 16}`. Required isolation L0; wall time is descriptive only.

| Arm | Attempts | Verified | Charged GAE sum (lower bound) | Peak RSS KiB |
|---|---:|---:|---:|---:|
| ic-recurrence | 4 | 4 | 375621.0 | 147040 |
| ic-scalar | 4 | 4 | 394341.0 | 147000 |
| ic-velu | 4 | 4 | 372417.0 | 147040 |
| rho-neg | 4 | 4 | 143513.0 | 128040 |

![Native recorded means; no timing or speed ranking.](recorded-costs.pdf)

Raw records: `research/conductor_gap_replication_20261025/sessions/pilot-b-c2/records.jsonl`; spec and seeds: that session's `spec.json`. Host: `host.json`; exact replay: `research/conductor_gap_replication_20261025/sessions/pilot-b-c2.audit.json`. All per-curve 0/0.1/1 GAE bands, native counters, resource and failure totals: `analysis/diagnostics-calibration.json`. The plotted means are native ecbench summaries of verified records. Missing prices remain unknown. These observations alone admit no full-cost method ranking. Calibration sessions are excluded from primary inference.
