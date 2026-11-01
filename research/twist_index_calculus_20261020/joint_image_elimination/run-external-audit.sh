#!/usr/bin/env bash
# Separate verifier of terminal external bases; no candidate oracle or IC timing.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_native="$joint_repo/research/twist_index_calculus_20261020/endomorphism_image_base/native"
joint_out=${1:?new output directory}
joint_external=${2:?existing external run}
test ! -e "$joint_out"
mkdir -p "$joint_out/metadata" "$joint_out/audits"
joint_out=$(cd "$joint_out" && pwd)
joint_external=$(cd "$joint_external" && pwd)
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
joint_public="$joint_study/runs/algebra-v1/prepared/public/p37-seed0.json"
joint_public_sha=$(sha256sum "$joint_public" | cut -d ' ' -f 1)
# Take a terminal-only snapshot after the verifier has been compiled. Pending
# external cases get no verdict and may be checked later with this frozen runner.
for joint_case in "$joint_external"/systems/*; do
  test -f "$joint_case/exit" || continue
  test "$(cat "$joint_case/exit")" = 0 || continue
  test -s "$joint_case/basis.txt" || continue
  joint_id=$(basename "$joint_case")
  case "$joint_id" in
    double-l0-active-*) joint_ref="/home/ec2-user/.cache/ephemeral/runs/20261023-052011-twist-image-doubling-e8f20f/out/reference-first/p37-seed0/double-l0-active.json" ;;
    phi-l0-active-*) joint_ref="/home/ec2-user/.cache/ephemeral/runs/20261023-041850-twist-image-nine-ref-0cbad7/out/reference-first/p37-seed0/phi-l0-active.json" ;;
    l0-peel-*) joint_ref="/home/ec2-user/.cache/ephemeral/runs/20261023-041850-twist-image-nine-ref-0cbad7/out/reference-first/p37-seed0/l0-peel.json" ;;
    *) exit 3 ;;
  esac
  read -r joint_system_sha joint_system < "$joint_case/inputs.sha256"
  joint_basis_sha=$(sha256sum "$joint_case/basis.txt" | cut -d ' ' -f 1)
  joint_phase "$joint_id" prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-external-audit "$joint_public" "$joint_public_sha" "$joint_system" "$joint_system_sha" "$joint_case/basis.txt" "$joint_basis_sha" "$joint_ref" "$joint_out/audits/$joint_id" || true
done
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'Terminal-only external audit snapshot. Inspect every per-case verdict; exit zero here is not a basis, completeness or speed claim.\n' > "$joint_out/complete.txt"
