#!/usr/bin/env bash
# Interleaved panel on one host: new pair table (1 and 4 threads), library default
# Semaev oracle, crypto #5184 all-witness source, equal-engineering rho (#5184).
set -u
R=$(cd "$(dirname "$0")/.." && pwd)
X=${X:-$R/../../target/release/examples/pair_table_rank_rate}
REF=${REF:?cost-v2 binary from research/b677_relation_cost_20261125}
RHO=${RHO:?rho-batched binary from research/b677_relation_cost_20261125}
D=$R/evidence/${PANEL:-panel-01}
for rep in $(seq 1 ${REPS:-10}); do
  python3 $R/tools/run.py $D/r$rep-pt1 $X pairtable 8 $rep 1
  python3 $R/tools/run.py $D/r$rep-ref $REF sample:13:$rep
  python3 $R/tools/run.py $D/r$rep-pt4 $X pairtable 8 $rep 4
  python3 $R/tools/run.py $D/r$rep-default $X default
  python3 $R/tools/run.py $D/r$rep-rho $RHO $rep
done
echo PANEL-DONE
