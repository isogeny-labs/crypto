# Complete raw native XOR / N83 evidence publication

The authorized bucket holds the69,465,536-byte raw archive. A fresh S3 GET
matches the complete archive bytes and SHA256; all1089payload SHA256 checks
pass after fresh extraction. The payload manifest itself matches its frozen
hash. HEAD/GET agree on ETag/length and report AES256 encryption.

- Logical name: `store:evidence/n83/xor-indexed-20261019/runs.tar.xz`.
- Bucket/region: `setec-astronomy-artifacts`, `us-east-2`.
- Object key: `store/sha256/1a/1afb93d5a253e3447eb4aeac2959233c9b2fc93acd99c1e0c22c3e44fcd06123`.
- Archive SHA256: `1afb93d5a253e3447eb4aeac2959233c9b2fc93acd99c1e0c22c3e44fcd06123`.
- Payload manifest SHA256: `f93f704bc9eb3bf11da2f710eafb8d0aaa0b11a622b0421747024491e8d43048`.
- [Native store index](store/index.json), [push](push.json), [HEAD](s3-head.json), [GET](s3-get.json).
- [Downloaded archive hash](download.sha256), [downloaded manifest hash](download-manifest.sha256),
  [payload manifest](payloads.sha256), [1089 successful payload checks](download-file-checks.log).
- [Deterministic fresh-extraction repack hash](repacked.sha256); byte comparison
  with the downloaded archive passed using the exact retained tar/xz recipe.

All27original audit-bearing records reproduce identically under the downloaded
native analysis runner, using freshly extracted raw stdout/stderr/source records
and unchanged original host input/worker paths. [Replay script](verify-fresh-native.sh)
and [all native replay outputs](fresh-replay/) are retained. This is fresh-record
replay on the collection host, not a claim of a new-host run. The extracted
inputs/workers also compare byte-for-byte with the original checked paths.
The old external capped-unknown failed audit remains unchanged; the additional
[typed native interpretation](fresh-replay/aa-e1-corrected-unknown.json) reports
UNKNOWN_EXTERNAL_CONFLICT_CAP/5001reported conflicts and verifies no model.
Refused/unattempted plans do not become solver runs or positive comparisons.

The bundle contains both XOR profiles, every internal/external A/A and five
matched rounds, both natural caps for each K, all failures/censors/refusals,
original cards/specs/actual relays/assessments including superseded plans,
worker and native-driver ELF files, complete current/baseline source snapshots,
Cargo manifests/locks, source custody, public inputs, full formula/models/CMS
statistics, original/restarted validation logs, host/toolchain observations,
report sources and renders. It preserves the official CMS asset/version/release
metadata and MIT license/authors. [Collection recipe](collect-evidence.sh).
No credentials, runtime environment dump, host filesystem or Cargo cache is
included. The original Docker filesystem is identified by immutable digest and
per-run inspect/config evidence rather than included as a full image export.

To retrieve, use the index with native `ecbench store pull`, or AWS CLI
`get-object` with the exact bucket/key above, then extract `tar -xJf runs.tar.xz`.
From `raw-bundle/`, run `sha256sum -c SHA256SUMS`. Frozen configs retain original
absolute paths; restore those paths for exact historical replay or implement a
separately audited relocation. Do not rewrite original custody as a new result.

The earlier49,012,484-byte [approved publication](../native-phase-publication-20261019/RESULT.md)
remains at its original content address. This new archive adds the XOR study and
continued search, and does not replace historical raw evidence or enlarge any
scientific claim. The raw evidence preservation requirement is verified complete;
useful natural yield/rank, positive scalar replay and full-cold selection remain open.

The fresh-download/repack/native-replay proof files are additionally stored
as `store:evidence/n83/xor-indexed-20261019/verification.tar.xz` in the same
bucket. [Proof push](verification-push.json), [fresh GET](verification-get.json)
and [matching fresh-download SHA256](verification-download.sha256) record its
content address and successful byte comparison. Its own transport receipts
remain in git to avoid recursive self-inclusion.
