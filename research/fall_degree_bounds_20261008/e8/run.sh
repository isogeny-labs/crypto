#!/usr/bin/env bash
# E8 (Amendment 9): d_F against m at fixed (n, l). Repo root; HKY_PAR = the E7 binary.
set -euo pipefail
S=research/fall_degree_bounds_20261008; E=$S/e8
B=${HKY_PAR:?set HKY_PAR}
[ "$(sha256sum "$B" | cut -c1-8)" = 24cf10d6 ] || { echo "wrong binary" >&2; exit 1; }
for cell in 3:8:2 4:8:2 5:8:2 3:6:2 4:6:2 5:6:2 6:6:2 7:6:2 8:6:2 7:8:2; do
  echo "$(date -u +%FT%TZ) start $cell" >>$E/run.log
  "$B" newcells --cells "$cell" --seed 20261012 --keep 4 --cmax 4 >"$E/E8_${cell//:/-}.jsonl" 2>"$E/E8_${cell//:/-}.err" || true
  echo "$(date -u +%FT%TZ) done $cell" >>$E/run.log
done
