#!/usr/bin/env bash
# Thin orchestration of the existing native ecbench analysis; no new estimator.
set -euo pipefail
: "${ECBENCH_BIN:?Set ECBENCH_BIN to the source-pinned native ecbench executable}"
session_dir=${1:?Pass the downloaded, verified native session directory}
for pair in 'ic-baseline ic-halftrace-tables' 'ic-baseline ic-aa' 'rho-strong rho-aa' 'rho-strong ic-halftrace-tables'; do
  read -r baseline_arm candidate_arm <<< "$pair"
  "$ECBENCH_BIN" compare --dir "$session_dir" --a "$baseline_arm" --b "$candidate_arm" \
    --resamples 4000 --seed 20261030 --json --save
done
for unpriced_weight in 0 0.1 1; do
  "$ECBENCH_BIN" compare --dir "$session_dir" --a ic-baseline --b ic-halftrace-tables \
    --resamples 4000 --seed 20261030 --weight "$unpriced_weight" --json --save
done
