# Native Certicom checkpoint: upload verified

The S3 checkpoint blocker is cleared. The user-authorized AWS identity
authenticated to lab account 104186811093, and upload/readback succeeded.
No credentials are included in Git or the checkpoint.

Archive:
`s3://setec-astronomy-artifacts/checkpoints/crypto/codex__hashed-ecdl-s3-checkpoint/20261030T031340Z-4b1834891e78/checkpoint.tar.gz`

Source: `4b1834891e78ca3f83b64540717042e0569d2a60`.
SHA-256: `b18902c499853a9859a569d78e537e637c45e0e872753a52c6e695f71deba952`.
Object size: 5,732,356 bytes; S3 AES256 server-side encryption.

Actual validation:

- `scripts/checkpoint.sh push` uploaded the checkpoint successfully.
- `aws s3 cp` downloaded it independently; metadata and resume notes matched.
- All five payload SHA-256 checks passed, covering 20,980,674 uncompressed bytes.
- Both previously saved baseline/candidate release binaries retained their
  original digests; the baseline source matches 21 pinned Git blobs and current
  source/study files match 55 Git blobs. These checks were repeated on readback.
- `aws s3api head-object` confirmed object metadata, size and encryption.
- `scripts/checkpoint.sh gate` exited zero for the recorded source/payload state.

The exact receipt is [hashed-ecdl-checkpoint-upload.json](hashed-ecdl-checkpoint-upload.json);
the per-file digests are [hashed-ecdl-checkpoint-payload.sha256](hashed-ecdl-checkpoint-payload.sha256).
The archive contains complete resume instructions, source metadata, the frozen
study/protocol, baseline/current native sources and the saved binaries. Extract
its `untracked.tar.gz`, then run `sha256sum -c checkpoint-native-payload.sha256`
from the restored root. Nested source archives retain source revisions and
complete original Git-blob manifests. Recheck the gate on the execution host.

Before authentication, STS failed with `NoCredentials` (253) and checkpoint push
failed (1). A Git lazy-fetch stalled during baseline archiving and was terminated
(143); a shared partial clone also failed (128). The successful baseline archive
was reconstructed using exact GitHub blob IDs and verified against the pinned
tree. These source/transport checks are infrastructure work, not timing evidence.

Read-only `codebuild:BatchGetProjects` inspection was denied. That action is not
one of the launcher's required caller permissions; the denial does not establish
whether `StartBuild` is permitted. Native image/submission authorization remains
unverified. No experiment, profiler or benchmark was launched.

Next: verify the native image/execution route, then profile and run the frozen
matched A/A and A/B panel under all existing admission/replay requirements. This
upload establishes no runtime speedup and closes neither the independent general
audit nor either challenge solve. Frozen earlier receipts remain unchanged.
