# r29 attempt-01 recovered partial output

This directory reconstructs the original local path structure from all 75 immutable S3 content-addressed snapshots under `runs/n83-fb-variants-r29-20261031-r1/attempt-01/`. Each snapshot body was fetched by its exact S3 key and checked against the SHA-256 encoded in that key and the S3 object size. The mapping from snapshot path hash to local relative path is retained in `DATA_INDEX.json`.

The partial contains 33 compressed binary-family shard payloads (`part-0000` through `part-0032`) and 32 matching JSON receipts. Each captured payload and receipt matches its binary-family manifest entry. `part-0032.bin.zst` exists without a corresponding receipt. The recovered run support files include the copied protocol, generator binary, format documents, host record, and selfcheck artifacts.

The S3 object listing contains no final `data/index.json`, `run-manifest.json`, producer log, final `manifest.sha256`, or `index-validation.json`; there are no Koblitz or prime family manifests or shard payloads. The binary family manifest records 128 shard descriptions, but the captured local tree contains only the files listed above. Treat that manifest as supporting metadata, not proof of completed enumeration. This attempt remains failed and incomplete, and every original snapshot remains in its original S3 attempt prefix.

The `data/` tree preserves the native directory and schema: `data/<family>/manifest.json`, `data/<family>/part-NNNN.bin.zst`, and `data/<family>/part-NNNN.json`. A shard file uses a 32-byte `N83FB02` header followed by selector-index/y records and zstd compression; the exact record format is in the adjacent `FORMAT.json` and `RUN_OUTPUT_FORMAT.json`.
