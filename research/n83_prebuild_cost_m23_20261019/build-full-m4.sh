#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
source=/var/tmp/n83-xor-cache-source
logs=/var/tmp/n83-full-m4-build-logs
test "$(cat "$root/xor-cache-v2/card-status.txt")" = COMPLETE_BOUNDED
test ! -e "$logs"
mkdir "$logs"
printf '%s  %s\n' 0adffcc69171d9cff28f026dfdc624cb055c1be4d30519fea3491a54da043150 /var/tmp/n83-full-m4-source.bundle | sha256sum -c - > "$logs/bundle-check.log"
date -u +%FT%TZ > "$logs/start.utc"
git -C "$source" fetch /var/tmp/n83-full-m4-source.bundle HEAD > "$logs/fetch.log" 2>&1
git -C "$source" checkout --detach d9ef42301cc3d22f437e1f89304e673836afb6a3 > "$logs/checkout.log" 2>&1
git -C "$source" sparse-checkout add /examples/n83_full_m4_card_check.rs /examples/n83_xor_cache_audit.rs /research/n83_prebuild_cost_m23_20261019/
git -C "$source" status --porcelain > "$logs/source-status.txt"
test ! -s "$logs/source-status.txt"
cd "$source"
export CARGO_TARGET_DIR="$root/target-small-m4" CARGO_BUILD_JOBS=4
cargo test --release --example n83_prebuild_cost_m23 --example n83_small_m4_replay --example n83_full_m4_card_check --example n83_xor_cache_audit > "$logs/tests.log" 2>&1
printf '0\n' > "$logs/tests-exit.txt"
cargo build --release --example n83_prebuild_cost_m23 --example n83_small_m4_replay > "$logs/build.log" 2>&1
printf '0\n' > "$logs/build-exit.txt"
sha256sum "$CARGO_TARGET_DIR/release/examples/n83_prebuild_cost_m23" "$CARGO_TARGET_DIR/release/examples/n83_small_m4_replay" > "$logs/binaries.sha256"
git rev-parse HEAD > "$logs/source-commit.txt"
rustc --version > "$logs/rustc-version.txt"
date -u +%FT%TZ > "$logs/end.utc"
