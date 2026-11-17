#!/usr/bin/env bash
# One busy reservation; reuse the completed 7.6 GiB construction validation.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_native="$joint_repo/research/twist_index_calculus_20261020/endomorphism_image_base/native"
joint_out=${1:?new verification directory}
test ! -e "$joint_out"
test -f "$joint_study/results/algebra-v1-validation.json"
mkdir -p "$joint_out/metadata"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
git rev-parse HEAD > "$joint_out/metadata/source-commit.txt"
sha256sum "$joint_native"/src/joint*.rs "$joint_native/src/main.rs" > "$joint_out/metadata/sources.sha256"
sha256sum "$joint_study/results/algebra-v1-validation.json" > "$joint_out/metadata/reused-construction-validation.sha256"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1
ulimit -c 0
joint_step() {
  local joint_name=$1 joint_rc
  shift
  printf '%q ' "$@" > "$joint_out/metadata/$joint_name.command.txt"
  printf '\n' >> "$joint_out/metadata/$joint_name.command.txt"
  set +e
  "$@" > "$joint_out/metadata/$joint_name.stdout" 2> "$joint_out/metadata/$joint_name.stderr"
  joint_rc=$?
  set -e
  printf '%s\n' "$joint_rc" > "$joint_out/metadata/$joint_name.exit"
  return "$joint_rc"
}
joint_step compile-tests cargo test --locked --release --manifest-path "$joint_native/Cargo.toml" --no-run --message-format=json
joint_test=$(jq -r 'select(.reason=="compiler-artifact" and .profile.test==true and .executable!=null) | .executable' "$joint_out/metadata/compile-tests.stdout" | tail -1)
test -x "$joint_test"
joint_step native-tests prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_test" --test-threads=1
joint_step compile-runner cargo build --locked --release --manifest-path "$joint_native/Cargo.toml"
cp "$joint_native/target/release/twist-endomorphism-image-study" "$joint_out/runner"
chmod 500 "$joint_out/runner"
sha256sum "$joint_out/runner" > "$joint_out/metadata/runner.sha256"
joint_public="$joint_study/runs/algebra-v1/prepared/public/p37-seed0.json"
joint_public_sha=$(sha256sum "$joint_public" | cut -d ' ' -f 1)
for joint_arm in l0-peel phi-l0-active double-l0-active; do
  joint_step "$joint_arm-variant-export" prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-variant-export "$joint_public" "$joint_public_sha" "$joint_arm" 2 "$joint_out/variants/$joint_arm"
done
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'Native repair tests and all six corrected variant inputs complete. No root solving or performance claim.\n' > "$joint_out/complete.txt"
