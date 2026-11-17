# Policy consumer integration handoff

## 2026-10-28 13:48 UTC — source checks passed; producer interoperability pending

### What was done (commands, files, features)

Created `codex/ml-consumer-policies-20261028` in the separate worktree
`/home/ec2-user/crypto-ml-consumer-policies-20261028`, based on PR2757 head
`b1ec04d1de276eabdca790754670aaa4fab5d87f`. The existing owner's branch and
worktree were preserved. Conductor T-739 reserves this research subtree;
its heartbeat metadata is in
`/home/ec2-user/ml-consumer-policies-coordination-20261028`.

Implemented strict plan/v2 recipes, canonical hash reconstruction, policy
radix and cell/base provenance binding. Added bounded structural status and
action consistency, v2 positive PDP domain arithmetic, separate static
view/v2 projection and explicit unsupported-feature exclusion. The v1 pilot
digest and all 720 cell indices remain covered. Details and boundaries are in
POLICY_COMPATIBILITY.md.

Standalone release checks passed 80 library and 7 integration tests, with
one opt-in producer interoperability test ignored. Strict all-target Clippy
and formatting passed. Raw logs are retained in
`/var/tmp/ml-consumer-policies-validation-20261028`; the Cargo target is
`/var/tmp/ml-consumer-policies-target-20261028`, distinct from the root target.

### Decisions made (and why)

The independent consumer remains a standalone crate. It checks declared
positive group relations and structural bindings, not factor-base
construction, subgroup orders, root sets, complete operation streams, UNSAT
or fall degrees. Its static view retains its own feature IDs and split rule;
it is explicitly not equivalent to the producer's v4 training export.
Unsupported policies retain provenance and labels with null features.

The opt-in local software fixture freezes six recipes, all three families,
both twists, m=2 and two solvers, for 72 bounded small cells. These tests do
not satisfy the requested large-size corpus. No AWS worker, IAM action,
S3 publication flag, budget extension or experiment retry is introduced.
The original USD200 total ceiling remains unchanged.

### Open threads (unfinished, pick up next time)

Obtain the parent's clean, source-bound producer executable, update the pinned
binary hash, and rerun interoperability into a new retained directory. Do not
overwrite interop-01 or bypass producer build provenance. Audit and replay
the actual sealed cells, record feature exclusions and positive-certificate
coverage, retain manifests/raw logs and deliver the exact evidence for S3.
The parent owns the required root-library pre-push gate and PR delivery.
This branch has not established live worker-to-S3-to-consumer publication.

### Errors (what went wrong, how it was handled)

The first Conductor check using project slug `crypto` returned 404; the
migrated project UUID resolved it and confirmed free scope before claim.
The first pinned producer binary (SHA256
`84ccabd039b5abc6cbf700c4d1e0d9ba9f73062df5a8fdf381e709c89eeb2080`)
planned successfully but rejected generation before any cell because its
build had no `CRYPTO_BUILD_GIT_COMMIT`. This failed admission is retained in
`interop-01.log` and `interop-01/`. No override or data retry occurred.

## 2026-10-28 14:00 UTC — sealed software interoperability and S3 readback passed

### What was done (commands, files, features)

Committed implementation as 8d55079e37f2c5f2f506edf20670f018354edd56. A complete
resume record passed the checkpoint gate and its canonical S3 checkpoint was
freshly downloaded, hashed, extracted and Git-bundle verified. The source-bound
producer d71ec2ef5ab9b7f2a038f61c941af10297a775ad was copied and its SHA checked.
The opt-in interop-02 test generated and audited all 72 software cells, checked
22 positive certificates independently, replayed the producer and exported
72 records with 12 unsupported-feature exclusions. Six root-artifact cells
remain explicitly outside independent root-set verification. All recipe, seed
and status substitution checks passed. The normal release suite passed 87 tests
and ignored the separately executed interoperability test; strict Clippy,
formatting and release build passed.

### Decisions made (and why)

Preserve the earlier zero-cell provenance failure and gate failure; use a new
directory for the successful fixture. Keep source byte hashes, observed
producer statuses and independent coverage separate. Exact cells, code,
consumer executable and raw logs were uploaded using conditional S3 creation.
An ETag-bound fresh download matched all archive bytes and 2,139 inner hashes.
The independent consumer then audited the restored 72 cells and checked 22
positive certificates again. Durability details are in
evidence/policy-compatibility-20261028/durability.json.

### Open threads (unfinished, pick up next time)

Root owns the root-library gate, PR publication and coordination with the
existing consumer owner. No new consumer implementation changes were needed
after source8d55079 passed interoperability. The producer training-export v4
bundle, actual EKS worker publication, completion-receipt fetch of these cells,
large-size corpus and mathematical goals remain outside this software check.
The original campaign budget, approvals and deadlines remain unchanged.

### Errors (what went wrong, how it was handled)

The checkpoint helper's explicit gate required a complete resume even with a
software-fixture skip reason, so the record was completed and persisted before
generation. An archive command initially used repository-root pathspecs from
a nested directory; it failed before publication and was rerun from the root.
Only the validated final archive was uploaded. No original cell was rewritten;
negative mutation tests used a separate copy and its contents were retained.

## 2026-10-28 14:32 UTC — required root gate passed; companion publication

### What was done (commands, files, features)

Reconciled the parent-owned root-library process: session 25826 exited zero,
and the saved full log records 4,987 passed, zero failed and 122 ignored.
Verified the immutable test executable's recorded SHA256. Runtime source is
8d55079e37f2c5f2f506edf20670f018354edd56; documentation/evidence commit
377319ecf33d561be7bec3de0de7e40642ca3ea5 and this follow-up preserve the native
subtree ffc2c5727e8902add679db8ac875677d2f8397de and root source/build/test files.
Archived the logs and metadata, excluding the large executable, to canonical
S3. Conditional PUT, ETag-bound fresh GET, SHA256, byte comparison and six
internal hashes passed. The committed root-validation and root-durability
receipts identify the archive, source and exact checks.

### Decisions made (and why)

Reclaimed free Conductor T-739 for this scoped publication. Fetched the stack
base and checked PR2757 remains open at b1ec04d1de276eabdca790754670aaa4fab5d87f;
its owner's branch/worktree are unchanged. Publish the companion as a regular
PR targeting codex/ml-dataset-phase-labels-20261028. GitHub auto-merge is for
the default branch, so it remains inapplicable to this stack until retargeted.
Reviewed AGENTS.md, docs/CONDUCTOR.md, docs/artifacts.md, the existing policy
contracts and AWS's primary [conditional-write documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/conditional-writes.html)
and [GetObject contract](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html).
The archive uses immutable creation and an ETag-conditioned readback, while
SHA256 and byte comparison establish the retrieved content. No Cargo suite,
cloud experiment or IAM action was repeated. The USD200 total ceiling remains.

### Open threads (unfinished, pick up next time)

Inspect the companion PR's exact-head remote checks and review before merger;
the parent owns the wider delivery and stack coordination. The 87 standalone
tests and 72-cell fixture (22 independent positive certificates, 12 feature
exclusions) remain the interoperability evidence. The consumer static view is
not equivalent to producer v4; producer-export-bundle validation, live worker
publication/completion-receipt fetch, larger corpus and mathematical claims
remain open. No new production or experiment was launched.

### Errors (what went wrong, how it was handled)

The shared daily session log remains held by parent task T-729. Its scope check
reported a collision, so this required four-part handoff stays in the reserved
research subtree and the parent was notified to include it in the shared log.
No source changes or validation bypass were needed for publication.
