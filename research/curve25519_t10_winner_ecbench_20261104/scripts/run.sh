#!/usr/bin/env bash
# T12 (PROTOCOL.md, frozen before data). One heavy process; counted units only.
set -euo pipefail
cd "$(dirname "$0")/.."
E=${ECB:-/home/user/ctarget/release/ecbench}
sha256sum "$E" > ecbench/binary.sha256
"$E" plan --spec ecbench/spec.json > ecbench/plan.txt 2>&1
uptime > ecbench/load_start.txt
"$E" run --wait --allow-busy --spec ecbench/spec.json --out ecbench/s1 > ecbench/run.log 2>&1
uptime > ecbench/load_end.txt
"$E" band --dir ecbench/s1 --ranks --json ecbench/band.json > ecbench/band.txt 2>&1
for b in pkm-iso-t6-g4 pkm-iso-t6-g4-s1308; do for w in 0 0.1 1; do
  "$E" compare --dir ecbench/s1 --a "$b" --b pkm-iso-t5-g4-s1307 --weight $w > "ecbench/compare_s1307_over_${b}_w${w}.txt" 2>&1
done; done
"$E" verify --dir ecbench/s1 --replay 16 --out ecbench/verify.json > ecbench/verify.txt 2>&1
echo DONE >> ecbench/run.log
