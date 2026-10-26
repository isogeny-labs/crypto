#!/usr/bin/env bash
# Separate oracle diagnostics, never a polynomial candidate or IC price.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_native="$joint_repo/research/twist_index_calculus_20261020/endomorphism_image_base/native"
joint_out=${1:?new output directory}
test ! -e "$joint_out"
mkdir -p "$joint_out/metadata" "$joint_out/zero" "$joint_out/population"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
git rev-parse HEAD > "$joint_out/metadata/source-commit.txt"
sha256sum "$joint_native"/src/joint*.rs "$joint_native/src/main.rs" "$joint_study/native/uniform.rs" > "$joint_out/metadata/sources.sha256"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
export RAYON_NUM_THREADS=1 CARGO_BUILD_JOBS=1
ulimit -c 0
joint_phase() {
  local joint_id=$1 joint_rc
  shift
  printf '%q ' "$@" > "$joint_out/metadata/$joint_id.command.txt"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/$joint_id.started.txt"
  set +e
  python3 "$joint_repo/tools/isolated_bench.py" busy -- "$@" > "$joint_out/metadata/$joint_id.stdout" 2> "$joint_out/metadata/$joint_id.stderr"
  joint_rc=$?
  set -e
  printf '%s\n' "$joint_rc" > "$joint_out/metadata/$joint_id.exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/$joint_id.finished.txt"
  return "$joint_rc"
}
joint_phase compile-tests cargo test --locked --release --manifest-path "$joint_native/Cargo.toml" --no-run --message-format=json
joint_test=$(jq -r 'select(.reason=="compiler-artifact" and .profile.test==true and .executable!=null) | .executable' "$joint_out/metadata/compile-tests.stdout" | tail -1)
test -x "$joint_test"
joint_phase native-tests prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_test" --test-threads=1
joint_phase compile-runner cargo build --locked --release --manifest-path "$joint_native/Cargo.toml"
cp "$joint_native/target/release/twist-endomorphism-image-study" "$joint_out/runner"
chmod 500 "$joint_out/runner"
sha256sum "$joint_out/runner" > "$joint_out/metadata/runner.sha256"
for joint_prime in 37 73 97 157 229 241 337; do
  for joint_seed in 0 1 2; do
    joint_id="p$joint_prime-seed$joint_seed"
    joint_input="$joint_study/runs/algebra-v1/prepared/public/$joint_id.json"
    joint_sha=$(sha256sum "$joint_input" | cut -d ' ' -f 1)
    joint_phase "zero-$joint_id" prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-zero-oracle "$joint_input" "$joint_sha" "$joint_out/zero/$joint_id" || true
  done
done
for joint_seed in 0 1 2; do
  joint_id="p37-seed$joint_seed"
  joint_input="$joint_study/runs/algebra-v1/prepared/public/$joint_id.json"
  joint_sha=$(sha256sum "$joint_input" | cut -d ' ' -f 1)
  joint_phase "population-$joint_id" prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-population-oracle "$joint_input" "$joint_sha" "$joint_out/population/$joint_id" || true
done
joint_phase summarize prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-population-summary "$joint_out" "$joint_out/summary.json"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'Oracle diagnostics complete; inspect signed physical sets, independent rank and population mass. No polynomial solve or timing claim.\n' > "$joint_out/complete.txt"
