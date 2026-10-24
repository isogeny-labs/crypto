# Manifest interpretation

`results/manifest.json` is the immutable provenance snapshot made at its
recorded `generated_utc` time. `artifact_sha256` and
`current_binary_sha256` describe the source tree and executable at that time;
they are not claims that a later checkout or a later build still has those
bytes. Subsequent review commits changed `Cargo.toml` and four native source
files while preserving the historical manifest and all raw run records.

The repository ignores `target/`, so a file currently found at the historical
build path is a mutable local build artifact. Reproducing or reviewing the
recorded run must use the digest in the manifest and the SHA-pinned raw and
replay records. It must not infer identity from the current contents of
`target/release/pencil-sparse`. The archive named by `archive_path` remains
content-addressed by `archive_sha256`.
