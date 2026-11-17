#!/bin/bash
# Per (m, arm) table from results.jsonl. median of conflicts over decided (sat/unsat) runs.
HERE=$(cd "$(dirname "$0")" && pwd)
jq -rs '
def med: if length==0 then null else sort | (if length%2==1 then .[length/2|floor] else ((.[length/2-1]+.[length/2])/2) end) end;
def dec(c): map(select(.config==c and .verdict!="unknown"));
group_by([.m,.arm]) | map(
  . as $g | ($g|map(select(.config=="gauss"))) as $G | ($g|map(select(.config=="nogauss"))) as $N |
  ($G|map(.panel_sat)|map(select(.!=null and (.status=="sat" or .status=="unsat")))) as $P |
  "m=\($g[0].m) arm=\($g[0].arm) files=\($G|length) | gauss decided=\($G|dec("gauss")|length) med_conf=\($G|dec("gauss")|map(.conflicts)|med) | nogauss decided=\($N|dec("nogauss")|length) med_conf=\($N|dec("nogauss")|map(.conflicts)|med) | panel_sat decided=\($P|length) med_conf=\($P|map(.conflicts)|med)"
) | .[]' $HERE/results.jsonl
echo "--- verdict disagreements vs panel sat status"
jq -r 'select(.verdict!="unknown" and .panel_sat!=null and .panel_sat.status!=.verdict) | "\(.system) \(.config) cms=\(.verdict) panel=\(.panel_sat.status)"' $HERE/results.jsonl
echo "--- undecided / timeouts"
jq -r 'select(.verdict=="unknown") | "\(.system) \(.config) timeout=\(.timeout) rc=\(.exit_code) conflicts=\(.conflicts) wall=\(.wall_s)"' $HERE/results.jsonl
echo "--- model checks failing"
jq -r 'select(.model_ok==false)|.system+" "+.config' $HERE/results.jsonl
echo "--- gauss vs nogauss verdict mismatch"
jq -rs 'group_by(.system)|.[]|select(length==2 and .[0].verdict!=.[1].verdict)|.[0].system' $HERE/results.jsonl
