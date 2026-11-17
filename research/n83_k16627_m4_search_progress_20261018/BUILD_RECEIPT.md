# Offline worker build receipt

The search-progress worker was built from source commit
`2e1ba2e476` after the six-file checkout and snapshot hashes in
[`source-files.sha256`](source-files.sha256) matched. The source snapshot
was prepared by [`prepare-source.sh`](prepare-source.sh). The retained
factor-base inputs passed [`input-check.log`](input-check.log).

The native Linux arm64 build used
`rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`,
`--network=none`, `CARGO_NET_OFFLINE=true`, `--read-only`, an 8 GiB
hard memory cap, zero swap, and the repository's locked no-default-features
exporter example. It started at `2026-10-18T06:45:40.643357134Z` and
finished at `2026-10-18T06:48:23.009944959Z`. The container exited 0
without OOM according to [`linux-build-state.json`](linux-build-state.json).
The binary SHA-256 is
`6d01aa0599124efdea5cf43acf2fdbc150ccac7330482893fd36a36f0cee7b09`,
recorded in [`worker-binary.sha256`](worker-binary.sha256). Build stdout
and stderr are retained beside this receipt.

This is a build and provenance gate. The degree-83 SAT diagnostic and its
search result are separate measurements.
