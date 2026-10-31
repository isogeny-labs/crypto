#!/usr/bin/env bash
# Correct only the managed-service PATH; preserve failed v2 and all limits.
set -euo pipefail
export PATH=/home/ec2-user/.cargo/bin:/home/ec2-user/.local/bin:/usr/local/bin:/usr/bin:/bin
cd /home/ec2-user/crypto-twist-ic
joint_study=research/twist_index_calculus_20261020/joint_image_elimination
bash "$joint_study/run-publication-gate.sh" "$joint_study/results/publication/post-latest-main-v3"
