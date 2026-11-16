#!/usr/bin/env bash
# Only empty frozen-field journals and refused invocations; no field sampling.
set -euo pipefail
nist_cli_build=${1:?binary directory required}
nist_cli_source=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
nist_cli_cohort="$nist_cli_source/../strict_elkies_20261029/inputs.tsv"
nist_cli_tmp=$(mktemp -d "${TMPDIR:-/tmp}/nist-journal-cli.XXXXXX")
nist_cli_nonce=0000000000000000000000000000000000000000000000000000000000000000
printf 'bits\tindex\tseed\tc\tb\tn\tstatus\n' > "$nist_cli_tmp/attempts.tsv"
printf 'bits\tordinal\tindex\tseed\tc\tb\tn\tGx\tGy\tstrict691\tramified691\n' > "$nist_cli_tmp/accepted.tsv"

expect_refusal() {
    local nist_cli_case=$1
    shift
    if "$@" > "$nist_cli_tmp/$nist_cli_case.stdout" 2> "$nist_cli_tmp/$nist_cli_case.stderr"; then
        printf 'FAIL: accepted %s\n' "$nist_cli_case" >&2
        exit 1
    else
        local nist_cli_status=$?
        test "$nist_cli_status" -eq 1
        printf 'ok CLI refusal %s exit=%s: ' "$nist_cli_case" "$nist_cli_status"
        cat "$nist_cli_tmp/$nist_cli_case.stderr"
    fi
}

"$nist_cli_build/journal_check" inspect "$nist_cli_cohort" 192 "$nist_cli_nonce" \
    "$nist_cli_tmp/attempts.tsv" "$nist_cli_tmp/accepted.tsv" sampler-v1 10
expect_refusal incomplete "$nist_cli_build/journal_check" verify "$nist_cli_cohort" 192 "$nist_cli_nonce" \
    "$nist_cli_tmp/attempts.tsv" "$nist_cli_tmp/accepted.tsv" sampler-v1 10
expect_refusal toy-field "$nist_cli_build/journal_check" inspect "$nist_cli_cohort" 13 "$nist_cli_nonce" \
    "$nist_cli_tmp/attempts.tsv" "$nist_cli_tmp/accepted.tsv" sampler-v1 10
expect_refusal target "$nist_cli_build/journal_check" inspect "$nist_cli_cohort" 192 "$nist_cli_nonce" \
    "$nist_cli_tmp/attempts.tsv" "$nist_cli_tmp/accepted.tsv" sampler-v1 11
expect_refusal format "$nist_cli_build/journal_check" inspect "$nist_cli_cohort" 192 "$nist_cli_nonce" \
    "$nist_cli_tmp/attempts.tsv" "$nist_cli_tmp/accepted.tsv" different-format 10
expect_refusal acknowledgement "$nist_cli_build/sampler_resume" --pilot "$nist_cli_cohort" 192 "$nist_cli_nonce" \
    "$nist_cli_tmp/never-created" NEW NO_ACK
expect_refusal worker-toy-field "$nist_cli_build/sampler_resume" --pilot "$nist_cli_cohort" 13 "$nist_cli_nonce" \
    "$nist_cli_tmp/never-created" NEW ACK_FROZEN_CHECKPOINT
test ! -e "$nist_cli_tmp/never-created"
printf 'CLI controls complete: one empty-prefix inspection, six refusals, no sampler launch\n'
