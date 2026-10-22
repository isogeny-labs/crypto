# Periodic qualification B — recovery passed, benchmark rejected

**Overall result: failed.** The frozen protocol required two committed
generations, restored computational progress, 3,606 fresh L2 observations,
3,600 native replays, admitted A/A and 19 S3 payload hashes. Recovery and replay
passed; the fresh epoch had 3,603 L2 and 3 L1 observations. L2 admission exited
42 before A/A evaluation, leaving 18 payloads. All 18 downloaded hashes matched.
No measurement was retried. This is infrastructure correctness/isolation
evidence, not an algorithm speed comparison or a full GitHub Action result.

## Frozen execution

Run: `ecbench-periodic-20261022-b`. The public 32-bit fixture used curve seed
59297, two targets with seed 20261002, `rho.negation`, `rho.plain` and a matching
negation A/A arm; 600 rounds plus one warmup, alternate order, seed 1. Each
operation had a 120-second timeout. Checkpoints were requested every 30 seconds,
with at most two restores and an original 3,600-second deadline ending at Unix
1792650847. The 96-vCPU regional quota and 64-vCPU lab cap were retained.

Controller/runtime source: `5499eeb9d72e6e777cb0d28b0ade41afbfff90b7`.
Crypto source: `cb2ec6873f30b683f7885b53c410d69965f02c20`.
Native binary SHA-256:
`583e62f0836eae43330c4b36bbe3d145dacb01de3132436c71f6924445c6a5a9`.
Runtime image:
`sha256:0ecb5bf15e2f22b8d12242c3cb306735b1e44e696c698f2dade470b517869dd9`.
The full immutable source/archive/image/request identities are in
[`protocol.json`](protocol.json), frozen before submission. Its original
acceptance criteria have not been changed after observing the result.

## Observed recovery

| Check | Observed result |
| --- | --- |
| Successive committed generations | g1 `snap-0d025b1c76948a8a5`, g2 `snap-042f5c51ec9f5df38`; complete, encrypted, full archive readback verified |
| Original worker and controller fence | `i-0ae9215e018bebc9a` terminated after its own controller was STOPPED and its Pods were checked for unrelated work |
| Replacement worker | `i-085d4c562929528bc`, c8a.2xlarge; actual fresh host had 8 logical CPUs, 8 physical cores and no SMT siblings |
| Restoration | attempt 1, generation 2, one restore; same request and original deadline |
| Preserved progress | first 2,486 records / 11,113,922 bytes identical; final 3,606 records |
| Native computation verification | 3,606 verified records, 3,600 matching replays, zero problems; all computational timing invalid |
| Restored credentials | PIDs 531 and 10617 stopped; UID/GID 1000, empty groups, zero capabilities, no_new_privs 1, Seccomp 2 |

The generation-intent repair is exercised by two actual committed generations.
Both admitted captures succeeded on their first capture, so B does **not** prove
the live zombie-recapture path. That branch has native real-zombie regression
coverage. Replacement briefly encountered Spot capacity/quota lag; it recovered
within the original deadline without terminating other consumers or raising caps.

## Fresh epoch and failure

Job: `ecbench-periodic-20261022-b-epoch-1`.
Job UID: `0e1512e7-e739-4bbb-b1fa-8163f9d8f2df`.
Pod UID: `fb1260f3-d86c-44c7-ae62-c5506963d3cb`.

The distinct fresh epoch completed 3,606 records; 3,600 native replays matched,
with no invalid mathematical outcome. Its unchanged L2 gate rejected sequences
676 (`rho-plain`), 1406 and 1826 (`rho-neg-aa`). Each recorded three excess busy
ticks on run CPU 4, zero steal, zero idle-sibling busy ticks and affinity `[4]`.
Their process CPU/wall times were approximately 20–25 ms, while the surrounding
per-CPU counter increased by five ticks. These observations do not identify a
foreign task or establish a kernel-accounting defect. The original worker has
since consolidated; no trace was collected during those windows. No threshold,
record, source binary or admission result was changed.

[`l2-gate.json`](l2-gate.json) contains every rejection and its counters.
A/A is **not evaluated**, not passed or failed: execution stopped at the L2 gate.
The controller's terminal state is failed with
`fresh benchmark epoch failed; measurement is not retried`.
The native client exited 1; the restarted CodeBuild controller finished FAILED.
This failed run is retained alongside earlier positive F evidence, not replaced
by it or retried under a new name to seek a passing sample.

## Evidence, storage and cleanup

Results:
`s3://isolab-results-104186811093-us-east-2/runs/ecbench-periodic-20261022-b-epoch-1/fb1260f3-d86c-44c7-ae62-c5506963d3cb/`.
Logical state, computation and private checkpoints:
`s3://isolab-results-104186811093-us-east-2/recovery/runs/ecbench-periodic-20261022-b/`.

`public-evidence.tar.xz` retains complete computational and fresh records, native
audits, Pod/node identities and logs. `verify-failure.sh`, `verification.json`,
the hash receipts and prefix comparison record the readback checks. The two full
checkpoint archives remain private encrypted S3 objects and are excluded from
Git. Their exact versions and hashes are in the generation receipts. Both owned
PVCs were deleted using UID preconditions after preservation; committed snapshots
remain retained. All three temporary EKS access associations created for this
qualification were removed after matching their original identities and scopes.

The current pin includes the checkpoint fixes and records this limited
qualification explicitly. An all-L2 pass for B is not established. The separately
frozen main-only GitHub Action remains the outstanding end-to-end acceptance
check; its original source/spec/L2/A-A/S3 criteria are unchanged. An identical B
rerun is not scheduled. Further diagnosis requires a separately preregistered
CPU-accounting/placement probe, not selective reruns of rejected observations.

`grades.svg` plots the recorded grade counts, not admitted timing. No timing
comparison or algorithm improvement was admitted, so the IC leaderboard,
canonical curve graphs and performance charts are unchanged. The report PDF
contains the same counts and scope; its source is `render.py`.
