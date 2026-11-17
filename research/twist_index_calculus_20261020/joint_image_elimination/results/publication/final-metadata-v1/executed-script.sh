#!/usr/bin/env bash
# Check only the concrete risk from the final added documents/store metadata.
set -euo pipefail
cd /home/ec2-user/crypto-twist-ic
joint_out=research/twist_index_calculus_20261020/joint_image_elimination/results/publication/final-metadata-v1
test ! -e "$joint_out"
mkdir "$joint_out"
cp "$0" "$joint_out/executed-script.sh"
git rev-parse HEAD > "$joint_out/source-commit.txt"
joint_phase() {
  local joint_id=$1 joint_rc
  shift
  printf '%q ' "$@" > "$joint_out/$joint_id.command.txt"
  set +e
  "$@" > "$joint_out/$joint_id.stdout" 2> "$joint_out/$joint_id.stderr"
  joint_rc=$?
  set -e
  printf '%s\n' "$joint_rc" > "$joint_out/$joint_id.exit"
  return "$joint_rc"
}
joint_phase names python3 scripts/check_curve_names.py
joint_phase validation python3 scripts/validation_gate.py check --base origin/main
joint_phase size bash scripts/check-large-files.sh origin/main
sha256sum -c research/twist_index_calculus_20261020/joint_image_elimination/results/publication/post-latest-main-v3/metadata/sources.sha256 > "$joint_out/root-sources-unchanged.txt"
sha256sum -c /home/ec2-user/scratch/twist-image-unrelated-dirty-before-publication.sha256 > "$joint_out/unrelated-dirty-unchanged.txt"
printf '0\n' > "$joint_out/exit"
