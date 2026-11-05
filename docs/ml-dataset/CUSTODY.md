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

This command performs **zero S3 requests**. The existing per-cell `publish`
and `restore` commands do not publish this full-run bundle. Whole-run S3
publication and download still require a budgeted, grant-bound, bounded CAS
transport with fresh HEAD/GET byte-hash checks for every chunk and the
manifest. Until that path is implemented and the worker grant is active, keep
restricted bundles local and do not report them as cloud checkpoints.
