#!/bin/bash
# I4 batch: for each "SYS c" line in a cell list, run degree-truncated msolve, summarize (delta_c, pairs),
# and run the Singular reduction check (all pairs if <= MAXPAIRS, else a seeded sample of SAMPLE pairs).
# Appends one JSON line per cell to results.jsonl.   Usage: run_i4.sh CELLS.txt
set -u
CELLS=$(readlink -f "$1")
cd "$(dirname "$0")/../r2/i4"
SYSD=../../systems
TOOL=../../scripts/i4tool.py
CAP=../../scripts/cap.py
BIN=/root/m8build/inst/bin/msolve
MAXPAIRS=${MAXPAIRS:-300000}
SAMPLE=${SAMPLE:-5000}
while read -r S c; do
  [ -z "$S" ] && continue
  case "$S" in \#*) continue;; esac
  tag=$S.c$c
  echo "$(date -u +%FT%TZ) start $tag" >> i4_progress.txt
  MSOLVE_TRUNC_DEG=$c python3 $CAP ${I4AS:-6e9} ${I4WALL:-3600} $tag.log $tag.res.json -- $BIN -t ${I4T:-1} ${I4EXTRA:-} -v 2 -g 2 -f $SYSD/$S.ms -o $tag.out > /dev/null 2>&1 < /dev/null
  stop=$(python3 -c "import json;print(json.load(open('$tag.res.json'))['stop'])")
  if [ "$stop" != "None" ]; then
    echo "{\"cell\":\"$tag\",\"status\":\"msolve_censored\",\"stop\":\"$stop\"}" >> results.jsonl
    echo "$(date -u +%FT%TZ) end $tag msolve_censored $stop" >> i4_progress.txt
    continue
  fi
  python3 $TOOL summary $SYSD/$S.ms $tag.out $c > $tag.summary.json 2> $tag.summary.err
  const=$(python3 -c "import json;print(json.load(open('$tag.summary.json'))['constant_in_basis'])")
  kept=$(python3 -c "import json;print(json.load(open('$tag.summary.json'))['pairs']['kept'])")
  chk=none
  if [ "$const" = "False" ]; then
    if [ "$kept" -le "$MAXPAIRS" ]; then K=0; else K=$SAMPLE; fi
    python3 $TOOL singular $SYSD/$S.ms $tag.out $c $tag.sing $tag.singres.txt $K 20261118 > $tag.singmeta.json
    rm -f $tag.singres.txt
    python3 $CAP 4e9 7200 $tag.sing.log $tag.sing.res.json -- Singular -q $tag.sing > /dev/null 2>&1 < /dev/null
    chk=$(tr '\n' ';' < $tag.singres.txt 2>/dev/null)
    rm -f $tag.sing   # large; regenerable from .out
  fi
  python3 - "$tag" "$chk" <<'EOF' >> results.jsonl
import json, sys
tag, chk = sys.argv[1], sys.argv[2]
s = json.load(open(tag + ".summary.json"))
r = json.load(open(tag + ".res.json"))
s.update(cell=tag, status="ok", msolve_wall_s=r["wall_s"], msolve_rss_kB=r["ru_maxrss_kB"], singular_check=chk)
try:
    s["singular_res"] = json.load(open(tag + ".sing.res.json"))
except Exception:
    pass
print(json.dumps(s))
EOF
  echo "$(date -u +%FT%TZ) end $tag const=$const kept=$kept chk=$chk" >> i4_progress.txt
done < "$CELLS"
echo "$(date -u +%FT%TZ) BATCH DONE $1" >> i4_progress.txt
