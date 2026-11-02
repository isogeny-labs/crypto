#!/usr/bin/env bash
# Thin orchestration for the P1 measurement (see PROTOCOL.md).  Run by the executor, unchanged.
# Usage: run_full.sh tier1|tier2|analyze   (OUT=runs by default; BIN overrides the binary path)
set -euo pipefail
cd "$(dirname "$0")"
OUT="${OUT:-runs}"; mkdir -p "$OUT"
BIN="${BIN:-rust/target/release/p1_eccp131}"
[ -x "$BIN" ] || (cd rust && cargo build --release --locked --offline 2>/dev/null || cargo build --release)
case "${1:-}" in
  tier1)  # m=2 t=3 and m=3 t=2: 58 arms (E0, 2 A/A, iso, 27 neighbours, 27 random curves)
    "$BIN" run --targets targets.json --tower tower.json --configs m2t3,m3t2 \
      --ntargets 32 --nplanted 8 --reps 3 --rnd 27 --workers 4 \
      --dcap 12 --max-cols 60000 --budget-s 120 --seed 20261102 \
      --kinds null,planted,ladder --out "$OUT/tier1_rows.jsonl" 2> "$OUT/tier1.log" ;;
  tier2)  # m=2 t=4: 39 arms, fewer targets, one rep (about 6 CPU-hours)
    "$BIN" run --targets targets.json --tower tower.json --configs m2t4 \
      --ntargets 8 --nplanted 2 --reps 1 --rnd 8 --workers 4 \
      --dcap 14 --max-cols 400000 --budget-s 900 --seed 20261102 \
      --kinds null,planted --out "$OUT/tier2_rows.jsonl" 2> "$OUT/tier2.log" ;;
  analyze)
    for t in tier1 tier2; do
      [ -f "$OUT/${t}_rows.jsonl" ] && "$BIN" analyze --rows "$OUT/${t}_rows.jsonl" --out "$OUT/${t}_analysis.json" \
        --boot 40000 --level 0.9995 --mask > "$OUT/${t}_analysis.txt"
    done ;;
  *) echo "usage: $0 tier1|tier2|analyze" >&2; exit 2 ;;
esac
