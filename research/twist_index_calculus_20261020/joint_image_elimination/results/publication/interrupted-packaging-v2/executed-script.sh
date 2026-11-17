#!/usr/bin/env bash
# Artifact operations only; no scientific program is rerun.
set -euo pipefail
cd /home/ec2-user/crypto-twist-ic
joint_study=research/twist_index_calculus_20261020/joint_image_elimination
joint_out=/home/ec2-user/scratch/twist-joint-complete-archive-v2
test ! -e "$joint_out"
mkdir "$joint_out"
cp "$0" "$joint_out/executed-script.sh"
git rev-parse HEAD > "$joint_out/source-commit.txt"
sha256sum "$joint_study/pack-complete-checkpoint.sh" "$joint_study/verify-complete-checkpoint.sh" > "$joint_out/sources.sha256"
systemctl list-units --all --no-legend --plain 'twist-joint-*' > "$joint_out/owner-inventory.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/started-utc.txt"
bash "$joint_study/pack-complete-checkpoint.sh" "$joint_out/runs.tar.xz" > "$joint_out/pack.stdout" 2> "$joint_out/pack.stderr"
printf '0\n' > "$joint_out/pack.exit"
bash "$joint_study/verify-complete-checkpoint.sh" "$joint_out/runs.tar.xz" "$joint_out/verified" > "$joint_out/verify.stdout" 2> "$joint_out/verify.stderr"
printf '0\n' > "$joint_out/verify.exit"
joint_sha=$(cut -d ' ' -f 1 "$joint_out/verified/repacked.sha256")
jq --arg sha "$joint_sha" '.+{byte_identical_repack:true,repacked_sha256:$sha,scope:"All 43 reconciled raw run directories. Content integrity only; no archived program executed and no new mathematical or performance validation implied."}' "$joint_out/verified/verification.json" > "$joint_out/verified/complete-verification.json"
sha256sum -c "$joint_out/sources.sha256" > "$joint_out/sources-unchanged.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/finished-utc.txt"
printf '0\n' > "$joint_out/exit"
