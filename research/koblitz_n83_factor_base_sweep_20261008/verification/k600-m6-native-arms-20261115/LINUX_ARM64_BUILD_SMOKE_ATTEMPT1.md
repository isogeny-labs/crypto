# Linux ARM64 build smoke: attempt 1

**Observed status: stopped by container OOM.** This is the bounded
engineering smoke frozen in [the protocol](LINUX_ARM64_BUILD_SMOKE_PROTOCOL.md),
not a K600 index-calculus run. The exact source HEAD was
`3ff510d8a280ee634346638a9e23830d2cb6e20d`; the tracked source
and `Cargo.lock` were mounted read-only. The base image ID was
`sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`
(`linux/arm64`, Rust 1.99.1). Local Docker Desktop reported
7,529,156,608 bytes of Linux RAM. Start was 2026-11-15 16:55:44 UTC;
the operator stopped the still-running container at 17:00:53 UTC
after Docker reported `OOMKilled=true` and the log showed
`rustc` for `crypto` terminated by SIGKILL. The `docker run` command
exited 137. The five requested executable outputs were absent.

The exact host invocation was:

```sh
docker run --rm --name n83-arm64-build-smoke-3ff510d8 \
  --platform linux/arm64 --memory 6g --cpus 4 --pids-limit 512 \
  --env CARGO_TARGET_DIR=/build-target \
  --mount type=bind,src=/Volumes/SSD990-2/tmp/n83-native-rank-query-wt,dst=/Volumes/SSD990-2/tmp/n83-native-rank-query-wt,readonly \
  --mount type=bind,src=/Volumes/SSD990/crypto/.git,dst=/Volumes/SSD990/crypto/.git,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019,dst=/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/tmp/n83-arm64-build-smoke-20261115-3ff510d8,dst=/build-target \
  --workdir /Volumes/SSD990-2/tmp/n83-native-rank-query-wt \
  rust@sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c \
  sh -c 'timeout 1200 cargo build --release --locked --no-default-features -j 2 \
    --example n83_k600_m6_linux_build --example n83_k600_m6_native_arms \
    --example n83_k600_m6_native_preflight --example n83_k600_m6_budget \
    --example koblitz_n83_mixed_orbit_worker'
```

The [raw compressed log](linux-arm64-build-smoke-attempt1.log.gz) is
20,213 bytes uncompressed, SHA-256
`3d9f458a9800ffaed71742b49f021b3c9d2331436840597304d28f53d303f82a`;
the [pre-stop Docker state](linux-arm64-build-smoke-attempt1.state)
records `running true 0` for status, OOM flag, and exit code before
`docker stop --time 5`. The log shows dependency compilation,
then the `crypto` library `rustc` command killed by signal 9.
Peak sampled `docker stats` memory reached 6 GiB of 6 GiB.
Scratch free space stayed above the 1 GiB stop threshold; the last
observed value was 2.2 GiB. A 906 MiB partial target directory
remains local outside the repository. It is not a durable result
artifact and was not counted as a completed build.

The [diagram](linux-arm64-build-smoke-attempt1.svg) records the
observed path. This run gives a concrete resource failure under the
frozen 6 GiB, two-job conditions. It does not establish whether one
build job or a larger host can compile the same revision. A
single-job retry must be versioned separately, preserve this failed
receipt, and state whether it reuses the partial target cache. The
source-bound `charge build` receipt, runtime image, solver, zero-trial
preflight and K600 arms remain pending. No canonical performance graph
was updated: no IC runtime or factor-base value was measured.
