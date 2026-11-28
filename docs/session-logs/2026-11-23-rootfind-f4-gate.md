# 2026-11-23: program B matched F4 gate (root-finding follow-up)

## 21:20Z: matched F4 gate published

### What was done
- A workflow worker froze `PROTOCOL.md` and built two F4 arms (`f4_fp::solve_visit`) through a shim crate.
- It ran a 20-chart panel at p = 4294967291, M = 128, 3 reps, plus a post-hoc M ladder.
- A red-team review raised four blocking items, all confirmed. A fix-up pass then:
  - relabelled F4-visit as an (M+1)^2 enumeration;
  - added `scripts/cubic_check.py`, which shows that M < p^(1/3) is solved exactly by an O(M) integer cubic scan;
  - relabelled the M^8/p^2 scaling as a model (measured M^5.5–5.9);
  - re-read the gate as FAIL by the frozen rule but inconclusive.

### Decisions made
- No 2x/8x claim survives, and the panel cannot characterise program B on hard instances `[pcc:unverified]`. No leaderboard or benchmarks rows.
- The worker's "base does not compile" deviation applied to its base 71a7881ede. Current main has already repaired `prime_root_trace.rs` (e9a30c9e8c; braces balance), so no separate fix PR is needed.

### Open threads
- A timed O(M) cubic-scan arm.
- Matched arms at M > p^(1/3), and more than one p.
- Freeze 20 genuine hard instances.

### Errors
- The freeze is unverifiable: the hash file was written after the data and was not committed.
