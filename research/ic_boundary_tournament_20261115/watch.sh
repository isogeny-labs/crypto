#!/usr/bin/env bash
# Observe the existing immutable Job; never submit, resume, retry or replace it.
set -u
here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
receipt="$root/research/sat_factor_base_review_20260908/autolab/runs/tournament-20261115-single-target"
mkdir -p "$receipt/monitor" "$receipt/results"
printf '%s\n' "$$" > "$receipt/monitor/pid.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$receipt/monitor/started-at.txt"
job=ic-boundary-single-20261115-1912
expected_uid=751ec9e0-1e39-4c6e-a592-cc76c66a59a9
# UTC 2026-11-16 08:11:12, frozen before submission; final grace is observation only.
deadline=1794816672
kube=(kubectl --request-timeout=15s -n isolab)
export AWS_PROFILE=ecc-challenges-new
while (( $(date +%s) <= deadline+120 )); do
  if ! "${kube[@]}" get job "$job" -o json > "$receipt/monitor/last-job.json.tmp"; then sleep 60; continue; fi
  mv "$receipt/monitor/last-job.json.tmp" "$receipt/monitor/last-job.json"
  uid=$(jq -r .metadata.uid "$receipt/monitor/last-job.json")
  if [[ "$uid" != "$expected_uid" ]]; then printf 'IDENTITY_MISMATCH\n' > "$receipt/monitor/status.txt"; exit 3; fi
  "${kube[@]}" get pods -l "job-name=$job" -o json > "$receipt/monitor/last-pods.json"
  terminal=$(jq -r '[.status.conditions[]? | select(.status=="True" and (.type=="Complete" or .type=="Failed")) | .type][0] // "ACTIVE_OR_QUEUED"' "$receipt/monitor/last-job.json")
  printf '%s\n' "$terminal" > "$receipt/monitor/status.txt"
  if [[ "$terminal" != ACTIVE_OR_QUEUED ]]; then break; fi
  sleep 60
done
"${kube[@]}" get events --field-selector "involvedObject.kind=Job,involvedObject.name=$job" -o json > "$receipt/monitor/events.json"
"${kube[@]}" logs "job/$job" --all-containers=true --tail=1000 > "$receipt/monitor/job.log" 2>&1 || true
aws s3 sync "s3://isolab-results-104186811093-us-east-2/runs/$job/" "$receipt/results/" --only-show-errors
fetch_status=$?
printf '%s\n' "$fetch_status" > "$receipt/monitor/fetch.exit"
if ((fetch_status==0)); then
  while IFS= read -r manifest; do
    (cd "$(dirname "$manifest")" && shasum -a 256 -c SHA256SUMS) >> "$receipt/monitor/hash-check.log" 2>&1 || exit 4
  done < <(find "$receipt/results" -type f -name SHA256SUMS)
fi
printf '%s\n' "$(date -u '+%Y-%m-%dT%H:%M:%SZ')" > "$receipt/monitor/stopped-at.txt"
exit "$fetch_status"
