# Run report: `e7a_h4` (2026-10-18T16:05:24Z to 2026-10-18T16:06:38Z UTC)

Descriptive record of one `ecbench` session.  Interpretation is in the
round's [README](../../README.md).  PDF: [`report.pdf`](report.pdf).

| item | value |
|:--|:--|
| protocol | [`PROTOCOL.md`](../../PROTOCOL.md), E7 |
| spec | [`specs/e7a_h4.json`](../../specs/e7a_h4.json) |
| command | `ecbench run --wait`, then `ecbench verify --replay 12` ([`run_sessions.sh`](../../run_sessions.sh)) |
| session | `ECBS1h54f0529e1598`, git commit `05ef9aff3`, `git_dirty` false |
| host | Claude Code cloud container; isolation earned per record: L2 on 395, L1 on 85 (the spec asked for L2) |
| runs | 480 requested, 480 completed, 480 verified |
| audit | [`sessions/e7a_h4.audit.json`](../../sessions/e7a_h4.audit.json): ok, 12 replays reproduced, 0 problems |
| records | [`sessions/e7a_h4/records.jsonl`](../../sessions/e7a_h4/records.jsonl) |
| per-curve values | [`TABLES_E7.md`](../../TABLES_E7.md) (from `examples/isogeny_gap_round5_e7_tables.rs`) |

The measured values are each arm's mean `S` (`ecbench.gae`, a lower
bound; table lookups are unpriced), 6 runs per arm and curve.  The graph
is [`s_by_arm.svg`](s_by_arm.svg), from [`plot.py`](plot.py).  No wall
times are reported.
