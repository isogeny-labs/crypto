# Executed native planted-control commands

The source checkout was clean at commit `1322cb5681d31849a2f33a6b47e16f9e3d598e0a` when the five-file snapshot was copied and checked. The five staged input SHA-256 values were checked from `/private/tmp/n83-k16627-m4-panel` with `sha256sum -c inputs-before-run.sha256`; the source snapshot was checked from `/private/tmp/n83-k16627-m4-planted-source` with `sha256sum -c source-files.sha256`. Both returned `OK` for every file. These local directories are execution coordinates; [the hashes](BUILD_RECEIPT.md), source commit and retained-base storage receipt provide durable identities.

The native macOS source and example gates were:

```sh
CARGO_TARGET_DIR=/Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-planted cargo check --locked --no-default-features --example koblitz_n83_factor_base_export
CARGO_TARGET_DIR=/Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-planted cargo test --locked --no-default-features --example koblitz_n83_factor_base_export
cargo fmt --all -- --check
```

The Linux release build ran from `/private/tmp/crypto-n83-k16627-planted`:

```sh
docker run --name n83-k16627-planted-build-20261018 --platform linux/arm64 \
  --cpus=4 --memory=8g --memory-swap=8g --network=none --read-only \
  --tmpfs /tmp:rw,size=512m --tmpfs /cargo:rw,size=64m \
  -v /private/tmp/crypto-n83-k16627-planted:/src:ro \
  -v /private/tmp/n83-structured-linux-target:/target \
  -v /Users/adamburan/.cargo/registry:/cargo/registry:ro \
  -v /Users/adamburan/.cargo/git:/cargo/git:ro \
  -e CARGO_HOME=/cargo -e CARGO_TARGET_DIR=/target -e CARGO_NET_OFFLINE=true \
  -w /src \
  rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
  timeout -s TERM -k 5 1200 cargo build --release --offline --locked \
  --no-default-features --example koblitz_n83_factor_base_export
```

The pinned positive arm ran in a fresh container. Its stdout/stderr were redirected to `pinned/`; the shell inside the container recorded UTC timestamps and cgroup memory peak even on a nonzero worker exit. The output directory was empty before launch.

```sh
docker run --name n83-k16627-planted-pinned-20261018 --platform linux/arm64 \
  --cpus=1 --memory=4096m --memory-swap=4096m --network=none --read-only \
  --tmpfs /tmp:rw,size=64m \
  -v /private/tmp/n83-structured-linux-target/release/examples/koblitz_n83_factor_base_export:/worker:ro \
  -v /private/tmp/n83-k16627-m4-panel:/panel:ro \
  -v /private/tmp/n83-k16627-m4-planted-source:/snapshot:ro \
  -v /private/tmp/n83-k16627-m4-planted-pinned:/out \
  -e ICV1_FROZEN_SOURCE_DIR=/snapshot \
  rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
  sh -c 'date -u +%FT%TZ > /out/start.utc; timeout -s TERM -k 5 1800 /worker primary-orbit-chain-planted /panel pinned 4096 /out/worker.json; result=$?; cat /sys/fs/cgroup/memory.peak > /out/peak.bytes; date -u +%FT%TZ > /out/end.utc; exit "$result"'
```

The container exited 124 with `OOMKilled=false`; no `/out/worker.json` was created. The complete [container state](pinned/container-state.json), [host config](pinned/host-config.json), [outer outcome](pinned/outcome.json), timestamps, peak and empty stdout/stderr are retained. The unpinned arm was **not run** because the protocol requires a valid pinned positive result first.
