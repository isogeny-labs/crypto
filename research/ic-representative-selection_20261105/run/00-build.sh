#!/usr/bin/env bash
# Stage 0a: build the two local tools at the checked-out commit and record their hashes.
# Local development work: no measurement. nice keeps a shared host usable.
source "$(dirname "$0")/common.sh"
nice -n 10 cargo build --release --locked --bin volcano-rho --bin ecbench
nice -n 10 cargo build --release --locked --manifest-path $D/native/Cargo.toml
nice -n 10 cargo test --release --locked --manifest-path $D/native/Cargo.toml
mkdir -p $D/inputs
{ git rev-parse HEAD; sha256sum "$VOLCANO_RHO" "$ECBENCH" "$REPSEL"; } > $D/inputs/tools.sha256
cat $D/inputs/tools.sha256
