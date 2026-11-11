#!/usr/bin/env bash
# Download one admitted job's pod artifacts from the results_uri in its
# receipts/JOB/admission.json, check them against the pod's SHA256SUMS, and put
# the session directory where the frozen analysis expects it:
#   research/prime_medium_rho_p4_confirm_20261108/sessions/JOB      (the session)
#   research/prime_medium_rho_p4_confirm_20261108/sessions/JOB.pod  (all pod artifacts)
#   fetch_session.sh JOB      (JOB is PC or M1 .. M9; run from the repository root)
# It refuses when either destination exists. Needs AWS_REGION. It starts no job.
# Adapted from research/prime_medium_tier_rho_20261107/fetch_session.sh: paths and job names only.
set -euo pipefail
[[ $# == 1 && "$1" =~ ^(PC|M[1-9])$ ]] || { echo 'usage: fetch_session.sh PC|M1|M2|M3|M4|M5|M6|M7|M8|M9' >&2; exit 2; }
: "${AWS_REGION:?}"
j=$1
D=research/prime_medium_rho_p4_confirm_20261108
adm=$D/receipts/$j/admission.json
lower=$(printf '%s' "$j" | tr 'A-Z' 'a-z')
job="prime-medium-rho-p4-20261108-$lower"
jq -e --arg job "$job" '.admitted_for_counts==true and .job_name==$job
  and (.results_uri|test("^s3://isolab-results-104186811093-us-east-2/runs/" + $job + "/[0-9a-f-]{36}/$"))' "$adm" >/dev/null \
  || { echo "FAIL: $adm is not an admission of this job" >&2; exit 50; }
[[ ! -e $D/sessions/$j && ! -e $D/sessions/$j.pod ]] || { echo "FAIL: sessions/$j or sessions/$j.pod exists" >&2; exit 51; }
uri=$(jq -er .results_uri "$adm")
mkdir -p "$D/sessions"
aws s3 cp --recursive "$uri" "$D/sessions/$j.pod" --only-show-errors
(cd "$D/sessions/$j.pod" && sha256sum -c SHA256SUMS > /dev/null) \
  || { echo "FAIL: pod artifacts of $j differ from SHA256SUMS" >&2; exit 52; }
jq -e '.exit_code==0 or .exit_code==42 or .exit_code==43' "$D/sessions/$j.pod/job-exit.json" >/dev/null \
  || { echo "FAIL: job-exit.json of $j is not exit 0, 42 or 43" >&2; exit 53; }
cp -r "$D/sessions/$j.pod/session" "$D/sessions/$j"
printf 'fetched %s from %s\n' "$j" "$uri"
printf 'records.jsonl sha256 %s\n' "$(sha256sum "$D/sessions/$j/records.jsonl" | cut -d ' ' -f1)"
