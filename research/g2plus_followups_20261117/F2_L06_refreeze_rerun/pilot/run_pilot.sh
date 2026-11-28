#!/bin/bash
# F2 pilot (tag "pilot" = 0x4C36...; never the decision tag). Run from the F2 directory.
set -u
B=l06r/target/release/l06r
run() { echo "+ $*" >> pilot/pilot_commands.log; local t0=$(date +%s); "$@"; echo "exit=$? seconds=$(( $(date +%s) - t0 ))" >> pilot/pilot_commands.log; }
case "$1" in
  ge_auto) run $B gate-enum --tag pilot --p 257 --count 200 --force auto --out pilot/pilot_ge_p257_auto.jsonl 2> pilot/pilot_ge_p257_auto.log ;;
  ge_B)    run $B gate-enum --tag pilot --p 257 --count 40 --force B --out pilot/pilot_ge_p257_B.jsonl 2> pilot/pilot_ge_p257_B.log ;;
  census)
    for pu in 257:3000:120 521:3000:120 1031:3000:120 2053:1000:120 4099:1000:120; do
      IFS=: read p u t <<< "$pu"
      run timeout 900 $B census --tag pilot --p $p --u-count $u --prime-target $t --draw-cap 40000 --out pilot/pilot_census_p$p.jsonl 2> pilot/pilot_census_p$p.log
    done ;;
esac
