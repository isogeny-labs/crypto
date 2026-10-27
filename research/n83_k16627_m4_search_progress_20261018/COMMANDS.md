# Executed N83 search-progress commands

The accepted native worker source is commit `2e1ba2e4763242bb4c8870e22f93b0183c7b30f7`, built and run after the protocol commit `c158b99cba`. The panel and source snapshot paths below are execution coordinates; the committed SHA-256 manifests, source revision, object BLAKE3 and worker binary SHA-256 are durable identities.

```sh
sh research/n83_k16627_m4_search_progress_20261018/prepare-source.sh /private/tmp/n83-k16627-m4-progress-source-v1
cd /private/tmp/n83-k16627-m4-panel
sha256sum -c /private/tmp/crypto-n83-k16627-progress/research/n83_k16627_m4_search_progress_20261018/inputs-before-run.sha256
```

The accepted release worker was built offline with `cargo build --release --offline --locked --no-default-features --example koblitz_n83_factor_base_export` inside the pinned arm64 `rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31` image, with the `n83-witness-linux` SSD target directory, read-only source checkout and local Cargo registry mounts. [BUILD_RECEIPT.md](BUILD_RECEIPT.md) retains its exact interval, image, binary digest and container state.

The bounded positive arm used a fresh output directory and this Docker invocation; host stdout and stderr were redirected to separate retained files:

```sh
docker run --name n83-k16627-progress-positive-20261018 --platform linux/arm64 \
  --cpus=1 --memory=4096m --memory-swap=4096m --network=none --read-only \
  --tmpfs /tmp:rw,size=64m \
  -v /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-witness-linux/release/examples/koblitz_n83_factor_base_export:/worker:ro \
  -v /private/tmp/n83-k16627-m4-panel:/panel:ro \
  -v /private/tmp/n83-k16627-m4-progress-source-v1:/snapshot:ro \
  -v /private/tmp/n83-k16627-m4-progress-positive:/out \
  -e ICV1_FROZEN_SOURCE_DIR=/snapshot \
  rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
  sh -c 'date -u +%FT%TZ > /out/start.utc; timeout -s TERM -k 5 1800 /worker primary-orbit-chain-search-progress /panel 4096 /out/worker.json; result=$?; cat /sys/fs/cgroup/memory.peak > /out/peak.bytes; date -u +%FT%TZ > /out/end.utc; exit "$result"'
```

After exit, `docker inspect --format '{{json .State}}' n83-k16627-progress-positive-20261018` and `docker inspect --format '{{json .HostConfig}}' n83-k16627-progress-positive-20261018` were retained as `positive/container-state.json` and `positive/host-config.json`. All seven worker/host output files were copied byte-for-byte into `positive/`, checked with `cmp`, and hashed in `positive-files.sha256`. No `worker.json` exists.

The checked capped outcome was derived from the copied files:

```sh
jq -n \
  --slurpfile state positive/container-state.json \
  --slurpfile host positive/host-config.json \
  --slurpfile phases positive/worker.json.phases.jsonl \
  --slurpfile progress positive/worker.json.progress.jsonl \
  --rawfile peak positive/peak.bytes \
  --rawfile start positive/start.utc \
  --rawfile end positive/end.utc \
  -f derive-capped-outcome.jq > positive/outcome.json
sha256sum positive/* > positive-files.sha256
sha256sum -c positive-files.sha256 > positive-files-check.log
```

The relevant source tests and exporter example suite are recorded in [VALIDATION.md](VALIDATION.md). The required full release library command, started after the container ended, is `CARGO_TARGET_DIR=/Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-progress cargo test --release --lib -- --test-threads=2`. Its raw log is `cargo-release-lib-prestack.log`.
