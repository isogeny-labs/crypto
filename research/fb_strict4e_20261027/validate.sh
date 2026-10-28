#!/usr/bin/env bash
# Independent recompute of the headline from raw records.jsonl (not from `ecbench compare`).
# Usage: validate.sh  -> prints TSV  n l seed D_w0   (D = log2(sum S_prefix / sum S_strict), measured verified rounds only)
cd "$(dirname "$0")"
for n in 13 19 23; do for l in 3 4; do for s in 1 2 3 4; do
  jq -s -r --arg n $n --arg l $l --arg s $s '
    map(select(.warmup==false)) as $r
    | ([$r[]|select(.arm=="ic-prefix-l"+$l)|.cost.s]|add) as $p
    | ([$r[]|select(.arm=="ic-strict-l"+$l+"-s"+$s)|.cost.s]|add) as $q
    | ([$r[]|select(.arm=="ic-prefix-l"+$l)]|length) as $np
    | ([$r[]|select(.arm=="ic-strict-l"+$l+"-s"+$s)]|length) as $nq
    | [$n,$l,$s,$np,$nq,(($p/$q)|log/(2|log))]|@tsv' sessions/n$n/records.jsonl
done; done; done
