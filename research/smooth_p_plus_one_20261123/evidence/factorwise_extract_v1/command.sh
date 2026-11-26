#!/usr/bin/env bash
# Frozen paired fixed-output controls. Run once from the repository root.
set -uo pipefail
umask 077
factor_evidence=research/smooth_p_plus_one_20261123/evidence/factorwise_extract_v1
for factor_name in f127_d4 f8191_d4 f8191_d5; do
  for factor_suffix in jsonl stderr exit; do
    if [[ -e "$factor_evidence/$factor_name.$factor_suffix" ]]; then
      printf '%s\n' 'Refusing to overwrite retained factorwise extraction evidence.' >&2
      exit 2
    fi
  done
done
factor_status=0
export LD_LIBRARY_PATH=/workspace/scratch/singular/root/usr/lib/x86_64-linux-gnu:/workspace/scratch/msolve/root/usr/lib/x86_64-linux-gnu
for factor_pair in '127 4' '8191 4' '8191 5'; do
  read -r factor_prime factor_depth <<< "$factor_pair"
  factor_name="f${factor_prime}_d${factor_depth}"
  RAYON_NUM_THREADS=1 \
    /workspace/research_stream/research-stream capture /workspace/research_stream/live-native.jsonl -- \
    timeout --kill-after=5s 60 bash -c 'ulimit -v 2097152; exec "$@"' "$factor_name" \
    /workspace/scratch/dickson-factorwise-extract-v1 "$factor_prime" "$factor_depth" \
    /workspace/scratch/msolve/root/usr/bin/msolve factorwise-cutoff cofactor-msolve \
    > "$factor_evidence/$factor_name.jsonl" 2> "$factor_evidence/$factor_name.stderr"
  factor_exit=$?
  printf '%s\n' "$factor_exit" > "$factor_evidence/$factor_name.exit"
  if (( factor_exit != 0 )); then factor_status=1; fi
done
exit "$factor_status"
