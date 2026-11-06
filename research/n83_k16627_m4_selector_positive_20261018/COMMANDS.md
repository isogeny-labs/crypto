# Executed selector-positive commands

The protocol was committed as `170b555fcc` on the clean execution branch and as `bda450501c` on the review branch before implementation. The accepted native source is clean execution commit `b4beefcb55f05713d1d63468fcbcd21e4339fa50`. Source and input manifests, snapshot checks and the worker binary digest are in [BUILD_RECEIPT.md](BUILD_RECEIPT.md). The first failed build and its different source revision are retained separately.

```sh
sh research/n83_k16627_m4_selector_positive_20261018/prepare-source.sh /private/tmp/n83-k16627-m4-selector-source-v2
cd /private/tmp/n83-k16627-m4-panel
sha256sum -c /private/tmp/crypto-n83-k16627-progress/research/n83_k16627_m4_selector_positive_20261018/inputs-before-run.sha256
```

The accepted offline arm64 build used this command with separate stdout and stderr redirects to the study:

```sh
docker run --name n83-k16627-selector-build-v2-20261018 --platform linux/arm64 \
  --cpus=4 --memory=8g --memory-swap=8g --network=none --read-only \
  --tmpfs /tmp:rw,size=512m --tmpfs /cargo:rw,size=64m \
  -v /private/tmp/crypto-n83-k16627-progress:/src:ro \
  -v /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-witness-linux:/target \
  -v /Users/adamburan/.cargo/registry:/cargo/registry:ro \
  -v /Users/adamburan/.cargo/git:/cargo/git:ro \
  -e CARGO_HOME=/cargo -e CARGO_TARGET_DIR=/target -e CARGO_NET_OFFLINE=true \
  -w /src \
  rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
  timeout -s TERM -k 5 1200 cargo build --release --offline --locked \
  --no-default-features --example koblitz_n83_factor_base_export
```

The degree-83 selector-pinned control started at `2026-10-18T08:32:07Z` with fresh `/out` and ended at `08:32:43Z`, exit 0 without OOM. Host stdout and stderr were redirected to `positive/host-stdout.log` and `positive/host-stderr.log`.

```sh
docker run --name n83-k16627-selector-positive-20261018 --platform linux/arm64 \
  --cpus=1 --memory=4096m --memory-swap=4096m --network=none --read-only \
  --tmpfs /tmp:rw,size=64m \
  -v /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-witness-linux/release/examples/koblitz_n83_factor_base_export:/worker:ro \
  -v /private/tmp/n83-k16627-m4-panel:/panel:ro \
  -v /private/tmp/n83-k16627-m4-selector-source-v2:/snapshot:ro \
  -v /private/tmp/n83-k16627-m4-selector-positive:/out \
  -e ICV1_FROZEN_SOURCE_DIR=/snapshot \
  rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
  sh -c 'date -u +%FT%TZ > /out/start.utc; timeout -s TERM -k 5 1800 /worker primary-orbit-chain-selector-positive /panel 4096 /out/worker.json; result=$?; cat /sys/fs/cgroup/memory.peak > /out/peak.bytes; date -u +%FT%TZ > /out/end.utc; exit "$result"'
```

After exit, `docker inspect --format '{{json .State}}' n83-k16627-selector-positive-20261018` and `docker inspect --format '{{json .HostConfig}}' n83-k16627-selector-positive-20261018` were retained as `positive/container-state.json` and `positive/host-config.json`. Six worker files were copied byte for byte from the execution directory and checked with `cmp`, as [raw-copy-check.log](raw-copy-check.log) records. [positive-files.sha256](positive-files.sha256) binds the resulting raw directory. The exact receipt was separately replayed with:

```sh
CARGO_TARGET_DIR=/Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-progress \
  cargo run --release --locked --no-default-features \
  --example n83_selector_receipt_replay -- \
  /private/tmp/n83-k16627-m4-panel/objects/1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d.jsonl.gz \
  research/n83_k16627_m4_selector_positive_20261018/positive/worker.json \
  research/n83_k16627_m4_selector_positive_20261018/replay/receipt.json
```

The native verifier returned `PASS_RELATION_REPLAY`. Two separate invocations substituted `replay/tampered-x-worker.json` and `replay/tampered-row-worker.json` for the worker receipt; both exited 1 with the retained [stderr records](replay/) and wrote no success file. [replay-files.sha256](replay-files.sha256), [replay-source.sha256](replay-source.sha256) and [replay-binary.sha256](replay-binary.sha256) bind that check.

The source controls passed in release mode: `cargo test --release --lib orbit_choice_observer_preserves_small_chain_group_lift_and_records_overhead -- --nocapture --test-threads=1`, `cargo test --release --lib optional_progress_observer_preserves_sat_and_unsat_search -- --test-threads=1`, and `cargo test --release --lib frobenius_orbit_domain_accepts_exactly_the_small_field_closure -- --test-threads=1`. The touched exporter example suite passed 19/19 with `CARGO_TARGET_DIR=/Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-progress cargo test --release --locked --no-default-features --example koblitz_n83_factor_base_export -- --test-threads=2`. [VALIDATION.md](VALIDATION.md) retains their raw logs and the failed first compile.

The full clean-source library suite used `CARGO_TARGET_DIR=/Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-progress cargo test --release --lib -- --test-threads=2` on execution branch commit `409aa254c1`. Its first [raw log](cargo-release-lib-prestack.log) retains the sandbox loopback and sparse-fixture failures. After materializing tracked `research/cryptopro_b_chain_sweep_20261006/frozen_inputs/chains.constants.json` and permitting local loopback sockets, the identical command passed 4,221 tests with 117 ignored in [the accepted recheck log](cargo-release-lib-prestack-recheck.log).

On the main-derived review head, the same library command with `CARGO_TARGET_DIR=/Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-stacked` and the touched example command with `--example koblitz_n83_factor_base_export` both exited 101 before tests due inherited ecbench/Gaudry compile errors; [their logs](cargo-release-lib-stacked.log) and [example log](exporter-example-tests-stacked.log) are retained. `node --check .github/scripts/ci-scope.mjs` also exited 1 on the unchanged duplicate `root` declaration in [ci-scope-stacked.log](ci-scope-stacked.log). The parent stack must incorporate upstream source repairs and rerun its own final-head gates before merge.
