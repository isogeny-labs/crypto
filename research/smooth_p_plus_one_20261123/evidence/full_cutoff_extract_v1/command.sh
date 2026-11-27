#!/usr/bin/env bash
# Frozen paired fixed-query panels; run once from the repository root.
set -uo pipefail
umask 077
extract_evidence=research/smooth_p_plus_one_20261123/evidence/full_cutoff_extract_v1
for extract_name in f127_d4 f8191_d4 f8191_d5; do
  for extract_suffix in jsonl stderr exit; do
    if [[ -e "$extract_evidence/$extract_name.$extract_suffix" ]]; then
      printf '%s\n' 'Refusing to overwrite retained full-cutoff extraction evidence.' >&2
      exit 2
    fi
  done
done
extract_status=0
for extract_pair in '127 4' '8191 4' '8191 5'; do
  read -r extract_prime extract_depth <<< "$extract_pair"
  extract_name="f${extract_prime}_d${extract_depth}"
  RAYON_NUM_THREADS=1 \
  LD_LIBRARY_PATH=/workspace/scratch/singular/root/usr/lib/x86_64-linux-gnu:/workspace/scratch/msolve/root/usr/lib/x86_64-linux-gnu \
    /workspace/research_stream/research-stream capture /workspace/research_stream/live-native.jsonl -- \
    timeout --kill-after=5s 60 bash -c 'ulimit -v 2097152; exec "$@"' "$extract_name" \
      /workspace/scratch/dickson-full-cutoff-extract-v1 "$extract_prime" "$extract_depth" \
      /workspace/scratch/msolve/root/usr/bin/msolve full-cutoff cofactor-msolve \
      > "$extract_evidence/$extract_name.jsonl" 2> "$extract_evidence/$extract_name.stderr"
  extract_exit=$?
  printf '%s\n' "$extract_exit" > "$extract_evidence/$extract_name.exit"
  if (( extract_exit != 0 )); then extract_status=1; fi
done
exit "$extract_status"
