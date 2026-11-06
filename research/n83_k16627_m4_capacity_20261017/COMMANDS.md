# Executed native capacity commands

The commands below ran on 2026-10-17 UTC from the clean source checkout at
`/private/tmp/crypto-n83-k16627`. These local paths are execution
coordinates; the content hashes and the S3 URI in the linked K=16,627
construction study identify the reusable inputs. The pinned source snapshot
was copied from the five files listed in `source-files.sha256`, checked with
`cmp`, and given `source-attestation.json` before either worker. The worker
binary SHA-256 was
`2f05a22a6365768e138c64f98275e1fe9a3d1dbe1ec1d780603c107260b3fd0a`.

The touched exporter example was tested with:

```sh
CARGO_TARGET_DIR=/private/tmp/n83-structured-m4-target \
  cargo test --release --example koblitz_n83_factor_base_export
```

The separate repository-wide release gate was started with:

```sh
CARGO_TARGET_DIR=/private/tmp/n83-structured-m4-target \
  cargo test --release --lib -- --test-threads=2
```

The arm64 worker build ran with read-only source and Cargo caches, a
writable target, `--network=none`, four CPUs, 8 GiB memory and zero swap.
Its full invocation was:

```sh
docker run --name n83-k16627-m4-build-20261017 --platform linux/arm64 \
  --cpus=4 --memory=8g --memory-swap=8g --network=none --read-only \
  --tmpfs /tmp:rw,size=512m --tmpfs /cargo:rw,size=64m \
  -v /private/tmp/crypto-n83-k16627:/src:ro \
  -v /private/tmp/n83-structured-linux-target:/target \
  -v /Users/adamburan/.cargo/registry:/cargo/registry:ro \
  -v /Users/adamburan/.cargo/git:/cargo/git:ro \
  -e CARGO_HOME=/cargo -e CARGO_TARGET_DIR=/target -e CARGO_NET_OFFLINE=true \
  -w /src \
  rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
  timeout -s TERM -k 5 600 cargo build --release --offline --locked \
  --no-default-features --example koblitz_n83_factor_base_export \
  > research/n83_k16627_m4_capacity_20261017/linux-build.stdout.log \
  2> research/n83_k16627_m4_capacity_20261017/linux-build.stderr.log
```

The exact negative-cap invocation was:

```sh
docker run --name n83-k16627-m4-neg-20261017 --platform linux/arm64 \
  --cpus=1 --memory=4096m --memory-swap=4096m --network=none --read-only \
  --tmpfs /tmp:rw,size=64m \
  -v /private/tmp/n83-structured-linux-target/release/examples/koblitz_n83_factor_base_export:/worker:ro \
  -v /private/tmp/n83-k16627-m4-panel:/panel:ro \
  -v /private/tmp/n83-k16627-m4-source:/snapshot:ro \
  -v /private/tmp/n83-k16627-m4-negative:/out \
  -e ICV1_FROZEN_SOURCE_DIR=/snapshot \
  rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
  sh -c 'date -u +%FT%TZ > /out/start.utc; timeout -s TERM -k 5 1800 /worker primary-orbit-chain-build /panel 16627 public_x_hash 2026100801 4 0 1 6000000 4096 /out/worker.json; result=$?; cat /sys/fs/cgroup/memory.peak > /out/peak.bytes; date -u +%FT%TZ > /out/end.utc; exit "$result"' \
  > /private/tmp/n83-k16627-m4-negative/stdout.log \
  2> /private/tmp/n83-k16627-m4-negative/stderr.log
```

The exact full-cap invocation changed only the fresh container/output and
`max_variables` argument:

```sh
docker run --name n83-k16627-m4-full-20261017 --platform linux/arm64 \
  --cpus=1 --memory=4096m --memory-swap=4096m --network=none --read-only \
  --tmpfs /tmp:rw,size=64m \
  -v /private/tmp/n83-structured-linux-target/release/examples/koblitz_n83_factor_base_export:/worker:ro \
  -v /private/tmp/n83-k16627-m4-panel:/panel:ro \
  -v /private/tmp/n83-k16627-m4-source:/snapshot:ro \
  -v /private/tmp/n83-k16627-m4-full:/out \
  -e ICV1_FROZEN_SOURCE_DIR=/snapshot \
  rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
  sh -c 'date -u +%FT%TZ > /out/start.utc; timeout -s TERM -k 5 1800 /worker primary-orbit-chain-build /panel 16627 public_x_hash 2026100801 4 0 250000 6000000 4096 /out/worker.json; result=$?; cat /sys/fs/cgroup/memory.peak > /out/peak.bytes; date -u +%FT%TZ > /out/end.utc; exit "$result"' \
  > /private/tmp/n83-k16627-m4-full/stdout.log \
  2> /private/tmp/n83-k16627-m4-full/stderr.log
```

Both containers were inspected for exit/OOM state before removal. The
full arm's Docker CPU, memory, swap, network and root settings are retained
in `full/host-config.json`. `negative/` and `full/` hold the exact worker
JSON, stdout/stderr, timestamps, cgroup peak and state from each run.
