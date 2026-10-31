#!/bin/bash
# reduced_compare.sh -- compare ./reduced (stages 0,1 and 12-bit grid) with the original run, ignoring timings (END/JOBEND lines) and the guard column (CFG field 24)
O=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E1-mixed-solution-census
R=${R:-${E1_RED_DIR:-$O/reduced}}; d=0
norm(){ grep -v -E '^(END|JOBEND)' "$1" | awk -F'\t' 'BEGIN{OFS="\t"} $1=="CFG"{$24="-"} {print}'; }
cmp $O/raw/stage0_oracle.out $R/raw/stage0_oracle.out && echo "stage0_oracle.out identical" || d=1
cmp $O/raw/stage1_controls.out $R/raw/stage1_controls.out && echo "stage1_controls.out identical" || d=1
n=0; bad=0; trunc=0
for f in $R/raw/grid_12_*.out; do b=$(basename $f); n=$((n+1))
  diff <(norm $f) <(norm $O/raw/$b) >/dev/null || { bad=$((bad+1)); echo "DIFF $b"; }
  trunc=$((trunc+$(grep '^END' $f | awk -F'\t' '$7==1' | wc -l))); done
echo "grid files (12-bit) compared: $n ; differing in CFG/TGT lines: $bad ; truncated configs in reduced run: $trunc"
[ $bad = 0 ] && [ $trunc = 0 ] || d=1
[ $d = 0 ] && echo "REDUCED_REPLICATION_PASS" || echo "REDUCED_REPLICATION_FAIL"
exit $d
