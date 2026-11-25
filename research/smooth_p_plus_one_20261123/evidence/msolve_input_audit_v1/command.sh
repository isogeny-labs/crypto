#!/usr/bin/env bash
# Frozen offline parser diagnostic. No msolve or root-search invocation.
set -uo pipefail
umask 077
parser_evidence=research/smooth_p_plus_one_20261123/evidence/msolve_input_audit_v1
for parser_suffix in jsonl stderr exit; do
  if [[ -e "$parser_evidence/audit.$parser_suffix" ]]; then
    printf '%s\n' 'Refusing to overwrite retained parser diagnostic evidence.' >&2
    exit 2
  fi
done
RAYON_NUM_THREADS=1 \
  /workspace/research_stream/research-stream capture /workspace/research_stream/live-native.jsonl -- \
  timeout --kill-after=5s 30 bash -c 'ulimit -v 1048576; exec "$@"' msolve-input-audit \
  /workspace/scratch/msolve-input-audit-v1 /workspace/scratch/msolve-parser-probe-v1 \
  research/smooth_p_plus_one_20261123/evidence/factorwise_extract_v1/f127_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/liftable_fibers_v1/f127_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/rur_record_audit_v1/audit.jsonl \
  > "$parser_evidence/audit.jsonl" 2> "$parser_evidence/audit.stderr"
parser_exit=$?
printf '%s\n' "$parser_exit" > "$parser_evidence/audit.exit"
exit "$parser_exit"
