# Current-head release-library validation: disk resource failure

At clean source `3383b91c6d56` (no root library or Cargo changes from
the preceding full test), the required macOS arm64 Rust 1.99.1 command
was attempted from a separate boot-disk target:

```sh
RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
CARGO_TARGET_DIR=/private/tmp/n83-ci-20261115 \
gtimeout --signal=TERM --kill-after=10s 1800 \
rustup run 1.99.1 cargo test --release --lib --locked --offline -j 2
```

The exit-code receipt was **101**. The [17,470-byte raw log](root-lib-disk-failure.log.gz),
uncompressed SHA-256
`8ab1b0475b3df2fba586f3c8da345cac41e1d17c4425942ccb1f63b5eede2ea7`,
ends with `rustc-LLVM ERROR: IO failure on output stream: No space left on
device` while linking the library test binary. The test binary did not
run; there is no current-head pass/fail count for library tests. Boot-disk
free space fell to 407 MiB during the attempt. The 30-minute time cap was
not reached.

The separate target was disposable validation cache. `cargo clean
--target-dir /private/tmp/n83-ci-20261115` removed 2,045 files / 1.4 GiB;
the two validation logs were preserved, and boot-disk free space was
1.6 GiB afterward. This cleanup did not change tracked source or evidence.

The last **completed** full release-library run on unchanged library
source remains the earlier 6,306 passed / 59 failed / 138 ignored
receipt. Those failures were attributed to sparse and host-sensitive
fixtures, and the local gate remains red. The current attempt adds an
independent disk-capacity obstruction, not a new code-test verdict.
Fresh capacity or a qualified CI runner is needed for a current-head
complete release-library run. Applicable CI and review remain merge gates.
