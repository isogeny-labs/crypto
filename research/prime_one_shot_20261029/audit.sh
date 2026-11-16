#!/usr/bin/env bash
# Recount frozen records; does not launch a solver or a timing experiment.
set -euo pipefail
root=${1:-/workspace}
out=${2:-/tmp/prime-one-shot-audit}
script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
mkdir -p "$out"
(cd "$root" && sha256sum --check "$script_dir/inputs.sha256" >&2)
sym="$root/crypto/research/pkm_symmetric_tower_20261026"
eb="$root/ecbench/research/summation_dreg_20261023"
for file in canonical_run1.json canonical_run2.json; do
    jq -e 'type == "array" and length == 20 and (unique_by(.kind,.m,.n,.l,.auxV)|length) == 20' "$eb/evidence/$file" >/dev/null
done
jq -S '[.[]]' "$eb/evidence/canonical_run1.json" > "$out/canonical1.json"
jq -S '[.[]]' "$eb/evidence/canonical_run2.json" > "$out/canonical2.json"
cmp "$out/canonical1.json" "$out/canonical2.json"
jq -S 'map(del(.ms))' "$eb/exotic/rank6_lattes_isogeny_factor_base/raw/run1.json" > "$out/lattes1.json"
jq -S 'map(del(.ms))' "$eb/exotic/rank6_lattes_isogeny_factor_base/raw/run2.json" > "$out/lattes2.json"
for file in "$out/lattes1.json" "$out/lattes2.json"; do
    jq -e 'length == 90 and (unique_by(.p,.family,.k,.d,.s,.seed)|length) == 90 and all(.root_check == true and .msolve.status == "ok" and .singular.status == "ok")' "$file" >/dev/null
done
cmp "$out/lattes1.json" "$out/lattes2.json"
jq -s '[.[]|select(.arm=="symmetric")|del(.ms)]|sort_by(.m,.t,.target)' "$sym/rows_m3.jsonl" > "$out/symmetric-original.json"
jq -s '[.[]|select(.arm=="symmetric" and .t<=4)|del(.ms)]|sort_by(.m,.t,.target)' "$sym/rows_m3_symmetric.jsonl" > "$out/symmetric-repeat.json"
for file in "$out/symmetric-original.json" "$out/symmetric-repeat.json"; do
    jq -e 'length == 6 and (unique_by(.m,.t,.target)|length) == 6 and all(.inconsistent == true and .timed_out == false)' "$file" >/dev/null
done
cmp "$out/symmetric-original.json" "$out/symmetric-repeat.json"
jq -s '[.[]|select(.arm=="plain-chain" or .arm=="symmetric")|{m,t,N,arm,target,D,degree_reached,inconsistent,timed_out,max_cols,field_ops}]' "$sym/rows_m3.jsonl" > "$out/symmetric-records.json"
awk -F, 'NR==1 {for(i=1;i<=NF;i++) {if($i in col) exit 1; col[$i]=i} if(!("verdict" in col) || !("exit" in col)) exit 1; width=NF; next} {if(NF!=width || seen[$1]++) exit 1; n++; status=$(col["exit"]); verdict=$(col["verdict"]); if(status!=0 && status!=124 && status!=139 && status!=143) exit 1; if((verdict=="no_solution") != (status==0)) exit 1; if(verdict!="no_solution" && verdict!="none") exit 1; exits[status]++; if(verdict=="no_solution") solved++; else censored++} END {if(n!=24 || solved!=16 || censored!=8) exit 1; printf "{\"rows\":%d,\"completed_no_solution\":%d,\"not_completed\":%d,\"exit_0\":%d,\"exit_124\":%d,\"exit_139\":%d,\"exit_143\":%d}\n",n,solved,censored,exits[0],exits[124],exits[139],exits[143]}' "$root/crypto/research/ic_next10_20261014/03-pkm-degree/results.csv" > "$out/pkm-completion.json"
jq -n --argjson canonical "$(jq length "$out/canonical1.json")" --argjson lattes "$(jq length "$out/lattes1.json")" --argjson symmetric "$(jq length "$out/symmetric-original.json")" --slurpfile pkm "$out/pkm-completion.json" '{canonical_records_identical:$canonical,lattes_records_identical:$lattes,symmetric_records_identical_excluding_ms:$symmetric,pkm_csv_completion:$pkm[0],interpretation:"Stored-record reanalysis only; no fresh solver replay, no admitted timing claim."}' > "$out/summary.json"
cat "$out/summary.json"
