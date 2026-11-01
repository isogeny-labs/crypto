# Manifest interpretation

`results/manifest.json` is the immutable provenance snapshot made at its
recorded `generated_utc` time. `artifact_sha256`, `native_source_sha256`,
`current_binary_sha256`, and `frozen_binary_sha256` record the source and
executable bytes used when that snapshot was packaged. They are historical
hashes, not assertions about a later checkout or build.

`frozen_binary_path` records the path used by the experiment, but that path is
under an ignored `target/` directory and is mutable. A later build currently
occupying the same path does not replace or invalidate the recorded digest;
reviewers must identify the experiment binary by `frozen_binary_sha256` and
the run records, rather than by the current file at that path. The archive and
control inputs remain pinned separately by their recorded SHA-256 values.
