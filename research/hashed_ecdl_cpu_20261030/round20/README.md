# Round20: matched CPU panel preparation

Status: **software preparation and design only; not frozen, not admitted and
not executed**. This directory contains no launch command. It does not authorize
a replacement for the terminal round19 attempt. The current 10x goal keeps its
original baseline and all three fixtures.

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

A valid diagnostic execution still needs a new experiment.protocol/v1 record,
native schema and semantic validation, an immutable pre-data freeze, admission,
and an explicit bounded placement. The old Python protocol checker is not an
active path under AGENTS.md. The current Rust exporter supports only round18
and round19 and must explicitly support/test the new panel before use; do not
rename a study to bypass that guard.

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
