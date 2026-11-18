# Read-only native adoption of archived primary K600 — frozen before execution

**Question.** Can the exact primary K600 object already archived by the PR
artifact bot be imported through the native v3 store reader from S3 without
issuing a new PUT or presenting the earlier `file://` receipt as an S3 push?
This is a custody/reader acceptance gate. It does not search for relations or
rank factor bases by index-calculus runtime.

## Frozen input and scope

- Curve: `icv1-f2m83-tm6151469093347-debefd74`, primary `a=0` subgroup.
- Original producer: clean source commit
  `498efdc53e821edb24fcebf0826ff94549ce266b`, policy `public_x_hash`,
  seed `2026100801`, K=600 signed-Frobenius columns, 99,600 points. The
  original `manifest.json` SHA-256 is
  `438cd804ba99160121fecce4e072b1b71ae87ea5be58110bbb7e12752df185ff`.
  Its replay and storage receipts are retained in
  [`../k600-v3-local-custody-20261115/attempt-1/`](../k600-v3-local-custody-20261115/attempt-1/).
- Archived object: 3,025,798 bytes, SHA-256
  `3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`,
  plain length 14,951,450 bytes and point-set BLAKE3
  `ac664f5f0c7122c52a08c0a288b543efb709111a99dc2a5cb2bef025ffffee21`.
  The PR bot's tracked
  [artifact pointer](../k600-v3-local-custody-20261115/attempt-1/objects/.artifacts.json)
  has SHA-256 `432355b8ac52b5b0f019e8bbb4dcec496f41946b1c20a413c38b8b175e243404`.
  The configured remote is exactly `s3://setec-astronomy-artifacts/store`,
  object key `sha256/3e/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`.
  The earlier [readback](../k600-v3-local-custody-20261115/S3_CUSTODY_ADDENDUM_20261115.md)
  verified a fresh S3 GET, but not the native v3 reader.
- Logical name: `factor-bases/icv1-f2m83-tm6151469093347-debefd74/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf.jsonl.gz`.
  The prior `local-store-index.json` is an input candidate, not an S3 publish
  receipt; its name, digest, byte count and provenance must be checked.
- Read identity: `ecc-challenges-new` in AWS account `104186811093`, used
  only for HEAD/GET. No native or manual S3 write is authorized by this gate.

## Acceptance and stop conditions

1. Commit the native adoption command and its tests before a real S3 run;
   freeze the final command/source commit and exact artifact pointer in an
   execution lock. Refuse a dirty or source-mismatched checkout.
2. Require the pinned original manifest SHA-256; check its v3 schema,
   original replay/storage bindings and exact curve/base identity. Check the
   artifact pointer's filename, digest and length. Reject any discrepancy
   before creating a new adoption bundle.
3. Create a new write-once output directory, select the exact S3 remote and
   a fresh empty cache, and use native `Store::ensure` to make one read-only
   remote fetch. Require `fetched=true`, direct SHA-256/length agreement and
   gzip/plain BLAKE3 agreement. HEAD ETag is not a SHA-256 certificate here:
   this bot-uploaded object has no SHA-256 user metadata.
4. Independently replay every imported point and signed-Frobenius label with
   the generic curve path. Create a new S3-bound v3 manifest, replay,
   storage and import receipts with `custody_mode=read_only_adoption` and
   `uploaded=false`. The existing native v3 reader must accept that bundle
   against the S3 remote and reject a changed remote or indexed digest.
5. Preserve a terminal failure/cap receipt if a post-directory step fails.
   A bounded read or replay failure is a custody failure, not a negative
   relation-yield observation. The complete native command is capped at 300
   local seconds including remote read, replay and import. Stop on the cap.

The positive control is the exact K600 object/manifest/pointer combination.
Negative controls change one of the pinned source-manifest digest, artifact
pointer digest, store index digest, and remote selection; each must fail
closed. A second run uses a separate empty cache to verify that the result
does not depend on the producer's local cache. The output records source,
host, command, caps, hashes, bytes and terminal exits. One shared-host L0
wall sample is diagnostic only; no comparative timing or selected-best claim
is allowed. `selected_best_total_runtime` remains null.

This adoption does not resolve the future native S3 writer choice, does not
rewrite the historical local-store result, and does not launch the separate
K600 m6 positive/natural search or spend its proposed 3,600-second budget.
