#!/bin/bash
# P2 frozen run driver (card CARD-p2-twist-coset-20261107-1). Orchestration only:
# every line below is one literal instrument call; nothing is computed here.
# Usage: p2-run-01.driver.sh <stage m1|m2|m3|m4> <run_dir> <p2coset binary>
# Seeds: first 8 bytes little endian of SHA-256("split_prime_ecdlp_20261107/P2/cosets/<p>/<i>/<t>"),
# curve index i = --cseed i (Addendum A). The values are written out in every call below.
# Raw outputs: <run_dir>/<stage>/<id>.out and .err per instrument call, <run_dir>/<stage>.jsonl one line per call.
set -u
STAGE=${1:?stage}; RUN=${2:?run dir}; B=${3:?binary}
MSOLVE=/usr/bin/msolve
[ -x "$B" ] || { echo "REFUSED: binary $B is not executable"; exit 2; }
mkdir -p "$RUN/$STAGE" || exit 2
J="$RUN/$STAGE.jsonl"
[ -e "$J" ] && { echo "REFUSED: $J exists"; exit 2; }
: > "$J"
NSTEP=0; BAD=0; EMITFAIL=0; MSNZ=0; MSTO=0; NCELL=0
now() { date -u +%Y-%m-%dT%H:%M:%SZ; }
step() { # step <id> <argv...>: one instrument call, stdout to <id>.out, stderr to <id>.err, one JSONL line
  local id=$1; shift
  local o="$RUN/$STAGE/$id.out" e="$RUN/$STAGE/$id.err" t0 t1 out
  if [ -e "$o" ] || [ -e "$e" ]; then echo "REFUSED: $o exists"; exit 2; fi
  t0=$(now)
  "$@" > "$o" 2> "$e"; LASTRC=$?
  t1=$(now)
  out=$(cat "$o"); [ -n "$out" ] || out=null
  printf '{"id":"%s","kind":"call","exit":%d,"start_utc":"%s","end_utc":"%s","argv":"%s","out":%s}\n' "$id" "$LASTRC" "$t0" "$t1" "$*" "$out" >> "$J"
  NSTEP=$((NSTEP+1)); [ "$LASTRC" -eq 0 ] || BAD=$((BAD+1))
}
cell() { # cell <id> <kind> <p> <t> <m> <C> <seed> <select> <tseed> <cseed>: emit, msolve (1200 s, 1.5 GiB), parse
  local id=$1 kind=$2 p=$3 t=$4 m=$5 C=$6 seed=$7 sel=$8 ts=$9 cs=${10} t0 t1 rc reason
  NCELL=$((NCELL+1))
  step "$id.emit" "$B" emit --kind "$kind" --p "$p" --t "$t" --m "$m" --C "$C" --seed "$seed" --cseed "$cs" --select "$sel" --tseed "$ts" --out "$RUN/$STAGE/$id.ms"
  if [ "$LASTRC" -ne 0 ]; then
    EMITFAIL=$((EMITFAIL+1)); BAD=$((BAD-1))
    printf '{"id":"%s","kind":"cell_not_emitted","emit_exit":%d,"reason":"emit exit %d, msolve and parse not run, see %s.emit.err"}\n' "$id" "$LASTRC" "$LASTRC" "$id" >> "$J"
    return 0
  fi
  t0=$(now)
  ( ulimit -v 1572864; nice -n 10 timeout 1200 "$MSOLVE" -g 2 -t 1 -v 2 -f "$RUN/$STAGE/$id.ms" -o "$RUN/$STAGE/$id.msolve.out" > "$RUN/$STAGE/$id.log" 2>&1 ); rc=$?
  t1=$(now)
  reason=null
  if [ "$rc" -eq 124 ]; then reason='"timeout_1200s"'; MSTO=$((MSTO+1)); MSNZ=$((MSNZ+1));
  elif [ "$rc" -ne 0 ]; then reason='"msolve_nonzero_exit_see_log"'; MSNZ=$((MSNZ+1)); fi
  printf '{"id":"%s","kind":"msolve","exit":%d,"start_utc":"%s","end_utc":"%s","reason":%s,"log":"%s.log"}\n' "$id" "$rc" "$t0" "$t1" "$reason" "$id" >> "$J"
  step "$id.parse" "$B" parse --log "$RUN/$STAGE/$id.log"
}

case "$STAGE" in
m1)
  step curve-1049089-c0 "$B" curve --p 1049089 --cseed 0
  step curve-1049089-c1 "$B" curve --p 1049089 --cseed 1
  step curve-1049089-c2 "$B" curve --p 1049089 --cseed 2
  step curve-16777729-c0 "$B" curve --p 16777729 --cseed 0
  step curve-16777729-c1 "$B" curve --p 16777729 --cseed 1
  step curve-16777729-c2 "$B" curve --p 16777729 --cseed 2
  step curve-1049599-c0 "$B" curve --p 1049599 --cseed 0
  step curve-1049599-c1 "$B" curve --p 1049599 --cseed 1
  step curve-1049599-c2 "$B" curve --p 1049599 --cseed 2
  step curve-16780031-c0 "$B" curve --p 16780031 --cseed 0
  step curve-16780031-c1 "$B" curve --p 16780031 --cseed 1
  step curve-16780031-c2 "$B" curve --p 16780031 --cseed 2
  step cosets-kummer-1049089-c0-t4 "$B" cosets --kind kummer --p 1049089 --t 4 --C 10000 --seed 13609300572028886034 --cseed 0
  step cosets-kummer-1049089-c0-t6 "$B" cosets --kind kummer --p 1049089 --t 6 --C 10000 --seed 16765669404794311197 --cseed 0
  step cosets-kummer-1049089-c0-t8 "$B" cosets --kind kummer --p 1049089 --t 8 --C 4098 --seed 11933998564104009250 --cseed 0
  step cosets-kummer-1049089-c1-t4 "$B" cosets --kind kummer --p 1049089 --t 4 --C 10000 --seed 17667823323466471606 --cseed 1
  step cosets-kummer-1049089-c1-t6 "$B" cosets --kind kummer --p 1049089 --t 6 --C 10000 --seed 18248182231806792612 --cseed 1
  step cosets-kummer-1049089-c1-t8 "$B" cosets --kind kummer --p 1049089 --t 8 --C 4098 --seed 3497424982144335580 --cseed 1
  step cosets-kummer-1049089-c2-t4 "$B" cosets --kind kummer --p 1049089 --t 4 --C 10000 --seed 8718496296732446773 --cseed 2
  step cosets-kummer-1049089-c2-t6 "$B" cosets --kind kummer --p 1049089 --t 6 --C 10000 --seed 1594821106893327385 --cseed 2
  step cosets-kummer-1049089-c2-t8 "$B" cosets --kind kummer --p 1049089 --t 8 --C 4098 --seed 2176223858794416553 --cseed 2
  step cosets-kummer-16777729-c0-t4 "$B" cosets --kind kummer --p 16777729 --t 4 --C 10000 --seed 11173020513567249145 --cseed 0
  step cosets-kummer-16777729-c0-t6 "$B" cosets --kind kummer --p 16777729 --t 6 --C 10000 --seed 1320760113827526238 --cseed 0
  step cosets-kummer-16777729-c0-t8 "$B" cosets --kind kummer --p 16777729 --t 8 --C 10000 --seed 15064022976432489644 --cseed 0
  step cosets-kummer-16777729-c1-t4 "$B" cosets --kind kummer --p 16777729 --t 4 --C 10000 --seed 5337470146575355189 --cseed 1
  step cosets-kummer-16777729-c1-t6 "$B" cosets --kind kummer --p 16777729 --t 6 --C 10000 --seed 14016555413821734805 --cseed 1
  step cosets-kummer-16777729-c1-t8 "$B" cosets --kind kummer --p 16777729 --t 8 --C 10000 --seed 2574276575191720477 --cseed 1
  step cosets-kummer-16777729-c2-t4 "$B" cosets --kind kummer --p 16777729 --t 4 --C 10000 --seed 2564293657957595623 --cseed 2
  step cosets-kummer-16777729-c2-t6 "$B" cosets --kind kummer --p 16777729 --t 6 --C 10000 --seed 424115612230040544 --cseed 2
  step cosets-kummer-16777729-c2-t8 "$B" cosets --kind kummer --p 16777729 --t 8 --C 10000 --seed 8812126797135636058 --cseed 2
  step cosets-dickson-1049599-c0-t4 "$B" cosets --kind dickson --p 1049599 --t 4 --C 10000 --seed 3155981005055438761 --cseed 0
  step cosets-dickson-1049599-c0-t6 "$B" cosets --kind dickson --p 1049599 --t 6 --C 8199 --seed 2118746520032967994 --cseed 0
  step cosets-dickson-1049599-c0-t8 "$B" cosets --kind dickson --p 1049599 --t 8 --C 2049 --seed 37506953598447328 --cseed 0
  step cosets-dickson-1049599-c1-t4 "$B" cosets --kind dickson --p 1049599 --t 4 --C 10000 --seed 5387756275857404945 --cseed 1
  step cosets-dickson-1049599-c1-t6 "$B" cosets --kind dickson --p 1049599 --t 6 --C 8199 --seed 7047629953779542884 --cseed 1
  step cosets-dickson-1049599-c1-t8 "$B" cosets --kind dickson --p 1049599 --t 8 --C 2049 --seed 9700398374005556631 --cseed 1
  step cosets-dickson-1049599-c2-t4 "$B" cosets --kind dickson --p 1049599 --t 4 --C 10000 --seed 16911874679314616427 --cseed 2
  step cosets-dickson-1049599-c2-t6 "$B" cosets --kind dickson --p 1049599 --t 6 --C 8199 --seed 4781021203000199802 --cseed 2
  step cosets-dickson-1049599-c2-t8 "$B" cosets --kind dickson --p 1049599 --t 8 --C 2049 --seed 13146580483541063712 --cseed 2
  step cosets-dickson-16780031-c0-t4 "$B" cosets --kind dickson --p 16780031 --t 4 --C 10000 --seed 5479868948465299270 --cseed 0
  step cosets-dickson-16780031-c0-t6 "$B" cosets --kind dickson --p 16780031 --t 6 --C 10000 --seed 4483130442468001098 --cseed 0
  step cosets-dickson-16780031-c0-t8 "$B" cosets --kind dickson --p 16780031 --t 8 --C 10000 --seed 15666102632841463945 --cseed 0
  step cosets-dickson-16780031-c1-t4 "$B" cosets --kind dickson --p 16780031 --t 4 --C 10000 --seed 8770691062182160944 --cseed 1
  step cosets-dickson-16780031-c1-t6 "$B" cosets --kind dickson --p 16780031 --t 6 --C 10000 --seed 10183228895524703850 --cseed 1
  step cosets-dickson-16780031-c1-t8 "$B" cosets --kind dickson --p 16780031 --t 8 --C 10000 --seed 11710007457311663476 --cseed 1
  step cosets-dickson-16780031-c2-t4 "$B" cosets --kind dickson --p 16780031 --t 4 --C 10000 --seed 6347742979283213921 --cseed 2
  step cosets-dickson-16780031-c2-t6 "$B" cosets --kind dickson --p 16780031 --t 6 --C 10000 --seed 3799876297577403297 --cseed 2
  step cosets-dickson-16780031-c2-t8 "$B" cosets --kind dickson --p 16780031 --t 8 --C 10000 --seed 17794471472019358775 --cseed 2
  ;;
m2)
  step cov-kummer-1049089-c0-t4-idx0 "$B" coverage --kind kummer --p 1049089 --t 4 --C 10000 --seed 13609300572028886034 --cseed 0 --select idx:0
  step cov-kummer-1049089-c0-t4-idx1 "$B" coverage --kind kummer --p 1049089 --t 4 --C 10000 --seed 13609300572028886034 --cseed 0 --select idx:1
  step cov-kummer-1049089-c0-t4-max "$B" coverage --kind kummer --p 1049089 --t 4 --C 10000 --seed 13609300572028886034 --cseed 0 --select max
  step cov-kummer-1049089-c0-t6-idx0 "$B" coverage --kind kummer --p 1049089 --t 6 --C 10000 --seed 16765669404794311197 --cseed 0 --select idx:0
  step cov-kummer-1049089-c0-t6-idx1 "$B" coverage --kind kummer --p 1049089 --t 6 --C 10000 --seed 16765669404794311197 --cseed 0 --select idx:1
  step cov-kummer-1049089-c0-t6-max "$B" coverage --kind kummer --p 1049089 --t 6 --C 10000 --seed 16765669404794311197 --cseed 0 --select max
  step cov-kummer-1049089-c0-t8-idx0 "$B" coverage --kind kummer --p 1049089 --t 8 --C 4098 --seed 11933998564104009250 --cseed 0 --select idx:0
  step cov-kummer-1049089-c0-t8-idx1 "$B" coverage --kind kummer --p 1049089 --t 8 --C 4098 --seed 11933998564104009250 --cseed 0 --select idx:1
  step cov-kummer-1049089-c0-t8-max "$B" coverage --kind kummer --p 1049089 --t 8 --C 4098 --seed 11933998564104009250 --cseed 0 --select max
  step cov-kummer-1049089-c1-t4-idx0 "$B" coverage --kind kummer --p 1049089 --t 4 --C 10000 --seed 17667823323466471606 --cseed 1 --select idx:0
  step cov-kummer-1049089-c1-t4-idx1 "$B" coverage --kind kummer --p 1049089 --t 4 --C 10000 --seed 17667823323466471606 --cseed 1 --select idx:1
  step cov-kummer-1049089-c1-t4-max "$B" coverage --kind kummer --p 1049089 --t 4 --C 10000 --seed 17667823323466471606 --cseed 1 --select max
  step cov-kummer-1049089-c1-t6-idx0 "$B" coverage --kind kummer --p 1049089 --t 6 --C 10000 --seed 18248182231806792612 --cseed 1 --select idx:0
  step cov-kummer-1049089-c1-t6-idx1 "$B" coverage --kind kummer --p 1049089 --t 6 --C 10000 --seed 18248182231806792612 --cseed 1 --select idx:1
  step cov-kummer-1049089-c1-t6-max "$B" coverage --kind kummer --p 1049089 --t 6 --C 10000 --seed 18248182231806792612 --cseed 1 --select max
  step cov-kummer-1049089-c1-t8-idx0 "$B" coverage --kind kummer --p 1049089 --t 8 --C 4098 --seed 3497424982144335580 --cseed 1 --select idx:0
  step cov-kummer-1049089-c1-t8-idx1 "$B" coverage --kind kummer --p 1049089 --t 8 --C 4098 --seed 3497424982144335580 --cseed 1 --select idx:1
  step cov-kummer-1049089-c1-t8-max "$B" coverage --kind kummer --p 1049089 --t 8 --C 4098 --seed 3497424982144335580 --cseed 1 --select max
  step cov-kummer-1049089-c2-t4-idx0 "$B" coverage --kind kummer --p 1049089 --t 4 --C 10000 --seed 8718496296732446773 --cseed 2 --select idx:0
  step cov-kummer-1049089-c2-t4-idx1 "$B" coverage --kind kummer --p 1049089 --t 4 --C 10000 --seed 8718496296732446773 --cseed 2 --select idx:1
  step cov-kummer-1049089-c2-t4-max "$B" coverage --kind kummer --p 1049089 --t 4 --C 10000 --seed 8718496296732446773 --cseed 2 --select max
  step cov-kummer-1049089-c2-t6-idx0 "$B" coverage --kind kummer --p 1049089 --t 6 --C 10000 --seed 1594821106893327385 --cseed 2 --select idx:0
  step cov-kummer-1049089-c2-t6-idx1 "$B" coverage --kind kummer --p 1049089 --t 6 --C 10000 --seed 1594821106893327385 --cseed 2 --select idx:1
  step cov-kummer-1049089-c2-t6-max "$B" coverage --kind kummer --p 1049089 --t 6 --C 10000 --seed 1594821106893327385 --cseed 2 --select max
  step cov-kummer-1049089-c2-t8-idx0 "$B" coverage --kind kummer --p 1049089 --t 8 --C 4098 --seed 2176223858794416553 --cseed 2 --select idx:0
  step cov-kummer-1049089-c2-t8-idx1 "$B" coverage --kind kummer --p 1049089 --t 8 --C 4098 --seed 2176223858794416553 --cseed 2 --select idx:1
  step cov-kummer-1049089-c2-t8-max "$B" coverage --kind kummer --p 1049089 --t 8 --C 4098 --seed 2176223858794416553 --cseed 2 --select max
  step cov-kummer-16777729-c0-t4-idx0 "$B" coverage --kind kummer --p 16777729 --t 4 --C 10000 --seed 11173020513567249145 --cseed 0 --select idx:0
  step cov-kummer-16777729-c0-t4-idx1 "$B" coverage --kind kummer --p 16777729 --t 4 --C 10000 --seed 11173020513567249145 --cseed 0 --select idx:1
  step cov-kummer-16777729-c0-t4-max "$B" coverage --kind kummer --p 16777729 --t 4 --C 10000 --seed 11173020513567249145 --cseed 0 --select max
  step cov-kummer-16777729-c0-t6-idx0 "$B" coverage --kind kummer --p 16777729 --t 6 --C 10000 --seed 1320760113827526238 --cseed 0 --select idx:0
  step cov-kummer-16777729-c0-t6-idx1 "$B" coverage --kind kummer --p 16777729 --t 6 --C 10000 --seed 1320760113827526238 --cseed 0 --select idx:1
  step cov-kummer-16777729-c0-t6-max "$B" coverage --kind kummer --p 16777729 --t 6 --C 10000 --seed 1320760113827526238 --cseed 0 --select max
  step cov-kummer-16777729-c0-t8-idx0 "$B" coverage --kind kummer --p 16777729 --t 8 --C 10000 --seed 15064022976432489644 --cseed 0 --select idx:0
  step cov-kummer-16777729-c0-t8-idx1 "$B" coverage --kind kummer --p 16777729 --t 8 --C 10000 --seed 15064022976432489644 --cseed 0 --select idx:1
  step cov-kummer-16777729-c0-t8-max "$B" coverage --kind kummer --p 16777729 --t 8 --C 10000 --seed 15064022976432489644 --cseed 0 --select max
  step cov-kummer-16777729-c1-t4-idx0 "$B" coverage --kind kummer --p 16777729 --t 4 --C 10000 --seed 5337470146575355189 --cseed 1 --select idx:0
  step cov-kummer-16777729-c1-t4-idx1 "$B" coverage --kind kummer --p 16777729 --t 4 --C 10000 --seed 5337470146575355189 --cseed 1 --select idx:1
  step cov-kummer-16777729-c1-t4-max "$B" coverage --kind kummer --p 16777729 --t 4 --C 10000 --seed 5337470146575355189 --cseed 1 --select max
  step cov-kummer-16777729-c1-t6-idx0 "$B" coverage --kind kummer --p 16777729 --t 6 --C 10000 --seed 14016555413821734805 --cseed 1 --select idx:0
  step cov-kummer-16777729-c1-t6-idx1 "$B" coverage --kind kummer --p 16777729 --t 6 --C 10000 --seed 14016555413821734805 --cseed 1 --select idx:1
  step cov-kummer-16777729-c1-t6-max "$B" coverage --kind kummer --p 16777729 --t 6 --C 10000 --seed 14016555413821734805 --cseed 1 --select max
  step cov-kummer-16777729-c1-t8-idx0 "$B" coverage --kind kummer --p 16777729 --t 8 --C 10000 --seed 2574276575191720477 --cseed 1 --select idx:0
  step cov-kummer-16777729-c1-t8-idx1 "$B" coverage --kind kummer --p 16777729 --t 8 --C 10000 --seed 2574276575191720477 --cseed 1 --select idx:1
  step cov-kummer-16777729-c1-t8-max "$B" coverage --kind kummer --p 16777729 --t 8 --C 10000 --seed 2574276575191720477 --cseed 1 --select max
  step cov-kummer-16777729-c2-t4-idx0 "$B" coverage --kind kummer --p 16777729 --t 4 --C 10000 --seed 2564293657957595623 --cseed 2 --select idx:0
  step cov-kummer-16777729-c2-t4-idx1 "$B" coverage --kind kummer --p 16777729 --t 4 --C 10000 --seed 2564293657957595623 --cseed 2 --select idx:1
  step cov-kummer-16777729-c2-t4-max "$B" coverage --kind kummer --p 16777729 --t 4 --C 10000 --seed 2564293657957595623 --cseed 2 --select max
  step cov-kummer-16777729-c2-t6-idx0 "$B" coverage --kind kummer --p 16777729 --t 6 --C 10000 --seed 424115612230040544 --cseed 2 --select idx:0
  step cov-kummer-16777729-c2-t6-idx1 "$B" coverage --kind kummer --p 16777729 --t 6 --C 10000 --seed 424115612230040544 --cseed 2 --select idx:1
  step cov-kummer-16777729-c2-t6-max "$B" coverage --kind kummer --p 16777729 --t 6 --C 10000 --seed 424115612230040544 --cseed 2 --select max
  step cov-kummer-16777729-c2-t8-idx0 "$B" coverage --kind kummer --p 16777729 --t 8 --C 10000 --seed 8812126797135636058 --cseed 2 --select idx:0
  step cov-kummer-16777729-c2-t8-idx1 "$B" coverage --kind kummer --p 16777729 --t 8 --C 10000 --seed 8812126797135636058 --cseed 2 --select idx:1
  step cov-kummer-16777729-c2-t8-max "$B" coverage --kind kummer --p 16777729 --t 8 --C 10000 --seed 8812126797135636058 --cseed 2 --select max
  step cov-dickson-1049599-c0-t4-idx0 "$B" coverage --kind dickson --p 1049599 --t 4 --C 10000 --seed 3155981005055438761 --cseed 0 --select idx:0
  step cov-dickson-1049599-c0-t4-idx1 "$B" coverage --kind dickson --p 1049599 --t 4 --C 10000 --seed 3155981005055438761 --cseed 0 --select idx:1
  step cov-dickson-1049599-c0-t4-max "$B" coverage --kind dickson --p 1049599 --t 4 --C 10000 --seed 3155981005055438761 --cseed 0 --select max
  step cov-dickson-1049599-c0-t6-idx0 "$B" coverage --kind dickson --p 1049599 --t 6 --C 8199 --seed 2118746520032967994 --cseed 0 --select idx:0
  step cov-dickson-1049599-c0-t6-idx1 "$B" coverage --kind dickson --p 1049599 --t 6 --C 8199 --seed 2118746520032967994 --cseed 0 --select idx:1
  step cov-dickson-1049599-c0-t6-max "$B" coverage --kind dickson --p 1049599 --t 6 --C 8199 --seed 2118746520032967994 --cseed 0 --select max
  step cov-dickson-1049599-c0-t8-idx0 "$B" coverage --kind dickson --p 1049599 --t 8 --C 2049 --seed 37506953598447328 --cseed 0 --select idx:0
  step cov-dickson-1049599-c0-t8-idx1 "$B" coverage --kind dickson --p 1049599 --t 8 --C 2049 --seed 37506953598447328 --cseed 0 --select idx:1
  step cov-dickson-1049599-c0-t8-max "$B" coverage --kind dickson --p 1049599 --t 8 --C 2049 --seed 37506953598447328 --cseed 0 --select max
  step cov-dickson-1049599-c1-t4-idx0 "$B" coverage --kind dickson --p 1049599 --t 4 --C 10000 --seed 5387756275857404945 --cseed 1 --select idx:0
  step cov-dickson-1049599-c1-t4-idx1 "$B" coverage --kind dickson --p 1049599 --t 4 --C 10000 --seed 5387756275857404945 --cseed 1 --select idx:1
  step cov-dickson-1049599-c1-t4-max "$B" coverage --kind dickson --p 1049599 --t 4 --C 10000 --seed 5387756275857404945 --cseed 1 --select max
  step cov-dickson-1049599-c1-t6-idx0 "$B" coverage --kind dickson --p 1049599 --t 6 --C 8199 --seed 7047629953779542884 --cseed 1 --select idx:0
  step cov-dickson-1049599-c1-t6-idx1 "$B" coverage --kind dickson --p 1049599 --t 6 --C 8199 --seed 7047629953779542884 --cseed 1 --select idx:1
  step cov-dickson-1049599-c1-t6-max "$B" coverage --kind dickson --p 1049599 --t 6 --C 8199 --seed 7047629953779542884 --cseed 1 --select max
  step cov-dickson-1049599-c1-t8-idx0 "$B" coverage --kind dickson --p 1049599 --t 8 --C 2049 --seed 9700398374005556631 --cseed 1 --select idx:0
  step cov-dickson-1049599-c1-t8-idx1 "$B" coverage --kind dickson --p 1049599 --t 8 --C 2049 --seed 9700398374005556631 --cseed 1 --select idx:1
  step cov-dickson-1049599-c1-t8-max "$B" coverage --kind dickson --p 1049599 --t 8 --C 2049 --seed 9700398374005556631 --cseed 1 --select max
  step cov-dickson-1049599-c2-t4-idx0 "$B" coverage --kind dickson --p 1049599 --t 4 --C 10000 --seed 16911874679314616427 --cseed 2 --select idx:0
  step cov-dickson-1049599-c2-t4-idx1 "$B" coverage --kind dickson --p 1049599 --t 4 --C 10000 --seed 16911874679314616427 --cseed 2 --select idx:1
  step cov-dickson-1049599-c2-t4-max "$B" coverage --kind dickson --p 1049599 --t 4 --C 10000 --seed 16911874679314616427 --cseed 2 --select max
  step cov-dickson-1049599-c2-t6-idx0 "$B" coverage --kind dickson --p 1049599 --t 6 --C 8199 --seed 4781021203000199802 --cseed 2 --select idx:0
  step cov-dickson-1049599-c2-t6-idx1 "$B" coverage --kind dickson --p 1049599 --t 6 --C 8199 --seed 4781021203000199802 --cseed 2 --select idx:1
  step cov-dickson-1049599-c2-t6-max "$B" coverage --kind dickson --p 1049599 --t 6 --C 8199 --seed 4781021203000199802 --cseed 2 --select max
  step cov-dickson-1049599-c2-t8-idx0 "$B" coverage --kind dickson --p 1049599 --t 8 --C 2049 --seed 13146580483541063712 --cseed 2 --select idx:0
  step cov-dickson-1049599-c2-t8-idx1 "$B" coverage --kind dickson --p 1049599 --t 8 --C 2049 --seed 13146580483541063712 --cseed 2 --select idx:1
  step cov-dickson-1049599-c2-t8-max "$B" coverage --kind dickson --p 1049599 --t 8 --C 2049 --seed 13146580483541063712 --cseed 2 --select max
  step cov-dickson-16780031-c0-t4-idx0 "$B" coverage --kind dickson --p 16780031 --t 4 --C 10000 --seed 5479868948465299270 --cseed 0 --select idx:0
  step cov-dickson-16780031-c0-t4-idx1 "$B" coverage --kind dickson --p 16780031 --t 4 --C 10000 --seed 5479868948465299270 --cseed 0 --select idx:1
  step cov-dickson-16780031-c0-t4-max "$B" coverage --kind dickson --p 16780031 --t 4 --C 10000 --seed 5479868948465299270 --cseed 0 --select max
  step cov-dickson-16780031-c0-t6-idx0 "$B" coverage --kind dickson --p 16780031 --t 6 --C 10000 --seed 4483130442468001098 --cseed 0 --select idx:0
  step cov-dickson-16780031-c0-t6-idx1 "$B" coverage --kind dickson --p 16780031 --t 6 --C 10000 --seed 4483130442468001098 --cseed 0 --select idx:1
  step cov-dickson-16780031-c0-t6-max "$B" coverage --kind dickson --p 16780031 --t 6 --C 10000 --seed 4483130442468001098 --cseed 0 --select max
  step cov-dickson-16780031-c0-t8-idx0 "$B" coverage --kind dickson --p 16780031 --t 8 --C 10000 --seed 15666102632841463945 --cseed 0 --select idx:0
  step cov-dickson-16780031-c0-t8-idx1 "$B" coverage --kind dickson --p 16780031 --t 8 --C 10000 --seed 15666102632841463945 --cseed 0 --select idx:1
  step cov-dickson-16780031-c0-t8-max "$B" coverage --kind dickson --p 16780031 --t 8 --C 10000 --seed 15666102632841463945 --cseed 0 --select max
  step cov-dickson-16780031-c1-t4-idx0 "$B" coverage --kind dickson --p 16780031 --t 4 --C 10000 --seed 8770691062182160944 --cseed 1 --select idx:0
  step cov-dickson-16780031-c1-t4-idx1 "$B" coverage --kind dickson --p 16780031 --t 4 --C 10000 --seed 8770691062182160944 --cseed 1 --select idx:1
  step cov-dickson-16780031-c1-t4-max "$B" coverage --kind dickson --p 16780031 --t 4 --C 10000 --seed 8770691062182160944 --cseed 1 --select max
  step cov-dickson-16780031-c1-t6-idx0 "$B" coverage --kind dickson --p 16780031 --t 6 --C 10000 --seed 10183228895524703850 --cseed 1 --select idx:0
  step cov-dickson-16780031-c1-t6-idx1 "$B" coverage --kind dickson --p 16780031 --t 6 --C 10000 --seed 10183228895524703850 --cseed 1 --select idx:1
  step cov-dickson-16780031-c1-t6-max "$B" coverage --kind dickson --p 16780031 --t 6 --C 10000 --seed 10183228895524703850 --cseed 1 --select max
  step cov-dickson-16780031-c1-t8-idx0 "$B" coverage --kind dickson --p 16780031 --t 8 --C 10000 --seed 11710007457311663476 --cseed 1 --select idx:0
  step cov-dickson-16780031-c1-t8-idx1 "$B" coverage --kind dickson --p 16780031 --t 8 --C 10000 --seed 11710007457311663476 --cseed 1 --select idx:1
  step cov-dickson-16780031-c1-t8-max "$B" coverage --kind dickson --p 16780031 --t 8 --C 10000 --seed 11710007457311663476 --cseed 1 --select max
  step cov-dickson-16780031-c2-t4-idx0 "$B" coverage --kind dickson --p 16780031 --t 4 --C 10000 --seed 6347742979283213921 --cseed 2 --select idx:0
  step cov-dickson-16780031-c2-t4-idx1 "$B" coverage --kind dickson --p 16780031 --t 4 --C 10000 --seed 6347742979283213921 --cseed 2 --select idx:1
  step cov-dickson-16780031-c2-t4-max "$B" coverage --kind dickson --p 16780031 --t 4 --C 10000 --seed 6347742979283213921 --cseed 2 --select max
  step cov-dickson-16780031-c2-t6-idx0 "$B" coverage --kind dickson --p 16780031 --t 6 --C 10000 --seed 3799876297577403297 --cseed 2 --select idx:0
  step cov-dickson-16780031-c2-t6-idx1 "$B" coverage --kind dickson --p 16780031 --t 6 --C 10000 --seed 3799876297577403297 --cseed 2 --select idx:1
  step cov-dickson-16780031-c2-t6-max "$B" coverage --kind dickson --p 16780031 --t 6 --C 10000 --seed 3799876297577403297 --cseed 2 --select max
  step cov-dickson-16780031-c2-t8-idx0 "$B" coverage --kind dickson --p 16780031 --t 8 --C 10000 --seed 17794471472019358775 --cseed 2 --select idx:0
  step cov-dickson-16780031-c2-t8-idx1 "$B" coverage --kind dickson --p 16780031 --t 8 --C 10000 --seed 17794471472019358775 --cseed 2 --select idx:1
  step cov-dickson-16780031-c2-t8-max "$B" coverage --kind dickson --p 16780031 --t 8 --C 10000 --seed 17794471472019358775 --cseed 2 --select max
  ;;
m3)
  cell kummer-1049089-m2-t4-idx0-ts1 kummer 1049089 4 2 10000 13609300572028886034 idx:0 1 0
  cell kummer-1049089-m2-t4-idx0-ts2 kummer 1049089 4 2 10000 13609300572028886034 idx:0 2 0
  cell kummer-1049089-m2-t4-idx0-ts3 kummer 1049089 4 2 10000 13609300572028886034 idx:0 3 0
  cell kummer-1049089-m2-t4-idx0-ts4 kummer 1049089 4 2 10000 13609300572028886034 idx:0 4 0
  cell kummer-1049089-m2-t4-idx1-ts1 kummer 1049089 4 2 10000 13609300572028886034 idx:1 1 0
  cell kummer-1049089-m2-t4-idx1-ts2 kummer 1049089 4 2 10000 13609300572028886034 idx:1 2 0
  cell kummer-1049089-m2-t4-idx1-ts3 kummer 1049089 4 2 10000 13609300572028886034 idx:1 3 0
  cell kummer-1049089-m2-t4-idx1-ts4 kummer 1049089 4 2 10000 13609300572028886034 idx:1 4 0
  cell kummer-1049089-m2-t4-max-ts1 kummer 1049089 4 2 10000 13609300572028886034 max 1 0
  cell kummer-1049089-m2-t4-max-ts2 kummer 1049089 4 2 10000 13609300572028886034 max 2 0
  cell kummer-1049089-m2-t4-max-ts3 kummer 1049089 4 2 10000 13609300572028886034 max 3 0
  cell kummer-1049089-m2-t4-max-ts4 kummer 1049089 4 2 10000 13609300572028886034 max 4 0
  cell kummer-1049089-m2-t6-idx0-ts1 kummer 1049089 6 2 10000 16765669404794311197 idx:0 1 0
  cell kummer-1049089-m2-t6-idx0-ts2 kummer 1049089 6 2 10000 16765669404794311197 idx:0 2 0
  cell kummer-1049089-m2-t6-idx0-ts3 kummer 1049089 6 2 10000 16765669404794311197 idx:0 3 0
  cell kummer-1049089-m2-t6-idx0-ts4 kummer 1049089 6 2 10000 16765669404794311197 idx:0 4 0
  cell kummer-1049089-m2-t6-idx1-ts1 kummer 1049089 6 2 10000 16765669404794311197 idx:1 1 0
  cell kummer-1049089-m2-t6-idx1-ts2 kummer 1049089 6 2 10000 16765669404794311197 idx:1 2 0
  cell kummer-1049089-m2-t6-idx1-ts3 kummer 1049089 6 2 10000 16765669404794311197 idx:1 3 0
  cell kummer-1049089-m2-t6-idx1-ts4 kummer 1049089 6 2 10000 16765669404794311197 idx:1 4 0
  cell kummer-1049089-m2-t6-max-ts1 kummer 1049089 6 2 10000 16765669404794311197 max 1 0
  cell kummer-1049089-m2-t6-max-ts2 kummer 1049089 6 2 10000 16765669404794311197 max 2 0
  cell kummer-1049089-m2-t6-max-ts3 kummer 1049089 6 2 10000 16765669404794311197 max 3 0
  cell kummer-1049089-m2-t6-max-ts4 kummer 1049089 6 2 10000 16765669404794311197 max 4 0
  cell kummer-1049089-m3-t4-idx0-ts1 kummer 1049089 4 3 10000 13609300572028886034 idx:0 1 0
  cell kummer-1049089-m3-t4-idx0-ts2 kummer 1049089 4 3 10000 13609300572028886034 idx:0 2 0
  cell kummer-1049089-m3-t4-idx0-ts3 kummer 1049089 4 3 10000 13609300572028886034 idx:0 3 0
  cell kummer-1049089-m3-t4-idx0-ts4 kummer 1049089 4 3 10000 13609300572028886034 idx:0 4 0
  cell kummer-1049089-m3-t4-idx1-ts1 kummer 1049089 4 3 10000 13609300572028886034 idx:1 1 0
  cell kummer-1049089-m3-t4-idx1-ts2 kummer 1049089 4 3 10000 13609300572028886034 idx:1 2 0
  cell kummer-1049089-m3-t4-idx1-ts3 kummer 1049089 4 3 10000 13609300572028886034 idx:1 3 0
  cell kummer-1049089-m3-t4-idx1-ts4 kummer 1049089 4 3 10000 13609300572028886034 idx:1 4 0
  cell kummer-1049089-m3-t4-max-ts1 kummer 1049089 4 3 10000 13609300572028886034 max 1 0
  cell kummer-1049089-m3-t4-max-ts2 kummer 1049089 4 3 10000 13609300572028886034 max 2 0
  cell kummer-1049089-m3-t4-max-ts3 kummer 1049089 4 3 10000 13609300572028886034 max 3 0
  cell kummer-1049089-m3-t4-max-ts4 kummer 1049089 4 3 10000 13609300572028886034 max 4 0
  cell kummer-16777729-m2-t4-idx0-ts1 kummer 16777729 4 2 10000 11173020513567249145 idx:0 1 0
  cell kummer-16777729-m2-t4-idx0-ts2 kummer 16777729 4 2 10000 11173020513567249145 idx:0 2 0
  cell kummer-16777729-m2-t4-idx0-ts3 kummer 16777729 4 2 10000 11173020513567249145 idx:0 3 0
  cell kummer-16777729-m2-t4-idx0-ts4 kummer 16777729 4 2 10000 11173020513567249145 idx:0 4 0
  cell kummer-16777729-m2-t4-idx1-ts1 kummer 16777729 4 2 10000 11173020513567249145 idx:1 1 0
  cell kummer-16777729-m2-t4-idx1-ts2 kummer 16777729 4 2 10000 11173020513567249145 idx:1 2 0
  cell kummer-16777729-m2-t4-idx1-ts3 kummer 16777729 4 2 10000 11173020513567249145 idx:1 3 0
  cell kummer-16777729-m2-t4-idx1-ts4 kummer 16777729 4 2 10000 11173020513567249145 idx:1 4 0
  cell kummer-16777729-m2-t4-max-ts1 kummer 16777729 4 2 10000 11173020513567249145 max 1 0
  cell kummer-16777729-m2-t4-max-ts2 kummer 16777729 4 2 10000 11173020513567249145 max 2 0
  cell kummer-16777729-m2-t4-max-ts3 kummer 16777729 4 2 10000 11173020513567249145 max 3 0
  cell kummer-16777729-m2-t4-max-ts4 kummer 16777729 4 2 10000 11173020513567249145 max 4 0
  cell kummer-16777729-m2-t6-idx0-ts1 kummer 16777729 6 2 10000 1320760113827526238 idx:0 1 0
  cell kummer-16777729-m2-t6-idx0-ts2 kummer 16777729 6 2 10000 1320760113827526238 idx:0 2 0
  cell kummer-16777729-m2-t6-idx0-ts3 kummer 16777729 6 2 10000 1320760113827526238 idx:0 3 0
  cell kummer-16777729-m2-t6-idx0-ts4 kummer 16777729 6 2 10000 1320760113827526238 idx:0 4 0
  cell kummer-16777729-m2-t6-idx1-ts1 kummer 16777729 6 2 10000 1320760113827526238 idx:1 1 0
  cell kummer-16777729-m2-t6-idx1-ts2 kummer 16777729 6 2 10000 1320760113827526238 idx:1 2 0
  cell kummer-16777729-m2-t6-idx1-ts3 kummer 16777729 6 2 10000 1320760113827526238 idx:1 3 0
  cell kummer-16777729-m2-t6-idx1-ts4 kummer 16777729 6 2 10000 1320760113827526238 idx:1 4 0
  cell kummer-16777729-m2-t6-max-ts1 kummer 16777729 6 2 10000 1320760113827526238 max 1 0
  cell kummer-16777729-m2-t6-max-ts2 kummer 16777729 6 2 10000 1320760113827526238 max 2 0
  cell kummer-16777729-m2-t6-max-ts3 kummer 16777729 6 2 10000 1320760113827526238 max 3 0
  cell kummer-16777729-m2-t6-max-ts4 kummer 16777729 6 2 10000 1320760113827526238 max 4 0
  cell kummer-16777729-m3-t4-idx0-ts1 kummer 16777729 4 3 10000 11173020513567249145 idx:0 1 0
  cell kummer-16777729-m3-t4-idx0-ts2 kummer 16777729 4 3 10000 11173020513567249145 idx:0 2 0
  cell kummer-16777729-m3-t4-idx0-ts3 kummer 16777729 4 3 10000 11173020513567249145 idx:0 3 0
  cell kummer-16777729-m3-t4-idx0-ts4 kummer 16777729 4 3 10000 11173020513567249145 idx:0 4 0
  cell kummer-16777729-m3-t4-idx1-ts1 kummer 16777729 4 3 10000 11173020513567249145 idx:1 1 0
  cell kummer-16777729-m3-t4-idx1-ts2 kummer 16777729 4 3 10000 11173020513567249145 idx:1 2 0
  cell kummer-16777729-m3-t4-idx1-ts3 kummer 16777729 4 3 10000 11173020513567249145 idx:1 3 0
  cell kummer-16777729-m3-t4-idx1-ts4 kummer 16777729 4 3 10000 11173020513567249145 idx:1 4 0
  cell kummer-16777729-m3-t4-max-ts1 kummer 16777729 4 3 10000 11173020513567249145 max 1 0
  cell kummer-16777729-m3-t4-max-ts2 kummer 16777729 4 3 10000 11173020513567249145 max 2 0
  cell kummer-16777729-m3-t4-max-ts3 kummer 16777729 4 3 10000 11173020513567249145 max 3 0
  cell kummer-16777729-m3-t4-max-ts4 kummer 16777729 4 3 10000 11173020513567249145 max 4 0
  cell dickson-1049599-m2-t4-idx0-ts1 dickson 1049599 4 2 10000 3155981005055438761 idx:0 1 0
  cell dickson-1049599-m2-t4-idx0-ts2 dickson 1049599 4 2 10000 3155981005055438761 idx:0 2 0
  cell dickson-1049599-m2-t4-idx0-ts3 dickson 1049599 4 2 10000 3155981005055438761 idx:0 3 0
  cell dickson-1049599-m2-t4-idx0-ts4 dickson 1049599 4 2 10000 3155981005055438761 idx:0 4 0
  cell dickson-1049599-m2-t4-idx1-ts1 dickson 1049599 4 2 10000 3155981005055438761 idx:1 1 0
  cell dickson-1049599-m2-t4-idx1-ts2 dickson 1049599 4 2 10000 3155981005055438761 idx:1 2 0
  cell dickson-1049599-m2-t4-idx1-ts3 dickson 1049599 4 2 10000 3155981005055438761 idx:1 3 0
  cell dickson-1049599-m2-t4-idx1-ts4 dickson 1049599 4 2 10000 3155981005055438761 idx:1 4 0
  cell dickson-1049599-m2-t4-max-ts1 dickson 1049599 4 2 10000 3155981005055438761 max 1 0
  cell dickson-1049599-m2-t4-max-ts2 dickson 1049599 4 2 10000 3155981005055438761 max 2 0
  cell dickson-1049599-m2-t4-max-ts3 dickson 1049599 4 2 10000 3155981005055438761 max 3 0
  cell dickson-1049599-m2-t4-max-ts4 dickson 1049599 4 2 10000 3155981005055438761 max 4 0
  cell dickson-1049599-m2-t6-idx0-ts1 dickson 1049599 6 2 8199 2118746520032967994 idx:0 1 0
  cell dickson-1049599-m2-t6-idx0-ts2 dickson 1049599 6 2 8199 2118746520032967994 idx:0 2 0
  cell dickson-1049599-m2-t6-idx0-ts3 dickson 1049599 6 2 8199 2118746520032967994 idx:0 3 0
  cell dickson-1049599-m2-t6-idx0-ts4 dickson 1049599 6 2 8199 2118746520032967994 idx:0 4 0
  cell dickson-1049599-m2-t6-idx1-ts1 dickson 1049599 6 2 8199 2118746520032967994 idx:1 1 0
  cell dickson-1049599-m2-t6-idx1-ts2 dickson 1049599 6 2 8199 2118746520032967994 idx:1 2 0
  cell dickson-1049599-m2-t6-idx1-ts3 dickson 1049599 6 2 8199 2118746520032967994 idx:1 3 0
  cell dickson-1049599-m2-t6-idx1-ts4 dickson 1049599 6 2 8199 2118746520032967994 idx:1 4 0
  cell dickson-1049599-m2-t6-max-ts1 dickson 1049599 6 2 8199 2118746520032967994 max 1 0
  cell dickson-1049599-m2-t6-max-ts2 dickson 1049599 6 2 8199 2118746520032967994 max 2 0
  cell dickson-1049599-m2-t6-max-ts3 dickson 1049599 6 2 8199 2118746520032967994 max 3 0
  cell dickson-1049599-m2-t6-max-ts4 dickson 1049599 6 2 8199 2118746520032967994 max 4 0
  cell dickson-1049599-m3-t4-idx0-ts1 dickson 1049599 4 3 10000 3155981005055438761 idx:0 1 0
  cell dickson-1049599-m3-t4-idx0-ts2 dickson 1049599 4 3 10000 3155981005055438761 idx:0 2 0
  cell dickson-1049599-m3-t4-idx0-ts3 dickson 1049599 4 3 10000 3155981005055438761 idx:0 3 0
  cell dickson-1049599-m3-t4-idx0-ts4 dickson 1049599 4 3 10000 3155981005055438761 idx:0 4 0
  cell dickson-1049599-m3-t4-idx1-ts1 dickson 1049599 4 3 10000 3155981005055438761 idx:1 1 0
  cell dickson-1049599-m3-t4-idx1-ts2 dickson 1049599 4 3 10000 3155981005055438761 idx:1 2 0
  cell dickson-1049599-m3-t4-idx1-ts3 dickson 1049599 4 3 10000 3155981005055438761 idx:1 3 0
  cell dickson-1049599-m3-t4-idx1-ts4 dickson 1049599 4 3 10000 3155981005055438761 idx:1 4 0
  cell dickson-1049599-m3-t4-max-ts1 dickson 1049599 4 3 10000 3155981005055438761 max 1 0
  cell dickson-1049599-m3-t4-max-ts2 dickson 1049599 4 3 10000 3155981005055438761 max 2 0
  cell dickson-1049599-m3-t4-max-ts3 dickson 1049599 4 3 10000 3155981005055438761 max 3 0
  cell dickson-1049599-m3-t4-max-ts4 dickson 1049599 4 3 10000 3155981005055438761 max 4 0
  cell dickson-16780031-m2-t4-idx0-ts1 dickson 16780031 4 2 10000 5479868948465299270 idx:0 1 0
  cell dickson-16780031-m2-t4-idx0-ts2 dickson 16780031 4 2 10000 5479868948465299270 idx:0 2 0
  cell dickson-16780031-m2-t4-idx0-ts3 dickson 16780031 4 2 10000 5479868948465299270 idx:0 3 0
  cell dickson-16780031-m2-t4-idx0-ts4 dickson 16780031 4 2 10000 5479868948465299270 idx:0 4 0
  cell dickson-16780031-m2-t4-idx1-ts1 dickson 16780031 4 2 10000 5479868948465299270 idx:1 1 0
  cell dickson-16780031-m2-t4-idx1-ts2 dickson 16780031 4 2 10000 5479868948465299270 idx:1 2 0
  cell dickson-16780031-m2-t4-idx1-ts3 dickson 16780031 4 2 10000 5479868948465299270 idx:1 3 0
  cell dickson-16780031-m2-t4-idx1-ts4 dickson 16780031 4 2 10000 5479868948465299270 idx:1 4 0
  cell dickson-16780031-m2-t4-max-ts1 dickson 16780031 4 2 10000 5479868948465299270 max 1 0
  cell dickson-16780031-m2-t4-max-ts2 dickson 16780031 4 2 10000 5479868948465299270 max 2 0
  cell dickson-16780031-m2-t4-max-ts3 dickson 16780031 4 2 10000 5479868948465299270 max 3 0
  cell dickson-16780031-m2-t4-max-ts4 dickson 16780031 4 2 10000 5479868948465299270 max 4 0
  cell dickson-16780031-m2-t6-idx0-ts1 dickson 16780031 6 2 10000 4483130442468001098 idx:0 1 0
  cell dickson-16780031-m2-t6-idx0-ts2 dickson 16780031 6 2 10000 4483130442468001098 idx:0 2 0
  cell dickson-16780031-m2-t6-idx0-ts3 dickson 16780031 6 2 10000 4483130442468001098 idx:0 3 0
  cell dickson-16780031-m2-t6-idx0-ts4 dickson 16780031 6 2 10000 4483130442468001098 idx:0 4 0
  cell dickson-16780031-m2-t6-idx1-ts1 dickson 16780031 6 2 10000 4483130442468001098 idx:1 1 0
  cell dickson-16780031-m2-t6-idx1-ts2 dickson 16780031 6 2 10000 4483130442468001098 idx:1 2 0
  cell dickson-16780031-m2-t6-idx1-ts3 dickson 16780031 6 2 10000 4483130442468001098 idx:1 3 0
  cell dickson-16780031-m2-t6-idx1-ts4 dickson 16780031 6 2 10000 4483130442468001098 idx:1 4 0
  cell dickson-16780031-m2-t6-max-ts1 dickson 16780031 6 2 10000 4483130442468001098 max 1 0
  cell dickson-16780031-m2-t6-max-ts2 dickson 16780031 6 2 10000 4483130442468001098 max 2 0
  cell dickson-16780031-m2-t6-max-ts3 dickson 16780031 6 2 10000 4483130442468001098 max 3 0
  cell dickson-16780031-m2-t6-max-ts4 dickson 16780031 6 2 10000 4483130442468001098 max 4 0
  cell dickson-16780031-m3-t4-idx0-ts1 dickson 16780031 4 3 10000 5479868948465299270 idx:0 1 0
  cell dickson-16780031-m3-t4-idx0-ts2 dickson 16780031 4 3 10000 5479868948465299270 idx:0 2 0
  cell dickson-16780031-m3-t4-idx0-ts3 dickson 16780031 4 3 10000 5479868948465299270 idx:0 3 0
  cell dickson-16780031-m3-t4-idx0-ts4 dickson 16780031 4 3 10000 5479868948465299270 idx:0 4 0
  cell dickson-16780031-m3-t4-idx1-ts1 dickson 16780031 4 3 10000 5479868948465299270 idx:1 1 0
  cell dickson-16780031-m3-t4-idx1-ts2 dickson 16780031 4 3 10000 5479868948465299270 idx:1 2 0
  cell dickson-16780031-m3-t4-idx1-ts3 dickson 16780031 4 3 10000 5479868948465299270 idx:1 3 0
  cell dickson-16780031-m3-t4-idx1-ts4 dickson 16780031 4 3 10000 5479868948465299270 idx:1 4 0
  cell dickson-16780031-m3-t4-max-ts1 dickson 16780031 4 3 10000 5479868948465299270 max 1 0
  cell dickson-16780031-m3-t4-max-ts2 dickson 16780031 4 3 10000 5479868948465299270 max 2 0
  cell dickson-16780031-m3-t4-max-ts3 dickson 16780031 4 3 10000 5479868948465299270 max 3 0
  cell dickson-16780031-m3-t4-max-ts4 dickson 16780031 4 3 10000 5479868948465299270 max 4 0
  ;;
m4)
  step purity-1009-m2-c0 "$B" purity --p 1009 --m 2 --cseed 0
  step purity-1009-m2-c1 "$B" purity --p 1009 --m 2 --cseed 1
  step purity-1009-m2-c2 "$B" purity --p 1009 --m 2 --cseed 2
  step purity-2003-m2-c0 "$B" purity --p 2003 --m 2 --cseed 0
  step purity-2003-m2-c1 "$B" purity --p 2003 --m 2 --cseed 1
  step purity-2003-m2-c2 "$B" purity --p 2003 --m 2 --cseed 2
  step purity-4001-m2-c0 "$B" purity --p 4001 --m 2 --cseed 0
  step purity-4001-m2-c1 "$B" purity --p 4001 --m 2 --cseed 1
  step purity-4001-m2-c2 "$B" purity --p 4001 --m 2 --cseed 2
  step purity-7001-m2-c0 "$B" purity --p 7001 --m 2 --cseed 0
  step purity-7001-m2-c1 "$B" purity --p 7001 --m 2 --cseed 1
  step purity-7001-m2-c2 "$B" purity --p 7001 --m 2 --cseed 2
  step purity-9973-m2-c0 "$B" purity --p 9973 --m 2 --cseed 0
  step purity-9973-m2-c1 "$B" purity --p 9973 --m 2 --cseed 1
  step purity-9973-m2-c2 "$B" purity --p 9973 --m 2 --cseed 2
  step purity-101-m3-c0 "$B" purity --p 101 --m 3 --cseed 0
  step purity-101-m3-c1 "$B" purity --p 101 --m 3 --cseed 1
  step purity-101-m3-c2 "$B" purity --p 101 --m 3 --cseed 2
  step purity-151-m3-c0 "$B" purity --p 151 --m 3 --cseed 0
  step purity-151-m3-c1 "$B" purity --p 151 --m 3 --cseed 1
  step purity-151-m3-c2 "$B" purity --p 151 --m 3 --cseed 2
  step purity-199-m3-c0 "$B" purity --p 199 --m 3 --cseed 0
  step purity-199-m3-c1 "$B" purity --p 199 --m 3 --cseed 1
  step purity-199-m3-c2 "$B" purity --p 199 --m 3 --cseed 2
  ;;
*) echo "REFUSED: unknown stage $STAGE"; exit 2 ;;
esac
echo "STAGE $STAGE calls=$NSTEP nonzero_exit_calls=$BAD cells=$NCELL cells_not_emitted=$EMITFAIL msolve_nonzero=$MSNZ msolve_timeout=$MSTO"
sha256sum "$J"
if [ "$STAGE" = m3 ]; then exit 0; fi
[ "$BAD" -eq 0 ] || exit 1
exit 0
