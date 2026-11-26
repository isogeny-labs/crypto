#!/usr/bin/env bash
# Fixed local software controls. Run once from repository root.
set -uo pipefail
umask 077
lookup_evidence=research/smooth_p_plus_one_20261123/evidence/base_lookup_v1
for lookup_policy in original lookup; do
  for lookup_suffix in jsonl stderr exit; do
    if [[ -e "$lookup_evidence/$lookup_policy.$lookup_suffix" ]]; then
      printf '%s\n' 'Refusing to overwrite a retained control; use a new evidence version.' >&2
      exit 2
    fi
  done
done
lookup_overall=0
for lookup_policy in original lookup; do
  lookup_args=(account-suite=781815 draws=30)
  if [[ "$lookup_policy" == lookup ]]; then lookup_args+=(base-lookup); fi
  RAYON_NUM_THREADS=1 \
  LD_LIBRARY_PATH=/workspace/scratch/singular/root/usr/lib/x86_64-linux-gnu:/workspace/scratch/msolve/root/usr/lib/x86_64-linux-gnu \
  /workspace/research_stream/research-stream capture /workspace/research_stream/live-native.jsonl -- \
    timeout --kill-after=5s 400 bash -c 'ulimit -v 2097152; exec "$@"' lookup-control \
    /workspace/scratch/dickson-base-lookup-v1 127 4 /workspace/scratch/msolve/root/usr/bin/msolve \
    "${lookup_args[@]}" direct refined cofactor-msolve \
    > "$lookup_evidence/$lookup_policy.jsonl" 2> "$lookup_evidence/$lookup_policy.stderr"
  lookup_exit=$?
  printf '%s\n' "$lookup_exit" > "$lookup_evidence/$lookup_policy.exit"
  if (( lookup_exit != 0 )); then lookup_overall=1; fi
done
exit "$lookup_overall"
