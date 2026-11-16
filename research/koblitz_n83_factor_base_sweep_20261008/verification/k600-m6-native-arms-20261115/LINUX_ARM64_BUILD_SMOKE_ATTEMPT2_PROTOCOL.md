# Linux ARM64 build smoke: attempt 2 protocol

Frozen before execution. This is a **warm-target engineering retry** of
[attempt 1](LINUX_ARM64_BUILD_SMOKE_ATTEMPT1.md), whose two-job build
hit the 6 GiB Docker memory cap. It is not an independent cold timing
sample or a K600 IC measurement.

Use the clean committed HEAD containing this protocol; record its
full SHA-1 before launch. Keep the same pinned Rust 1.99.1 arm64 image
`sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`,
five examples, `--release --locked --no-default-features`, 6 GiB
container RAM, four assigned CPUs, 512 PID limit, 1,200-second cap,
and read-only source/Git mounts as attempt 1. Change Cargo concurrency
from `-j 2` to `-j 1`. Reuse the 906 MiB partial target directory
`/Volumes/SSD990-2/tmp/n83-arm64-build-smoke-20261115-3ff510d8`;
record this cache reuse and do not treat any resulting wall interval as
a cold build cost. The macOS release target was separately removed with
`cargo clean --release` (3,481 files, 2.7 GiB) to restore local scratch
headroom; no source file was changed by this cleanup.

The container command is:

```sh
timeout 1200 cargo build --release --locked --no-default-features -j 1 \
  --example n83_k600_m6_linux_build \
  --example n83_k600_m6_native_arms \
  --example n83_k600_m6_native_preflight \
  --example n83_k600_m6_budget \
  --example koblitz_n83_mixed_orbit_worker
```

Success: Docker exit 0 and all five executable outputs with Linux ARM64
ELF headers; record SHA-256 and sizes. Stop on container OOM, timeout,
other nonzero exit, or scratch free space below 1 GiB; retain a terminal
failure receipt and partial log. If this succeeds, it establishes only
single-job build feasibility with a warm partial target. The exact
`charge build` receipt, solver binary, runtime image, host-specific
committed build plan, zero-trial preflight, and K600 arm gates still
need independent execution and cost accounting.
