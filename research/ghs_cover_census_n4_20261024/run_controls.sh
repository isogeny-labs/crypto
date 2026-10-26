#!/bin/sh
# Reproduction controls and R4/P1 (PROTOCOL.md). Usage: sh run_controls.sh   (from the study directory; bin/ccn4 built from cover_census_n4.rs)
O=results/run-20261024; mkdir -p $O
t() { name=$1; shift; s=$(date +%s); ./bin/ccn4 "$@" 4 > $O/$name.jsonl 2> $O/$name.stderr; echo "$name: ccn4 $* 4 -> exit $?, wall_s $(( $(date +%s) - s ))" >> $O/timings.txt; }
t R1-n3-q5 5 3 exhaustive cosets
t R1-n3-q7 7 3 exhaustive cosets
t R1b-n3-q3 3 3 exhaustive cosets
t R4-n4-q3-cosets 3 4 exhaustive cosets
t R4-n4-q3-cocycles 3 4 exhaustive cocycles
[ "${SKIP_N5:-0}" = 1 ] || t R1c-n5-q3 3 5 exhaustive cosets
echo DONE >> $O/timings.txt
