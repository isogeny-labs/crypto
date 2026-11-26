# Fixed RR5 B677 payload

This directory connects the registered
[B677 gate](../../../../research/prime31_four_summand_20261122/scaling-b677/PROTOCOL.md)
to the maintained IsoLab command route. It does not change the extractor,
factor base, targets, limits, arithmetic or independently authored row checker.
It does not produce an ECDLP cost comparison.

## Fixed execution contract

| Item | Value |
| --- | --- |
| Curve | The registered ordinary prime-field curve, p = 2147483647 |
| Base | Original 509 Dickson + 168 Y7 points, total 677 |
| First call | Positive, structured selector |
| Second call | Positive, cofactor reference, including after an ordinary first-call failure |
| Conditional calls | H0 then W0, structured only, after the structured positive completes and all its rows pass native replay |
| Scientific worker count | 1; RAYON, OMP, OpenBLAS and Rust test threads set to 1 |
| Reservation | 2 CPUs, 2 GiB, standard isolation, zero Kubernetes retries |
| Scientific outer limit | 180 seconds, hard kill of the owned process group |
| Address-space ceiling | 2147483648 bytes |
| Per-call limits | Frozen degree 10, 30-second internal limit, 20000 terms, 4096 assignments |
| Native resource limits | 20000 pairs, 8192 symbolic monomials, 2000000 matrix entries |
| Job deadline | 300–1800 seconds including provisioning and archival; does not enlarge the scientific limit |

A failed cofactor call does not suppress H0/W0 if the structured positive
qualified. A hard outer timeout, OOM or unrecoverable I/O failure can prevent
remaining calls; it leaves an incomplete failed run. Stopped, unsupported,
resource-limited and incomplete outputs cannot be classified as Empty.
No enumeration fallback, new target, automatic retry, parameter search or
resume mode is added.

## Build, publish, run and replay

The workflow input is `rr5_b677_gate=true` in
`.github/workflows/isolab-experiment.yml`. The mode is restricted to the
workflow on main and cannot be combined with publish-only, size-sweep,
norm-branch-census or resumable modes. A GitHub rerun is a new attempt; inspect
the original request after ambiguous submission instead of rerunning it.

1. `build-bundle` uses Rust 1.100.0, the existing Cargo lock and the
   x86_64 musl target. It first compiles, then runs the library, independent
   checker and public example tests, then builds both binaries. It records
   the compiler, source checksums, original-source selection manifest and
   executable/script hashes. The source must be a clean exact Git commit.
2. Before publication, the workflow checks every inventoried input against
   that published source revision and runs the existing checkpoint gate.
   The checkpoint destination remains `setec-astronomy-artifacts/checkpoints/`.
   A checkpoint failure stops publication and submission.
3. `publish-image` uses trusted workflow tooling and treats the build bundle
   as data. It validates the complete required hash maps, copies an explicit
   file allowlist, builds the fixed Dockerfile and records the immutable ECR
   image digest. No selected-source executable runs in this credentialed job.
4. `prepare-request` emits the existing command-request schema with the fixed
   command and limits. The maintained launcher supplies readiness checks,
   write-once Job creation, request hash, Job/Pod identity and bounded status.
5. `run` checks source, protocol, metadata and executable hashes, starts the
   fixed sequence under a hard timeout, records failures, hashes available
   evidence and uploads it to the actual launcher destination,
   `isolab-results-104186811093-us-east-2/runs/<job>/<pod-uid>/`.
   An upload error or archive-generation error returns nonzero.
6. `collect` downloads both launcher and payload evidence, including failed
   Jobs, and records both download outcomes. `verify-artifacts` binds request,
   manifest, live Pod command/resources/image, zero retries, build metadata
   and every payload file. It rejects path escape, duplicate manifests,
   symlinks, omitted/extra files and incomplete identity.
7. A separate GitHub job, with no AWS credential step or OIDC write permission,
   runs `replay`. Only the separately authored Rust row checker and the
   hash-bound receipt wrapper execute there. Fresh replay directories preserve
   original evidence. Raw records and replay failures remain downloadable.

The runner's completeness flags come from the reviewed producer and its
recorded traversal. The independent checker authenticates all submitted
signed rows, augmented rank and any recovered scalar against public points.
It does **not** supply an independent proof of extraction completeness. The replay summary
separates transport checks, per-call identity, row replay and overall acceptance;
it never supplies a complete cold cost or a signed PCC verdict.

## Admission before dispatch

The existing [cost controls](../COST_CONTROLS.md), [harness](../HARNESS.md),
and frozen [admission record](../../../../research/prime31_four_summand_20261122/scaling-b677/ADMISSION.md)
still apply. Before a new cloud build or attempt, establish the caller's actual
authorization, inspect existing requests/jobs, check live all-tenant pool and
quota headroom, and record a finite ceiling with known remaining daily/monthly
and lane budget. Include builds, failures, idle time and storage. Neither a
successful compile nor the launcher's readiness check supplies this accounting.
Unknown remaining budget or missing admission stops a new launch.

**The committed GitHub trigger policy is insufficient for full readback.**
In [remote/bridge.yaml](../remote/bridge.yaml), that role has no ListBucket
permission and permits only selected sanitized payload filenames; raw records,
stdout and stderr are deliberately outside its boundary. The new mode checks
the necessary list permission before publication/submission and stops if it is
absent. Passing that check does not establish the required object-read rights.
Full replay needs an already authorized results-reader transport, such as the
documented harness identity with appropriate grants. This PR changes no IAM
policy and does not export raw data through a sanitized-object exception.
The workflow uses the existing CHECKPOINT_UPLOAD_TOKEN secret when available;
the trigger policy alone also cannot write the separate checkpoint bucket.
Token presence, validity, live IAM grants and actual readback have not been
established by the source implementation. These remain deployment prerequisites.

After the reviewed workflow is on main and those prerequisites are established,
dispatch it with a full published 40-character `source_revision`,
`rr5_b677_gate=true`, the admitted instance type and the original bounded
`seconds`. Retain the run ID/attempt and inspect that attempt's three artifact
groups: `rr5-b677-build-*`, `rr5-b677-evidence-*`, and `rr5-b677-replay-*`.
The workflow does not create new credentials, broaden permissions, raise quotas
or replace the selected source with an integration fixture.

## Local development checks

`test-contract.sh` and `test-artifacts.sh` use synthetic shell fixtures and mock
storage commands. They require Bash, jq and ordinary GNU utilities, and work
from the taskq-only CI checkout. They do not execute the registered extractor.
The native build script is a development qualification path; its tests include
public small controls and zero-budget admission controls.

The deliberate whole-file source-pin refresh is documented in
[SOURCE_REVIEW.md](../../../../research/prime31_four_summand_20261122/scaling-b677/payload-integration/SOURCE_REVIEW.md).
Future source mismatches must stop at the existing guard and receive a new
dependency review. Existing frozen scientific evidence remains unchanged.
