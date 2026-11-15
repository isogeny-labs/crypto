# Root release-library validation on the native K600 preflight branch

**Result: failed required gate; no research measurement.** On macOS arm64,
Rust 1.99.1, source commit `c01b1b6d92d5af1b7e5e245c7f890b2a906b7d1a`, run:

```sh
RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
  cargo test --release --lib --locked --offline
```

The process exited **101** after 538.99 seconds of Cargo-reported test time:
**6,305 passed, 60 failed, 138 ignored**. The [complete compressed raw log](ARTIFACT_READBACK.md)
was moved to the artifact store by the GitHub bot and freshly read back: 120,746 bytes, SHA-256
`0c885577d55a9f387d56dc11b0cc0838780d0de622c9e2e6d7c5dfb9fd2fa2cb`.
The [sorted failing names](lib-test-current-failures.txt) are 5,725 bytes,
SHA-256 `a8753892cc85de7e8c64bc88512bb6f168cb50131b7353e357d3aa0bc0e03b09`.
Compared bytewise with the preceding 59-name
[receipt](../k600-v3-s3-adoption-20261115/lib-failures.txt), the only added
name was
`ml_dataset::solvers::tests::wide_frozen_m13_m15_boundary_retains_terminal_on_both_twists`.
Its broad-run panic was `m=15, n=47, twist=false: wide dynamic linear retention stopped: Deadline`.

That test was rerun alone against the same commit with:

```sh
RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
  cargo test --release --lib --locked --offline \
  ml_dataset::solvers::tests::wide_frozen_m13_m15_boundary_retains_terminal_on_both_twists \
  -- --exact --nocapture
```

It exited **0**, **1 passed**, in 3.43 seconds. The [compressed isolated log](lib-test-isolated.log.gz)
is 3,275 bytes, SHA-256
`0820b0c7d5c104e395ddd391b312ecebf7db33597da186e40a6f041c8357bc42`.
The current PR changes the N83 example, its study adapter and documentation;
it changes no root `src/` file or Cargo manifest/lock. The isolated pass
supports a load-sensitive deadline for this extra broad-run failure; it does
not turn the failed root gate into a pass or explain all 59 recurring failures.
The touched release example suites passed 5/5 and 27/27 separately.
