# Restricted full-run custody bundle

`crypto dataset custody` snapshots a quiescent dataset run into a local,
content-addressed directory. It includes every regular file under the run,
including incomplete attempts, worker traces, root logs, `plan.json`,
`protocol.json`, and `run.json`. Report or export directories outside the run
must be named explicitly. The bundle is restricted: it can contain planted
scalars, witnesses, raw traces, and private labels.

```sh
crypto dataset custody create --out RUN --bundle NEW_BUNDLE \
  --include report=REPORT_DIR --include export=EXPORT_DIR
crypto dataset custody verify --bundle NEW_BUNDLE
crypto dataset custody restore --bundle NEW_BUNDLE --destination NEW_RESTORE_DIR
crypto dataset custody publish --out RUN --bundle NEW_BUNDLE \
  --remote s3://setec-astronomy-artifacts/APPROVED_PREFIX \
  --budget-policy POLICY.json --budget-grant GRANT.json \
  --budget-owner OWNER.json --budget-state BUDGET.sqlite
crypto dataset custody restore-remote --receipt NEW_BUNDLE/publication/full-run-REMOTE-MANIFEST.json \
  --bundle NEW_DOWNLOAD_DIR --destination NEW_RESTORE_DIR \
  --budget-policy POLICY.json --budget-grant GRANT.json \
  --budget-owner OWNER.json --budget-state BUDGET.sqlite
```

`create` takes the run writer lock and replays completed cells before copying.
It refuses symlinks, special files, unsafe names, a live run writer, and an
existing destination. The default limits are one million files and 1 TiB of
uncompressed members; callers may set smaller `--max-files` and `--max-bytes`
limits. Files are split into 4 MiB SHA-256-addressed objects. The manifest
records every relative file path, byte length, file SHA-256 and ordered chunk
hashes, plus the exact run plan, protocol, source commit, and binary digest.
An interrupted `create` may leave an unverified bundle directory; only a
successful `verify` confirms its contents.

`verify` checks all manifest members and object bytes without executing solver
code. `restore` requires a fresh destination, verifies the bundle first,
reconstructs all files, and runs native dataset verification with independent
known-answer replay on the restored run. A failed restore leaves its partial
directory for diagnosis; it must not be treated as complete.

The local `create`, `verify`, and `restore` commands perform **zero S3 requests**.
`publish` requires an active actor-bound budget journal, the run binding, and
an explicit approved S3 prefix. It verifies the entire local bundle and
native run before publishing every unique 4 MiB chunk and the custody
manifest through the immutable conditional-write store. A publication binding,
round-trip completion, and catalog make the exact object set discoverable.
The command reports the local receipt path; use that exact path for
`restore-remote`. An interrupted publish is not a completed cloud checkpoint.

`restore-remote` also requires an active budget and fresh download and restore
directories. It checks the receipt's namespace and object set, charges and
checks a fresh HEAD/GET plus byte hash for each object, then performs native
member reconstruction and known-answer replay. The budget journal records
requests and retains local checkpoints. The actual worker grant and budget
must be approved and active before either remote command is run. Receipt and
download directories are restricted artifacts; a failed download may leave a
partial directory for diagnosis.
