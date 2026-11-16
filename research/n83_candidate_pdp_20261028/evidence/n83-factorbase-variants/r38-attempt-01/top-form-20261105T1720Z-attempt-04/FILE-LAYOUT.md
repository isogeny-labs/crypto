# Attempt 04 file layout

- `protocol-amendment.json` and `protocol-amendment-binding.json`: operational continuation tied to V2.
- `preflight.json`: cohort, input hashes, resource/time/cost/retry caps and fresh S3 prefix.
- `factor-base-metadata/data/index.json`: family selector index.
- `factor-base-metadata/data/{binary,koblitz,prime}/part-*.json`: recovered JSON selector parts describing the local base structure.
- `factor-base-metadata/s3-file-index.json`: immutable S3 key, byte count and SHA-256 for each compressed shard.
- `factor-base-metadata/r38-recovery-audit.json`: recovery and integrity audit.
- `source-pdp.rs`, `source-top.rs`, `instruments/{pdp,top}`: pinned source and exact binaries.
- `launch.sh`, `run-spike38-topform.sh`: one-core controller, exact error-line capture, shard validation, generation and replay.
- `candidate-inputs/`: generated systems and input metadata.
- `top-certificates/`: generated top-form quotient Dreg certificates.
- `runner-error.json`, `terminal.json`, `controller-status.json`: failure and completion state.
- `manifest.sha256`, `S3_PERSISTENCE_INDEX.json`: final byte/hash manifest and per-file S3 URI.

- `factor-base-metadata-v2.tar.gz`: Git-friendly archive of the factor-base metadata tree; `factor-base-archive-manifest-v2.json` lists each extracted path, byte length and hash. The expanded copy is retained beside it for direct inspection.
- `prior-attempt-03/`: terminal reconciliation, index correction and correction to the earlier mistaken local-shard description.
- `factor-base-metadata/s3-live-inventory.json`: live 786-object key/size check for the immutable snapshot.
