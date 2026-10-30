#!/usr/bin/env bash
# opcounts.sh -- derive operation-count metrics from recorded out/results.csv and out/semaev.csv, merge into results.json (idempotent).
set -eu
D=${E3DIR:-/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E3-eigen-decomposition-Fq2}
cd "$D"
E3OUT=out timeout 300 gp -q -f e3_opcounts.gp > out/opcounts.out 2>&1
jq -R -s '
  split("\n") | map(select(length>0)) as $L
  | ($L[0]|split(",")) as $h
  | [ $L[1:][] | split(",") as $f | [range(0;$h|length)] | map({key:$h[.], value:($f[.]|tonumber? // $f[.])}) | from_entries ]' out/opcounts.csv > out/opcounts.json
jq -R -s '
  split("\n") | map(select(length>0)) as $L
  | ($L[0]|split(",")) as $h
  | [ $L[1:][] | split(",") as $f | [range(0;$h|length)] | map({key:$h[.], value:($f[.]|tonumber? // $f[.])}) | from_entries ]' out/semaev.csv > out/semaev_ops.json
jq --slurpfile oc out/opcounts.json --slurpfile sm out/semaev_ops.json \
   '.op_counts={definition:"see e3_opcounts.gp header; units: univariate root-solves, solves*degree, table-arm sign-vector group sums; derived from out/results.csv only",cells:$oc[0],semaev_solver_counts_per_target:$sm[0]}' \
   results.json > results.json.tmp && mv results.json.tmp results.json
