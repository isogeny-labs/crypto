#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "usage: $0 OUTPUT_DIRECTORY" >&2
  exit 64
fi

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"
output_dir="$1"
case "$output_dir" in
  /*) ;;
  *) output_dir="$repo_root/$output_dir" ;;
esac
if [ -e "$output_dir" ]; then
  echo "refusing to overwrite existing attempt: $output_dir" >&2
  exit 65
fi
mkdir -p "$output_dir"
exec > >(tee "$output_dir/driver.log") 2>&1

protocol_dir="$repo_root/research/prime31_ic_20261115/rr9"
native_dir="$repo_root/research/prime31_ic_20261115/native"
producer_source="$native_dir/examples/prime31_rr9_plucker_span.rs"
checker_source="$native_dir/examples/prime31_rr9_plucker_span_check.rs"
original_commit="1df714645aa683cf975bc76fcacb9842cb4780fd"
time_bin="${TIME_BIN:-/usr/bin/time}"
if [ ! -x "$time_bin" ]; then
  echo "GNU time executable not found: $time_bin" >&2
  exit 69
fi
if [ -n "${CARGO_BIN:-}" ]; then
  cargo_bin="$CARGO_BIN"
elif command -v cargo >/dev/null 2>&1; then
  cargo_bin="$(command -v cargo)"
elif [ -x /workspace/.prime31-rust/cargo/bin/cargo ]; then
  export CARGO_HOME=/workspace/.prime31-rust/cargo
  export RUSTUP_HOME=/workspace/.prime31-rust/rustup
  cargo_bin=/workspace/.prime31-rust/cargo/bin/cargo
else
  echo "Rust 1.100.0 cargo proxy not found" >&2
  exit 69
fi
if [ -n "${RUSTFMT_BIN:-}" ]; then
  rustfmt_bin="$RUSTFMT_BIN"
elif command -v rustfmt >/dev/null 2>&1; then
  rustfmt_bin="$(command -v rustfmt)"
elif [ -x /workspace/.prime31-rust/cargo/bin/rustfmt ]; then
  rustfmt_bin=/workspace/.prime31-rust/cargo/bin/rustfmt
else
  echo "Rust 1.100.0 rustfmt proxy not found" >&2
  exit 69
fi

candidate_commit="$(git rev-parse HEAD)"
printf '%s\n' "$candidate_commit" > "$output_dir/candidate-commit.txt"
date -u +%Y-%m-%dT%H:%M:%SZ > "$output_dir/started-utc.txt"
uname -a > "$output_dir/uname.txt"
"$cargo_bin" +1.100.0 --version --verbose > "$output_dir/cargo-version.txt"
"$rustfmt_bin" +1.100.0 --version > "$output_dir/rustfmt-version.txt"
"$time_bin" --version > "$output_dir/time-version.txt"
(cd "$protocol_dir" && sha256sum -c RR9_PLUCKER_SPAN_PROTOCOL_20261119.sha256) \
  > "$output_dir/protocol-digest-check.txt" 2>&1

build_root="$(mktemp -d /tmp/prime31-plucker-local.XXXXXX)"
printf '%s\n' "$build_root" > "$output_dir/build-root.txt"
git worktree add --detach "$build_root/pinned" "$original_commit"
git -C "$build_root/pinned" sparse-checkout add \
  /src/cryptanalysis/f4_fp_tower.rs \
  /src/cryptanalysis/sig_fp_tower.rs
cp "$producer_source" \
  "$build_root/pinned/research/prime31_ic_20261115/native/examples/prime31_rr9_plucker_span.rs"
cp "$checker_source" \
  "$build_root/pinned/research/prime31_ic_20261115/native/examples/prime31_rr9_plucker_span_check.rs"

manifest="$build_root/pinned/research/prime31_ic_20261115/native/Cargo.toml"
target_dir="$build_root/target"
export CARGO_TARGET_DIR="$target_dir"

run_step() {
  local name="$1"
  shift
  set +e
  "$@" > "$output_dir/$name.stdout" 2> "$output_dir/$name.stderr"
  local status=$?
  set -e
  printf '%s\n' "$status" > "$output_dir/$name.exit"
  if [ "$status" -ne 0 ]; then
    echo "step failed: $name (exit $status)"
    return "$status"
  fi
  echo "step passed: $name"
}

run_capped_json() {
  local name="$1"
  shift
  set +e
  (
    ulimit -v 4194304
    "$time_bin" -v -o "$output_dir/$name.time" "$@"
  ) > "$output_dir/$name.json" 2> "$output_dir/$name.stderr"
  local status=$?
  set -e
  printf '%s\n' "$status" > "$output_dir/$name.exit"
  if [ "$status" -ne 0 ]; then
    echo "step failed: $name (exit $status)"
    return "$status"
  fi
  echo "step passed: $name"
}

run_step rustfmt "$rustfmt_bin" +1.100.0 --edition 2021 --check \
  "$build_root/pinned/research/prime31_ic_20261115/native/examples/prime31_rr9_plucker_span.rs" \
  "$build_root/pinned/research/prime31_ic_20261115/native/examples/prime31_rr9_plucker_span_check.rs"
run_step original-lib-tests "$cargo_bin" +1.100.0 test --locked --release \
  --manifest-path "$manifest" --lib
run_step producer-tests "$cargo_bin" +1.100.0 test --locked --release \
  --manifest-path "$manifest" --example prime31_rr9_plucker_span
run_step checker-tests "$cargo_bin" +1.100.0 test --locked --release \
  --manifest-path "$manifest" --example prime31_rr9_plucker_span_check
run_step release-build "$cargo_bin" +1.100.0 build --locked --release \
  --manifest-path "$manifest" \
  --example prime31_rr9_plucker_span \
  --example prime31_rr9_plucker_span_check

producer="$target_dir/release/examples/prime31_rr9_plucker_span"
checker="$target_dir/release/examples/prime31_rr9_plucker_span_check"
run_capped_json producer-a "$producer"
run_capped_json producer-b "$producer"
run_step producer-aa cmp "$output_dir/producer-a.json" "$output_dir/producer-b.json"
run_capped_json checker-a "$checker" check "$output_dir/producer-a.json"
run_capped_json checker-b "$checker" check "$output_dir/producer-a.json"
run_step checker-aa cmp "$output_dir/checker-a.json" "$output_dir/checker-b.json"
run_capped_json mutations "$checker" mutations "$output_dir/producer-a.json"

sha256sum \
  "$producer_source" \
  "$checker_source" \
  "$protocol_dir/RR9_PLUCKER_SPAN_PROTOCOL_20261119.md" \
  "$protocol_dir/RR9_PLUCKER_SPAN_LOCAL_RUN_ADDENDUM_20261119.md" \
  "$protocol_dir/RR9_PLUCKER_SPAN_LOCAL_RUN_AMENDMENT_A1_20261119.md" \
  "$protocol_dir/RR9_PLUCKER_SPAN_LOCAL_RUN_AMENDMENT_A2_20261119.md" \
  "$protocol_dir/run_prime31_rr9_plucker_span_local_a2.sh" \
  "$output_dir/producer-a.json" \
  "$output_dir/producer-b.json" \
  "$output_dir/checker-a.json" \
  "$output_dir/checker-b.json" \
  "$output_dir/mutations.json" \
  > "$output_dir/SHA256SUMS"
date -u +%Y-%m-%dT%H:%M:%SZ > "$output_dir/finished-utc.txt"
printf '%s\n' complete > "$output_dir/run-status.txt"
echo "local Pluecker-span attempt complete: $output_dir"
