# V16 infrastructure retry with identical measured source

V15 created pod ndiwvrvehbf4hy at 20:10:04UTC but failed before compilation:
GitHub's partial-clone lazy fetch requested authentication for a root blob.
Its bootstrap exited128, controller1, zero trials; the archive is retained in
failed-launch-v15 and the pod was deleted. The v15 one-attempt contract remains
closed. V16 is a new frozen version, not an unrecorded retry.

V16 keeps original source commit23dc8a61ca928e47237f8fb5a3a8d87228649628 and
run-filter.sh/ladder-filter.rs/analyze-filter.rs unchanged. Inputs, arm order,
preflight, budgets and stopping rules are identical. Only source delivery changes.

The administrative archive contains a prehydrated sparse Git checkout: original
commit and tree objects, selected original blobs, shallow boundary, sparse index
and materialized files. It has no remote, credential helper, tokens or custom
hooks. All1030 materialized files were checked against their committed Git blob
IDs; original HEAD and empty git status were verified without network access.
An initial construction step warned about an absent out-of-cone skill blob;
final checkout and full materialized-file verification passed. Out-of-cone blobs
are intentionally absent; this archive is for the declared build, not full-repo
history or arbitrary paths. No source bytes or commit IDs were synthesized.

Artifact: s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/e2e-20261030-v16/source-checkout.tgz
SHA256:6062ea5ab67dedb0df2ff01deafcd8a0dcf0d6da41b6fa3a933f58f3d8593655.
The worker checks this digest before extraction, then exact HEAD, clean worktree
and absence of remotes before the original launcher. A short-lived signed GET
is injected privately; it is not stored in project artifacts. Source setup
remains outside all solve intervals. Full device/Rust/sanitizer preflight still
precedes data; source rehearsal is not GPU validation.

Relevant primary documentation reviewed before packaging:
https://git-scm.com/docs/git-pack-objects (pack existing Git objects) and
https://git-scm.com/docs/git-sparse-checkout (cone selection includes ancestor
files). This is ordinary reproducible source delivery, with no algorithm change.
