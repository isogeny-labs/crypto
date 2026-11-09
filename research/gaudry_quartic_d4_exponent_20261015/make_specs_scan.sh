#!/usr/bin/env bash
# Amendment 6, stage 2: specs/SPEC-scan-*.json and jobs_scan.txt. The curves are
# the 3 fewest / 3 most columns at p = 101 and 149 chosen from scan/summary.json
# (frozen rule). One target each, plain arm (ic-d4-w) plus rho-neg. Thin
# orchestration only: every number is ecbench's.
set -euo pipefail
cd "$(dirname "$0")"
ORACLE='gaudry-d4:macaulay=26,weighted=1,chains=1,sparsest=1,replay=1,charge_unsolved=1'
MSEED=202610150
curve() { printf '{"kind":"fp4_torsion","p":%d,"seed":%d,"max_cofactor":16}' "$1" "$2"; }
ic='{"name":"ic-d4-w","role":"candidate","method":{"id":"ic.pipeline","params":{"factor_base":"fghr-y-line:zero=1","oracle":"'"$ORACLE"'","linalg":"wiedemann","max_trials":"400000"}}}'
rho='{"name":"rho-neg","role":"reference","method":{"id":"rho.negation"}}'
: > jobs_scan.txt
spec() { # job p start target_seed
  [ -e "specs/SPEC-$1.json" ] && { echo "refusing to overwrite specs/SPEC-$1.json" >&2; exit 1; }
  cat > "specs/SPEC-$1.json" <<J
{"schema":"ecbench.spec/v1","label":"k=4 IC and rho, screening-scan curve $1","description":"research/gaudry_quartic_d4_exponent_20261015/PROTOCOL.md amendment 6 stage 2, job $1","workloads":{"curves":[$(curve "$2" "$3")],"targets_per_curve":1,"target_seed":$4},"arms":[$rho,$ic],"measurement":{"rounds":1,"warmup":0,"order":"alternate","seed":$MSEED,"isolation_required":"L0","timeout_seconds":172800}}
J
  echo "$1" >> jobs_scan.txt
}
spec scan-p101-fewest1 101 1477 2026110910111
spec scan-p101-fewest2 101 580 2026110910112
spec scan-p101-fewest3 101 1989 2026110910113
spec scan-p101-most1 101 1370 2026110910116
spec scan-p101-most2 101 1171 2026110910117
spec scan-p101-most3 101 1702 2026110910118
spec scan-p149-fewest1 149 1651 2026110914991
spec scan-p149-fewest2 149 1700 2026110914992
spec scan-p149-fewest3 149 1764 2026110914993
spec scan-p149-most1 149 520 2026110914996
spec scan-p149-most2 149 1863 2026110914997
spec scan-p149-most3 149 1815 2026110914998
