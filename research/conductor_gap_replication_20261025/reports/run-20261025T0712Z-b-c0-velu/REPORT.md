# Recorded run: b-c0-velu

Session `ECBS1h102f9bd15c62`; spec `ECS1hed84b6e0e232`. Source commit `68e6c8b12d9185824bc071d7bc29ce2c2b5a2372`. Public synthetic, independent one-target solves.

Requested 32; recorded 32. Statuses: `{'verified': 32}`. Required isolation L0; wall time is descriptive only.

| Arm | Attempts | Verified | Charged GAE sum (lower bound) | Peak RSS KiB |
|---|---:|---:|---:|---:|
| ic-velu | 32 | 32 | 1975363.0 | 147256 |

![Native recorded means; no timing or speed ranking.](recorded-costs.pdf)

Raw records: `research/conductor_gap_replication_20261025/sessions/b-c0-velu/records.jsonl`; spec and seeds: that session's `spec.json`. Host: `host.json`; exact replay: `research/conductor_gap_replication_20261025/sessions/b-c0-velu.audit.json`. All per-curve 0/0.1/1 GAE bands, native counters, resource and failure totals: `analysis/diagnostics-B.json`. The plotted means are native ecbench summaries of verified records. Missing prices remain unknown. These observations alone admit no full-cost method ranking. Calibration sessions are excluded from primary inference.
