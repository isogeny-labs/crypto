#!/bin/bash
# Existing artifact transport only; all replay verification runs in native ecbench.
set -euo pipefail
if ! command -v aws >/dev/null; then
  python3 -m venv "$RUNNER_TEMP/artifacts-awscli"
  "$RUNNER_TEMP/artifacts-awscli/bin/python" -m pip install --disable-pip-version-check awscli==1.46.2
  export ARTIFACTS_AWS="$RUNNER_TEMP/artifacts-awscli/bin/aws"
fi
# The second pass finds hash-bound stderr attachments in the hydrated records.
for replay_pass in core attachments; do
  bash scripts/ic_audit/check_plan_coverage.sh --list-inputs > "$RUNNER_TEMP/replay-$replay_pass-inputs.txt"
  mapfile -t replay_inputs < "$RUNNER_TEMP/replay-$replay_pass-inputs.txt"
  test "${#replay_inputs[@]}" -gt 0
  python3 tools/artifacts/artifacts.py pull --jobs 16 "${replay_inputs[@]}"
done
