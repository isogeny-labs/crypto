#!/bin/sh
# One-command rerun of the smallest cell (l=3, n=11): validation + all six arms + analysis of that cell.
# Writes only under ${OUT:-/tmp/f08_rerun}; about 15 s.
set -e
cd "$(dirname "$0")"; HERE=$(pwd)
OUT=${OUT:-/tmp/f08_rerun}; mkdir -p "$OUT"
(cd "$OUT" && PYTHONPATH="$HERE" python3 "$HERE/validate.py" 11 3 >/dev/null && echo "validation ok")
python3 run.py 11 3 50 10 INC,FRESH-A,FRESH-U,SPEC,FRESH-A2,INC2 "$OUT/raw_l3.jsonl" >/dev/null
python3 - "$OUT/raw_l3.jsonl" raw_l3.jsonl <<'PY'
import json, sys
def rows(f): return [{k: v for k, v in json.loads(L).items() if k != 'secs'} for L in open(f)]
a, b = rows(sys.argv[1]), rows(sys.argv[2])
print('rows', len(a), 'identical to committed raw_l3.jsonl except wall time:', a == b)
PY
(cd "$OUT" && PYTHONPATH="$HERE" python3 "$HERE/analyze.py" "$OUT/raw_l3.jsonl" | sed -n '3,12p')
