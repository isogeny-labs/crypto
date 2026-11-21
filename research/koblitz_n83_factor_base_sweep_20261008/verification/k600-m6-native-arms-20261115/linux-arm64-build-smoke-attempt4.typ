#set page(paper: "a4", margin: 1.6cm)
#set text(font: "Libertinus Serif", size: 10pt)
= N83 K600 Linux ARM64 build smoke: attempt 4

*Status:* five executable Linux ARM64 examples built in a warm-target
engineering check. This is not a cold charged build or IC runtime sample.

Clean source `45c5a8c49e8aecb3e36ed51bdd9518e0ff89031c` and three
tracked JSON inputs were read-only. Rust 1.99.1 ARM64 image ID:
`sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`.
The container reused the earlier partial target with a 12 GiB memory
cap, four CPUs, 512 PIDs and one Cargo job.

Docker ran 2026-11-15 18:54:13–18:54:39 UTC, exited 0 and reported
`OOMKilled=false`. Cargo's warm build interval was 26.21 s. All five
outputs passed executable and ELF ARM aarch64 checks; hashes and byte
counts are in `linux-arm64-build-smoke-attempt4.outputs.tsv`. Scratch
free space remained about 1.8 GiB. Docker Desktop was restored to
7,424 MiB and its engine memory verified.

#image("linux-arm64-build-smoke-attempt4.svg", width: 100%)

The frozen protocol is `LINUX_ARM64_BUILD_SMOKE_ATTEMPT4_PROTOCOL.md`;
the detailed report is `LINUX_ARM64_BUILD_SMOKE_ATTEMPT4.md`. The raw
log is `linux-arm64-build-smoke-attempt4.log.gz`, 8,539 bytes
uncompressed, SHA-256
`7fad77a577738b87bbc10b32b41c45f3ceb219973927c8427dba4aeff1c16617`.
The Docker state is `linux-arm64-build-smoke-attempt4.state.json`.

A fresh source-bound charged build, solver binary, runtime image and
K600 controls remain open. No IC performance graph changed.
