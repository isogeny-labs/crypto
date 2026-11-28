#set page(paper: "us-letter", margin: 0.72in)
#set text(font: "New Computer Modern", size: 10pt)

= N83 Colima toolchain image provision

#text(size: 9pt)[Run started 2026-11-16T00:14Z · Clean source `317f53dad49872ba9c9ef1aaa647a78bc89fad0b` · Local Linux ARM64]

*Observed result: image provision passed.* The tracked Dockerfile was extracted by exact Git blob and SHA-256 from clean source. Docker built the ARM64 toolchain image from a pinned Rust 1.99.1 base with child/wrapper exit 0 in a 12-second outer interval. The local image ID is `sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573`. A separate no-network, read-only no-work smoke exited 0 and checked Rust, Cargo, CMake, Git, Docker CLI and GMP versions. The complete receipts and raw logs are retained.

#figure(image("image-flow.svg", width: 100%), caption: [Observed source, build and smoke path. The dashed edge points to an unexecuted charged-build gate.])

== Evidence and limits

The source-linked result is `verification/k600-colima-toolchain-20261116/RESULT.md`. The Dockerfile has Git blob `8af2c7ac304bfb09f20caaec5be9eadfbdc33c75`, SHA-256 `93e2eada27062e311824f4f5f88a49b1763e0c30a90ae8457517552de2437e88`, and uses `rust@sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`. The build log is 12,643 bytes, SHA-256 `56e41a65432a36dc2c88de989a85eb9d27ef988f9f5af1111b7730c5ff2364a4`. Image inspection and history are retained; apt package resolution is captured but not immutable from the Dockerfile alone.

This infrastructure result requires a versioned amendment to the old build plan's image ID before the charged K600 binaries/solver build. It does not establish a Linux release-library pass, source/input preflight, constructed or natural K600 relation, useful rank, target descent, matched rho, or cold one-target IC total. Canonical IC graphs remain unchanged because this run supplies no algorithm performance row.
