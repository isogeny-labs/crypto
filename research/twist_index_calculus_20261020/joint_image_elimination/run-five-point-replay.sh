#!/usr/bin/env bash
# Independent public-point enumeration; no BSGS, factor logs or solver runs.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_native="$joint_study/../endomorphism_image_base/native"
joint_out=${1:?new output directory}
if test "${2:-}" = --execute; then
  export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1
  ulimit -c 0
  joint_phase() {
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
  joint_phase compile-tests timeout --kill-after=10s 1200s cargo test --locked --offline --release --manifest-path "$joint_native/Cargo.toml" --no-run --message-format=json
  joint_test=$(jq -r 'select(.reason=="compiler-artifact" and .profile.test==true and .executable!=null) | .executable' "$joint_out/metadata/compile-tests.stdout" | tail -1)
  test -x "$joint_test"
  joint_phase native-tests prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_test" --test-threads=1
  joint_phase compile-runner timeout --kill-after=10s 1200s cargo build --locked --offline --release --manifest-path "$joint_native/Cargo.toml"
  cp "$joint_native/target/release/twist-endomorphism-image-study" "$joint_out/runner"
  chmod 500 "$joint_out/runner"
  sha256sum "$joint_out/runner" > "$joint_out/metadata/runner.sha256"
  for joint_p in 37 73 97 157 229 241 337; do
    for joint_s in 0 1 2; do
      joint_id="p$joint_p-seed$joint_s"
      joint_public="$joint_study/runs/algebra-v1/prepared/public/$joint_id.json"
      joint_sha=$(sha256sum "$joint_public" | cut -d ' ' -f 1)
      joint_phase "$joint_id" prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-five-point-replay "$joint_public" "$joint_sha" "$joint_study/runs/five-zero-v1/cases/$joint_id" "$joint_out/cases/$joint_id" || true
    done
  done
  joint_phase summarize prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-five-point-summary "$joint_out" "$joint_out/summary.json"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
  printf 'Point-keyed enumeration and matrix replay terminal; no polynomial, target-recovery or timing claim.\n' > "$joint_out/complete.txt"
  exit 0
fi
test ! -e "$joint_out"
test "$(jq -r .status "$joint_study/runs/five-zero-v1/summary.json")" = complete-oracle-census
test "$(jq -r .completed_cases "$joint_study/runs/five-zero-v1/summary.json")" = 21
mkdir -p "$joint_out/metadata" "$joint_out/cases"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
cp "$joint_study/SUPPLEMENT_FIVE_POINT_REPLAY.md" "$joint_out/metadata/frozen-protocol.md"
git rev-parse HEAD > "$joint_out/metadata/source-commit.txt"
sha256sum "$joint_native"/src/*.rs "$joint_study"/native/*.rs > "$joint_out/metadata/sources.sha256"
sha256sum "$joint_study/runs/five-zero-v1/summary.json" > "$joint_out/metadata/prior-census.sha256"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
python3 "$joint_repo/tools/isolated_bench.py" busy -- bash "$joint_out/metadata/executed-script.sh" "$joint_out" --execute
