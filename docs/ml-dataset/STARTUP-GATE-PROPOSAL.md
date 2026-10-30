# Proposed minimal native dataset startup gate

**Status: implementation saved and independently source-reviewed on 2026-10-29; CLI registration applied under T785 after T766 released its reservation. Source `7a1bd350b26eaeccf51bf9078915649a7c4335fc` passed all 11 startup unit tests within 142 dataset tests, the full release library suite (5,170 passed, zero failed, 123 ignored), strict maintained Clippy, and actual CLI registration/rejection checks. These tests do not establish positive fixed-path production activation, real grant delivery, or cloud execution.** Written 2026-10-28 from [STARTUP-GRANT-DELIVERY.md](STARTUP-GRANT-DELIVERY.md) and the current native budget/generation APIs. The initial scratch proposal made no tracked edits, Cargo runs, cloud calls, resources, grants, or permission changes. This committed revision incorporates the independent recovery review on 2026-10-29; that documentation revision changed no runtime source. The earlier unregistered snapshot is retained in `research/ml_dataset_20261028/startup-implementation-status.json`; it is historical evidence, not the current registration state. Delivery permissions, actual resource lifetime, and worker IAM remain separate unresolved prerequisites.

Combined runtime source `f4249f62df082b401717a03ae2cae1b7b1aeddd3` integrates
main `5a250bd47f7674b1f8fbc8a5ca577805aa235ecc`. Fresh validation passed the full
release library suite (5,170 passed, zero failed, 123 ignored), strict maintained
Clippy, both changed hyperelliptic catalog tests, a rebuilt release CLI, eight
CLI checks and the ten-command two-twist fixture. Independent audit verified
158 inventory files and 159 archive members. Evidence is retained separately in
`research/ml_dataset_20261028/validation-startup-main-20261029/`; earlier-source
records remain unchanged. PR [#2836](https://github.com/isogeny-labs/crypto/pull/2836)
carries this increment; exact-head remote checks and merge status are separate
from these local results. This adds no research corpus rows or verified cloud
cells.

The gate accepts one frozen local launch configuration, waits for one locally projected grant bundle, checks its bindings, activates the existing budget session, and calls the existing finite generator. It does not create Jobs, issue grants, fetch startup files, choose hardware, implement recovery orchestration, or accept arbitrary executable arguments.

## Proposed CLI and native boundary

```text
crypto dataset start
crypto dataset start --resume
```

No path, owner, deadline, solver, shell-command, credential, or remote override flags. The reviewed manifest supplies the fixed paths and downward-API environment below. `--resume` reopens retained gate authority; it never initializes missing authority or renews a deadline. The ordinary manifest remains `restartPolicy: Never` with zero automatic Job retries.

Proposed module boundary, not existing signatures:

```rust
pub(super) fn start(resume: bool) -> Result<GateOutcome, GateError>;

// Pure internal validation, independently testable; production identities and
// local hashes are collected by start(), never accepted from command-line JSON.
fn validate_delivery(
    config: &GateConfig,
    bundle: &GateBundle,
    actual_owner: &OwnerIdentity,
    local: &VerifiedLocalInputs,
) -> Result<PreparedRun, GateError>;
```

Reuse existing budget validation inside a small `pub(super)` budget helper for the reservation/binding/grant relationship; do not duplicate its schema, resource, activation-key, or expiry logic in the gate. `PreparedRun` is internal and contains typed existing `BudgetArgs`/`GenerateArgs`, not a caller-supplied `validated: true` flag. Test clock/filesystem/activation seams must remain internal; the production CLI has no fake-identity or bypass option.

## Minimal typed inputs

All new records use exact schema strings, bounded integers/strings, `deny_unknown_fields`, and duplicate-field rejection. They contain no credentials. JSON bytes and native typed hashes have distinct labels.

```rust
struct GateConfig { // crypto.ml-dataset.startup-config/v1
    schema: String,
    policy_sha256: String,
    reservation_sha256: String,
    startup_deadline_unix_seconds: u64,
    max_wait_seconds: u32, // frozen 1..=300, includes projection delay
    generate: GateGenerate,
}
struct GateGenerate {
    max_cases: u64,       // nonzero, fixed first-wave limit
    seconds_per_case: u64,
    memory_mib: u64,
    trace_mib: u64,
    remote: String,      // explicit approved S3 destination; no default fallback
    lab_receipt_prefix: Option<String>, // explicit approved base prefix, if used
    cell_envelope: Resources,
}
struct GateBundle { // crypto.ml-dataset.startup-bundle/v1
    schema: String,
    policy: CampaignPolicy,
    reservation: LaunchReservation,
    binding: LaunchBinding,
    grant: WorkerGrant,
}
```

Expected launch ID, target, source/binary/plan/protocol hashes, shard, expiry, and worker journal path are already committed by the pinned reservation digest; do not add independently mutable duplicates to `GateConfig`. The policy digest pins campaign/resource/remote constraints. Finite generation limits and the cell envelope are frozen in the configuration, whose exact byte hash belongs in the manifest/launcher observation receipt. The configuration is a trusted frozen input; a user-writable replacement is not accepted as equivalent launch authority.

Config, plan, and protocol must already be present in the digest-pinned image or a separately reviewed read-only mount. This contract assumes neither an existing ConfigMap nor a new mount's availability. The gate does not retrieve missing inputs. New fixed bounds: config 64 KiB, projected bundle 256 KiB, local plan/protocol 32 MiB each; compare descriptor metadata and use bounded reads with one-byte overflow detection. These are proposed gate limits, not changes to current general CLI limits.

## Fixed paths and provenance

| Purpose | Path |
|---|---|
| Frozen gate configuration | `/opt/crypto/dataset/startup-gate.json` |
| Frozen native plan / exact protocol | `/opt/crypto/dataset/plan.json`, `/opt/crypto/dataset/protocol.json` |
| Projected late-delivery bundle | `/run/dataset-admission/bundle.json` |
| Gate authority and journal | `/work/admission/gate.authority`, `/work/admission/gate.sqlite` |
| Accepted local bundle | `/work/admission/bundle.json` |
| Existing CLI budget inputs | `/work/admission/policy.json`, `/work/admission/grant.json`, `/work/admission/owner.json` |
| Existing CLI cell envelope | `/work/admission/cell-envelope.json` |
| Original worker budget journal | `/work/budget/worker.sqlite` plus existing authority/lock files |
| Dataset output | `/work/dataset` |
| Local gate receipts | `/work/admission/receipts/` |

Paths are constants; neither delivered JSON nor environment selects an alternate location. Reuse existing safe-directory and immutable-file writers for local artifacts. Check directory ownership and reject unexpected symlinks/hard-link aliases where native authority requires an ordinary private file. Keep gate/budget authority distinct from exportable audit checkpoints. All gate/config/grant/owner/provenance artifacts are restricted labels or operational evidence, never public ML features.

The reviewed manifest adds `POD_UID` from `metadata.uid`, `JOB_CONTROLLER_UID` from `metadata.labels['batch.kubernetes.io/controller-uid']`, and namespace/Pod name for receipts. Fail on missing or invalid IDs; do not fall back to the Job name or a supplied owner document. The gate derives `owner.json` from these values. **These values are worker-side consistency inputs, not independent Kubernetes attestation.** Their credibility relies on the trusted manifest/kubelet and the launcher's separate reconciliation of actual Job UID, Pod owner reference, image, placement, and lifetime evidence. The gate verifies the observation digest's binding, not the truth of the referenced observation bundle.

## One bounded wait, including restarts

Before polling, validate the frozen configuration/local inputs and obtain actual owner values. Take an exclusive process lease. On fresh start, create and fsync a retained authority marker before initializing the gate journal; a partial marker with missing/incomplete journal fails closed. The marker binds config hash, actual owner, host boot ID, and exact journal path. There is no automatic migration or authority reconstruction.

Persist `first_start_unix_seconds`, the Linux host boot ID, and an absolute `CLOCK_BOOTTIME` deadline before the first poll. The deadline is the earlier of `first_start_boottime + max_wait_seconds` and the initial remaining time to the fixed startup UTC deadline. Each poll checks both that retained monotonic deadline and the original UTC deadline. Use checked arithmetic, a short fixed sleep (for example 200 ms), and the earlier remaining interval. A backwards wall-clock jump cannot extend the monotonic allowance; a forwards jump may stop conservatively. Once delivered, also enforce grant/campaign expiry; the startup deadline must not exceed either. The launcher must enforce this relationship before submission too.

`--resume` requires the same owner, config hash, original path, boot ID, authority and journal, then uses the stored deadline. It cannot replace the timestamp, reset the wait, or accept an expired campaign. Changed host/Pod or lost authority stops. The tiny gate journal is not a spendable budget ledger and must not introduce its own refundable quota. The startup reservation already covers provisioning and waiting compute; no zero-cost waiting claim is made.

Only `NotFound` for the fixed pending bundle is waitable. A complete but malformed/oversized/mismatched bundle is a terminal rejection. Permission errors, wrong file type, unsupported safe-resolution primitives, or unexpected path changes are errors, not retry conditions. No solver, credential provider, STS call, S3 request, external fetch, or worker budget creation occurs while waiting.

## Safe handling of the projected file

Kubernetes projection commonly uses symlink indirection, so `O_NOFOLLOW` on the projected leaf alone is not a correct implementation. Conversely, `canonicalize` followed by an unrestricted reopen introduces a path-change window.

For this Linux-only gate, open the exact trusted projection directory once as a directory descriptor and verify its type/ownership/read-only mount expectations. Resolve `bundle.json` relative to that descriptor with `openat2` confinement (`RESOLVE_BENEATH | RESOLVE_NO_MAGICLINKS`), permitting only ordinary relative symlinks that remain inside that directory. Reject absolute/escaping links. Require a regular final file and bounded bytes, then parse the bytes from that already-open descriptor. Do not add `RESOLVE_NO_SYMLINKS` or blindly forbid the normal same-mount projection indirection. If the required syscall semantics are unavailable, fail explicitly; no unconstrained fallback.

Atomic projection replacement after opening cannot change the captured descriptor's file. Persist the accepted exact bundle bytes/hash before activation, then use only safe local immutable copies. A later observed different bundle is an error; it never updates the accepted grant. Once acceptance is durable, a resume uses the retained accepted bytes and rejects any differing projection that remains visible. The projected source may disappear without authorizing a new bundle or fresh authority.

## Validation before any activation

1. Require policy SHA and native reservation SHA to equal the frozen configuration. Validate policy/schema/region/caps/expiry and the reservation's policy/campaign linkage. Check binding reservation digest, exact target, launch ID, and evidence digest format. Require actual owner = binding owner = grant owner. Reconstruct the expected `GrantRequest` from the reservation plus that owner and require the entire actual request to match; validate the canonical activation destination too.
2. Independently obtain current source/build identity using the existing clean-build check and hash the running executable. Require both to equal the reservation/grant binding. Parse and validate the local plan and use the existing `verify_protocol` for the full protocol contract. Use **`DatasetPlan::binding_sha256()`**, its compact native typed representation, for `plan_sha256`; the saved pretty JSON file hash is not interchangeable. Hash the protocol's exact bytes and require equality with both the plan's frozen protocol SHA and grant binding. Require valid equal shard index/count throughout.
3. Require the journal path to be exactly `/work/budget/worker.sqlite`, not an equivalent alias. Validate explicit S3 remote against the policy, all existing generator bounds and checked cell-envelope arithmetic. The existing protocol verifier checks frozen bytes/schema; it does not interpret schedule/resource prose. The trusted frozen GateConfig must be reviewed against the protocol before submission; this gate does not claim to automate that semantic review. If `lab_receipt_prefix` is present, require the manifest-provided `ISOLAB_RESULTS_URI` to equal that approved slash-terminated base plus the reserved Job name and actual Pod UID, each followed by `/`; run existing receipt-route validation before activation. If absent, require that environment variable absent/empty. This prevents inheriting the generic launcher's old-bucket destination or an unpinned extra publication route. The gate does not mutate process environment to repair a mismatch. Validate these before spending activation requests; do not defer obvious malformed-generation failures until after activation.
4. Persist immutable accepted inputs, actual owner, hashes, and an `inputs_accepted` journal transition. Identical local files may be reused; differing existing files, unexpected authority, or unexplained worker journal state stop. No claim is made that the gate itself verifies actual instance lifetime or global allocator issuance against Kubernetes.

## Handoff to existing activation and generation

Invoke existing native handlers with typed arguments, not a shell or a command supplied in the bundle. The conceptual sequence is:

```text
dataset budget activate [--resume] --budget-policy /work/admission/policy.json
  --budget-grant /work/admission/grant.json --budget-owner /work/admission/owner.json
  --budget-state /work/budget/worker.sqlite

dataset generate --plan /opt/crypto/dataset/plan.json
  --protocol /opt/crypto/dataset/protocol.json --out /work/dataset
  --max-cases <frozen> --shard-index <bound> --shard-count <bound>
  --seconds-per-case <frozen> --memory-mib <frozen> --trace-mib <frozen>
  --publish --remote <frozen-approved-S3>
  --budget-policy /work/admission/policy.json
  --budget-grant /work/admission/grant.json --budget-owner /work/admission/owner.json
  --budget-state /work/budget/worker.sqlite
  --budget-cell-envelope /work/admission/cell-envelope.json
```

These are proposed internal invocations of existing commands, not commands executed by this design. The existing session/transport owns every activation/STS/S3 attempt and exact owner/journal checks. Separate activation and generation handlers may obtain credentials separately; any additional STS request remains charged and must fit the reserved allowance. The gate must not hide an extra bootstrap provider or preflight upload.

## Minimal state machine and crash rules

| Durable phase | Allowed restart behavior |
|---|---|
| `waiting` | Exact `--resume` continues only the original bounded wait. |
| `inputs_accepted` | Verify retained inputs; persist `activation_started`, then create the worker budget session once. Unexpected existing budget state stops. |
| `activation_started` | Require retained valid worker journal/authority and inspect its validated phase. If already active and `require_active()` succeeds, persist `activated` locally without SDK work. Only `activation_pending` invokes metered activation with `--resume`. Expired, closed, unknown, missing or partial state stops; never interpret it as pending. |
| `activated` | Validate the existing active session; do not reset activation or its accounting. Persist `generation_started` before invoking generation. |
| `generation_started` without terminal receipt | Stop with `generation_outcome_unknown`; do not automatically invoke generation again. |
| `complete` / `failed` / gate rejection | Return retained result or explicit terminal status; perform no new work. |

Before invoking a separate activation or generation handler, drop the temporary validation session and its lease; the gate retains its own distinct lease. A crash after durable activation but before the gate transition must not repeat PUT/GET requests or spend the finalization allowance again. Add this exact crash boundary to acceptance tests.

The generation ambiguity rule prevents reopening a fresh `max_cases` window after a crash. Existing dataset reconciliation/recovery remains a separate authorized procedure; this gate does not automate it. Keep the gate lease for the whole invocation and rely on the generator's existing per-output lease and child cleanup. If generation errors after partial work, retain that outcome and the original charged session rather than retrying automatically.

Return bounded typed errors distinguishing missing input timeout, invalid bundle, identity/build mismatch, changed/lost authority, activation failure, generation failure and unknown generation outcome. Persist local terminal evidence where possible; failure to persist evidence is also an error and never permission to recreate state. No automatic cloud publication is added solely to report a pre-activation error.

## Acceptance before claiming functionality

Software tests must cover late atomic file appearance; absent/oversized/malformed input; normal confined projection symlinks versus escapes and file replacement; all owner and hash mismatches; exact native-plan versus file digest semantics; monotonic/UTC expiry; nonrenewable wait on resume; concurrent gates; crashes at each phase; missing authority; metered activation handoff; and refusal to start generation twice. Fake local files and injected transport/clock seams are software evidence only. No new SDK request may occur before successful validation.

Separate deployment evidence must establish the approved delivery object/permissions and mount, real downward-API fields, trusted launcher observation, finite resource admission, and worker-role activation/publication. This future gate contract does not satisfy those gates, prove an account-wide spending cutoff, establish CRIU recovery, or add any verified dataset cells to the current release.
