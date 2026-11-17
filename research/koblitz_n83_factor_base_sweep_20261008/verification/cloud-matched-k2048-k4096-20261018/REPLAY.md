# Replaying the compact review receipts

The full raw ZIP remains local pending approval of its external transfer.
Its exact local path, SHA-256 and proposed S3 key are in
`raw-artifact-status.json`; `raw-artifact-manifest.json` lists its contents.
The ZIP includes the frozen native worker and helper, source snapshots,
input metadata, complete logs, observer/auditor scripts and negative controls.
It excludes home configuration, credentials, Git internals and duplicate
factor-base archives. The factor bases are already in the user-selected bucket.

On a compatible Linux x86-64 host with the raw ZIP available, verify its
SHA-256 and every manifest entry, extract it and hydrate the two omitted
`inputs/kK/objects/HASH.jsonl.gz` files from their exact `current_s3_uri`
values in `inputs/input.json`. Verify their SHA-256/BLAKE3. Fetch the two
protocol commits named by the manifest into the research repository.

Set `N83_EVIDENCE_ROOT` to the extracted raw root, `N83_REPLAY_CHECKOUT` to
the repository, and `N83_REPLAY_WORKER` to the extracted `worker`. Make the
included `b3file` executable. Docker must be accessible for cleanup-state
checks; the original task used its recorded sudo wrapper, without changing
socket permissions. Run `audit_stage.py STAGE K` for capacity/search/control
and K2048/K4096. The auditor verifies source snapshots against the frozen
commits, byte identities, resource inspection, cleanup, exact counts and
original-group replay wherever a terminal receipt exists. It does not
reinterpret wall censoring as a terminal solver result.

`negative_controls.py STAGE K` modifies temporary copies, recomputes worker
hashes and requires semantic forgery rejection. Use a fresh evidence copy for
those controls because their report directories are created without overwrite.
Initial failed control templates and their corrections are preserved in the
raw ZIP and summarized in `VALIDATION.md`.
