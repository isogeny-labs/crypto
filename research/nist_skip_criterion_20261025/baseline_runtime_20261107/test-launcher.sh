#!/bin/bash
# Refusal-only development controls: never invoke a valid random-curve command.
set -euo pipefail
nist_test_tmp=$(mktemp -d /work/compute/launcher-controls.XXXXXX)
nist_test_zero=0000000000000000000000000000000000000000000000000000000000000000
mkdir "$nist_test_tmp/existing"
mkdir "$nist_test_tmp/existing/field-192"
refusal() {
    local nist_case=$1
    shift
    set +e
    env -i PATH=/usr/bin:/bin HOME=/work TMPDIR=/tmp "$@" \
        > "$nist_test_tmp/$nist_case.stdout" 2> "$nist_test_tmp/$nist_case.stderr"
    local nist_status=$?
    set -e
    [[ $nist_status == 2 ]] || { echo "Unexpected exit: $nist_case $nist_status" >&2; exit 1; }
    printf 'ok launcher refusal %s exit=%s: ' "$nist_case" "$nist_status"
    cat "$nist_test_tmp/$nist_case.stderr"
}
nist_launcher=/opt/nist/bin/run-field
refusal marker "$nist_launcher" --pilot 192 "$nist_test_zero" ACK_FROZEN_CHECKPOINT
refusal mode ISOLAB_CRIU=1 "$nist_launcher" --other 192 "$nist_test_zero" ACK_FROZEN_CHECKPOINT
refusal field ISOLAB_CRIU=1 "$nist_launcher" --pilot 13 "$nist_test_zero" ACK_FROZEN_CHECKPOINT
refusal nonce ISOLAB_CRIU=1 "$nist_launcher" --pilot 192 invalid ACK_FROZEN_CHECKPOINT
refusal acknowledgement ISOLAB_CRIU=1 "$nist_launcher" --pilot 192 "$nist_test_zero" NO_ACK
refusal output ISOLAB_CRIU=1 "$nist_launcher" --pilot 192 "$nist_test_zero" ACK_FROZEN_CHECKPOINT
refusal identity ISOLAB_CRIU=1 AWS_ACCESS_KEY_ID=public-refusal-fixture \
    "$nist_launcher" --pilot 192 "$nist_test_zero" ACK_FROZEN_CHECKPOINT
refusal overwrite ISOLAB_CRIU=1 ISOLAB_OUTPUT_DIR="$nist_test_tmp/existing" \
    "$nist_launcher" --pilot 192 "$nist_test_zero" ACK_FROZEN_CHECKPOINT
[[ $(find "$nist_test_tmp/existing/field-192" -mindepth 1 -print | wc -l) == 0 ]]
[[ $(find "$nist_test_tmp" -name journal -print | wc -l) == 0 ]]
printf 'Eight launcher refusals; no producer launched and no journal created.\n'
