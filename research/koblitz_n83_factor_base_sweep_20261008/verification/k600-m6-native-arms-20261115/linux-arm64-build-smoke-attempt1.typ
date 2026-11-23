#set page(paper: "a4", margin: 1.6cm)
#set text(font: "Libertinus Serif", size: 10pt)
= N83 K600 Linux ARM64 build smoke: attempt 1

*Status:* stopped by container OOM. This is an engineering build smoke,
not an index-calculus measurement.

Source: `3ff510d8a280ee634346638a9e23830d2cb6e20d`. The
`rust:1.99-bookworm` Linux ARM64 image ID was
`sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`.
The source and Cargo lock were mounted read-only. The container had
6 GiB memory, four CPUs, 512 PID limit, and Cargo used two build jobs.

The run started at 2026-11-15 16:55:44 UTC. The `crypto` library
`rustc` was killed by signal 9, Docker reported `OOMKilled=true`,
and the operator stopped the container at 17:00:53 UTC. Docker exited
137. None of the five requested executable outputs existed.

#image("linux-arm64-build-smoke-attempt1.svg", width: 100%)

The raw log is `linux-arm64-build-smoke-attempt1.log.gz`, whose
uncompressed SHA-256 is
`3d9f458a9800ffaed71742b49f021b3c9d2331436840597304d28f53d303f82a`.
The Docker state receipt is `linux-arm64-build-smoke-attempt1.state`.
Peak sampled memory reached the 6 GiB cap; the last observed scratch
free space was 2.2 GiB. A 906 MiB partial target cache remains only
locally and is not a completed build.

The frozen input and stop conditions are in
`LINUX_ARM64_BUILD_SMOKE_PROTOCOL.md`; the full invocation is in
`LINUX_ARM64_BUILD_SMOKE_ATTEMPT1.md`. The next discriminating
check is a separately versioned one-job retry, stating cache reuse.
The source-bound build, runtime image and K600 controls remain open.
