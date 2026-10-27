#!/usr/bin/env bash
# Native oracle diagnostic only. Each science subprocess keeps its own cap.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_native="$joint_study/../endomorphism_image_base/native"
joint_direct_phase() {
  local joint_id=$1 joint_rc
  shift
  printf '%q ' "$@" > "$joint_out/metadata/$joint_id.command.txt"
  set +e
  "$@" > "$joint_out/metadata/$joint_id.stdout" 2> "$joint_out/metadata/$joint_id.stderr"
  joint_rc=$?
  set -e
  printf '%s\n' "$joint_rc" > "$joint_out/metadata/$joint_id.exit"
  return "$joint_rc"
}
if test "${1:-}" = --compile; then
  joint_out=$2
  export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1
  ulimit -c 0
  joint_direct_phase compile-tests timeout --kill-after=10s 1200s cargo test --locked --release --manifest-path "$joint_native/Cargo.toml" --no-run --message-format=json
  joint_test=$(jq -r 'select(.reason=="compiler-artifact" and .profile.test==true and .executable!=null) | .executable' "$joint_out/metadata/compile-tests.stdout" | tail -1)
  test -x "$joint_test"
  joint_direct_phase native-tests prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_test" --test-threads=1
  joint_direct_phase compile-runner timeout --kill-after=10s 1200s cargo build --locked --release --manifest-path "$joint_native/Cargo.toml"
  cp "$joint_native/target/release/twist-endomorphism-image-study" "$joint_out/runner"
  chmod 500 "$joint_out/runner"
  sha256sum "$joint_out/runner" > "$joint_out/metadata/runner.sha256"
  exit 0
fi
if test "${1:-}" = --cases; then
  joint_out=$2
  export RAYON_NUM_THREADS=1
  ulimit -c 0
  for joint_prime in 37 73 97 157 229 241 337; do
    for joint_seed in 0 1 2; do
      joint_id="p$joint_prime-seed$joint_seed"
      joint_public="$joint_study/runs/algebra-v1/prepared/public/$joint_id.json"
      joint_sha=$(sha256sum "$joint_public" | cut -d ' ' -f 1)
      joint_replay="$joint_study/runs/fp-pruning-audit-v1/cases/$joint_id/summary.json"
      joint_direct_phase "$joint_id" prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-five-zero "$joint_public" "$joint_sha" "$joint_replay" "$joint_out/cases/$joint_id" || true
    done
  done
  joint_direct_phase summarize prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-five-zero-summary "$joint_out" "$joint_out/summary.json"
  exit 0
fi
joint_out=${1:?new output directory}
test ! -e "$joint_out"
mkdir -p "$joint_out/metadata" "$joint_out/cases"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
cp "$joint_study/SUPPLEMENT_FIVE_FACTOR_ZERO.md" "$joint_out/metadata/frozen-protocol.md"
git rev-parse HEAD > "$joint_out/metadata/source-commit.txt"
sha256sum "$joint_native"/src/*.rs "$joint_study/native/five_zero.rs" > "$joint_out/metadata/sources.sha256"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
# Compile and freeze in one reservation; no address-space cap on compilation.
python3 "$joint_repo/tools/isolated_bench.py" busy -- bash "$joint_out/metadata/executed-script.sh" --compile "$joint_out"
# Existing point-verified matrix replays are required before any census.
# Waiting uses no reservation and performs no BSGS or oracle recomputation.
while systemctl is-active --quiet twist-joint-fp-pruning-audit-v1.service; do sleep 10; done
test "$(jq -r .status "$joint_study/runs/fp-pruning-audit-v1/summary.json")" = hypothesis-audit-pass
sha256sum "$joint_study/runs/fp-pruning-audit-v1/summary.json" > "$joint_out/metadata/required-replay.sha256"
# A single reservation prevents per-case short replays being interleaved with
# other long solves. Each of the21 science processes retains8GiB/1200s.
python3 "$joint_repo/tools/isolated_bench.py" busy -- bash "$joint_out/metadata/executed-script.sh" --cases "$joint_out"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'Native five-factor oracle census terminal. No target recovery or polynomial timing claim; independent point-keyed completeness replay remains separate.\n' > "$joint_out/complete.txt"
