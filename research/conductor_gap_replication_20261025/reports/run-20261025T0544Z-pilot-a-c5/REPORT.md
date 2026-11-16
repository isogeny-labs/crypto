# Recorded run: pilot-a-c5

Session `ECBS1h471802fb4f6c`; spec `ECS1hb04ad99a0861`. Source commit `45def5ca7a5546113bc2fb47b44c43138323ac1d`. Public synthetic, independent one-target solves.

Requested 48; recorded 48. Statuses: `{'verified': 48}`. Required isolation L0; wall time is descriptive only.

| Arm | Attempts | Verified | Charged GAE sum (lower bound) | Peak RSS KiB |
|---|---:|---:|---:|---:|
| bsgs-neg | 16 | 16 | 890052.0 | 129064 |
| rho-neg-aa | 16 | 16 | 831127.0 | 128040 |
| rho-plain | 16 | 16 | 1300808.0 | 127848 |

![Native recorded means; no timing or speed ranking.](recorded-costs.pdf)

Raw records: `research/conductor_gap_replication_20261025/sessions/pilot-a-c5/records.jsonl`; spec and seeds: that session's `spec.json`. Host: `host.json`; exact replay: `research/conductor_gap_replication_20261025/sessions/pilot-a-c5.audit.json`. All per-curve 0/0.1/1 GAE bands, native counters, resource and failure totals: `analysis/diagnostics-calibration.json`. The plotted means are native ecbench summaries of verified records. Missing prices remain unknown. These observations alone admit no full-cost method ranking. Calibration sessions are excluded from primary inference.
