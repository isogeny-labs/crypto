#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
sed 's/^JSON //' paired.txt | jq -s '
def median: sort | if length%2==1 then .[length/2|floor] else (.[length/2-1]+.[length/2])/2 end;
group_by([.genus,.p])|map(.[0] as $id|([.[]|select(.arm=="fast")]) as $f|([.[]|select(.arm=="hybrid")]) as $h|([.[]|select(.arm=="oracle")]) as $o|
{g:$id.genus,p:$id.p,n:($f|length),hybrid_solve_ratio:(([$h[].solve_ns]|median)/([$f[].solve_ns]|median)),hybrid_total_ratio:(([$h[].total_ns]|median)/([$f[].total_ns]|median)),oracle_ratio:(([$o[].oracle_ns]|median)/([$f[].oracle_ns]|median)),oracle_total_ratio:(([$o[].total_ns]|median)/([$f[].total_ns]|median))})'
