# Recorded run: pilot-b-c0

Session `ECBS1h68bbf035b472`; spec `ECS1hcff7b72b25e6`. Source commit `45def5ca7a5546113bc2fb47b44c43138323ac1d`. Public synthetic, independent one-target solves.

Requested 16; recorded 16. Statuses: `{'verified': 16}`. Required isolation L0; wall time is descriptive only.

| Arm | Attempts | Verified | Charged GAE sum (lower bound) | Peak RSS KiB |
|---|---:|---:|---:|---:|
| ic-recurrence | 4 | 4 | 257047.0 | 147164 |
| ic-scalar | 4 | 4 | 274807.0 | 147036 |
| ic-velu | 4 | 4 | 254811.0 | 146780 |
| rho-neg | 4 | 4 | 53214.0 | 127656 |

![Native recorded means; no timing or speed ranking.](recorded-costs.pdf)

Raw records: `research/conductor_gap_replication_20261025/sessions/pilot-b-c0/records.jsonl`; spec and seeds: that session's `spec.json`. Host: `host.json`; exact replay: `research/conductor_gap_replication_20261025/sessions/pilot-b-c0.audit.json`. All per-curve 0/0.1/1 GAE bands, native counters, resource and failure totals: `analysis/diagnostics-calibration.json`. The plotted means are native ecbench summaries of verified records. Missing prices remain unknown. These observations alone admit no full-cost method ranking. Calibration sessions are excluded from primary inference.
