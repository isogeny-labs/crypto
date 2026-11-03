#!/usr/bin/env bash
# Compile/freeze the reporter, then validate the existing population run once.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_native="$joint_study/../endomorphism_image_base/native"
joint_out=${1:?new output directory}
joint_population=${2:?existing population run}
test ! -e "$joint_out"
mkdir -p "$joint_out/metadata"
joint_out=$(cd "$joint_out" && pwd)
joint_population=$(cd "$joint_population" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
git rev-parse HEAD > "$joint_out/metadata/source-commit.txt"
sha256sum "$joint_native"/src/joint*.rs "$joint_native/src/main.rs" "$joint_study/native/"*.rs > "$joint_out/metadata/sources.sha256"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
export RAYON_NUM_THREADS=1 CARGO_BUILD_JOBS=1
ulimit -c 0
joint_phase() {
  local joint_id=$1 joint_rc
  shift
  printf '%q ' "$@" > "$joint_out/metadata/$joint_id.command.txt"
  set +e
  python3 "$joint_repo/tools/isolated_bench.py" busy -- "$@" > "$joint_out/metadata/$joint_id.stdout" 2> "$joint_out/metadata/$joint_id.stderr"
  joint_rc=$?
  set -e
  printf '%s\n' "$joint_rc" > "$joint_out/metadata/$joint_id.exit"
  return "$joint_rc"
}
joint_phase compile-tests cargo test --locked --release --manifest-path "$joint_native/Cargo.toml" --no-run --message-format=json
joint_test=$(jq -r 'select(.reason=="compiler-artifact" and .profile.test==true and .executable!=null) | .executable' "$joint_out/metadata/compile-tests.stdout" | tail -1)
joint_phase native-tests prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_test" --test-threads=1
joint_phase compile-runner cargo build --locked --release --manifest-path "$joint_native/Cargo.toml"
cp "$joint_native/target/release/twist-endomorphism-image-study" "$joint_out/runner"
chmod 500 "$joint_out/runner"
sha256sum "$joint_out/runner" > "$joint_out/metadata/runner.sha256"
# Waiting is outside the heavy-work reservation. This is only the owned
# variant population postprocessor, unrelated to the legacy freeze lane.
while systemctl is-active --quiet twist-joint-variant-population-v1.service; do sleep 20; done
systemctl show twist-joint-variant-population-v1.service --property=Id,ActiveState,SubState,ExecMainStatus > "$joint_out/metadata/population-owner-terminal.txt"
joint_phase validate prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-variant-report "$joint_population" "$joint_out/summary.json"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'Variant record validator terminal. Inspect status, all failures and missing records; no root, ideal or cost certificate is implied.\n' > "$joint_out/complete.txt"
