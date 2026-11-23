#!/usr/bin/env bash
# Thin development-test orchestration; no benchmark or experimental comparison.
set -euo pipefail
repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"
receipt_dir=${1:?provide a NEW receipt directory}
if [ -e "$receipt_dir" ]; then
  echo 'receipt directory already exists' >&2
  exit 2
fi
mkdir -p "$receipt_dir"
study=research/pkm_larger_scale_20261123
manifest=$study/native/Cargo.toml
sha256sum src/bin/ic/prime_workload.rs src/bin/ic/bench.rs \
  "$manifest" "$study/native/Cargo.lock" "$study/native/qualify-frozen.sh" \
  "$study/native/fixtures/toy-workload.json" > "$receipt_dir/precall.sha256"
rustc --version > "$receipt_dir/toolchain.txt"
cargo --version >> "$receipt_dir/toolchain.txt"
cargo test --release --locked --manifest-path "$manifest" > "$receipt_dir/test.stdout" 2> "$receipt_dir/test.stderr"
cargo build --release --locked --manifest-path "$manifest" > "$receipt_dir/build.stdout" 2> "$receipt_dir/build.stderr"
worker=$study/native/target/release/pkm-prime-workload
sha256sum "$worker" > "$receipt_dir/binary.sha256"
"$worker" validate "$study/native/fixtures/toy-workload.json" > "$receipt_dir/validate.json"
if "$worker" generate "$study/native/fixtures/toy-curve.json" "$study/native/fixtures/toy-workload.json" 8 20261123 pkm-large-v1/unit-test > "$receipt_dir/overwrite.stdout" 2> "$receipt_dir/overwrite.stderr"; then
  echo 'ERROR: immutable fixture was overwritten' >&2
  exit 1
fi
sha256sum --check "$receipt_dir/precall.sha256" > "$receipt_dir/postcall-source-check.txt"
printf '%s\n' '{"schema":"pkm.native-development-check/v1","status":"passed","scientific_benchmark":false,"full_ic_consumer_qualified":false}' > "$receipt_dir/result.json"
