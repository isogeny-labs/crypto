#!/bin/sh
# replication_check.sh -- compares this directory (original run, 2026-10-21) with a second complete run of run.sh
# executed 2026-10-26 in ../E1-replication (run_rep.sh = run.sh with D changed and the plot line removed).
T=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp
O=$T/E1-mixed-solution-census; R=$T/E1-replication
cut -f1-47,50 $O/summary.tsv > /tmp/e1_o.s; cut -f1-47,50 $R/summary.tsv > /tmp/e1_r.s
cmp /tmp/e1_o.s /tmp/e1_r.s && echo "summary.tsv identical except timing columns 48-49 (tab_s, loop_s)"
for f in pooled.tsv verdict.txt results.json; do cmp $O/$f $R/$f && echo "$f identical"; done
diff $(echo) <(sed 's/\t[0-9.]*$//' $O/controls_summary.tsv) <(sed 's/\t[0-9.]*$//' $R/controls_summary.tsv) >/dev/null && echo "controls_summary.tsv identical except trailing baseline column"
for s in stage0_oracle stage1_controls; do cmp $O/raw/$s.out $R/raw/$s.out && echo "raw/$s.out identical"; done
n=0; d=0; for f in $O/raw/grid_*.out; do b=$(basename $f); n=$((n+1)); diff <(grep '^TGT' $f) <(grep '^TGT' $R/raw/$b) >/dev/null || d=$((d+1)); done
echo "grid .out files compared (TGT per-target lines): $n ; files differing: $d"
