#!/usr/bin/env bash
# Bounded correctness diagnostics. Native code owns all mathematical work.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_algebra=${2:-"$joint_study/runs/algebra-v1"}
joint_output=${1:?new output directory}
joint_refs=${3:-/home/ec2-user/.cache/ephemeral/runs/20261023-041850-twist-image-nine-ref-0cbad7/out/reference-first}
joint_double=${4:-/home/ec2-user/.cache/ephemeral/runs/20261023-052011-twist-image-doubling-e8f20f/out/reference-first}
test ! -e "$joint_output"
mkdir -p "$joint_output/metadata" "$joint_output/orbit-inputs" "$joint_output/solves" "$joint_output/audits"
joint_output=$(cd "$joint_output" && pwd)
cp "$joint_repo/research/twist_index_calculus_20261020/endomorphism_image_base/native/target/release/twist-endomorphism-image-study" "$joint_output/runner"
chmod 500 "$joint_output/runner"
cp "$0" "$joint_output/metadata/executed-script.sh"
git rev-parse HEAD > "$joint_output/metadata/source-commit.txt"
git diff --name-only HEAD -- research/twist_index_calculus_20261020/endomorphism_image_base/native/src > "$joint_output/metadata/native-dirty-paths.txt"
sha256sum "$joint_output/runner" > "$joint_output/metadata/runner.sha256"
lscpu --json > "$joint_output/metadata/cpu.json"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_output/metadata/started-utc.txt"
printf '%s\n' "$BASHPID" > "$joint_output/metadata/launcher-pid.txt"
export RAYON_NUM_THREADS=1
joint_public="$joint_algebra/prepared/public/p37-seed0.json"
joint_public_sha=$(sha256sum "$joint_public" | cut -d ' ' -f 1)
joint_phase() {
  local joint_name=$1 joint_rc
  shift
  printf '%q ' "$@" > "$joint_output/metadata/$joint_name.command.txt"
  printf '\n' >> "$joint_output/metadata/$joint_name.command.txt"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_output/metadata/$joint_name.started.txt"
  set +e
  python3 "$joint_repo/tools/isolated_bench.py" busy -- \
    prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$@" \
    > "$joint_output/metadata/$joint_name.stdout" 2> "$joint_output/metadata/$joint_name.stderr"
  joint_rc=$?
  set -e
  printf '%s\n' "$joint_rc" > "$joint_output/metadata/$joint_name.exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_output/metadata/$joint_name.finished.txt"
  return "$joint_rc"
}
joint_check() {
  local joint_name=$1 joint_arm=$2 joint_reference
  if test "$joint_arm" = double-l0-active; then
    joint_reference="$joint_double/p37-seed0/$joint_arm.json"
  else
    joint_reference="$joint_refs/p37-seed0/$joint_arm.json"
  fi
  if test -f "$joint_output/solves/$joint_name/summary.json"; then
    joint_phase "$joint_name-check" "$joint_output/runner" --joint-check \
      "$joint_public" "$joint_public_sha" "$joint_output/solves/$joint_name" \
      "$joint_reference" "$joint_output/audits/$joint_name" || true
  fi
}
for joint_arm in l0-peel phi-l0-active double-l0-active; do
  joint_phase "$joint_arm-orbit-export" "$joint_output/runner" --joint-orbit-export \
    "$joint_public" "$joint_public_sha" "$joint_arm" 2 "$joint_output/orbit-inputs/$joint_arm" || true
done
# Three independent processes per batch, each with its own 8 GiB / 1,200 s
# limit and one Rayon thread. Durations are unadmitted diagnostics.
for joint_encoding in finite-domain geometric orbit-union; do
  joint_pids=()
  for joint_arm in l0-peel phi-l0-active double-l0-active; do
    (
      joint_name="$joint_arm-$joint_encoding"
      if test "$joint_encoding" = orbit-union; then
        joint_system="$joint_output/orbit-inputs/$joint_arm/manifest.json"
        joint_mode=--joint-orbit-solve
      else
        joint_system="$joint_algebra/exports/p37-s0-$joint_arm-q2/$joint_encoding.json"
        joint_mode=--joint-solve
      fi
      if test -f "$joint_system"; then
        joint_system_sha=$(sha256sum "$joint_system" | cut -d ' ' -f 1)
        joint_phase "$joint_name" "$joint_output/runner" "$joint_mode" \
          "$joint_public" "$joint_public_sha" "$joint_system" "$joint_system_sha" "$joint_output/solves/$joint_name" || true
        joint_check "$joint_name" "$joint_arm"
      fi
    ) &
    joint_pids+=("$!")
  done
  for joint_pid in "${joint_pids[@]}"; do wait "$joint_pid"; done
done
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_output/metadata/finished-utc.txt"
printf 'Regression suite terminal. Inspect every solver exit and independent audit; completion is not a pass verdict.\n' > "$joint_output/complete.txt"
