#set page(paper: "a4", margin: 1.6cm)
#set text(font: "Libertinus Serif", size: 10pt)
= N83 K600 Linux ARM64 build smoke: attempt 2

*Status:* container OOM during single-job compilation. This warm-target
retry is an engineering check, not an index-calculus measurement.

Source: `1ae9333dd8619c73cff1e68b2afbe8519028e9f4`. The
Linux ARM64 Rust 1.99.1 image ID was
`sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`.
The source was read-only; the partial target from attempt 1 was reused.
The container had 6 GiB RAM, four CPUs, 512 PIDs, and one Cargo job.

Start was 2026-11-15 17:06:28 UTC. The Docker daemon recorded
`oom` at 17:17:09 UTC. The `crypto` library `rustc` received
SIGKILL and Cargo exited 101 by 17:17:11 UTC. None of five
requested executables existed.

#image("linux-arm64-build-smoke-attempt2.svg", width: 100%)

Raw records: `linux-arm64-build-smoke-attempt2.log.gz` (uncompressed
SHA-256 `e4fce7e1ad646d6733142e02445ed86670a749ad26816ae16a9e873d26216722`)
and `linux-arm64-build-smoke-attempt2.events`. Peak sampled
memory was 5.995 GiB of 6 GiB; scratch free space after exit was
about 4.7 GiB.

The frozen protocol is in `LINUX_ARM64_BUILD_SMOKE_ATTEMPT2_PROTOCOL.md`;
the full command and interpretation are in
`LINUX_ARM64_BUILD_SMOKE_ATTEMPT2.md`. A larger host with a fresh
target is the next discriminating build check. The charged K600
build and subsequent controls remain open.
