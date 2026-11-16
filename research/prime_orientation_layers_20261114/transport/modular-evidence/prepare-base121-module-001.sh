#!/usr/bin/env bash
set -euo pipefail
# Packaging recipe only. Source121 is the accepted, exact immutable qualifier.
# Run through this study's existing `lake env` with the pinned Lean toolchain.
# Use a NEW output directory and NEW pre/post-call logs for another replay.
if [ "$#" -ne 1 ]; then
  printf '%s\n' 'usage: prepare-base121-module-001.sh NEW_OUTPUT_DIRECTORY' >&2
  exit 2
fi
task_source=/workspace/audit/norm-recovery-completion-worktree/research/prime_orientation_layers_20261114/transport/modular-evidence/draft-121.lean.txt
task_expected=ab37a418d5d2f2500f541dc324463aec9b1e1783a61e9b945dbf5192f8e121cb
task_output=$1
printf '%s  %s\n' "$task_expected" "$task_source" | sha256sum --check --status
mkdir -- "$task_output"
cp -- "$task_source" "$task_output/OrientationStudyBase121.lean"
chmod a-w -- "$task_output/OrientationStudyBase121.lean"
export LEAN_PATH="/workspace/audit/crypto-lean-first-moment-worktree/.lake/build/lib/lean:${LEAN_PATH:-}"
lean --root="$task_output" -o "$task_output/OrientationStudyBase121.olean" "$task_output/OrientationStudyBase121.lean"
