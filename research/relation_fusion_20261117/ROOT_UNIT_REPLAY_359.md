# Replay source348 root qualification

Packet360 preserves the seven current root source files, exact before344 contexts, before-import348 PKM source, every new raw compiler/test/example log, scripts, native352's complete181 files and the frozen saved-proof checker. It is a delta over published trusted parent95f6003d167c91f6e40e82980f00a67b5adf75b2 and original packet337; it does not redistribute the557MiB unit executable or every build dependency. Report358 states scope/limits. No performance result. [pcc:unverified]

Check `root-unit-qualification-360.tar.xz.sha256`, extract into a fresh directory and verify every indexed path/size/SHA-256, with index.json the sole unindexed file. Source348's1452 pins resolve by using source-delta/ first, then the source/ view in assembled/extracted337. Source345 uses the same view except its PKM file comes from contexts/pkm-before-import-348.rs. Every original pin remains unchanged.

Replay the36 current saved native proofs and two semantic canaries:

```sh
taskset -c 0 bin/flat-root-audit native-352 references-85 \
  study/MATCHED_PINS_BEFORE_85.json /tmp/root-unit-proof-replay.json
cmp /tmp/root-unit-proof-replay.json study/FLAT_ROOT_AUDIT_331.json
```

For actual source rebuilding, start a clean checkout of the published parent above with its pinned local dependencies/submodules and Cargo locks; copy source-delta/ over it. The delta includes unchanged native root-integration and small source-only example harness manifests/locks. Check source348 and example357 pins before executing. Use one CPU/numerical/test thread:

```sh
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_BUILD_JOBS=1
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
export RUST_TEST_THREADS=1
taskset -c 0 cargo test --release --locked --lib --no-run
```

Run the exact ten module filters in ROOT_RECOVERED_UNIT_PLAN_346.json with the generated root unit executable and `--test-threads=1`, as runner350 does. Expected declared scope:338pass,41existingignored,7837outside scope. Then use ROOT_API_QUALIFICATION_PLAN_356.json; require every filter to resolve with `--list` before invoking, expected16pass. Failed354's wrong module path is retained separately and executes zero tests. Do not force ignored benchmarks or infer a full-suite pass. [pcc:unverified]

Run the unchanged actual native matrix with a fresh output directory:

```sh
RELATION_ROOT_INTEGRATION_OUT=/tmp/root-native-rebuilt \
  taskset -c 0 cargo test --release --locked \
  --manifest-path research/relation_fusion_20261117/root-integration/Cargo.toml \
  --test flat_constructor -- --test-threads=1
```

Every181 native output file must equal saved native-352, with36cases,252mutation assertions and two refusals. Original observed Fp solving degree stays null. Native352 is explicitly default-features=false, distinct from default-feature root units. [pcc:unverified]

Run the original example main with both documented argument sets, via the small harness using actual crypto and identical native352 lock resolution:

```sh
taskset -c 0 cargo run --release --locked \
  --manifest-path research/relation_fusion_20261117/root-example-tests/Cargo.toml \
  --bin prime-pair-table-check -- 16 120
taskset -c 0 cargo run --release --locked \
  --manifest-path research/relation_fusion_20261117/root-example-tests/Cargo.toml \
  --bin prime-pair-table-check -- 36 512
```

Require exit0, identical=true, word_path_used=true and equality of all BigUint reference pairs/indices/counts. Original raw outputs are saved. No setup cost, unsigned executable hash, synthetic fixture or component check admits pipeline speed. LocalCPU0/one thread/zero cloud and checkpoint/S3 waiver persist. Canonical protocol access remains required before real comparative timing. [pcc:unverified]
