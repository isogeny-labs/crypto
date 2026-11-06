# Approved source checkpoint through GitHub

The Oct30 approval covers the APS source/worktree checkpoint under
`s3://setec-astronomy-artifacts/checkpoints/crypto/`. It persists. The local
runtime has no bound checkpoint credentials. Existing repository workflows
reference `secrets.CHECKPOINT_UPLOAD_TOKEN` and the artifact-store AWS key
bindings. The job uses the stock token-first transport and its existing AWS
fallback. Presence, validity and permission to write the checkpoint prefix
are determined by the actual gate, not inferred from a secret's name.

The source-only workflow adds a possible authenticated path without exporting
that token to a workspace, chat, source file or artifact. It runs **no solver,
profiler, performance trial or EKS experiment**. It does not grant a new AWS
identity, mint a key, configure a profile or change the standard checkpoint
script. File-token and instance-metadata discovery are disabled in this
source-only job; it uses the explicitly named existing repository bindings.

## Capture contract

* A fresh native **T10** claim owns the resume scope before publication. Its
  exact fence is retained privately; the admission publishes only its digest,
  the original lease expiry and the existing approval reference. This is an
  operator custody record, not a signed execution proof [pcc:unverified].
* The source selection hashes every captured changed source/evidence file.
  The source-only worker must be clean, at the exact pushed event commit,
  with no unpushed history. Its native ledger must equal the reviewed public
  main baseline. No PCC subtree is copied into this checkpoint; scoped public
  claim JSON remains in Git, and the private native ledger remains private.
* All task-relative changes except those scoped public claim records and the
  selection itself must appear in the selection. Only named APS source paths
  and the owned study are allowed. Relative regular files, unique paths,
  SHA-256 equality, 128-file and 8-MiB bounds apply.
* The worker copies these bytes into one untracked source snapshot and writes
  the scoped resume card. It verifies the complete untracked set. The stock
  gate then captures that snapshot, copied spec/lead files, the resume and its
  standard minimal metadata. No branch bundle is permitted in this path.
  Shared full journals/index, raw build/host capsules, private draft history
  and rejected fixtures cannot enter the selection.
* The helper checks that the existing token, if present, targets the approved
  S3 bucket and standard prefix. Only native presigner form fields are
  accepted; key/file overrides and curl multipart/file-reading syntax are
  refused. It never prints or persists the credential. Missing or expired
  authentication fails the actual standard gate; it is not treated as a pass.

Before the gate starts, at least 150 seconds must remain in the original
fenced window. The complete gate process group is limited to 120 seconds,
with five seconds of termination grace. A queued job outside the window
refuses transfer. The caller retains the claim through that window; any
lease renewal must preserve the original expiry until allowed uploads have
ended. No automatic retry or relaxed deadline is configured.

The standard script remains byte-pinned and is invoked as
`bash scripts/checkpoint.sh gate`. The helper never fabricates its
`.resume/.last-push` receipt. Source-host/user/worktree fields in the standard
minimal metadata stay in the approved private checkpoint, not the GitHub
artifact; no detailed host capsule is produced.

## Outcome and continuation

The snapshot contains a deterministic local `.gitignore` that exposes only
the selected source paths, its selection and that ignore file. This retains
tracked lockfiles that the repository's general ignore rules would omit
after copying. The helper still requires exact equality between the reviewed
payload and the stock uploader's untracked-file enumeration. Repository
ignore rules, the standard checkpoint script and all publication exclusions
remain unchanged.

Only normalized custody/gate receipts are retained as GitHub artifacts.
The source archive, standard metadata, token and raw gate log are excluded
from that artifact. A zero gate exit with the stock receipt records the
upload acknowledgement; independent stored-object readback remains an
explicit separate field and is not invented. A failure may have left partial
approved source objects; preserve its receipt and inspect before any retry.

The workflow is triggered only by a new admission file on the owned source
branch. New code alone does not silently start another checkpoint. It never
marks T10, T9 or the research panel done. T10 may be released ready after the
bounded capture window while the actual outcome awaits a read-only follow-up.
All native-ledger publication/reconciliation and other denied-publication
holds remain in force; keep PR auto-merge off.

The checkpoint does not supply a frozen measurement protocol or run admission.
After a successful upload, independently inspect the stored objects when
authorized read access is available, finish the pilot/accounting/replay/export
and held-out protocol requirements, and only then launch the requested panel.
