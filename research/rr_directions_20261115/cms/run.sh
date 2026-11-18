#!/bin/bash
# Re-runnable CMS 5.11.21 runs, one job at a time.
#   CMS_BIN=<path to cms_driver> ./run.sh            (writes inputs.txt, results.jsonl, logs/)
# Gauss-Jordan on : cms_driver --gauss 1  (library default, gaussconf.max_num_matrices=5)
# Gauss-Jordan off: cms_driver --gauss 0  (gaussconf.max_num_matrices=0, upstream --maxnummatrices 0)
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
CMS_BIN=${CMS_BIN:-/tmp/claude-0/-home-user/87aed7c9-04ad-528e-8252-34148b4b4848/scratchpad/cms_src/build/cms_driver}
CHECK=${CHECK:-/tmp/claude-0/-home-user/87aed7c9-04ad-528e-8252-34148b4b4848/scratchpad/cms_src/check_model}
RUNS=/home/user/crypto/research/rr_directions_20261115/runs
DIRS="$RUNS/rust_panel_20261115 $RUNS/rust_panel_20261115_sat"
MAXTIME=${MAXTIME:-600}
OUT=$HERE/results.jsonl; LOGS=$HERE/logs; mkdir -p "$LOGS"
if [ ! -f $HERE/inputs.txt ]; then
  # snapshot of existing files; norm first by m, then incidence by m; identical content listed once
  for f in $(for d in $DIRS; do ls $d/cells/*.cnf; done); do
    b=$(basename $f .cnf); m=${b#m}; m=${m%%_*}; arm=${b##*_}
    echo "$([ $arm = norm ] && echo 0 || echo 1) $(printf %02d $m) $b $f $(sha256sum $f | cut -d' ' -f1)"
  done | sort -k1,1n -k2,2n -k3,3 -s | awk '{ if (!($3 SUBSEP $5 in seen)) { seen[$3,$5]=1; print $4, $3, $5 } else print "# duplicate (same name and sha256), skipped: " $4 }' > $HERE/inputs.txt
fi
done_key() { grep -q "\"file\":\"$1\",.*\"config\":\"$2\"" $OUT 2>/dev/null; }
grep -v '^#' $HERE/inputs.txt | while read f name sha; do
  [[ $name =~ ^m([0-9]+)_l([0-9]+)_d([0-9]+)_(norm|incidence)$ ]] || { echo "bad name $name" >&2; continue; }
  m=${BASH_REMATCH[1]}; ell=${BASH_REMATCH[2]}; d=${BASH_REMATCH[3]}; arm=${BASH_REMATCH[4]}
  run=$(dirname $(dirname $f))
  prow=$(for r in $run/results.jsonl $(for d2 in $DIRS; do echo $d2/results.jsonl; done); do jq -c --arg n $name 'select(.solver=="sat" and .system==$n)|{status:.result.status,conflicts:.result.conflicts,wall_s:.result.wall_s}' $r 2>/dev/null; done | head -1)
  [ -z "$prow" ] && prow=null
  for cfg in gauss nogauss; do
    done_key "$f" $cfg && continue
    g=$([ $cfg = gauss ] && echo 1 || echo 0)
    log=$LOGS/${name}.${cfg}.$(echo $f | sed 's#.*/runs/##; s#/cells/.*##').out
    TIMEFORMAT='%R %U %S'
    t=$( { time nice -n 10 timeout $((MAXTIME+60)) "$CMS_BIN" --verb 1 --maxtime $MAXTIME --gauss $g "$f" > "$log" 2>&1; echo $? > "$log.rc"; } 2>&1 )
    rc=$(cat "$log.rc"); rm -f "$log.rc"
    wall=$(echo $t | cut -d' ' -f1); cpu=$(echo $t | awk '{printf "%.3f", $2+$3}')
    verdict=unknown; grep -q '^s SATISFIABLE' $log && verdict=sat; grep -q '^s UNSATISFIABLE' $log && verdict=unsat
    gv() { grep "^c DRIVER $1:" $log | awk '{print $4}'; }
    conf=$(gv conflicts); dec=$(gv decisions); prop=$(gv propagations)
    timeout_flag=false; { [ $rc -eq 124 ] || grep -q '^s INDETERMINATE' $log; } && timeout_flag=true
    model_ok=null
    if [ $verdict = sat ]; then $CHECK "$f" "$log" >/dev/null 2>&1 && model_ok=true || model_ok=false; fi
    jq -cn --arg file "$f" --arg system "$name" --argjson m $m --argjson ell $ell --argjson draw $d --arg arm $arm --arg config $cfg \
       --arg verdict $verdict --argjson conflicts ${conf:-null} --argjson decisions ${dec:-null} --argjson propagations ${prop:-null} \
       --argjson cpu_s $cpu --argjson wall_s $wall --argjson timeout $timeout_flag --argjson rc $rc --argjson model_ok $model_ok \
       --arg sha256 $sha --argjson panel_sat "$prow" \
       '{file:$file,system:$system,m:$m,ell:$ell,draw:$draw,arm:$arm,config:$config,verdict:$verdict,conflicts:$conflicts,decisions:$decisions,propagations:$propagations,cpu_s:$cpu_s,wall_s:$wall_s,timeout:$timeout,exit_code:$rc,model_ok:$model_ok,cnf_sha256:$sha256,panel_sat:$panel_sat}' >> $OUT
  done
done
