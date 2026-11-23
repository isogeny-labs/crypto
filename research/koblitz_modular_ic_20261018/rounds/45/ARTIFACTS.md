# Verified R45 raw archive

SHA256 `3cae4207881c0093813a24c9ed5c30524d270f9cde3021cb14e503894409ce2a`;966228bytes;71regular files.
Stored in setec-astronomy-artifacts/store; fresh GetObject/member hashes and
byte-identical deterministic repack pass. All31historical source
pins and per-attempt source hashes checked. Source revision `3253ac2027e9bd96f637d092c16f9771ac01c7c8`.
Four accepted native attempts and one accepted Lean workflow retained.
This is byte/receipt validation, not independent mathematical replay.

Restore with `ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/45`,
then extract runs.tar.xz and run `sha256sum -c runs.sha256` in that directory.
Read FORMAT.md for coordinate/physical-descent encoding. Do not reuse attempts.
