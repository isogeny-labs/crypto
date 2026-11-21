# XP-20261029-dg-blocked: verdict (§6.2)

Frozen protocol: `backlog/XP-20261029-dg-blocked/PROTOCOL.md` (commit cd04cfd2d1df), with the Coordinator ruling `cards/RULING-P2.md` (condition A1). Every number below is copied from the tool-written files named in each row; this file computes nothing.

Candidate `dgb` = `mitm-prime-blocked:m=2` (PackedPairTable, cache-blocked build); baseline `dg` = `mitm:negation_folded=1`; both `targets=stride-renew-deferred-guard` at the incumbent factor base. Decision statistic: `wall.corrected_ratio` of `compare --a dg --b dgb --weight 0` (= Σ wall(dgb) / Σ wall(dg-aa) per charged GAE).

| b | wall-session §6.1 (1–9) and A1 | `wall.corrected_ratio` dgb / dg | `wall.corrected_ci95` | §6.2 reading | source |
|--:|:--|:--|:--|:--|:--|
| 24 | fail: 1, 5, 6, 7, 9 (2 of 1344 executions L1, solves under 1 ms) | 0.9074948190706239 | [0.9044866448552347, 0.9105129979800146] | undecidable (descriptive only) | `P24/wall/analysis/compare-dg__dgb-w0.json`, `wall_gate.json` |
| 28 | hold | 0.9299532537640001 | [0.926605401878962, 0.9333132015338899] | **improvement** | `P28/wall/analysis/compare-dg__dgb-w0.json`, `wall_gate.json` |
| 32 | fail: 1, 5, 6, 7, 9 (1 of 1344 executions L1: `dg-aa` seq 1313, 3 foreign task slices) | 0.8974834510901225 | [0.8938669791946178, 0.9011145547700823] | undecidable (descriptive only) | `P32/wall/analysis/compare-dg__dgb-w0.json`, `wall_gate.json` |

Counted sessions (`P<b>/counted/analysis/counted_gate-stdout.txt`): `EXIT42_CLAUSE=true` at 28 and 32 bits; `false` at 24 bits (one counted `dgb` execution was a sub-millisecond IC solve). Charged work is identical between `dg` and `dgb` record by record at every size (condition 8), so the counted S of the two arms is the same and no counted ranking changes.

P24 and P32 are not rerun (APPROVAL.md "Residual risks"). The P28 improvement is a wall-clock component speed-up of the oracle (ns per charged GAE), not a counted improvement; its leaderboard placement is a component section outside tables A–C.
