#!/usr/bin/env bash
# Finish a launch whose client was interrupted after `codebuild start-build`: read the build id that
# launch-remote saved in OUTPUT_DIR/build-start.json and run wait_launcher_result.sh on that exact
# build. It never starts a build; it fails when build-start.json is missing.
#   resume_wait.sh JOB_NAME OUTPUT_DIR MAX_WAIT_SECONDS
set -euo pipefail
[[ $# == 3 ]] || { echo 'usage: resume_wait.sh JOB_NAME OUTPUT_DIR MAX_WAIT_SECONDS' >&2; exit 2; }
job=$1 out=$2 max=$3
build_id=$(jq -er .build.id "$out/build-start.json") \
  || { echo "FAIL: no build id in $out/build-start.json; nothing was started" >&2; exit 60; }
exec research/prime_medium_rho_p4_confirm_20261108/cards/wait_launcher_result.sh "$build_id" "$job" "$out" "$max"
