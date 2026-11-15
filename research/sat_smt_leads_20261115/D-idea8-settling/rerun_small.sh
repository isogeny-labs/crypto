#!/bin/sh
# One-command rerun of a small cell: l=4 (n=13), seed 0, all four arms, 1,000 targets; about 2 minutes.
# Writes only under $OUT; compares counters with the committed rows.
set -e
cd "$(dirname "$0")"
OUT=${OUT:-/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/D/rerun}; mkdir -p "$OUT"
nice python3 run.py 4 0 ALL "$OUT/raw_l4_s0_ALL.jsonl"
python3 - "$OUT/raw_l4_s0_ALL.jsonl" raw_l4_s0_ALL.jsonl <<'PY'
import json, sys
def rows(f):
    return [{k: v for k, v in json.loads(L).items() if not k.startswith('secs') and k != 'wall'} for L in open(f)]
a, b = rows(sys.argv[1]), rows(sys.argv[2])
print('rows', len(a), 'identical to committed raw_l4_s0_ALL.jsonl except wall time:', a == b)
PY
