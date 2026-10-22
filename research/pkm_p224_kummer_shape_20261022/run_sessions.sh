#!/bin/bash
# The ecbench sessions of PROTOCOL.md 4: one per size, audited with every run
# replayed, then the saved comparisons, the pricing band, the one-unit table and
# the resource vectors. ECBENCH=path/to/ecbench research/pkm_p224_kummer_shape_20261022/run_sessions.sh [bits ...]
set -u
here=$(cd "$(dirname "$0")" && pwd)
bin=${ECBENCH:?set ECBENCH}
sizes=${*:-"20 24 26 28"}
mkdir -p "$here/sessions" "$here/audits" "$here/tables"
for b in $sizes; do
  out=$here/sessions/s$b
  echo "$(date -u +%FT%TZ) s$b run" >> "$here/progress.txt"
  "$bin" run --wait --spec "$here/specs/SPEC-s$b.json" --out "$out" --cpus none --allow-busy > "$here/sessions/s$b.run.log" 2>&1
  echo "$(date -u +%FT%TZ) s$b run exit $?" >> "$here/progress.txt"
  "$bin" verify --dir "$out" --replay-all --out "$here/audits/s$b.audit.json" > "$here/audits/s$b.verify.log" 2>&1
  echo "$(date -u +%FT%TZ) s$b audit exit $?" >> "$here/progress.txt"
  for pair in "rho-neg ic-rootfind" "rho-neg ic-mitm" "ic-mitm ic-rootfind" "rho-neg rho-neg-aa"; do
    set -- $pair
    "$bin" compare --dir "$out" --a "$1" --b "$2" --save > "$here/tables/s$b.compare.$1.vs.$2.txt" 2>&1 || true
  done
  "$bin" table --dir "$out" > "$here/tables/s$b.table.txt" 2>&1
  "$bin" band --dir "$out" --ranks --json "$here/tables/s$b.band.json" > "$here/tables/s$b.band.txt" 2>&1
  "$bin" resources --dir "$out" --out "$here/tables/s$b.resources.json" > /dev/null 2>&1
  echo "$(date -u +%FT%TZ) s$b tables done" >> "$here/progress.txt"
done
echo "$(date -u +%FT%TZ) ALLDONE" >> "$here/progress.txt"
