#!/usr/bin/env bash
# Frozen local controls and separate whole-output audits; run from repo root.
set -uo pipefail
umask 077
large_evidence=research/smooth_p_plus_one_20261123/evidence/larger_full_output_v1
large_prior=research/smooth_p_plus_one_20261123/evidence/full_output_v1
for large_run in audit_f127_full audit_f127_lookup f8191_d4 f8191_d5 audit_f8191_d4 audit_f8191_d5; do
  for large_suffix in jsonl stderr exit; do
    if [[ -e "$large_evidence/$large_run.$large_suffix" ]]; then
      printf '%s\n' 'Refusing to overwrite retained evidence; use a new version.' >&2
      exit 2
    fi
  done
done
if [[ -e "$large_evidence/unrun.json" ]]; then exit 2; fi
record_large() {
  local large_name=$1 large_limit=$2
  shift 2
  RAYON_NUM_THREADS=1 \
  LD_LIBRARY_PATH=/workspace/scratch/singular/root/usr/lib/x86_64-linux-gnu:/workspace/scratch/msolve/root/usr/lib/x86_64-linux-gnu \
  /workspace/research_stream/research-stream capture /workspace/research_stream/live-native.jsonl -- \
    timeout --kill-after=5s "$large_limit" bash -c 'ulimit -v 2097152; exec "$@"' "$large_name" "$@" \
    > "$large_evidence/$large_name.jsonl" 2> "$large_evidence/$large_name.stderr"
  local large_exit=$?
  printf '%s\n' "$large_exit" > "$large_evidence/$large_name.exit"
  return "$large_exit"
}
large_history=0
record_large audit_f127_full 60 /workspace/scratch/dickson-replay-v1 127 4 /unused \
  replay-account "$large_prior/full.jsonl" || large_history=1
record_large audit_f127_lookup 60 /workspace/scratch/dickson-replay-v1 127 4 /unused \
  replay-account "$large_prior/full_lookup.jsonl" || large_history=1
if (( large_history != 0 )); then
  printf '%s\n' '{"reason":"historical output audit failed","unrun":["f8191_d4","f8191_d5","audit_f8191_d4","audit_f8191_d5"]}' > "$large_evidence/unrun.json"
  exit 1
fi
large_overall=0
for large_depth in 4 5; do
  record_large "f8191_d$large_depth" 400 /workspace/scratch/dickson-full-output-v1 \
    8191 "$large_depth" /workspace/scratch/msolve/root/usr/bin/msolve \
    account-suite=781815 draws=30 base-lookup full-output direct refined cofactor-msolve || large_overall=1
done
for large_depth in 4 5; do
  record_large "audit_f8191_d$large_depth" 60 /workspace/scratch/dickson-replay-v1 \
    8191 "$large_depth" /unused replay-account "$large_evidence/f8191_d$large_depth.jsonl" || large_overall=1
done
exit "$large_overall"
