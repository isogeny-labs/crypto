#!/usr/bin/env bash
# Interleaved panel: library arms vs the all-witness reference (#5184 cost-v2), same host.
set -u
R=$(cd "$(dirname "$0")/.." && pwd)
X=${X:-$R/../../target/release/examples/semaev_fast_path_rank_rate}
REF=${REF:?path to b677_relation_cost cost-v2 binary}
D=$R/evidence/${PANEL:-panel-01}
for rep in 1 2 3 4 5; do
  python3 $R/tools/run.py $D/r$rep-default $X default 600
  python3 $R/tools/run.py $D/r$rep-ref $REF sample:13:$rep
  if [ $rep -le 2 ]; then
    python3 $R/tools/run.py $D/r$rep-first $X first 600
    python3 $R/tools/run.py $D/r$rep-fast $X fast 600
  fi
done
python3 $R/tools/run.py $D/r1-generic $X generic 1500
echo PANEL-DONE
