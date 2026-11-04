# Unapplied controller owner-exit collection proposal

Prepared 2026-10-30. Private source copies only; no working-tree/shared-source edits, cloud/Kubernetes/AWS requests, jobs, IAM/RBAC changes, builds, tests, publication or deployment. Rustfmt parsed the private after-file and emitted a separate formatted copy. The task adds collection/custody software; it does not activate a campaign or extend a budget.

Patch: `controller-owner-exit-collection.patch`
SHA-256: `b170e5a4bb2bf42c054e05949a8ce8dd28c30420b79f2545102cfb466ea170df`.
Base: retained `source/taskq/deploy/eks-karpenter/resume/controller/src/cloud.rs`, SHA-256 `ec633a6af0e9475731f0b7f9757828bfdd6b6baab6918ec6d082803a6cd0ec91`.
Private before/after copies and base manifest: `controller-owner-fetch-proposal/`.

## Sources and coordinated interface

Read the owner-closure-interface inventory; retained cloud.rs status/tick/run/publication paths; supervisor.rs Status/control directory interface; io.rs replacement writer; and existing resume/run-controller EXIT upload. No AGENTS.md was found in the retained source tree or its inspected project ancestors. Existing codebase/session research rules remain in force; this is software preparation with no scientific observation.

Coordinated with the supervisor owner at private `owner-receipt-proposal/owner_observation.rs`:

- Protected fixed path `/work/control/attempt-{attempt}/owner-exit.json`.
- Schema `isolab.recovery.owner-exit/v1`, cap 256KiB.
- Binding fields `request_sha256`, integer `attempt`, `accepted_external_closure:null`, `total_active_worker_ns:null`.
- Additional raw observations include payload PID/starttime, terminal reason, reap/clock/CPU/session data and findings. These remain exact raw custody, not independently certified by the controller fetch.

The supervisor may preserve a partial receipt without an observed reap. Receipt presence therefore never establishes closure, elapsed active-worker totals, exact runner exit, or a successful computation.

## Collection behavior

After the bound supervisor status is saved, the controller attempts collection for failed, completed and verified phases before their existing branches. It uses the existing kubectl exec transport with fixed argv `head -c 262145 /work/control/attempt-{u32}/owner-exit.json`. No shell interpolates a request or receipt path. The attempt is derived from the existing bound state; no arbitrary receipt path is accepted.

Both local stdout and stderr readers retain bounded prefixes (256KiB+1 and 16KiB+1) and drain excess to a sink. The existing timeout-wrapped kubectl boundary is retained. Oversize output cannot be labelled an exact receipt. The use of `head` adds an explicit runtime utility dependency: the source inventory does not prove its presence in a particular pinned image. If unavailable, collection records transport failure/unknown. No shell fallback, package installation or role expansion is proposed.

Nonempty raw capture bytes are retained without reserialization under a digest-addressed top-level .json name. Metadata records exact capture hash/byte count and whether it is truncated. Only successful, in-bound, schema/request/attempt/null-closure/null-active binding checks set exact_receipt_sha256 and fetched_binding_checked_only. Derive rejects duplicate known binding fields; other observation fields are intentionally not semantically validated by this transport layer. The collection receipt is not a general supervisor evidence validator.

Existing `save()` replaces files, so the proposal adds an exclusive hard-link publication helper with checked file/directory sync. An existing identical object is re-read and synced; differing objects and symlink targets reject without overwrite. Partial temp/published files remain on publication failure. Digest-addressed collection summaries preserve differing attempts/results instead of overwriting prior failures.

Both raw captures and collection summaries are top-level `receipts/*.json`, so the existing controller EXIT upload carries them through its existing authenticated artifact channel. This patch does not modify that transport, output bucket, role, IAM policy, or supervisor command interface.

## Failure semantics

A missing, failed, oversize or malformed remote fetch is retained as explicit unavailable collection evidence, with null closure and active time. It does not suppress the original payload failure or invent an exit receipt. Existing computation/verification branching continues; successful computation does not imply complete resource accounting.

A local failure to retain the evidence is returned. On an already failed supervisor state, the original computation error is preserved first and the local retention failure is appended. On other branches it is reported as owner receipt retention failure, using the existing controller failure/stop handling. No original error is replaced by a misleading successful receipt status.

Collection is attached to existing supervisor terminal observations during tick; it does not create a new fetch from a controller state already terminal on entry, a vanished Pod, or a preexisting unknown terminal state. Those cases remain missing evidence. The patch does not infer completion or retry a generator to fill those gaps.

## Software fixtures written, not executed

Three isolated tests cover known binding duplicates/wrong attempts and null-closure enforcement; bounded in-memory stream retention; and exact-byte idempotence/non-overwrite of local captures. They do not call kubectl, AWS, the controller reconciler, supervisor, or a mathematical solver.

Root owns application, compilation, independent source review and integration with the supervisor proposal. The source/transport dependency and missing external closure remain explicit; this note introduces no new experiment or approval condition.
