# Bounded authentic validation: compile failure

One explicitly authorized local attempt ran against `1cfe91d3173f41a0f7d8448f4afe278cf0d9663f`, with two build jobs, a 1,560-second shared deadline, and zero retries/cloud jobs. The prepared runner and 1,470-input source manifest are preserved in the preceding validation-runner packet. The full authorization, immutable attempt marker, command logs and prior CLI binary are retained in the S3 archive linked by `s3-readback.json`.

`cargo +1.98 test --release --lib --no-run --locked --offline --message-format=json` exited 101 after 323.058 seconds. The entire attempt took 335.692 seconds. Rust reported E0063 at `src/ml_dataset/owned_replay_caller.rs:712`: the retained test fixture initializes `CellManifest` without `schema` and `plan_sha256`. There were 20 warning diagnostics. No registry control or native acceptance cell executed, and no scientific dataset was generated. Cleanup reported no surviving child processes.

After failure, all 1,470 source hashes and command log hashes were checked, and the prior CLI binary matched its retained copy and original digest. The S3 archive was freshly downloaded; all 15 files and all archive bytes matched. Large logs and binaries are kept in S3 rather than duplicated in Git.

`proposed-fixture-fix.patch` supplies the cell schema and the digest of the fixture's existing synthetic `{}` run binding. Inspection of `CellManifest`, the retained fixture and `Session::verify` identified the missing fields. `git apply --check` passed. The patch is deliberately unapplied and uncompiled; it is a proposed correction, not a successful validation. The one-attempt allowance is consumed.

For replay, retrieve the content-addressed URI in `s3-readback.json`, verify its SHA-256 and byte count, then compare every member with `S3_CONTENTS.json`. Do not restart the consumed attempt or remove its marker. A further build/native attempt requires a new bounded authorization and source publication binding. No new scientific campaign authority is implied.
