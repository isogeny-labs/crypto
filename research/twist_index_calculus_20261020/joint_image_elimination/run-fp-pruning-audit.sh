#!/usr/bin/env bash
# Check a counterfactual rejection predicate against the frozen complete oracle.
# Never reruns the historical polynomial collector or the relation oracle.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_native="$joint_study/../endomorphism_image_base/native"
joint_out=${1:?new output directory}
test ! -e "$joint_out"
# A managed caller may queue this audit while the required independent
# replay is still running. Waiting consumes no heavy-work reservation.
while systemctl is-active --quiet twist-joint-square-zero-verify-v1.service; do sleep 10; done
# The stronger square result, rather than the weaker discriminant statement,
# is required before this experiment by SUPPLEMENT_SQUARE_PRUNING.md.
test "$(cat "$joint_study/runs/square-ideal-lean-v1/exit")" = 0
test "$(jq -r .status "$joint_study/runs/square-zero-verify-v1/verified/summary.json")" = pass
test "$(jq -r .target "$joint_study/runs/square-zero-verify-v1/verified/summary.json")" = square-defect-cube
test ! -e "$joint_out"
mkdir -p "$joint_out/metadata" "$joint_out/cases"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
git rev-parse HEAD > "$joint_out/metadata/source-commit.txt"
sha256sum "$joint_native"/src/*.rs "$joint_study/native/fp_pruning_audit.rs" > "$joint_out/metadata/sources.sha256"
cp "$joint_study/runs/algebra-v1/workload.json" "$joint_out/metadata/workload.json"
sha256sum "$joint_out/metadata/workload.json" > "$joint_out/metadata/workload.sha256"
cp "$joint_study/runs/square-zero-verify-v1/verified/summary.json" "$joint_out/metadata/required-square-idc.json"
cp "$joint_study/runs/square-ideal-lean-v1/exit" "$joint_out/metadata/required-square-lean.exit"
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
test -x "$joint_test"
joint_phase native-tests prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_test" --test-threads=1
joint_phase compile-runner cargo build --locked --release --manifest-path "$joint_native/Cargo.toml"
cp "$joint_native/target/release/twist-endomorphism-image-study" "$joint_out/runner"
chmod 500 "$joint_out/runner"
sha256sum "$joint_out/runner" > "$joint_out/metadata/runner.sha256"
# Use the actual preserved cache location; do not regenerate a missing oracle.
joint_refs=/home/ec2-user/.cache/ephemeral/runs/20261023-041850-twist-image-nine-ref-0cbad7/out/reference-first
joint_double=/home/ec2-user/.cache/ephemeral/runs/20261023-052011-twist-image-doubling-e8f20f/out/reference-first
for joint_prime in 37 73 97 157 229 241 337; do
  for joint_seed in 0 1 2; do
    joint_id="p$joint_prime-seed$joint_seed"
    joint_public="$joint_study/runs/algebra-v1/prepared/public/$joint_id.json"
    joint_sha=$(sha256sum "$joint_public" | cut -d ' ' -f 1)
    joint_phase "$joint_id" prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-fp-pruning-audit "$joint_public" "$joint_sha" "$joint_out/metadata/workload.json" "$joint_refs" "$joint_double" "$joint_out/cases/$joint_id" || true
  done
done
joint_phase summarize prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-fp-pruning-summary "$joint_out" "$joint_out/summary.json"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'Counterfactual audit terminal. No collector or group oracle was repeated; no elapsed cost measured.\n' > "$joint_out/complete.txt"
