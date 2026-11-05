#!/bin/bash
# Smoke test for the item 04 repair pass (Deviation 10): one invocation of the
# instrument on every cell at the smallest draw count, for every arm and
# control that a revision card uses, plus the regression for the failure that
# killed card 02 (cell (15,3,4), draw 1) and a check that every draw that
# passed in the first run still returns the same basis.
# usage: smoke_r2.sh SCRATCH_DIR   (run from the repository root)
set -u
SC=${1:?scratch dir}
mkdir -p "$SC"
rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -o "$SC/b04" examples/blindspots_04_presentation_search.rs 2>/dev/null || { echo "BUILD FAILED"; exit 1; }
B="$SC/b04"
RAW=research/blindspots_20261018/04-presentation-search/raw
fail=0
chk() { if [ "$2" = 0 ]; then echo "PASS  $1"; else echo "FAIL  $1 (exit $2)"; fail=1; fi; }
basis() { sed -n 's/.*basis=\(\[[^]]*\]\).*/\1/p' "$1" | head -1; }
cells="17,4,3 19,4,3 13,3,4 15,3,4 17,4,4"
echo "== 1. smallest draw count, arm a, draws 0 and 1, every cell (a 1-target run; --dmax-u 12 on the big cell)"
for c in $cells; do IFS=, read n l m <<<"$c"
  for d in 0 1; do
    "$B" cell --n $n --l $l --m $m --draw $d --arms a --random 1 --planted 0 --dmax-u 12 --time-limit 20 --job-time 15 >"$SC/a_${n}_${l}_${m}_$d.jsonl" 2>"$SC/a_${n}_${l}_${m}_$d.log"
    chk "cell ($n,$l,$m) draw $d arm a: $(basis $SC/a_${n}_${l}_${m}_$d.log)" $?
  done
done
echo "== 2. draw-0 bases and A/A draw-1 bases of the first run are unchanged"
for pair in "17_4_3 $RAW/card-01-cheap-cell1.log" "19_4_3 $RAW/card-01-cheap-cell2.log" "13_3_4 $RAW/card-01-cheap-cell3.log" "15_3_4 $RAW/card-01-cheap-cell4.log" "17_4_4 $RAW/card-01-cheap-cell5-d.log"; do set -- $pair
  [ "$(basis $SC/a_${1}_0.log)" = "$(basis $2)" ]; chk "draw 0 basis of cell $1 equals the first run ($2)" $?
done
for pair in "17_4_3 $RAW/card-02-aa-cell1.log" "19_4_3 $RAW/card-02-aa-cell2.log" "13_3_4 $RAW/card-02-aa-cell3.log"; do set -- $pair
  [ "$(basis $SC/a_${1}_1.log)" = "$(basis $2)" ]; chk "draw 1 basis of cell $1 equals the first run ($2)" $?
done
echo "== 3. regression: the panic of card 02 command aa-cell4 (cell (15,3,4), draw 1) now returns a basis"
"$B" cell --n 15 --l 3 --m 4 --draw 1 --arms a --random 8 --planted 0 --time-limit 20 --job-time 15 >"$SC/aa4.jsonl" 2>"$SC/aa4.log"; rc=$?
chk "aa-cell4 exit $rc, $(wc -l <"$SC/aa4.jsonl") rows, $(basis $SC/aa4.log)" $rc
echo "== 4. controls: linear and nonlinear recodings of arm a on cells 1 to 4 (never reached in the first run)"
for c in 17,4,3 19,4,3 13,3,4 15,3,4; do IFS=, read n l m <<<"$c"
  for rc_ in linear nonlinear; do
    "$B" cell --n $n --l $l --m $m --arms a --recode $rc_ --random 4 --planted 2 --time-limit 20 --job-time 15 >"$SC/rc_${n}_$rc_.jsonl" 2>"$SC/rc_${n}_$rc_.log"
    chk "recode $rc_ cell ($n,$l,$m): $(wc -l <"$SC/rc_${n}_$rc_.jsonl") rows" $?
  done
done
echo "== 5. arm e, smallest count: cell 3 (--first 4), cell 4 (--first 0), cell 5 (10 s cap)"
"$B" cell --n 13 --l 3 --m 4 --arms e --first 4 --random 1 --planted 0 --dmax-e 5 --time-limit 60 --job-time 40 >"$SC/e3.jsonl" 2>"$SC/e3.log"; chk "e cell 3 t4" $?
"$B" cell --n 15 --l 3 --m 4 --arms e --first 0 --random 1 --planted 0 --dmax-e 5 --time-limit 90 --job-time 80 >"$SC/e4.jsonl" 2>"$SC/e4.log"; chk "e cell 4 t0" $?
"$B" cell --n 17 --l 4 --m 4 --arms e --random 1 --planted 0 --dmax-e 5 --time-limit 12 --job-time 10 >"$SC/e5.jsonl" 2>"$SC/e5.log"; chk "e cell 5 t0 (10 s cap)" $?
echo "== 6. arm b, smallest count: cell 1 (random and planted), cell 4 (random t0 20 s cap, planted)"
"$B" cell --n 17 --l 4 --m 3 --arms b --dmax-b 3 --random 1 --planted 0 --time-limit 90 --job-time 80 >"$SC/b1.jsonl" 2>"$SC/b1.log"; chk "b cell 1 random t0" $?
"$B" cell --n 17 --l 4 --m 3 --arms b --dmax-b 3 --random 0 --planted 1 --time-limit 90 --job-time 80 >"$SC/b1p.jsonl" 2>"$SC/b1p.log"; chk "b cell 1 planted p0" $?
"$B" cell --n 15 --l 3 --m 4 --arms b --dmax-b 3 --first 0 --random 1 --planted 0 --time-limit 25 --job-time 20 >"$SC/b4.jsonl" 2>"$SC/b4.log"; chk "b cell 4 random t0 (20 s cap)" $?
"$B" cell --n 15 --l 3 --m 4 --arms b --dmax-b 2 --random 0 --planted 2 --time-limit 40 --job-time 30 >"$SC/b4p.jsonl" 2>"$SC/b4p.log"; chk "b cell 4 planted" $?
echo "== every row has a status"
for f in "$SC"/a_*.jsonl "$SC"/aa4.jsonl "$SC"/rc_*.jsonl "$SC"/e[345].jsonl "$SC"/b1.jsonl "$SC"/b1p.jsonl "$SC"/b4.jsonl "$SC"/b4p.jsonl; do python3 -c "
import json,sys
rows=[json.loads(l) for l in open('$f') if l.startswith('{')]
assert rows and all('status' in r for r in rows), '$f'" || { echo "FAIL  rows of $f"; fail=1; }; done
[ $fail = 0 ] && echo "SMOKE OK" || echo "SMOKE FAILED"
exit $fail
