# Native execution and verification amendment

The original export, replay, custody and overlap receipts remain historical
evidence. Their Python guard and verifier source snapshots are retained as
non-executable text under `historical/`. The active execution path uses
`examples/n83_d18_k2048_guard.rs` for the bounded Docker runner and
`examples/n83_d18_k2048_native.rs` for S3 custody and full-row U/D overlap
verification. The frozen `PROTOCOL.md` describes the original execution;
this amendment supersedes its Python runner instruction for future replay.

The native guard pins exporter source commit
`a1e6b778dcee8f4ec72f359df7ce3728b302b78a`, Linux arm64 exporter
binary SHA-256
`a0f9b7fe67ffea4816d077b8017e3ed4512cc36fd04689e3eae1257975831a5d`,
and Docker image ID
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`.
It requires a clean checkout of that source commit, mounts it read-only,
enforces one CPU, 4 GiB memory, zero swap, no network and a 900-second
container plus host watchdog, then records Docker exit/OOM/cgroup receipts.
The Rust guard replayed both export and independent replay in a separate
scratch output directory. Its [export](evidence/export-native-guard.json) and
[replay](evidence/replay-native-guard.json) guards passed. The reproduced
compressed object SHA-256 and point-set BLAKE3 match the historical run.

The native [custody](evidence/s3-custody-native.json) and
[overlap](evidence/structural-overlap-native.json) receipts also passed.
After excluding only the versioned schema and native verifier identity, each
receipt's result fields matched its historical counterpart byte for byte in
sorted JSON. The native overlap audit checked all 339,968 point rows in each
archive, uniqueness and 2,048-column coverage, yielding zero common points.
The S3 verifier checked the local and fresh object digests, manifest/replay
bytes, and S3 HEAD size and AES256 encryption. The original archive is
unchanged. The source-linked [visual report](reports/run-20261027T1915Z/REPORT.md)
and [PDF](reports/run-20261027T1915Z/REPORT.pdf) present the result.
The [native negative controls](evidence/native-negative-controls.json)
rejected a copied archive with its first byte changed and a copied manifest
with its policy changed, without writing success receipts.

The historical guard's 5.594 s export and 3.602 s replay outer times and the
native guard's 4.623 s and 3.240 s are separate shared-host setup observations.
They are retained with provenance and do not form a speed comparison.
