#!/usr/bin/env bash
# Thin orchestration only. All algebra, selection and verification are native.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_binary="$joint_repo/research/twist_index_calculus_20261020/endomorphism_image_base/native/target/release/twist-endomorphism-image-study"
joint_output=${1:?new output directory}
joint_refs=${2:-/home/ec2-user/.cache/ephemeral/runs/20261023-041850-twist-image-nine-ref-0cbad7/out/reference-first}
joint_double=${3:-/home/ec2-user/.cache/ephemeral/runs/20261023-052011-twist-image-doubling-e8f20f/out/reference-first}
joint_tape="$joint_repo/research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson"
test ! -e "$joint_output"
test -f "$joint_refs/p37-seed0/phi-l0-active.json"
test -f "$joint_double/p37-seed0/double-l0-active.json"
mkdir -p "$joint_output/metadata" "$joint_output/exports"
joint_output=$(cd "$joint_output" && pwd)
cp "$joint_binary" "$joint_output/runner"
chmod 500 "$joint_output/runner"
joint_binary="$joint_output/runner"
cp "$0" "$joint_output/metadata/executed-script.sh"
git rev-parse HEAD > "$joint_output/metadata/source-commit.txt"
git status --short --untracked-files=no > "$joint_output/metadata/working-tree.txt"
sha256sum "$joint_binary" "$joint_tape" \
  "$joint_repo/research/twist_index_calculus_20261020/endomorphism_image_base/native/src/joint.rs" \
  "$joint_repo/research/twist_index_calculus_20261020/endomorphism_image_base/native/src/main.rs" \
  "$joint_repo/research/twist_index_calculus_20261020/endomorphism_image_base/native/src/image.rs" \
  > "$joint_output/metadata/source-and-input.sha256"
lscpu --json > "$joint_output/metadata/cpu.json"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_output/metadata/started-utc.txt"
printf '%s\n' "$BASHPID" > "$joint_output/metadata/launcher-pid.txt"
export RAYON_NUM_THREADS=1
joint_step() {
  local joint_name=$1 joint_status
  shift
  printf '%q ' "$@" > "$joint_output/metadata/$joint_name.command.txt"
  printf '\n' >> "$joint_output/metadata/$joint_name.command.txt"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_output/metadata/$joint_name.started.txt"
  set +e
  python3 "$joint_repo/tools/isolated_bench.py" busy -- \
    prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$@" \
    > "$joint_output/metadata/$joint_name.stdout" \
    2> "$joint_output/metadata/$joint_name.stderr"
  joint_status=$?
  set -e
  printf '%s\n' "$joint_status" > "$joint_output/metadata/$joint_name.exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_output/metadata/$joint_name.finished.txt"
  return "$joint_status"
}
joint_step prepare "$joint_binary" --joint-prepare "$joint_tape" "$joint_output/prepared"
joint_step audit "$joint_binary" --joint-audit "$joint_tape" "$joint_refs" "$joint_double" "$joint_output/audit"
joint_step workload "$joint_binary" --joint-workload "$joint_output/prepared" "$joint_refs" "$joint_double" "$joint_output/workload.json"
sha256sum "$joint_output/workload.json" > "$joint_output/metadata/workload.sha256"
while IFS=$'\t' read -r joint_id joint_public joint_sha joint_arm joint_query; do
  if joint_step "$joint_id" "$joint_binary" --joint-export "$joint_public" "$joint_sha" "$joint_arm" "$joint_query" "$joint_output/exports/$joint_id"; then
    printf 'exported %s\n' "$joint_id"
  else
    printf 'retained failed export %s; see its exit and partial output\n' "$joint_id"
  fi
done < <(jq -r '.tasks[] | [.id,.public_path,.public_sha256,.arm,(.query_index|tostring)] | @tsv' "$joint_output/workload.json")
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_output/metadata/finished-utc.txt"
printf 'Construction suite terminal; individual exits decide success. No root solving or performance claim.\n' > "$joint_output/complete.txt"
