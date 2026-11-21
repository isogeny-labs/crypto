# Linux ARM64 build smoke: attempt 2

**Observed status: container OOM during single-job `crypto` library
compilation.** This is the warm-target engineering retry frozen in
[the attempt-2 protocol](LINUX_ARM64_BUILD_SMOKE_ATTEMPT2_PROTOCOL.md).
It reused the partial target of [attempt 1](LINUX_ARM64_BUILD_SMOKE_ATTEMPT1.md)
and is not a cold build sample or an index-calculus measurement.

The source was clean committed HEAD
`1ae9333dd8619c73cff1e68b2afbe8519028e9f4`, mounted read-only
with its shared Git metadata and alternate object store. The exact
`rust:1.99-bookworm` Linux ARM64 image was pinned by
`sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`.
The container had 6 GiB memory, four assigned CPUs and 512 PIDs.
`CARGO_TARGET_DIR` pointed to the existing attempt-1 target. Cargo
used `-j 1`, `--release --locked --no-default-features`, and the
same five requested examples. The invocation was otherwise exactly
the `docker run` command in attempt 1 with container name
`n83-arm64-build-smoke-1ae9333d`, the single-job change and the
source HEAD above.

The host started the container at 2026-11-15 17:06:28 UTC. It
completed `isogeny_algos`, then spent time compiling the `crypto`
library. The latest Docker memory sample was 5.995 GiB of 6 GiB.
The [daemon events](linux-arm64-build-smoke-attempt2.events) record
`oom` at **17:17:09 UTC**, then `die` and `destroy` at
17:17:10 UTC. The Rust compiler command ended with signal 9;
Docker/Cargo exited 101 at 17:17:11 UTC. None of the five requested
executables existed. Scratch free space was about 4.7 GiB after
the exit, above the 1 GiB stop threshold.

The [raw compressed log](linux-arm64-build-smoke-attempt2.log.gz)
is 12,874 bytes uncompressed, SHA-256
`e4fce7e1ad646d6733142e02445ed86670a749ad26816ae16a9e873d26216722`.
The daemon-event text is 374 bytes, SHA-256
`4b8ec065d42cf16ef10f137364d491c18d94108ba9ba7f874a59ce0f9a71ca6c`.
The partial target is now about 1.1 GiB and remains local outside
the repository; it is not a durable build artifact.

The [diagram](linux-arm64-build-smoke-attempt2.svg) and
[PDF report](linux-arm64-build-smoke-attempt2.pdf) record the
failure path. Two bounded configurations, `-j 2`
and `-j 1`, both reached the 6 GiB memory limit while compiling
the same library. The one-job result rules out simultaneous Cargo
jobs as a sufficient fix under this host cap; it does not establish
a minimum memory requirement or failure on a larger host.
The next discriminating check is a frozen build on a host with
more than 6 GiB of available container RAM, with 12 GiB as the
initial candidate, a clean source and a fresh target. The charged
build receipt, solver, image, zero-trial preflight, and K600 arms
remain open. No canonical IC performance graph changed because this
run measured no IC workload.
