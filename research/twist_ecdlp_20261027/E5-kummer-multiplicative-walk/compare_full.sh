#!/bin/bash
# usage: compare_full.sh REPLICA_DIR   -- compares a full-run replica (run.sh with D=REPLICA_DIR REPLICATE=1) with the original raw/ and out/.
# Scientific content only: wall-clock fields (hdr done_ms, per-trial ms, time_*.sec, us/step, meanT) are dropped.  Run only AFTER the replica run has finished.
set -u
O=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk
Rr=$1; fail=0
strip() { grep -v 'done_ms' "$1" | sed -E 's/, [0-9]+, ([0-9]+)\]\);$/, \1]);/'; }
for f in curves identity chains exhaustive; do cmp -s $O/raw/$f.txt $Rr/raw/$f.txt || { echo "DIFF $f"; fail=1; }; done
n=0
for f in $O/raw/cell_*.txt; do b=$(basename $f); n=$((n+1))
  [ -f $Rr/raw/$b ] || { echo "MISSING in replica: $b"; fail=1; continue; }
  diff <(strip $f) <(strip $Rr/raw/$b) > /dev/null || { echo "DIFF $b"; fail=1; }
done
for f in $Rr/raw/cell_*.txt; do [ -f $O/raw/$(basename $f) ] || { echo "EXTRA in replica: $(basename $f)"; fail=1; }; done
cmp -s $O/results.json $Rr/results.json || { echo "DIFF results.json"; fail=1; }
cmp -s $O/out/results.json $Rr/out/results.json || { echo "DIFF out/results.json"; fail=1; }
cmp -s $O/out/costs.csv $Rr/out/costs.csv || { echo "DIFF out/costs.csv"; fail=1; }
cmp -s $O/out/opcounts.csv $Rr/out/opcounts.csv || { echo "DIFF out/opcounts.csv"; fail=1; }
for f in opcounts_pooled.txt opcounts_ratios.txt opcounts_pooled.json; do cmp -s $O/out/$f $Rr/out/$f || { echo "DIFF out/$f"; fail=1; }; done
# summary.csv / summary.txt without timing columns: compare after removing the wall-clock columns named in the header
for f in summary.csv; do
  python3 - "$O/out/$f" "$Rr/out/$f" <<'PY' || fail=1
import csv,sys
def load(p):
    r=list(csv.reader(open(p))); h=r[0]; keep=[i for i,c in enumerate(h) if c not in ("meanT","us_per_step")]
    return [[row[i] for i in keep] for row in r]
a,b=load(sys.argv[1]),load(sys.argv[2])
if a!=b: print("DIFF summary.csv (non-timing columns)"); sys.exit(1)
PY
done
diff <(sort $O/out/MISSING.txt | sed 's/ SKIPPED.*/ SKIPPED/') <(sort $Rr/out/MISSING.txt | sed 's/ SKIPPED.*/ SKIPPED/') > /dev/null || { echo "DIFF MISSING set"; fail=1; }
echo "cells compared: $n"
echo "full replication fail=$fail"
