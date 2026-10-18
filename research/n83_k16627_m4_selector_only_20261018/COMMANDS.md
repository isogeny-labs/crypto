# Selector-only comparison commands

The protocol was committed as `3ed109bd24` before implementation. The accepted worker source was frozen as `48ebe3b370810e6da2d48e5234e42ba92bba259b` before degree-83 execution. [BUILD_RECEIPT.md](BUILD_RECEIPT.md) binds its source and input files, image, binary and build state.

```sh
sh research/n83_k16627_m4_selector_only_20261018/prepare-source.sh /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-selector-only-source-v2-20261018
cd /private/tmp/n83-k16627-m4-panel
sha256sum -c /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-selector-only-worktree/research/n83_k16627_m4_selector_only_20261018/inputs-before-run.sha256
```

The pinned offline build used this command, with stdout and stderr redirected to the study directory:

```sh
docker run --name n83-k16627-selector-only-build-20261018 --platform linux/arm64 \
  --cpus=4 --memory=8g --memory-swap=8g --network=none --read-only \
  --tmpfs /tmp:rw,size=512m --tmpfs /cargo:rw,size=64m \
  -v /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-selector-only-worktree:/src:ro \
  -v /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-witness-linux:/target \
  -v /Users/adamburan/.cargo/registry:/cargo/registry:ro \
  -v /Users/adamburan/.cargo/git:/cargo/git:ro \
  -e CARGO_HOME=/cargo -e CARGO_TARGET_DIR=/target -e CARGO_NET_OFFLINE=true \
  -w /src \
  rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
  timeout -s TERM -k 5 1200 cargo build --release --offline --locked \
  --no-default-features --example koblitz_n83_factor_base_export
```

Each planned A1, B, A2, and conditional C degree-83 arm uses [run-arm.sh](run-arm.sh) with a fresh output directory and container name. Exact calls and outcomes will be appended after execution. The script fixes one CPU, hard 4 GiB, zero swap, offline execution, 1,800-second worker cap, read-only worker/panel/snapshot mounts, container state and host logs.
