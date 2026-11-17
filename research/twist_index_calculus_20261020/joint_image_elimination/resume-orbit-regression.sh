#!/usr/bin/env bash
# Resume only the three source-block unions with no terminal receipt.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_old="$joint_study/runs/regression-v1"
joint_out=${1:?new recovery directory}
test ! -e "$joint_out"
mkdir -p "$joint_out/metadata" "$joint_out/solves" "$joint_out/audits"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
cp "$joint_old/runner" "$joint_out/runner"
chmod 500 "$joint_out/runner"
sha256sum "$joint_old/runner" "$joint_out/runner" > "$joint_out/metadata/frozen-runner.sha256"
git rev-parse HEAD > "$joint_out/metadata/orchestration-source-commit.txt"
cp "$joint_old/metadata/source-commit.txt" "$joint_out/metadata/runner-source-commit.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
export RAYON_NUM_THREADS=1
ulimit -c 0
joint_public="$joint_study/runs/algebra-v1/prepared/public/p37-seed0.json"
joint_public_sha=$(sha256sum "$joint_public" | cut -d ' ' -f 1)
for joint_arm in l0-peel phi-l0-active double-l0-active; do
  joint_id="$joint_arm-orbit-union"
  test ! -e "$joint_old/metadata/$joint_id.exit"
  test ! -e "$joint_old/solves/$joint_id"
  joint_manifest="$joint_old/orbit-inputs/$joint_arm/manifest.json"
  joint_sha=$(sha256sum "$joint_manifest" | cut -d ' ' -f 1)
  joint_command=("$joint_out/runner" --joint-orbit-solve "$joint_public" "$joint_public_sha" "$joint_manifest" "$joint_sha" "$joint_out/solves/$joint_id")
  printf '%q ' "${joint_command[@]}" > "$joint_out/metadata/$joint_id.command.txt"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/$joint_id.started.txt"
  set +e
  python3 "$joint_repo/tools/isolated_bench.py" busy -- prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "${joint_command[@]}" > "$joint_out/metadata/$joint_id.stdout" 2> "$joint_out/metadata/$joint_id.stderr"
  joint_rc=$?
  set -e
  printf '%s\n' "$joint_rc" > "$joint_out/metadata/$joint_id.exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/$joint_id.finished.txt"
  if test -f "$joint_out/solves/$joint_id/summary.json"; then
    if test "$joint_arm" = double-l0-active; then
      joint_ref="/home/ec2-user/.cache/ephemeral/runs/20261023-052011-twist-image-doubling-e8f20f/out/reference-first/p37-seed0/$joint_arm.json"
    else
      joint_ref="/home/ec2-user/.cache/ephemeral/runs/20261023-041850-twist-image-nine-ref-0cbad7/out/reference-first/p37-seed0/$joint_arm.json"
    fi
    set +e
    python3 "$joint_repo/tools/isolated_bench.py" busy -- prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-check "$joint_public" "$joint_public_sha" "$joint_out/solves/$joint_id" "$joint_ref" "$joint_out/audits/$joint_id" > "$joint_out/metadata/$joint_id-check.stdout" 2> "$joint_out/metadata/$joint_id-check.stderr"
    joint_rc=$?
    set -e
    printf '%s\n' "$joint_rc" > "$joint_out/metadata/$joint_id-check.exit"
  fi
done
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'Only previously interrupted source-block unions were executed. Inspect completeness and independent audits.\n' > "$joint_out/complete.txt"
