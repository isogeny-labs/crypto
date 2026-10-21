# Sealed completed m=4 controls

`runs.tar.xz` contains exactly the two preflights, two constructed controls,
two independent Linux replays and their host/build/input records. All six
containers exited zero without OOM. This archive excludes the active
K1,D1,D2,K2 public query card; it does not claim that card completed.

Archive SHA-256:
`076c60d6ba10127e9d07be2e6ccab5240f7dcc9356c763f2f96f5b86556d03ae`.
Verify and extract from this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c files.sha256
```

The archive uses GNU tar sorted paths, mtime `2026-10-19 00:00:00Z`, uid/gid
zero, numeric owners, mode `u+rwX,go+rX,go-w`, GNU format and `xz -6 -T1`.
Repacking `runs host files.sha256` under those settings yields the same
bytes. Extracted raw trees are ignored; the compressed archive is the
committed source of truth. `hash-check.log` records the local manifest
verification. The [control report](../../reports/run-small-m4-controls-20261019/RUN_REPORT.md)
records the scope and the still-open natural relation/rank gate.
