# K600 preflight Clippy repair: focused local validation

The CI `clippy (crypto)` job on PR #4295 head
`ebf1130e36744d7759f83715eed2e5778c5028a5` rejected
`examples/n83_k600_m6_native_preflight.rs:602` for
`clippy::manual_contains`. Commit `8416d4cdc3ff1962fe49db7de666a3a3071e6e1c`
changes the equivalent `Vec<&str>` membership test to `contains`.

`rustfmt --check examples/n83_k600_m6_native_preflight.rs` and
`git diff --check` passed. The first macOS arm64 Rust 1.99.1 release
example validation started with an empty separate target and stopped
before a test result when its tool process handle disappeared. Its
[18,862-byte log](clippy-ci-preflight-first-incomplete.log.gz), SHA-256
`29454cc65403447cc2a33a8808337e464f7964ffeda1558c288cc006254ef8ed`,
ends during library compilation. Its exit status is unknown; it is not
counted as a test pass.

The retry ran from the partial target in a detached tmux session with an
explicit exit-code file. Exact command from the clean committed checkout:

```sh
RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
CARGO_TARGET_DIR=/private/tmp/n83-ci-20261115 \
rustup run 1.99.1 cargo test --release --locked --offline \
  --example n83_k600_m6_native_preflight -j 2
```

The exit file held **0**. Cargo finished the release build in **20m 51s**;
the example ran **6 passed, 0 failed, 0 ignored** in 0.35 s. The
[12,096-byte raw retry log](clippy-ci-preflight-retry.log.gz) has
uncompressed SHA-256
`e35c7e207127dd2f8dd0af0cc580cde4bcf0cfb0ae4c2a6d82ba2693c7c652fa`.
This local test covers the touched example but is not the repository's
full Clippy job or release-library gate. The last full release-library
attempt on unchanged library source had 6,306 passes and 59 failures
from sparse and host-sensitive fixtures; that gate remains red. Applicable
CI on the pushed repair head must be checked before merge.
