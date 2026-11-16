#!/usr/bin/env bash
# T13 (PROTOCOL.md, frozen before data): 16 one-target chunks, pooled analysis.
set -euo pipefail
cd "$(dirname "$0")/.."
E=${ECB:-/home/user/ctarget/release/ecbench}
sha256sum "$E" > ecbench/binary.sha256
"$E" plan --spec ecbench/spec.json > ecbench/plan.txt 2>&1
python3 scripts/chunked_run.py ecbench/spec.json ecbench/chunks 16 "$E"
C=$(ls -d ecbench/chunks/c[0-9][0-9])
"$E" band --dir $C --ranks --json ecbench/band_pooled.json > ecbench/band_pooled.txt 2>&1
: > ecbench/pooled_compare.jsonl
for pair in "pkm-iso-t5-g4-s1307 pkm-iso-t4-g3-s1193" "pkm-iso-t4-g3-s1193 pkm-iso-t4-g3-s1193-trace" "pkm-iso-t5-g4-s1307 pkm-iso-t4-g3-s1193-trace" "proj-a pkm-iso-t4-g3-s1193" "rho-neg pkm-iso-t4-g3-s1193" "rho-neg pkm-iso-t4-g3-s1193-trace"; do
  set -- $pair; for w in 0 0.1 1; do python3 scripts/pooled_compare.py $1 $2 $w $C >> ecbench/pooled_compare.jsonl; done
done
mkdir -p ecbench/verify
for d in $C; do c=$(basename $d); "$E" verify --dir $d --replay 1 --out ecbench/verify/$c.json > ecbench/verify/$c.txt 2>&1 || true; done
(cd ecbench && sha256sum binary.sha256 spec.json band_pooled.json pooled_compare.jsonl verify/*.json $(ls -d chunks/c[0-9][0-9] | sed 's|$|/records.jsonl|') > SHA256SUMS)
touch ecbench/ALL_DONE
