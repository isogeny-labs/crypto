# Parent-library record: incomplete, full archive local only

The parent run at `95cf0a81` did not return its final result. The sole case
unreturned in the inventory comparison was the legacy isogeny-walk test named
in [the problem record](../../../problems/root-library-isogeny-walk-runtime.md).
Cargo and its executable later showed encoded exit status 9 (SIGKILL) and
state Z; the cause of the signal is unknown. No validation process remains.

All 18 initial missing-fixture failures pass in exact-name reruns against the
same executable after restoring tracked files. This is not a fresh green full
suite and does not discharge the remaining case. Default ignored tests were
not enabled or reclassified.

The additional `isolated-replay.json` / `isolated-replay.txt` record uses the
same source and executable with a declared 180-second development deadline.
The case produced no result before GNU timeout terminated it, exit 124. This
bounded retry does not explain the original SIGKILL or make the parent scope
green. Its complete small output is committed independently of the archive.

The complete log, inventory, both fixture-replay records and process observation
are preserved in `runs.tar.xz`, produced with the repository's deterministic
raw-tree recipe. `RAW-SHA256SUMS` identifies every uncompressed record. The
archive is 103,396 bytes, above the data-file Git threshold; it is intentionally
ignored rather than committed.

The maintained artifact uploader was actually invoked for the authorized S3
store and exited 2 with **Unable to locate credentials**. The failed upload
receipt and `LOCAL-ONLY.json` record the exact hash, size and attempted location.
The remote object is unverified; no `.artifacts.json` claims a successful upload.
The full archive currently has no verified durable remote copy. An authorized
existing storage connection is needed to finish archival and replay its hash.

Local reconstruction, from this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
cd runs
sha256sum -c SHA256SUMS
```

This limitation concerns the broader parent check. The affected native package,
proof replay and public network-construction control have separate complete
records in the parent study directory.
