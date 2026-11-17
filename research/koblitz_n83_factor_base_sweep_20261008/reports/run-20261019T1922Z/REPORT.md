# Native N83 200000-conflict continuation

Requested: update PR2036 with the approved description, raise the conflict
cap again for K2048/K4096, continue native search and upload all raw evidence
to setec-astronomy-artifacts. The approved description update succeeded.

[Frozen protocol](../../verification/native-natural-200k-20261019/PROTOCOL.md)
raises each cap100000→200000, with1200seconds per worker and1290seconds per
mechanical command; maximum new worker allocation2400seconds. Historical
1000/10000/100000-conflict trials remain immutable, separate observations.

Public synthetic a=0 `icv1-f2m83-tm6151469093347-debefd74` over
x^83+x^45+x²+x+1; subgroup order2417851639230796216685689. The positive-trace
a=1 cold N83 holdout is distinct; no EC1 equivalence is invented. Exactly
one retained public query, public_x_hash seed2026100801, m5, identitymask0,
and the same retained K2048/K4096 public inputs.

## Guard and placement correction

Source914b0fba02 and immutable guardv7 extend natural admission to200000
conflicts/1200s. Witness100000/900 and external1000000/900 limits stay
unchanged. Native budget tests reject zero/above-boundary values;16unit and
7card tests passed. K2048v7 stopped before solver launch: full core15/31 was
unavailable because a peer container restricted its threads to busy CPU31.
The actual refused relay, receipt, stderr, isolation sample, director
assessment and diagnosis remain in the raw archive. No timed solve was retried.
The superseded K4096v7 card was never executed.

[Separate placement addendum](../../verification/native-natural-200k-20261019/RESERVATION-V8.md)
freezes source7f07562459 and immutable guardv8 SHA256
2ec47f22528893777941de5dc4a68e8b2ac02d920c87449ed835137ddb2c0273.
An optional ascending CPU-ID array now exposes existing strict whole-core
planning. Empty, unordered, duplicate, negative or non-array requests fail;
existing topology/cgroup/SMT-sibling checks remain. Both fresh specs reserve
full core14/30. No peer's fixed cpuset is changed or its job killed. Lock,
eviction/restoration, quiet thresholds and readiness bounds are unchanged.
Guard17unit+7card tests pass. Worker/algorithm/inputs/budgets are unchanged.
Placement differs from historical observations, precluding timing pooling.

## Validation and measurement limits

Retained worker SHA256
3a31bd2bedee5519d30b517f39ec27f72561ce898213657688c55b51bbcdf7b6
was not rebuilt. All9watched solver/encoding/example/adapter sources match
source660f0f4b10; new HEADs identify guard metadata. Full relevant source,
root/standalone manifests and lockfiles, exact ELFs, commands and public
inputs are retained. Native library4398pass/118ignored validation is
inherited. Post-base-merge full serial run had4398pass/9socket-denied/118ignored;
elevated affected-module rerun38pass covers all9denials, exporter19 and model
checker2pass. Both failed and successful raw logs are preserved. These are
not a newly clean single full-suite run; no library change was made here.

Immutable Docker image541096ae9b05…,1CPU quota/4GiB/noextra swap,
networknone, readonlyroot/source/input/worker,capdropALL,no-new-privileges,
64PIDs. Actual inspect, CPU reservations/conditions/cleanup and every raw
stream/phase/query/config/audit are retained. Fixed kernel threads and
missing worker context-switch/fault counters prevent automatic L2 admission.
No builds/tests/heavy work from this task occurs during a measured arm.

Native query replay checks aG+bQ over the original field/group, phase order,
input/source/ELF/stream custody, budgets and exit/OOM/cleanup. Positive natural
models remain fail-closed until independent original-group/rank/scalar
verification supports them. Unknown is bounded, not UNSAT or absent
relations. No cold winner, best base or unknown-scalar recovery follows.

Native Rust derives solver interval as return.elapsed_ms-enter.elapsed_ms.
Outer wall includes setup/supervision and is separately recorded. These are
n=1 per K/cap with no A/A interval and no admitted time/base ranking.
CryptoMiniSat5.17.0 remains faster on the separate verified witness corpus,
with different reported algorithm-CPU versus native-wall timers, as retained
in the upstream XOR study. No fresh matched external comparison is attempted
for these capped unknown natural queries; no fastest-tool claim follows.

## Visuals and publication

Same recorded data generates [SVG](cap-continuation.svg) and150dpi
[PNG](cap-continuation.png); [execution diagram](execution.svg),
[editable render source](render.py), [Typst](report.typ), [PDF](report.pdf).
Python only lays out recorded values and inspects PDFs; research, replay and
derived values remain native. Benchmark copies are generated from these files.

Canonical graphs checked and left unchanged: docs/ic-leaderboard.html and
its twins, docs/index-calculus-scoreboard.html, docs/ic/progress-timeline.json,
docs/curves identity/route diagrams and docs/performance-gains. This budget
extension introduces no solver improvement, verified scalar recovery,
matched-rho result or admitted cold-row change. The new local cap graph
records the bounded continuation without changing those claim gates.

[Native summary](../../verification/native-natural-200k-20261019/SUMMARY.json),
[cards/assessments](../../verification/native-natural-200k-20261019/cards/),
[compact records](../../verification/native-natural-200k-20261019/records/),
[raw S3 publication and fresh replay](../../verification/native-natural-200k-20261019/PUBLICATION.md).

## Preserved Docker launch failure

The first v8 K2048 attempt passed strict CPU14/30 readiness, then the
15-second Docker RPC limit expired during container creation. No timed
worker launch is established by that attempt. The immediate failure-cleanup
snapshot reported absence, but a later read found the delayed container in
Created state, Runningfalse, Pid0, StartedAt allzeros. The full later inspect
is preserved; root removed only this task's orphan and confirmed absence.
The immediate snapshot is not a durable cleanup assertion. A separately
bounded, network-disabled/read-only /bin/true readiness check on the same
image passed before fresh v8b planning. Failed raw streams, specs, cards and
assessments stay immutable; no recorded solver work is erased or retried.

## Analysis classification correction

The first summary draft's native helper treated any isolation-before file
without outer.json as a preflight refusal. The v8 Docker launch failure had
passed preflight, so that draft label was inaccurate. A native analysis-only
v9 correction now prioritizes launcher-error.json as LAUNCH_FAILED_NO_AUDIT,
and requires actual failed quiet/eviction observations before labeling a
preflight refusal. Completed-run calculations and audits are unchanged.
The native regression test covers passed-preflight launch failure, actual
preflight refusal, incomplete unknown failure, outer-record presence and an
unattempted run. Analysis18unit+7card tests pass. Both measured worker and
measured guard-v8 remain byte-identical; fresh audit replay uses guard-v8.
Earlier draft summaries are retained in the raw archive as superseded drafts,
not silently used as final outcome evidence. The final table lists only
completed audited runs; both setup failures are described separately above.

| requested deliverable | status | evidence |
|:--|:--|:--|
| Approved PR2036 description | verified complete | exact approved body applied before new runs |
| Raise caps for both bases and run native sweep | verified complete | both terminal counts200000; zero models/relations/rank; native audits/director checks pass |
| Preserve all failures and raw evidence | verified complete locally | exact streams/cards/specs, v7 reservation and v8 Docker launch failures retained |
| S3 publication and fresh replay | publication record | PUBLICATION.md supplies content addresses and final checks |
| Useful natural relations, best base, scalar recovery, cold IC/rho ratio | unresolved | no natural model or relation at these finite caps |

## Audited recorded outcomes

| run | native status | terminal conflicts | models | relations | rank | native solver ms | supervised outer ms |
|:--|:--|--:|--:|--:|--:|--:|--:|
| k2048-200k | PASS_NATIVE_BOUNDED_ZERO_RELATION_REPLAY | 200000 | 0 | 0 | 0 | 759947.84721 | 765409.242176 |
| k4096-200k | PASS_NATIVE_BOUNDED_ZERO_RELATION_REPLAY | 200000 | 0 | 0 | 0 | 786700.6455450001 | 797133.398274 |
