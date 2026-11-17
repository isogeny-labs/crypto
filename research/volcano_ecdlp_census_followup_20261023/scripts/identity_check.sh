#!/bin/sh
# G-A checks. 1) `ecbench plan` on EVERY spec (checks/plan.jsonl). 2) Transported-instance identity on the isogeny-transported
# families D, Ua, Ub, S (census scripts/identity.jq): ecbench's i-th planted_shared target == ca's carried [k_i]G_src (hk_i).
# 3) Tcr, M (and the extra checks on ctl and N) with scripts/identity_tcr_m.py. All lines go to checks/identity.jsonl.
. "$(dirname "$0")/env.sh"; export STUDY CENSUS ECB; cd $STUDY; mkdir -p checks; : > checks/identity.jsonl; : > checks/plan.jsonl
mkdir -p $SP/t/plans
for spec in specs/*.json; do
  n=$(basename $spec .json)
  $ECB plan --spec $spec --json > $SP/t/plans/$n.json 2> $SP/t/plans/$n.err; rc=$?
  jq -nc --arg n $n --argjson rc $rc --slurpfile pl $SP/t/plans/$n.json '{spec:$n,rc:$rc,executions:$pl[0].executions,workloads:($pl[0].workloads|length),spec_id:$pl[0].spec_id}' >> checks/plan.jsonl 2>/dev/null || echo "{\"spec\":\"$n\",\"rc\":$rc,\"executions\":null}" >> checks/plan.jsonl
done
for spec in specs/*-D-L*.json specs/*-Ua-L*.json specs/*-Ub-L*.json specs/*-S-L*.json; do
  n=$(basename $spec .json); V=${n%%-*}; fam=$(echo $n | cut -d- -f2)
  case $fam in D) F=D;; Ua) F=floor-a;; Ub) F=floor-b;; S) F=crater-b;; esac
  jq -c --arg n $n --arg fam $fam --slurpfile tr census/$V/transport/$F.jsonl -f scripts/identity.jq $SP/t/plans/$n.json >> checks/identity.jsonl
done
python3 -I scripts/identity_tcr_m.py >> checks/identity.jsonl
jq -s -c 'group_by(.family)|map({family:.[0].family,specs:length,curves:(map(.curves)|add),targets:(map(.targets)|add),mismatches:(map(.mismatches)|add)})' checks/identity.jsonl
jq -s -c '{specs:length, plan_failures:(map(select(.rc!=0))|length)}' checks/plan.jsonl
