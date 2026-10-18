# Executed root-phase commands

The accepted worker source is the clean pre-stack commit
`43c9850a73f1d960f65ed470fdb8adac981a5ffe`. The protocol was
committed before solver edits. The current arm uses the same exact retained
object, four summand columns, two intermediate x values and target x as the
prior complete-witness study.

The source and input checks ran from the execution checkout and panel:

```sh
sh research/n83_k16627_m4_root_phase_20261018/prepare-source.sh /private/tmp/n83-k16627-m4-root-source-v3
cd /private/tmp/n83-k16627-m4-panel
sha256sum -c /private/tmp/crypto-n83-k16627-planted/research/n83_k16627_m4_root_phase_20261018/inputs-before-run.sha256
```

The successful native tests used
`CARGO_TARGET_DIR=/Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-planted`:

```sh
cargo test --release --lib root_propagation -- --test-threads=2
cargo test --release --lib named_complete_witness_pins_all_four_summand_chain_blocks -- --test-threads=2
cargo test --locked --no-default-features --example koblitz_n83_factor_base_export
rustfmt --check --edition 2021 src/cryptanalysis/sat.rs src/cryptanalysis/binary_semaev_chain_sat.rs research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs examples/koblitz_n83_factor_base_export.rs
```

The accepted offline arm64 release build used the pinned image:

```sh
docker run --name n83-k16627-root-build-v3-20261018 --platform linux/arm64 \
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

The separately capped root-phase arm started with a fresh `/out` directory.
Docker stdout and stderr are retained separately; the host-side invocation
was:

```sh
docker run --name n83-k16627-root-positive-20261018 --platform linux/arm64 \
  --cpus=1 --memory=4096m --memory-swap=4096m --network=none --read-only \
  --tmpfs /tmp:rw,size=64m \
  -v /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-witness-linux/release/examples/koblitz_n83_factor_base_export:/worker:ro \
  -v /private/tmp/n83-k16627-m4-panel:/panel:ro \
  -v /private/tmp/n83-k16627-m4-root-source-v3:/snapshot:ro \
  -v /private/tmp/n83-k16627-m4-root-positive:/out \
  -e ICV1_FROZEN_SOURCE_DIR=/snapshot \
  rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
  sh -c 'date -u +%FT%TZ > /out/start.utc; timeout -s TERM -k 5 1800 /worker primary-orbit-chain-root-phase /panel 4096 /out/worker.json; result=$?; cat /sys/fs/cgroup/memory.peak > /out/peak.bytes; date -u +%FT%TZ > /out/end.utc; exit "$result"'
```

The local paths above are execution coordinates. Source revision, exact
file hashes, retained-object digest, image digest and binary SHA-256 are
the durable identities. Earlier failed build attempts are recorded in
[`BUILD_RECEIPT.md`](BUILD_RECEIPT.md), not omitted from the run history.
