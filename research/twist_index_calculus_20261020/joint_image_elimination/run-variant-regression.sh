#!/usr/bin/env bash
# The already frozen implicit/full-symmetry regression; no admitted timings.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_inputs="$joint_study/runs/verification-v8"
joint_out=${1:?new output directory}
test ! -e "$joint_out"
test -f "$joint_inputs/complete.txt"
mkdir -p "$joint_out/metadata" "$joint_out/solves" "$joint_out/audits"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
cp "$joint_inputs/runner" "$joint_out/runner"
chmod 500 "$joint_out/runner"
sha256sum "$joint_inputs/runner" "$joint_out/runner" > "$joint_out/metadata/runner.sha256"
cp "$joint_inputs/metadata/source-commit.txt" "$joint_out/metadata/runner-source-commit.txt"
git rev-parse HEAD > "$joint_out/metadata/orchestration-source-commit.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
export RAYON_NUM_THREADS=1
ulimit -c 0
joint_public="$joint_study/runs/algebra-v1/prepared/public/p37-seed0.json"
joint_public_sha=$(sha256sum "$joint_public" | cut -d ' ' -f 1)
joint_phase() {
  local joint_id=$1 joint_rc
  shift
  printf '%q ' "$@" > "$joint_out/metadata/$joint_id.command.txt"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/$joint_id.started.txt"
  set +e
  python3 "$joint_repo/tools/isolated_bench.py" busy -- prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$@" > "$joint_out/metadata/$joint_id.stdout" 2> "$joint_out/metadata/$joint_id.stderr"
  joint_rc=$?
  set -e
  printf '%s\n' "$joint_rc" > "$joint_out/metadata/$joint_id.exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/$joint_id.finished.txt"
  return "$joint_rc"
}
for joint_arm in l0-peel phi-l0-active double-l0-active; do
  for joint_encoding in geometric system; do
    joint_id="$joint_arm-$joint_encoding"
    joint_system="$joint_inputs/variants/$joint_arm/$joint_encoding.json"
    joint_sha=$(sha256sum "$joint_system" | cut -d ' ' -f 1)
    joint_phase "$joint_id" "$joint_out/runner" --joint-solve "$joint_public" "$joint_public_sha" "$joint_system" "$joint_sha" "$joint_out/solves/$joint_id" || true
    if test -f "$joint_out/solves/$joint_id/summary.json"; then
      if test "$joint_arm" = double-l0-active; then
        joint_ref="/home/ec2-user/.cache/ephemeral/runs/20261023-052011-twist-image-doubling-e8f20f/out/reference-first/p37-seed0/$joint_arm.json"
      else
        joint_ref="/home/ec2-user/.cache/ephemeral/runs/20261023-041850-twist-image-nine-ref-0cbad7/out/reference-first/p37-seed0/$joint_arm.json"
      fi
      joint_phase "$joint_id-check" "$joint_out/runner" --joint-check "$joint_public" "$joint_public_sha" "$joint_out/solves/$joint_id" "$joint_ref" "$joint_out/audits/$joint_id" || true
    fi
  done
done
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'Six implicit/full-symmetry solves terminal. Each complete result still needs its independent exact-set audit.\n' > "$joint_out/complete.txt"
