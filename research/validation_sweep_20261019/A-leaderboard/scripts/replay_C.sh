#!/bin/sh
# Table C replay: per size, 3 archived runs (set/round/arm) drawn with the committed seed; each is re-executed
# natively (`ic price` on the run's own params file) and compared on `counts` and `recovered`
# (all 32 scalars) with the archive. No target points are archived for these runs, so no independent
# certificate check is possible (recorded as such).
# usage: replay_C.sh <ic-binary> <outdir>   (repo root)
IC="$1"; OUT="$2"
R=research/ic_descent_20260930/runs-isolated/main
here=research/validation_sweep_20261019/A-leaderboard/scripts
for dir in $R/k*n*; do
  kn=$(basename "$dir")
  curve=$(jq -r .curve $dir/M1/r1-baseline.price.json)
  for id in $(for j in 1 2 3 4; do for i in 1 2 3 4 5; do for arm in baseline candidate; do [ -f "$dir/M$j/r$i-$arm.price.json" ] && echo "M$j/r$i-$arm"; done; done; done | $here/sample.sh "C:$curve" 3); do
    arch="$dir/$id.price.json"; params=$(jq -r .params "$arch")
    safe=$(echo "$kn-$id" | tr '/' '_'); out="$OUT/$safe.json"; rm -f "$out"
    RAYON_NUM_THREADS=1 "$IC" price --params "$params" --json --out "$out" >/dev/null 2>"$OUT/$safe.err"
    jq -n --arg run "$kn/$id" --slurpfile n "$out" --slurpfile o "$arch" '
      {run:$run, all_verified:$n[0].all_verified, recovered_equal:($n[0].recovered==$o[0].recovered),
       archived_counters_equal:([ $o[0].counts | paths(scalars) as $p | (($o[0].counts|getpath($p))==($n[0].counts|getpath($p))) ]|all),
       new_binary_extra_counters:([ $n[0].counts | paths(scalars) | select(. as $p | ($o[0].counts|getpath($p))==null) | join(".") ]|unique)}' -c
  done
done
