#!/bin/bash
# Replay of raw/COMMANDS.log experiment commands, outputs redirected into replay/.
# Run from the L06 directory. Build/test lines are not re-run (see REPLAY.md).
cd /home/user/crypto/research/g2plus_leads_20261116/L06_odd_ell_isogeny_census || exit 1
LOG=replay/COMMANDS.log
run() { # $1 = label, rest = command
  local label="$1"; shift
  local start; start=$(date -u +%FT%TZ)
  echo "[$start] CMD: $*" >> "$LOG"
  bash -c "$*"; local ec=$?
  echo "  exit=$ec" >> "$LOG"
  return $ec
}
: > "$LOG"
run lagr2 "./l06/target/release/l06 lagr2 --out replay/l2_lagrangians.json"
run gsp "./l06/target/release/l06 gsp --samples 200000 --out replay/gsp_reference.json 2> replay/gsp.log"
run ge101 "./l06/target/release/l06 gate-enum --p 101 --count 200 --out replay/gate_enum_p101.jsonl 2> replay/gate_enum_p101.log"
run ge257 "./l06/target/release/l06 gate-enum --p 257 --count 40 --out replay/gate_enum_p257.jsonl 2> replay/gate_enum_p257.log"
run c257 "timeout 900 ./l06/target/release/l06 census --p 257 --u-count 3000 --prime-target 120 --draw-cap 40000 --out replay/census_p257.jsonl 2> replay/census_p257.log"
run c521 "timeout 900 ./l06/target/release/l06 census --p 521 --u-count 3000 --prime-target 120 --draw-cap 40000 --out replay/census_p521.jsonl 2> replay/census_p521.log"
run c1031 "timeout 900 ./l06/target/release/l06 census --p 1031 --u-count 3000 --prime-target 120 --draw-cap 40000 --out replay/census_p1031.jsonl 2> replay/census_p1031.log"
run c2053 "timeout 900 ./l06/target/release/l06 census --p 2053 --u-count 1000 --prime-target 120 --draw-cap 40000 --out replay/census_p2053.jsonl 2> replay/census_p2053.log"
run c4099 "timeout 900 ./l06/target/release/l06 census --p 4099 --u-count 1000 --prime-target 120 --draw-cap 40000 --out replay/census_p4099.jsonl 2> replay/census_p4099.log"
run analyze "python3 -I analyze.py replay > replay/summary.json"
echo "DONE" >> "$LOG"
