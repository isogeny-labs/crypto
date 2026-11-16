#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
sed 's/^JSON //' paired.txt | jq -s '
def median: sort | if length%2==1 then .[length/2|floor] else (.[length/2-1]+.[length/2])/2 end;
group_by([.genus,.p])|map(.[0] as $id|([.[]|select(.arm=="fast")]) as $f|([.[]|select(.arm=="hybrid")]) as $h|([.[]|select(.arm=="oracle")]) as $o|
{g:$id.genus,p:$id.p,n:($f|length),hybrid_solve_ratio:(([$h[].solve_ns]|median)/([$f[].solve_ns]|median)),hybrid_total_ratio:(([$h[].total_ns]|median)/([$f[].total_ns]|median)),oracle_ratio:(([$o[].oracle_ns]|median)/([$f[].oracle_ns]|median)),oracle_total_ratio:(([$o[].total_ns]|median)/([$f[].total_ns]|median))})'
jq -Rn '[inputs|select(startswith("LP "))|split(" ")|.[1:]|map(split("="))|map({key:.[0],value:.[1]})|from_entries|with_entries(if .key=="verified" or .key=="capped" then .value=(.value=="true") else .value|=tonumber end)]' lp.txt > lp.json
jq 'def med:sort|.[length/2|floor]; group_by([.p,.m,.lp])|map({p:.[0].p,m:.[0].m,lp:.[0].lp,n:length,trials_median:([.[].trials]|med),total_ms_median:([.[].total_ms]|med),verified:all(.verified),identity_checks:([.[].identity_checks]|add)})' lp.json > lp_summary.json
