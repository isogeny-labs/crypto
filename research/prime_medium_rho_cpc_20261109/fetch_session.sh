#!/usr/bin/env bash
# Download one admitted job's pod artifacts from the results_uri in its
# receipts/JOB/admission.json, check them against the pod's SHA256SUMS, and put
# the session directory where analysis.sh expects it:
#   research/prime_medium_rho_cpc_20261109/sessions/JOB      (the session)
#   research/prime_medium_rho_cpc_20261109/sessions/JOB.pod  (all pod artifacts)
#   fetch_session.sh JOB      (JOB is PC or M1 .. M4; run from the repository root)
# It refuses when either destination exists. Needs AWS_REGION.
# Adapted from research/prime_medium_tier_rho_20261107/fetch_session.sh (paths and job names only).
set -euo pipefail
[[ $# == 1 && "$1" =~ ^(PC|M[1-4])$ ]] || { echo 'usage: fetch_session.sh PC|M1|M2|M3|M4' >&2; exit 2; }
: "${AWS_REGION:?}"
j=$1
D=research/prime_medium_rho_cpc_20261109
adm=$D/receipts/$j/admission.json
lower=$(printf '%s' "$j" | tr 'A-Z' 'a-z')
jq -e --arg job "prime-medium-rho-cpc-20261109-$lower" '.admitted_for_counts==true and .job_name==$job
  and (.results_uri|test("^s3://isolab-results-104186811093-us-east-2/runs/prime-medium-rho-cpc-20261109-(pc|m[1-4])/[0-9a-f-]{36}/$"))
  and (.results_uri|contains("/runs/" + $job + "/"))' "$adm" >/dev/null \
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
