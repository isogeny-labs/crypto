# Separately frozen 8 GiB repeats after both 4 GiB OOMs

Frozen before either v13 timed attempt. Original PROTOCOL.md and both v12
attempts remain immutable. Sourcef94d5da32ecd9b6a8956749978cd5a771482a0ed
v12 K2048/K4096 attempts executed exactly as specified; card checks verified
stream digests and actual verbatim relays. Both Docker final states record
OOMKilledtrue,exit137,cleanupverifiedtrue. Outer failed-attempt costs:
K2048 2856001.340487ms, K4096 2575759.292233ms. Native SAT-trace checks are
VALID_PREFIX_NONTERMINAL, terminal counters null, last progress789000/702000.
Neither is an admitted search result; zero models/relations/rank are not
inferred for killed prefixes. Native summaries preserve this distinction.

To complete the requested higher-cap sweep, separately repeat K2048 once,
assess, then K4096 once, each with explicit memory_limit_bytes8589934592
(8GiB), MemorySwap equal memory (no extra swap). Retain conflict800000,
workerwall4800s, command5400s, worker/query/panels/source9 files, image,
1CPUquota, strict fullcore14/30 runCPU14, networknone/read-only/capdropALL/
no-new-privileges/64PIDs, exclusive lock/quiet/evict-restore rules.
No heavy task work during measured arms. These are separate resource-envelope
observations, not same-memory repetitions or a speed ranking. New maximum
worker allocation9600seconds; cumulative19200across all4preregistered attempts.
Readiness/build/test/replay/render/storage are separate overhead. No automatic
retry; assess and preserve every failure before another new protocol.

Guard-v13 SHA256 cdfd8bf4dcc506c7f949bacbdafea0dc0506c72d0466cc639ad5a57ca599bf50.
The worker is unchanged, not rebuilt. Missing memory field defaults to4GiB.
Explicit8GiB is admitted only for natural K2048/K4096 at800000conflicts/
4800seconds; other budgets/control/external/arbitrary memory overrides reject.
Both Docker memory/swap and recorded worker cgroup limit must match the frozen
spec/config. Witness/external budgets, CPU and all existing audit gates remain.
OOM still refuses full search admission. Native summary can now label OOM
censors, preserving observed failed-attempt cost and null terminal counters;
its diagnostic passes only earlier config/spec/log custody and live isolation
checks plus native SAT-prefix syntax, not a full search/group/source replay.
Fresh guard21unit+7execution-card tests pass, including memory-boundary/swap
forgeries. Native integration against both actual v12 attempts reproduces
OOM_CENSORED_NOT_ADMITTED with null terminal counters and original prefix ends.
Preserve intermediate/final build/test logs and both immutable guard ELFs.

Commit clean guard/addendum/problem records before freezing fresh v13 specs
and write-once cards/paths. The native freeze protocol hash is this addendum;
card binds also retain the original protocol and prior attempts' assessments.
Director compile -> exact mechanical actual stdout -> director assessment at
unchanged HEAD, separately for each K. No overwrite or silent replacement.
All original correctness, identity/claim, inherited validation and report/S3
custody requirements remain in force. Current migrated-role S3 access denial
is separate: keep all bytes locally and attempt publication, never report an
upload or fresh GET as complete without actual successful receipts.

[Resource obstruction](../../../../problems/n83-native-800k-memory-limit.md).
Even complete 8GiB terminals would resolve only this bounded execution gap;
4GiB execution, useful rank, cold minimum runtime, rho comparisons and unknown
scalar recovery remain separate obligations. A further resource/launcher
failure requires a new discriminating protocol, not an automatic timed retry.
