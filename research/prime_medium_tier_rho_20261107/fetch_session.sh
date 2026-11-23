#!/usr/bin/env bash
# Download one admitted job's pod artifacts from the results_uri in its
# receipts/JN/admission.json, check them against the pod's SHA256SUMS, and put
# the session directory where fit.sh expects it:
#   research/prime_medium_tier_rho_20261107/sessions/JN      (the session)
#   research/prime_medium_tier_rho_20261107/sessions/JN.pod  (all pod artifacts)
#   fetch_session.sh JN      (JN is J1 .. J6; run from the repository root)
# It refuses when either destination exists. Needs AWS_REGION.
set -euo pipefail
[[ $# == 1 && "$1" =~ ^J[1-6]$ ]] || { echo 'usage: fetch_session.sh J1|J2|J3|J4|J5|J6' >&2; exit 2; }
: "${AWS_REGION:?}"
j=$1
D=research/prime_medium_tier_rho_20261107
adm=$D/receipts/$j/admission.json
lower=$(printf '%s' "$j" | tr 'J' 'j')
jq -e --arg job "prime-medium-rho-20261107-$lower" '.admitted_for_counts==true and .job_name==$job
  and (.results_uri|test("^s3://isolab-results-104186811093-us-east-2/runs/prime-medium-rho-20261107-j[1-6]/[0-9a-f-]{36}/$"))
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
