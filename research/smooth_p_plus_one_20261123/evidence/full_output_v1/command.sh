#!/usr/bin/env bash
# Four frozen local policy suites; run once from repository root.
set -uo pipefail
umask 077
full_evidence=research/smooth_p_plus_one_20261123/evidence/full_output_v1
for full_policy in original lookup full full_lookup; do
  for full_suffix in jsonl stderr exit; do
    if [[ -e "$full_evidence/$full_policy.$full_suffix" ]]; then
      printf '%s\n' 'Refusing to overwrite retained evidence; use a new version.' >&2
      exit 2
    fi
  done
done
full_overall=0
for full_policy in original lookup full full_lookup; do
  full_args=(account-suite=781815 draws=30)
  case "$full_policy" in lookup|full_lookup) full_args+=(base-lookup);; esac
  case "$full_policy" in full|full_lookup) full_args+=(full-output);; esac
  RAYON_NUM_THREADS=1 \
  LD_LIBRARY_PATH=/workspace/scratch/singular/root/usr/lib/x86_64-linux-gnu:/workspace/scratch/msolve/root/usr/lib/x86_64-linux-gnu \
  /workspace/research_stream/research-stream capture /workspace/research_stream/live-native.jsonl -- \
    timeout --kill-after=5s 400 bash -c 'ulimit -v 2097152; exec "$@"' full-output-control \
    /workspace/scratch/dickson-full-output-v1 127 4 /workspace/scratch/msolve/root/usr/bin/msolve \
    "${full_args[@]}" direct refined cofactor-msolve \
    > "$full_evidence/$full_policy.jsonl" 2> "$full_evidence/$full_policy.stderr"
  full_exit=$?
  printf '%s\n' "$full_exit" > "$full_evidence/$full_policy.exit"
  if (( full_exit != 0 )); then full_overall=1; fi
done
exit "$full_overall"
