# XOR cache v2: completed execution

**Completed 2026-10-20 02:00:10 UTC.** All sixteen natural attempts
returned `UNKNOWN_search_cap` at exactly 10,000 conflicts with normal
exits and no OOM. The [native audit](final/audit.json) passes all 294
archived files and the four matched ABBA cells. Checked row visits fall
by 82.960–85.361%; the recorded L0 worker off/on ratios are
1.321160–1.345360. Natural yield/rank remain censored, and the cache
remains disabled by default. [Complete report](../reports/run-xor-cache-natural-20261020/RUN_REPORT.md).

The gated [full-base panel](../full-m4-v1/STATUS.md) is the subsequent
run. This completed card is immutable and must not be restarted.

## Historical live observation, superseded by the final audit

**Observed 2026-10-20 01:24:11 UTC: running.** Supervisor PID 57715 and
container `n83-xor-cache-K2048-t0-b2` were active on the authorized SSH
runner. This timestamped observation does not guarantee later liveness.
The card started at 01:16:34 UTC. Full retained observation:
[host-status.txt](observation-20261020T0124Z/host-status.txt).

- Four constructed controls and four separate general-field replays
  passed. Off/on complete SAT-model and row digests agree within each
  base. The [control report](../reports/run-xor-cache-controls-20261020/RUN_REPORT.md)
  covers those completed controls and both retained launch failures.
- Two of sixteen natural attempts completed: K2048 target 0, A1 and B1,
  both `UNKNOWN_search_cap` at 10,000 conflicts, 37,942 decisions,
  43 restarts, 101,087 XOR passes, 9,346,237 row operations and
  302,579,749 row scans. A1 records 59,469,058 repivot visits; B1 records
  8,705,524 visits and 50,763,534 cached skips. Their sum matches A1.
  This is one partial pair; no final comparison or improvement is selected.
- B2 was in progress at a 100,627.424549 ms sample, with 6,400 conflicts.
  A2 and the other three target/base cells remain scheduled by the same
  already-running shell card. Target 1 is the fresh m=4 holdout.
- Natural yield/rank and complete IC cost remain unresolved. Capped
  attempts retain their status and charged work; no zero-yield inference.

Run root: `/home/ec2-user/n83-m23-20261019/xor-cache-v2`.
Worker revision: `1de47a9d4186bff08abc84c224a111183e820106`.
Driver source revision: `f1667c313910a1021daed455c9e48cd03524f42a`.
Frozen [protocol](../XOR_CACHE_PROTOCOL.md),
[host card](../XOR_CACHE_HOST_CARD.md), and
[driver continuation](../XOR_CACHE_DRIVER_CONTINUATION.md).

On completion, collect a new deterministic archive of every attempt,
audit all final or capped progress and resource receipts, replay returned
relations, and compare each A1/B1/B2/A2 cell with exact work/model/row
checks. Preserve the earlier control/failure archives. Stop on incorrect
results or instrumentation; a Spot interruption or further retry requires
a new explicit continuation. Do not rerun into this directory.

At that earlier observation the SSH script continued independently of the chat. No automatic chat
check-in or PR-activity subscription is configured; none was available in
this session. The PR's catalog evidence gates remain separate blockers.
