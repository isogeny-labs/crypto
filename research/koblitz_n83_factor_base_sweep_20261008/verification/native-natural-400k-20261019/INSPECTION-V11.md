# Assessed inspection failure and separately frozen K4096 repeat

K2048-v10 completed400000conflicts, native bounded-zero replay passed and
exact card/relay/output digests passed director assessment. Preserve it;
do not rerun K2048.

K4096-v10's worker wrote a complete UNKNOWN_solver_cap summary reporting
400000conflicts, zero models/relations/rank, and a terminal solve-return phase.
Its launcher stopped at command1 because Docker inspect exceeded the original
15-second RPC wait and returned empty stderr. The mechanical command took
1598.372932784seconds. There is no audit.json, outer.json or final Docker
state record. Worker pre-summary wall1549664.280075ms is worker-reported,
not an admitted complete run time or proof of normal exit/no OOM. Keep its
actual summary, events, failed stdout/stderr, config/source/custody, immediate
cleanup snapshot, exact card/relay/receipt and director assessment unchanged.
No completed-run admission is fabricated from those partial orchestration
records. A later exact-name Docker ps query exited0 with empty stdout,
confirming absence; preserve that observation separately.

After director assessment, the reasoning tier explicitly decides on one
corrective K4096 repeat to obtain complete exit/OOM/custody audit under the
user's instruction to keep going. This is a documented additional timed
attempt, not an automatic or silent retry. New maximum cumulative worker
allocation7200seconds covers all3attempts (K2048 once, K4096 twice), instead
of the original4800seconds. Each worker still has400000conflicts/2400seconds.
New mechanical command ceiling3000seconds accommodates bounded inspection
and cleanup overhead; overhead is separate from solver time. No further
timed retry is preregistered. Any new failure must be assessed before a new
plan. All attempts and their costs remain visible; do not pool the repeated
worker as an independent query or count it as an extra successful workload.

Native guard-v11 SHA256
4c3d3f6658134d68da132b2efb0948195ef6b6031a40168dc875e25d946f765b
extends only the read-only Docker inspect RPC wait from15to60seconds.
Other operations retain15seconds; no Docker command is automatically retried.
Errors now include the process exit status and stderr, so a future timeout
is diagnosable. Worker/algorithm/query/input/budget/resource/CPU/quiescence
checks and native audit predicates remain unchanged. Native19unit+7card
tests pass before timing; the new policy test ensures the longer wait applies
only to inspect, never run/kill/rm/logs/ps/image operations.

Preserve the original immutable guard-v10 ELF and old frozen source/cards.
Commit this addendum and a clean source before fresh native spec/card freeze,
with the same worker and explicit full core14/30, fresh exec/k4096-v11 paths
and noclobber actual relay capture. Director compiles, mechanical executes,
director assesses. Source HEAD now identifies guard/readiness metadata; the
retained worker remains byte-identical. Native summary should label the
v10 failure LAUNCH_FAILED_NO_AUDIT, and only independently audited runs enter
the quantitative table. K4096 has two attempts with one orchestration failure,
not n=2 admitted repetitions. All raw records go to the authorized S3 bucket.
