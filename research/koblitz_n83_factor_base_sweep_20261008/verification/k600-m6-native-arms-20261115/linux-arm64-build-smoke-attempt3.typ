#set page(paper: "a4", margin: 1.6cm)
#set text(font: "Libertinus Serif", size: 10pt)
= N83 K600 Linux ARM64 build smoke: attempt 3

*Status:* three tracked JSON inputs were absent from the sparse checkout
when the worker example compiled. This warm-target engineering check is
not a cold build or index-calculus measurement.

Frozen source `687f7d3f162b3b3f173db4b825b61e57f4052c88` was
read-only. The pinned Rust 1.99.1 ARM64 image had ID
`sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`.
Docker Desktop provided 14,336 MiB, with a 12 GiB container cap, four
CPUs and one Cargo job. The prior partial target was reused.

The container ran 18:37:17–18:47:10 UTC on 2026-11-15 and exited 101,
`OOMKilled=false`. The library compiled; the worker example then failed
on three `include_bytes!` files absent from the sparse working tree.
They are tracked by Git. No requested executable output existed.
An observed memory sample was 8.131 GiB; it is not a measured peak.

#image("linux-arm64-build-smoke-attempt3.svg", width: 100%)

The frozen protocol is `LINUX_ARM64_BUILD_SMOKE_ATTEMPT3_PROTOCOL.md`;
the detailed report is `LINUX_ARM64_BUILD_SMOKE_ATTEMPT3.md`.
The raw log is `linux-arm64-build-smoke-attempt3.log.gz`, 10,249 bytes
uncompressed, SHA-256
`b0bed2c4fc0619abad693c35077645e2dee15c0323e1706169f7d338cf25731f`.
The Docker state is `linux-arm64-build-smoke-attempt3.state.json`.
The original 7,424 MiB Docker allocation was restored and verified.

Materializing the exact tracked files and freezing another bounded
build is the next check. The charged source-bound build and subsequent
K600 controls remain open. No IC performance graph changed.
