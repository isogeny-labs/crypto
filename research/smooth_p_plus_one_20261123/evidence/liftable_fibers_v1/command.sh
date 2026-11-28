#!/usr/bin/env bash
# Frozen qualification followed by fixed-target paired extraction. Run once.
set -uo pipefail
umask 077
lift_evidence=research/smooth_p_plus_one_20261123/evidence/liftable_fibers_v1
lift_prior=research/smooth_p_plus_one_20261123/evidence/factorwise_v1
for lift_name in audit_f127_d4 audit_f8191_d4 audit_f8191_d5 f127_d4 f8191_d4 f8191_d5; do
  for lift_suffix in jsonl stderr exit; do
    if [[ -e "$lift_evidence/$lift_name.$lift_suffix" ]]; then
      printf '%s\n' 'Refusing to overwrite retained liftable-fiber evidence.' >&2
      exit 2
    fi
  done
done
if [[ -e "$lift_evidence/audit_reconcile.json" ]]; then exit 2; fi
export LD_LIBRARY_PATH=/workspace/scratch/singular/root/usr/lib/x86_64-linux-gnu:/workspace/scratch/msolve/root/usr/lib/x86_64-linux-gnu
lift_status=0
for lift_pair in '127 4' '8191 4' '8191 5'; do
  read -r lift_prime lift_depth <<< "$lift_pair"
  lift_name="audit_f${lift_prime}_d${lift_depth}"
  RAYON_NUM_THREADS=1 \
    /workspace/research_stream/research-stream capture /workspace/research_stream/live-native.jsonl -- \
    timeout --kill-after=5s 120 bash -c 'ulimit -v 2097152; exec "$@"' "$lift_name" \
    /workspace/scratch/dickson-liftable-fibers-v1 "$lift_prime" "$lift_depth" \
    /workspace/scratch/msolve/root/usr/bin/msolve liftable-audit \
    > "$lift_evidence/$lift_name.jsonl" 2> "$lift_evidence/$lift_name.stderr"
  lift_exit=$?
  printf '%s\n' "$lift_exit" > "$lift_evidence/$lift_name.exit"
  if (( lift_exit != 0 )); then lift_status=1; fi
done
if (( lift_status != 0 )); then exit 1; fi
jq -s -f "$lift_evidence/audit_reconcile.jq" \
  "$lift_evidence"/audit_f*.jsonl "$lift_prior"/f*.jsonl > "$lift_evidence/audit_reconcile.json"
lift_exit=$?
if (( lift_exit != 0 )); then exit 1; fi
if ! jq -e 'length == 3 and all(.[]; .records_present and .counts_match and .charts_match and .full_rank)' \
  "$lift_evidence/audit_reconcile.json" > /dev/null; then exit 1; fi
for lift_pair in '127 4' '8191 4' '8191 5'; do
  read -r lift_prime lift_depth <<< "$lift_pair"
  lift_name="f${lift_prime}_d${lift_depth}"
  RAYON_NUM_THREADS=1 \
    /workspace/research_stream/research-stream capture /workspace/research_stream/live-native.jsonl -- \
    timeout --kill-after=5s 60 bash -c 'ulimit -v 2097152; exec "$@"' "$lift_name" \
    /workspace/scratch/dickson-liftable-fibers-v1 "$lift_prime" "$lift_depth" \
    /workspace/scratch/msolve/root/usr/bin/msolve liftable-cutoff cofactor-msolve \
    > "$lift_evidence/$lift_name.jsonl" 2> "$lift_evidence/$lift_name.stderr"
  lift_exit=$?
  printf '%s\n' "$lift_exit" > "$lift_evidence/$lift_name.exit"
  if (( lift_exit != 0 )); then lift_status=1; fi
done
exit "$lift_status"
