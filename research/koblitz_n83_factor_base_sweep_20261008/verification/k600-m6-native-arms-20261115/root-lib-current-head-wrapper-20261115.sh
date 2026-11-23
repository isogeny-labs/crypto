#!/bin/zsh
cd /Volumes/SSD990-2/tmp/n83-native-rank-query-wt || exit 70
printf '%s\n' "$(git rev-parse HEAD)" > /private/tmp/n83-current-head-lib-20261115.commit
RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
CARGO_TARGET_DIR=/private/tmp/n83-ci-20261115 \
gtimeout 2400 cargo test --release --lib --locked --offline -j 2 \
  > /private/tmp/n83-current-head-lib-20261115.log 2>&1
status=$?
printf '%s\n' "$status" > /private/tmp/n83-current-head-lib-20261115.exit
