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

The A1/B/A2 degree-83 sequence used [run-arm.sh](run-arm.sh) with fresh output directories and container names. The script fixes one CPU, hard 4 GiB, zero swap, offline execution, 1,800-second worker cap, read-only worker/panel/snapshot mounts, container state and host logs. In the committed execution worktree, the three calls were:

```sh
sh research/n83_k16627_m4_selector_only_20261018/run-arm.sh full n83-k16627-selector-only-a1-20261018 research/n83_k16627_m4_selector_only_20261018/arms/a1-full /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-witness-linux/release/examples/koblitz_n83_factor_base_export /private/tmp/n83-k16627-m4-panel /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-selector-only-source-v2-20261018
sh research/n83_k16627_m4_selector_only_20261018/run-arm.sh selector-only n83-k16627-selector-only-b-20261018 research/n83_k16627_m4_selector_only_20261018/arms/b-selector-only /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-witness-linux/release/examples/koblitz_n83_factor_base_export /private/tmp/n83-k16627-m4-panel /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-selector-only-source-v2-20261018
sh research/n83_k16627_m4_selector_only_20261018/run-arm.sh full n83-k16627-selector-only-a2-20261018 research/n83_k16627_m4_selector_only_20261018/arms/a2-full /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-witness-linux/release/examples/koblitz_n83_factor_base_export /private/tmp/n83-k16627-m4-panel /Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-selector-only-source-v2-20261018
```

All three returned exit 0, `OOMKilled=false`, one checked relation and the same rank-one row. B returned before its cap, so the protocol's conditional C phase-only command was not executed. Each invocation's host stdout and stderr, exact UTC interval, cgroup peak, container state and configuration, worker JSON, phase stream and progress stream are retained in [arms/](arms/).

The separate native relation verifier was compiled with `cargo build --release --locked --no-default-features --example n83_selector_receipt_replay`. Each arm used the same command shape with `full` or `selector-only` as appropriate:

```sh
/Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-selector-only-build/release/examples/n83_selector_receipt_replay \
  48ebe3b370810e6da2d48e5234e42ba92bba259b selector-only \
  /private/tmp/n83-k16627-m4-panel/objects/1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d.jsonl.gz \
  research/n83_k16627_m4_selector_only_20261018/arms/b-selector-only/worker.json \
  research/n83_k16627_m4_selector_only_20261018/replay/b.json
```

The A1 and A2 replays substituted `full`, their arm directories and their new output files. For each arm, `jq` generated one copied worker receipt with `x_decimal[0]="0"` and one with a zero `relation.dense_decimal_blake3`; the same verifier rejected all six with exit 1 and no success output. [NEGATIVE_CHECKS.md](replay/NEGATIVE_CHECKS.md) and the retained stderr files record the exact guards. The [report renderer](../../examples/n83_selector_only_report.rs) was run as `cargo run --release --locked --no-default-features --example n83_selector_only_report -- research/n83_k16627_m4_selector_only_20261018`; its initial missing-replay-path failure is retained in [report-generation-first.log](report-generation-first.log), and its accepted invocation in [report-generation.log](report-generation.log). It produced [summary.json](summary.json) and [the SVG](selector-only-comparison.svg) from the raw records. Typst rendered the same SVG into the 150-dpi PNG and both PDFs. The study files were copied byte for byte into `benchmarks/20261018-n83-k16627-selector-only/`.
