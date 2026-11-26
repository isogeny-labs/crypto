#!/usr/bin/env bash
# Frozen finite qualification, run once from the repository root.
set -uo pipefail
umask 077
cutoff_evidence=research/smooth_p_plus_one_20261123/evidence/full_cutoff_v1
for cutoff_name in identity_issue identity_replay f127_d4 f8191_d4 f8191_d5; do
  for cutoff_suffix in json jsonl stderr exit; do
    if [[ -e "$cutoff_evidence/$cutoff_name.$cutoff_suffix" ]]; then
      printf '%s\n' 'Refusing to overwrite retained full-cutoff evidence.' >&2
      exit 2
    fi
  done
done
cutoff_status=0
timeout --kill-after=5s 30 /workspace/scratch/dickson-cutoff-identity-v1 \
  > "$cutoff_evidence/identity_issue.json" 2> "$cutoff_evidence/identity_issue.stderr"
cutoff_exit=$?
printf '%s\n' "$cutoff_exit" > "$cutoff_evidence/identity_issue.exit"
if (( cutoff_exit != 0 )); then cutoff_status=1; fi
record_cutoff() {
  local cutoff_name=$1 cutoff_limit=$2
  shift 2
  RAYON_NUM_THREADS=1 \
    /workspace/research_stream/research-stream capture /workspace/research_stream/live-native.jsonl -- \
    timeout --kill-after=5s "$cutoff_limit" bash -c 'ulimit -v 2097152; exec "$@"' "$cutoff_name" "$@" \
    > "$cutoff_evidence/$cutoff_name.jsonl" 2> "$cutoff_evidence/$cutoff_name.stderr"
  local cutoff_exit=$?
  printf '%s\n' "$cutoff_exit" > "$cutoff_evidence/$cutoff_name.exit"
  if (( cutoff_exit != 0 )); then cutoff_status=1; fi
}
record_cutoff identity_replay 30 /workspace/scratch/dickson-cutoff-identity-v1 \
  "$cutoff_evidence/identity_issue.json"
for cutoff_pair in '127 4' '8191 4' '8191 5'; do
  read -r cutoff_prime cutoff_depth <<< "$cutoff_pair"
  record_cutoff "f${cutoff_prime}_d${cutoff_depth}" 120 \
    /workspace/scratch/dickson-full-cutoff-v1 "$cutoff_prime" "$cutoff_depth" /unused partition-audit
done
exit "$cutoff_status"
