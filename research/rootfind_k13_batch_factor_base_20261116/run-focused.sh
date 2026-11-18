#!/bin/zsh
set -u
cd /Volumes/SSD990-2/crypto/wt/codex-rootfind-inline-page-index-20261116 || exit 99
study=research/rootfind_k13_batch_factor_base_20261116
RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
  /Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/cargo \
  test --release --locked -j 2 --lib batched_ -- --nocapture \
  > "$study/test.stdout" 2> "$study/test.stderr"
task_rc=$?
printf '%s\n' "$task_rc" > "$study/test.exit"
exit "$task_rc"
