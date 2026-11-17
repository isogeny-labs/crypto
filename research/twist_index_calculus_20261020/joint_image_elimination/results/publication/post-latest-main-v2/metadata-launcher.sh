#!/usr/bin/env bash
# Software integration only; the sealed 43-run scientific archive stays fixed.
set -euo pipefail
cd /home/ec2-user/crypto-twist-ic
joint_study=research/twist_index_calculus_20261020/joint_image_elimination
bash "$joint_study/run-publication-gate.sh" "$joint_study/results/publication/post-latest-main-v2"
