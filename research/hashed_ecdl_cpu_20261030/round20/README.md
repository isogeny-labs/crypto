# Round20: matched CPU panel preparation

Status: **terminal scheduling timeout; zero executions**. The owner authorized
one 2-vCPU/4-GiB/300-second/zero-retry attempt and subsequently allowed normal
Spot-pool autoscaling. Job hashed-ecdl-cpu-round20-20261030 was submitted on
2026-11-01 at 03:50:31 UTC and reached DeadlineExceeded at 03:55:31 UTC;
terminal Failed was recorded at 03:55:32 UTC. No node was assigned, no worker
logs were returned, and the S3 listing contained no result objects. All 180
planned executions are missing. No timing or certificate result is claimed.

The exact request, admission, Job/Pod identities and terminal receipts are
retained here. Historical preparation/qualification records remain unchanged;
their earlier unsubmitted state is superseded by these execution receipts.
No retry or replacement was submitted. The current 10x goal keeps its original
baseline and all three fixtures.

The later [capacity diagnosis](capacity-diagnosis.md) identifies a one-node
cpu-spot cap in the control-plane audit records. The occupied node plus required
anti-affinity prevented another placement; nominal spare CPU did not override
the node-count limit. No cap was raised and no other job was changed.

## Planned contrasts

| Arm | Integrated source | Binary name | Batch | Purpose |
| --- | --- | --- | --- | --- |
| u | 2bd4588fba603ba1787e1d8ad82f084ef461d7cf | baseline | 64 | Original full-pipeline baseline |
| x | Same executable as u | baseline | 64 | A/A noise control |
| v | fab61dde9ada8fafcaa4d15af735f3b422e886d0 | candidate | 64 | All accumulated changes |
| w | Same executable as v | candidate | 1 | Batch scheduling control |
| y | a6117b9ec7743d879223806b1cc28556ba83903e | previous | 64 | Streamed matrix, before endpoint-product removal |
| z | 5cb7a4072078c0eadabd9aabfcc3be3baaa14d37 | inverse | 64 | Projective relation checks, before streamed matrix |

The historical basename inverse is only a packaging label: z is an integrated
revision, not the patched z control from round19. Candidate-versus-y isolates
batch endpoint products; y-versus-z isolates streamed incidence staging.
Candidate-versus-original baseline tests the accumulated implementation as a
whole and does not attribute an effect to one constituent change. No unrelated
upstream source changes may enter either component contrast.

## Fixed scope and accounting

Keep the exact three public-point fixtures and seeds 111, 222 and 333 from
round19/run.sh and its complete-example input contract. Keep one warmup and
nine measured repetitions for every arm/fixture cell, totaling 180 executions
(18 warmups, 162 measured). Preserve deterministic interleaving with alternating
reversal, one software thread, max_trials=16 and max_work=1,000,000.

The complete cost boundary includes curve/target construction, every trial's
setup including discarded trials, factor bases, relation generation, reduction,
internal verification, independent replay and certificate serialization. No
cache may cross the two replays or process invocations. Preserve all counts,
full certificates and failures. No row may be excluded after seeing its time.
Known scalars construct fixtures; the solver receives public points. The
positive scalar/replay and corrupted-scalar negative controls remain mandatory.

The retained native analyzer requires 180 complete rows, all 18 cells and full
certificate equality across every arm. No existing result rows are pooled into
this panel. Freeze the final analysis source/hash before any measurement.

## Execution and acceptance dependencies

A valid diagnostic execution still needs admission and an eligible bounded
placement. The protocol now passes the existing Rust canonical schema and
semantic checker, rebuilt from commit dac149f065de. The freeze wrapper uses
that library's canonical hash, validates before and after freezing, and creates
output exclusively. No Python checker is invoked. Native export now explicitly
supports round20; its synthetic tests check job identity and reject unknown
study names. Do not rename a study to bypass the supported-study guard.

This design does not choose a paid host or submit a job. The previously
approved single cloud attempt was consumed by round19. Its frozen deadline
must not be extended or reused. A future diagnostic envelope may retain
2 vCPUs, 4 GiB, 300 seconds and zero retries only after separate admission.
L0 timing is diagnostic and cannot establish the 10x acceptance criterion.
Actual 10x acceptance requires the protocol's isolated timed-primary gate,
matching A/A evidence and complete-cost measurement across all three fixtures;
no lower claim is substituted for that objective.

## Reproducible software artifacts

prepare.sh is thin shell build/test orchestration only. It materializes exact
Git snapshots, builds with locked offline dependencies and runs native software
correctness tests. It never invokes the benchmark executable. Its required
argument is a fresh artifact directory; existing output is refused by mkdir.

Local artifacts: /home/ec2-user/hashed-ecdl-cpu-round20/. Executables remain
outside Git; binaries.sha256 and materialization/source manifests bind them.
The original baseline, pre-streaming candidate and analyzer are copied from
the preserved round19 bundle and checked against its committed hashes.
Tests/build logs identify the fresh source snapshots; software-test runtimes
are not performance measurements.

Preparation references: round19's frozen contract and terminal outcome,
streamed-matrix/README.md, batch-endpoints/README.md, and the retained native
analyzer correctness tests. This is a matched engineering comparison, with
no new mathematical algorithm or novelty claim. T-1027 owns preparation only.

## Native qualification (T-1028)

Protocol SHA-256: `2b263251e9c81fb7f7da12e4a3387e9f1f5c44529f5977a49a90553904017b8b`.
Both validation and freeze receipts retain launch_authorized=false. The earlier
readiness.json records the historical pre-freeze preparation; qualification.json
records the later current state. Historical draft and preparation records remain.

The checker source snapshot is pinned by protocol-checker-source.sha256 and its
binary by protocol-tools.sha256. The existing implementation is under
research/pencil_large_prime_20261029/native_canonical_export; only its generic
protocol validation/hash functions are reused, not its experiment-specific
exporter. The schema and supported semantic rules are checked. Unsupported
blinding is explicitly false in this new panel; neutral labels are not human
blinding. prepare-protocol logic is retained as protocol-draft.jq.

Seven synthetic negative checks reject altered frozen contents, unknown arms,
duplicate identities, unsupported blinding, changed analysis source, refreezing,
and overwrites. Twelve synthetic native analyzer/export tests pass for round20
and round18; the historical parity test remains an explicit ignored test in
these invocations. Strict native-tool Clippy, formatting and CLI build pass.
These are software checks and generate no solver timing data.
