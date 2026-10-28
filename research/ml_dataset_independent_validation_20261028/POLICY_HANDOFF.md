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
