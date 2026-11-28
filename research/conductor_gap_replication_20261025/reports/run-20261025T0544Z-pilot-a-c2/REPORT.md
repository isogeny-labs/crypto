# Recorded run: pilot-a-c2

Session `ECBS1hd307604b8c73`; spec `ECS1ha57bcbf15add`. Source commit `45def5ca7a5546113bc2fb47b44c43138323ac1d`. Public synthetic, independent one-target solves.

Requested 48; recorded 48. Statuses: `{'verified': 48}`. Required isolation L0; wall time is descriptive only.

| Arm | Attempts | Verified | Charged GAE sum (lower bound) | Peak RSS KiB |
|---|---:|---:|---:|---:|
| bsgs-neg | 16 | 16 | 94056.0 | 128036 |
| rho-neg-aa | 16 | 16 | 130339.0 | 128044 |
| rho-plain | 16 | 16 | 148242.0 | 127848 |

![Native recorded means; no timing or speed ranking.](recorded-costs.pdf)

Raw records: `research/conductor_gap_replication_20261025/sessions/pilot-a-c2/records.jsonl`; spec and seeds: that session's `spec.json`. Host: `host.json`; exact replay: `research/conductor_gap_replication_20261025/sessions/pilot-a-c2.audit.json`. All per-curve 0/0.1/1 GAE bands, native counters, resource and failure totals: `analysis/diagnostics-calibration.json`. The plotted means are native ecbench summaries of verified records. Missing prices remain unknown. These observations alone admit no full-cost method ranking. Calibration sessions are excluded from primary inference.
