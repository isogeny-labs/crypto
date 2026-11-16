# Executed complete-witness commands

The worker source was the clean pre-stack commit
`f68b9a94ada669360f88be4d8c8a5288cca55d73`; the three touched Rust
files are byte-identical to the stacked review branch. The following native
gates ran from `/private/tmp/crypto-n83-k16627-planted` with
`CARGO_TARGET_DIR=/Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-planted`:

```sh
cargo check --locked --no-default-features --example koblitz_n83_factor_base_export
cargo fmt --all -- --check
cargo test --locked --no-default-features --lib named_complete_witness_pins_all_four_summand_chain_blocks -- --exact --nocapture
cargo test --locked --no-default-features --lib named_complete_witness_pins_all_four_summand_chain_blocks -- --nocapture
cargo test --locked --no-default-features --example koblitz_n83_factor_base_export
```

The first core-test command selected zero tests; the rerun selected one and
passed. The five retained inputs were checked from
`/private/tmp/n83-k16627-m4-panel` with `sha256sum -c
/private/tmp/crypto-n83-k16627-planted/research/n83_k16627_m4_witness_20261018/inputs-before-run.sha256`.
The five compiled source files were checked from the source checkout with
the study's `source-files.sha256` and from
`/private/tmp/n83-k16627-m4-witness-source` with its copied manifest. The
checks and exact hash lists are in this study.

The offline Linux release build used the pinned image and SSD build target:

```sh
docker run --name n83-k16627-witness-build-20261018 --platform linux/arm64 \
  --cpus=4 --memory=8g --memory-swap=8g --network=none --read-only \
  --tmpfs /tmp:rw,size=512m --tmpfs /cargo:rw,size=64m \
  -v /private/tmp/crypto-n83-k16627-planted:/src:ro \
  -v /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-witness-linux:/target \
  -v /Users/adamburan/.cargo/registry:/cargo/registry:ro \
  -v /Users/adamburan/.cargo/git:/cargo/git:ro \
  -e CARGO_HOME=/cargo -e CARGO_TARGET_DIR=/target -e CARGO_NET_OFFLINE=true \
  -w /src \
  rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
  timeout -s TERM -k 5 1200 cargo build --release --offline --locked \
  --no-default-features --example koblitz_n83_factor_base_export
```

The separately capped positive arm was launched with an empty `/out`
directory and the read-only source snapshot:

```sh
docker run --name n83-k16627-witness-positive-20261018 --platform linux/arm64 \
  --cpus=1 --memory=4096m --memory-swap=4096m --network=none --read-only \
  --tmpfs /tmp:rw,size=64m \
  -v /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-witness-linux/release/examples/koblitz_n83_factor_base_export:/worker:ro \
  -v /private/tmp/n83-k16627-m4-panel:/panel:ro \
  -v /private/tmp/n83-k16627-m4-witness-source:/snapshot:ro \
  -v /private/tmp/n83-k16627-m4-witness-positive:/out \
  -e ICV1_FROZEN_SOURCE_DIR=/snapshot \
  rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
  sh -c 'date -u +%FT%TZ > /out/start.utc; timeout -s TERM -k 5 1800 /worker primary-orbit-chain-witness /panel 4096 /out/worker.json; result=$?; cat /sys/fs/cgroup/memory.peak > /out/peak.bytes; date -u +%FT%TZ > /out/end.utc; exit "$result"'
```

Docker stdout and stderr were redirected separately to
`positive.stdout.log` and `positive.stderr.log`. The output directory's
timestamps, phase JSONL, worker JSON if produced, cgroup peak, container
state and host configuration are preserved under `positive/` after the arm
ends. The local absolute paths here are execution coordinates; the source
commit, byte hashes and retained-base receipt are the durable identities.
