# Provenance of the counted-S v2 run (build, command, audit receipts)

Added after `VALIDATOR.md` found that the build chain was recorded only in S3. Nothing here changes a result.

## Files
- `pipeline.sh`: the exact restartable script that built the two binaries, ran C-16 and audited C-10-14 and B (copied verbatim from `s3://setec-astronomy-artifacts/validation/hashed-ecdl-counts/20261031/v2/pipeline.sh`). Its paths (`/home/user/wt-counts`, `/dev/shm`) are those of the producing host.
- `rustc-c.txt`, `rustc-b.txt`: `rustc -vV` of the two builds.
- `receipts/{C-10-14,B}.json` and `receipts/verify-{C-10-14,B}.log`: `ecbench verify --replay-all` receipts (ecbench.audit/v1) with the session file hashes. `receipts/verify-C-16.log`: the same audit for C-16 (the pipeline did not write a JSON receipt for it; `decide.py` ran the audit and kept this output as `raw/verify-C-C-16.txt`).

## Builds (measured; both are producer rebuilds, not the binaries that wrote the sessions)
| binary | source commit | toolchain | flags | sha256 |
|---|---|---|---|---|
| `ecbench-C` | `4df337e64d59bbe0145c204048ab1dc69a43c187` | rustc 1.97.0 (2d8144b78 2026-07-07), host x86_64-unknown-linux-gnu | `cargo build --locked --release --bin ecbench` | `5cec72d2bd645dd9371213ea63a5f32353c24c0c12c1fe21ce15968ec6cdf04f` |
| `ecbench-B` | `3ae1ac2bd63d3fb0e86b7e8c0af2b5ec0f1bd09d` | rustc 1.99.1 (58cf8f9c3 2026-10-13), musl | `cargo +1.99.1 build --locked --release --no-default-features --target x86_64-unknown-linux-musl --bin ecbench`, `CC_x86_64_unknown_linux_musl=gcc` | `603c6c863e4658f36181b85d76935138eec48d5bd939799bc6dd8bdd595d4f4b` |

The S3 copies of both binaries (`.../v2/bin/`) were re-hashed on 2026-11-02 and match both table rows and the `auditor_binary_sha256` of the receipts (env class `ECBENV3h13e37662a118`).

## Source pin for B
`3ae1ac2bd63d3fb0e86b7e8c0af2b5ec0f1bd09d` is reachable only from branch `codex/hashed-ecdl-ecbench-e2e` (it is not on main). Until it is merged or tagged, that branch is the only place the B source lives; do not delete it.

## What the rebuilds mean
The original C-10-14 binary (679cdc2f...) and B binary (a133714d..., glibc, rustc 1.97.0) no longer exist, and all three sessions record `git_commit = null`. Replay checks the recovered scalar, exhaustion flag, bit-exact total and per-phase GAE, counters and factor base, and reproduced all 416 executions with these different builds, so the counts are properties of the algorithm. The binary-to-commit chain of custody for the original sessions is still not established. C-16 was run by the rebuilt `ecbench-C` itself.

## Raw sessions
`s3://setec-astronomy-artifacts/validation/hashed-ecdl-counts/20261031/v2/sessions/{B,C-10-14,C-16}/` (all three, hashes match the receipts). They are not in git (about 3.4 MB); a validation gate row would need them committed.
