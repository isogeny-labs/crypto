#!/usr/bin/env bash
# External comparison (AGENTS.md §15): the cryptanalysis repo's `ca solve
# --alg rho` (one thread, pinned to CPU 2) on every workload and target of
# the paired session, three seeds each. One JSON line per solve.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
ca=${CA:-/home/user/cryptanalysis/build/ca}
jq -r 'select(.arm=="rho-neg" and .warmup==false and .round==1) | .workload |
  [.workload_id, .curve_spec.p, .curve_spec.a, .curve_spec.b, .curve_spec.r, .curve_spec.gx, .curve_spec.gy, .target[0], .target[1], .planted] | @tsv' \
  "$here/session/records.jsonl" | sort -u > "$here/external_ca.inputs.tsv"
: > "$here/external_ca.jsonl"
while IFS=$'\t' read -r w p a b n gx gy tx ty k; do
  for seed in 1 2 3; do
    out=$(taskset -c 2 "$ca" solve --alg rho --group ec --p "$p" --a "$a" --b "$b" --order "$n" \
      --g "$gx,$gy" --h "$((tx)),$((ty))" --threads 1 --seed "$seed")
    echo "$out" | jq -c --arg w "$w" --argjson k "$k" --argjson seed "$seed" '. + {workload_id:$w, planted:$k, seed:$seed, correct:(.x==$k)}' >> "$here/external_ca.jsonl"
  done
done < "$here/external_ca.inputs.tsv"
