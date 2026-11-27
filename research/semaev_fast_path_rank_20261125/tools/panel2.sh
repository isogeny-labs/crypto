#!/usr/bin/env bash
# Panel-02 (patched decompose_four_fast early exit): default vs first vs reference, interleaved.
set -u
R=$(cd "$(dirname "$0")/.." && pwd)
X=${X:-$R/../../target/release/examples/semaev_fast_path_rank_rate}
REF=${REF:?path to b677_relation_cost cost-v2 binary}
D=$R/evidence/panel-02
for rep in 1 2 3 4 5 6 7 8 9 10; do
  python3 $R/tools/run.py $D/r$rep-default $X default 600
  python3 $R/tools/run.py $D/r$rep-first $X first 600
  python3 $R/tools/run.py $D/r$rep-ref $REF sample:13:$rep
done
python3 $R/tools/run.py $D/r1-fast $X fast 600
echo PANEL-DONE
