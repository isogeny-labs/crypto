#!/usr/bin/env bash
# End-to-end check of the native checkpoints on a real committed draw:
#   ckpt-validate.sh DREG_CLOSURE_BIN OUT_DIR [CELL DRAW]
# Four runs of the same closure (default m4-12-3, draw 1, degree 4):
#   plain      no checkpoint
#   ckpt       checkpoint every INTERVAL s, never interrupted (the overhead)
#   term       SIGTERM at KILL_AT s (saves, exits 143), then resumed to the end
#   kill       SIGKILL at KILL_AT s (last committed generation), then resumed
# and compares every field of the result except `secs`.  Writes OUT_DIR/summary.json.
set -uo pipefail
BIN=$1; OUT=$2; CELL=${3:-m4-12-3}; DRAW=${4:-1}
INTERVAL=${INTERVAL:-5}; KILL_AT=${KILL_AT:-25}
HERE=$(cd "$(dirname "$0")" && pwd); REPO=$(cd "$HERE/../.." && pwd)
FILE=research/dreg_closure_scale_20261016/draws/$CELL.jsonl
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000
mkdir -p "$OUT"; cd "$REPO"
now() { date +%s.%N; }
run() { # tag [ckpt dir] -> OUT/tag.out, tag.err, tag.wall
  local tag=$1 dir=${2:-} s; s=$(now)
  if [ -n "$dir" ]; then DREG_CKPT_SECONDS=$INTERVAL "$BIN" --to 4 --max-cols 1100000 --draw "$DRAW" --checkpoint-dir "$dir" "$FILE" >"$OUT/$tag.out" 2>"$OUT/$tag.err"
  else "$BIN" --to 4 --max-cols 1100000 --draw "$DRAW" "$FILE" >"$OUT/$tag.out" 2>"$OUT/$tag.err"; fi
  echo "$? $(echo "$(now) - $s" | bc)" > "$OUT/$tag.wall"
}
interrupted() { # tag signal dir: start, signal after KILL_AT s, wait
  local tag=$1 sig=$2 dir=$3 s; s=$(now)
  DREG_CKPT_SECONDS=$INTERVAL "$BIN" --to 4 --max-cols 1100000 --draw "$DRAW" --checkpoint-dir "$dir" "$FILE" >"$OUT/$tag.out" 2>"$OUT/$tag.err" &
  local p=$!; sleep "$KILL_AT"; kill -"$sig" $p 2>/dev/null; wait $p 2>/dev/null; echo "$? $(echo "$(now) - $s" | bc)" > "$OUT/$tag.wall"
}
W=$(mktemp -d); trap 'rm -rf "$W"' EXIT
run plain
run ckpt "$W/ckpt"
interrupted term1 TERM "$W/term"; run term2 "$W/term"
interrupted kill1 KILL "$W/kill"; run kill2 "$W/kill"
python3 - "$OUT" <<'PY'
import json,sys,re,os
out=sys.argv[1]
def load(t):
    txt=open(f"{out}/{t}.out").read().strip()
    return json.loads(txt) if txt else None
def wall(t): return [float(x) for x in open(f"{out}/{t}.wall").read().split()]
keys=['degree','xl_refuted','closure_refuted','saturated','xl_rank','closure_rank','columns','closure_products','closure_word_ops']
plain=load('plain'); res={'cell_draw':[sys.argv[1]]}
def eq(r): 
    return r is not None and all(all(a[k]==b[k] for k in keys) for a,b in zip(plain['scan'],r['scan'])) and len(plain['scan'])==len(r['scan'])
def saves(t):
    e=open(f"{out}/{t}.err").read(); return [(int(m.group(1)),float(m.group(2)),float(m.group(3))) for m in re.finditer(r"generation (\d+) committed, ([0-9.]+) GB in ([0-9.]+)s",e)]
summary={'fields_compared':keys,'plain_wall':wall('plain')[1],'plain_secs':[d['secs'] for d in plain['scan']]}
for t,final in [('ckpt','ckpt'),('term','term2'),('kill','kill2')]:
    first=t if t=='ckpt' else t+'1'
    r=load(final)
    s=saves(first)+(saves(final) if final!=first else [])
    summary[t]={'identical_to_plain':eq(r),'first_exit':wall(first)[0],'first_wall':wall(first)[1],
      'final_exit':wall(final)[0],'final_wall':wall(final)[1],'segments':r.get('checkpoint_segments') if r else None,
      'saves':len(s),'save_gb_total':round(sum(x[1] for x in s),3),'save_secs_total':round(sum(x[2] for x in s),2),
      'summed_secs':[d['secs'] for d in r['scan']] if r else None}
json.dump(summary,open(f"{out}/summary.json","w"),indent=1)
print(json.dumps(summary,indent=1))
PY
