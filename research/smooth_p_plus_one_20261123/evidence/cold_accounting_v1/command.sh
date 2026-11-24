#!/usr/bin/env bash
# Frozen local correctness/accounting control. Run once from repository root.
set -uo pipefail
umask 077
cold_evidence=research/smooth_p_plus_one_20261123/evidence/cold_accounting_v1
for cold_suffix in jsonl stderr exit; do
  if [[ -e "$cold_evidence/control.$cold_suffix" ]]; then
    printf '%s\n' 'Refusing to overwrite the frozen control; use a new evidence version.' >&2
    exit 2
  fi
done
RAYON_NUM_THREADS=1 \
LD_LIBRARY_PATH=/workspace/scratch/singular/root/usr/lib/x86_64-linux-gnu:/workspace/scratch/msolve/root/usr/lib/x86_64-linux-gnu \
/workspace/research_stream/research-stream capture /workspace/research_stream/live-native.jsonl -- \
 timeout --kill-after=5s 400 bash -c 'ulimit -v 2097152; exec "$@"' cold-control \
 /workspace/scratch/dickson-cold-accounting-v1 127 4 /workspace/scratch/msolve/root/usr/bin/msolve \
 account-suite=781815 draws=30 direct refined cofactor-msolve \
 > "$cold_evidence/control.jsonl" 2> "$cold_evidence/control.stderr"
cold_exit=$?
printf '%s\n' "$cold_exit" > "$cold_evidence/control.exit"
exit "$cold_exit"
