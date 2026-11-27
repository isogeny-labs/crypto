#!/usr/bin/env bash
# Frozen offline audit of two retained records. No solver calls.
set -uo pipefail
umask 077
rur_evidence=research/smooth_p_plus_one_20261123/evidence/rur_record_audit_v1
for rur_suffix in jsonl stderr exit; do
  if [[ -e "$rur_evidence/audit.$rur_suffix" ]]; then
    printf '%s\n' 'Refusing to overwrite retained RUR diagnostic evidence.' >&2
    exit 2
  fi
done
RAYON_NUM_THREADS=1 \
  /workspace/research_stream/research-stream capture /workspace/research_stream/live-native.jsonl -- \
  timeout --kill-after=5s 30 bash -c 'ulimit -v 1048576; exec "$@"' rur-record-audit \
  /workspace/scratch/rr5-rur-record-audit-v1 \
  research/smooth_p_plus_one_20261123/evidence/factorwise_extract_v1/f127_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/liftable_fibers_v1/f127_d4.jsonl \
  > "$rur_evidence/audit.jsonl" 2> "$rur_evidence/audit.stderr"
rur_exit=$?
printf '%s\n' "$rur_exit" > "$rur_evidence/audit.exit"
exit "$rur_exit"
