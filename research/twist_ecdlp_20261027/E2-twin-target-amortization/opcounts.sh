#!/bin/bash
# opcounts.sh -- extracts the recorded raw OPERATION COUNTS per (bits,mu) cell (medians over runs) from raw/run_*.out RESULT lines.
# No recomputation; reads only what e2_run.gp already wrote.  Output: opcounts.csv (also printed).
cd "$(dirname "$0")" || exit 1
for f in raw/run_b*.out; do
  h=$(grep '^RESULT_HEADER' $f); r=$(grep '^RESULT,' $f | head -1); echo "$h"; echo "$r"
done | awk -F, '
/^RESULT_HEADER/ { for(i=1;i<=NF;i++) col[$i]=i; next }
{ key=$col["bits"]"|"sprintf("%.2f",$col["mu"]>0?$(col["mu"])+0:0); k=key; n[k]++
  split("N W scanE scanEp KE SQE KEp SQEp cmpE cmpEp cmpJ TE prE_A prE_J TEp prEp_A prEp_J relE relEp", nm, " ")
  for(j=1;j<=19;j++){ v[k,j,n[k]]=$(col[nm[j]])+0 } }
END { print "bits,mu,n_runs,N,W,legendre_scan_E,legendre_scan_Ep,pair_steps_inversions_E,sqrts_E,pair_steps_E_prime,sqrts_Ep,sortcmp_E,sortcmp_Ep,sortcmp_joint,group_adds_targets_E,probes_E_standalone,probes_E_joint,group_adds_targets_Ep,probes_Ep_standalone,probes_Ep_joint,rel_E,rel_Ep"
  for(k in n){ split(k,kk,"|"); line=kk[1]","kk[2]","n[k]
    for(j=1;j<=19;j++){ m=n[k]; for(a=1;a<=m;a++) t[a]=v[k,j,a]
      for(a=1;a<=m;a++)for(b=a+1;b<=m;b++) if(t[b]<t[a]){x=t[a];t[a]=t[b];t[b]=x}
      med=(m%2)?t[(m+1)/2]:(t[m/2]+t[m/2+1])/2; line=line","sprintf("%.10g",med) }
    print line } }' | { read hd; echo "$hd"; sort -t, -k1,1n -k2,2n; } > opcounts.csv
cat opcounts.csv
# rho medians (steps = comparisons) and merge both into results.json (keys opcounts, rho_opcounts); idempotent.
for f in raw/rho_b*.out; do grep '^RHO' $f | sed -E 's/.*bits=([0-9]+),curve=([0-9]+).*steps=([0-9]+),cmp=[0-9]+,init=([0-9]+),setup=([0-9]+).*/\1 \2 \3 \4 \5/'; done | awk '{k=$1; n[k]++; s[k,n[k]]=$3; init[k]=$4; set[k]=$5} END{for(k in n){m=n[k]; for(a=1;a<=m;a++)t[a]=s[k,a]; for(a=1;a<=m;a++)for(b=a+1;b<=m;b++)if(t[b]<t[a]){x=t[a];t[a]=t[b];t[b]=x}; med=(m%2)?t[(m+1)/2]:(t[m/2]+t[m/2+1])/2; print k, m, med, init[k], set[k]}}' | sort -n > raw_rho_opcounts.txt
python3 - <<'PY'
import csv, json
d = json.load(open('results.json'))
d['opcounts'] = [{k: float(v) for k, v in r.items()} for r in csv.DictReader(open('opcounts.csv'))]
d['rho_opcounts'] = [dict(zip(['bits','walks','median_steps_eq_comparisons','init_gae_per_attempt','setup_gae'], map(float, l.split()))) for l in open('raw_rho_opcounts.txt')]
json.dump(d, open('results.json', 'w'), indent=1)
PY
