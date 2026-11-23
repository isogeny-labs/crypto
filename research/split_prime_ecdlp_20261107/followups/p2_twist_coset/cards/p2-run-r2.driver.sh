#!/bin/bash
# P2 round 2 frozen run driver (card CARD-p2-twist-coset-20261108-r2, protocol PROTOCOL-R2.md with Addendum A).
# Orchestration only: every line below is one literal instrument call; nothing is computed here.
# Usage: p2-run-r2.driver.sh <stage m3> <run_dir> <p2coset binary>
# Only the M3 stage exists. Targets: --target unplanted, --tseed 101..104 (ids -ts101..-ts104).
# Seeds: first 8 bytes little endian of SHA-256("split_prime_ecdlp_20261107/P2/cosets/<p>/<i>/<t>"),
# curve index i = --cseed i (Addendum A of round 1). The values are written out in every call below.
# Raw outputs: <run_dir>/m3/<id>.out and .err per instrument call, <run_dir>/m3.jsonl one line per call.
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
  step "$id.emit" "$B" emit --kind "$kind" --p "$p" --t "$t" --m "$m" --C "$C" --seed "$seed" --cseed "$cs" --select "$sel" --tseed "$ts" --target unplanted --out "$RUN/$STAGE/$id.ms"
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
m3)
  cell kummer-1049089-m2-t4-idx0-ts101 kummer 1049089 4 2 10000 13609300572028886034 idx:0 101 0
  cell kummer-1049089-m2-t4-idx0-ts102 kummer 1049089 4 2 10000 13609300572028886034 idx:0 102 0
  cell kummer-1049089-m2-t4-idx0-ts103 kummer 1049089 4 2 10000 13609300572028886034 idx:0 103 0
  cell kummer-1049089-m2-t4-idx0-ts104 kummer 1049089 4 2 10000 13609300572028886034 idx:0 104 0
  cell kummer-1049089-m2-t4-idx1-ts101 kummer 1049089 4 2 10000 13609300572028886034 idx:1 101 0
  cell kummer-1049089-m2-t4-idx1-ts102 kummer 1049089 4 2 10000 13609300572028886034 idx:1 102 0
  cell kummer-1049089-m2-t4-idx1-ts103 kummer 1049089 4 2 10000 13609300572028886034 idx:1 103 0
  cell kummer-1049089-m2-t4-idx1-ts104 kummer 1049089 4 2 10000 13609300572028886034 idx:1 104 0
  cell kummer-1049089-m2-t4-max-ts101 kummer 1049089 4 2 10000 13609300572028886034 max 101 0
  cell kummer-1049089-m2-t4-max-ts102 kummer 1049089 4 2 10000 13609300572028886034 max 102 0
  cell kummer-1049089-m2-t4-max-ts103 kummer 1049089 4 2 10000 13609300572028886034 max 103 0
  cell kummer-1049089-m2-t4-max-ts104 kummer 1049089 4 2 10000 13609300572028886034 max 104 0
  cell kummer-1049089-m2-t6-idx0-ts101 kummer 1049089 6 2 10000 16765669404794311197 idx:0 101 0
  cell kummer-1049089-m2-t6-idx0-ts102 kummer 1049089 6 2 10000 16765669404794311197 idx:0 102 0
  cell kummer-1049089-m2-t6-idx0-ts103 kummer 1049089 6 2 10000 16765669404794311197 idx:0 103 0
  cell kummer-1049089-m2-t6-idx0-ts104 kummer 1049089 6 2 10000 16765669404794311197 idx:0 104 0
  cell kummer-1049089-m2-t6-idx1-ts101 kummer 1049089 6 2 10000 16765669404794311197 idx:1 101 0
  cell kummer-1049089-m2-t6-idx1-ts102 kummer 1049089 6 2 10000 16765669404794311197 idx:1 102 0
  cell kummer-1049089-m2-t6-idx1-ts103 kummer 1049089 6 2 10000 16765669404794311197 idx:1 103 0
  cell kummer-1049089-m2-t6-idx1-ts104 kummer 1049089 6 2 10000 16765669404794311197 idx:1 104 0
  cell kummer-1049089-m2-t6-max-ts101 kummer 1049089 6 2 10000 16765669404794311197 max 101 0
  cell kummer-1049089-m2-t6-max-ts102 kummer 1049089 6 2 10000 16765669404794311197 max 102 0
  cell kummer-1049089-m2-t6-max-ts103 kummer 1049089 6 2 10000 16765669404794311197 max 103 0
  cell kummer-1049089-m2-t6-max-ts104 kummer 1049089 6 2 10000 16765669404794311197 max 104 0
  cell kummer-1049089-m3-t4-idx0-ts101 kummer 1049089 4 3 10000 13609300572028886034 idx:0 101 0
  cell kummer-1049089-m3-t4-idx0-ts102 kummer 1049089 4 3 10000 13609300572028886034 idx:0 102 0
  cell kummer-1049089-m3-t4-idx0-ts103 kummer 1049089 4 3 10000 13609300572028886034 idx:0 103 0
  cell kummer-1049089-m3-t4-idx0-ts104 kummer 1049089 4 3 10000 13609300572028886034 idx:0 104 0
  cell kummer-1049089-m3-t4-idx1-ts101 kummer 1049089 4 3 10000 13609300572028886034 idx:1 101 0
  cell kummer-1049089-m3-t4-idx1-ts102 kummer 1049089 4 3 10000 13609300572028886034 idx:1 102 0
  cell kummer-1049089-m3-t4-idx1-ts103 kummer 1049089 4 3 10000 13609300572028886034 idx:1 103 0
  cell kummer-1049089-m3-t4-idx1-ts104 kummer 1049089 4 3 10000 13609300572028886034 idx:1 104 0
  cell kummer-1049089-m3-t4-max-ts101 kummer 1049089 4 3 10000 13609300572028886034 max 101 0
  cell kummer-1049089-m3-t4-max-ts102 kummer 1049089 4 3 10000 13609300572028886034 max 102 0
  cell kummer-1049089-m3-t4-max-ts103 kummer 1049089 4 3 10000 13609300572028886034 max 103 0
  cell kummer-1049089-m3-t4-max-ts104 kummer 1049089 4 3 10000 13609300572028886034 max 104 0
  cell kummer-16777729-m2-t4-idx0-ts101 kummer 16777729 4 2 10000 11173020513567249145 idx:0 101 0
  cell kummer-16777729-m2-t4-idx0-ts102 kummer 16777729 4 2 10000 11173020513567249145 idx:0 102 0
  cell kummer-16777729-m2-t4-idx0-ts103 kummer 16777729 4 2 10000 11173020513567249145 idx:0 103 0
  cell kummer-16777729-m2-t4-idx0-ts104 kummer 16777729 4 2 10000 11173020513567249145 idx:0 104 0
  cell kummer-16777729-m2-t4-idx1-ts101 kummer 16777729 4 2 10000 11173020513567249145 idx:1 101 0
  cell kummer-16777729-m2-t4-idx1-ts102 kummer 16777729 4 2 10000 11173020513567249145 idx:1 102 0
  cell kummer-16777729-m2-t4-idx1-ts103 kummer 16777729 4 2 10000 11173020513567249145 idx:1 103 0
  cell kummer-16777729-m2-t4-idx1-ts104 kummer 16777729 4 2 10000 11173020513567249145 idx:1 104 0
  cell kummer-16777729-m2-t4-max-ts101 kummer 16777729 4 2 10000 11173020513567249145 max 101 0
  cell kummer-16777729-m2-t4-max-ts102 kummer 16777729 4 2 10000 11173020513567249145 max 102 0
  cell kummer-16777729-m2-t4-max-ts103 kummer 16777729 4 2 10000 11173020513567249145 max 103 0
  cell kummer-16777729-m2-t4-max-ts104 kummer 16777729 4 2 10000 11173020513567249145 max 104 0
  cell kummer-16777729-m2-t6-idx0-ts101 kummer 16777729 6 2 10000 1320760113827526238 idx:0 101 0
  cell kummer-16777729-m2-t6-idx0-ts102 kummer 16777729 6 2 10000 1320760113827526238 idx:0 102 0
  cell kummer-16777729-m2-t6-idx0-ts103 kummer 16777729 6 2 10000 1320760113827526238 idx:0 103 0
  cell kummer-16777729-m2-t6-idx0-ts104 kummer 16777729 6 2 10000 1320760113827526238 idx:0 104 0
  cell kummer-16777729-m2-t6-idx1-ts101 kummer 16777729 6 2 10000 1320760113827526238 idx:1 101 0
  cell kummer-16777729-m2-t6-idx1-ts102 kummer 16777729 6 2 10000 1320760113827526238 idx:1 102 0
  cell kummer-16777729-m2-t6-idx1-ts103 kummer 16777729 6 2 10000 1320760113827526238 idx:1 103 0
  cell kummer-16777729-m2-t6-idx1-ts104 kummer 16777729 6 2 10000 1320760113827526238 idx:1 104 0
  cell kummer-16777729-m2-t6-max-ts101 kummer 16777729 6 2 10000 1320760113827526238 max 101 0
  cell kummer-16777729-m2-t6-max-ts102 kummer 16777729 6 2 10000 1320760113827526238 max 102 0
  cell kummer-16777729-m2-t6-max-ts103 kummer 16777729 6 2 10000 1320760113827526238 max 103 0
  cell kummer-16777729-m2-t6-max-ts104 kummer 16777729 6 2 10000 1320760113827526238 max 104 0
  cell kummer-16777729-m3-t4-idx0-ts101 kummer 16777729 4 3 10000 11173020513567249145 idx:0 101 0
  cell kummer-16777729-m3-t4-idx0-ts102 kummer 16777729 4 3 10000 11173020513567249145 idx:0 102 0
  cell kummer-16777729-m3-t4-idx0-ts103 kummer 16777729 4 3 10000 11173020513567249145 idx:0 103 0
  cell kummer-16777729-m3-t4-idx0-ts104 kummer 16777729 4 3 10000 11173020513567249145 idx:0 104 0
  cell kummer-16777729-m3-t4-idx1-ts101 kummer 16777729 4 3 10000 11173020513567249145 idx:1 101 0
  cell kummer-16777729-m3-t4-idx1-ts102 kummer 16777729 4 3 10000 11173020513567249145 idx:1 102 0
  cell kummer-16777729-m3-t4-idx1-ts103 kummer 16777729 4 3 10000 11173020513567249145 idx:1 103 0
  cell kummer-16777729-m3-t4-idx1-ts104 kummer 16777729 4 3 10000 11173020513567249145 idx:1 104 0
  cell kummer-16777729-m3-t4-max-ts101 kummer 16777729 4 3 10000 11173020513567249145 max 101 0
  cell kummer-16777729-m3-t4-max-ts102 kummer 16777729 4 3 10000 11173020513567249145 max 102 0
  cell kummer-16777729-m3-t4-max-ts103 kummer 16777729 4 3 10000 11173020513567249145 max 103 0
  cell kummer-16777729-m3-t4-max-ts104 kummer 16777729 4 3 10000 11173020513567249145 max 104 0
  cell dickson-1049599-m2-t4-idx0-ts101 dickson 1049599 4 2 10000 3155981005055438761 idx:0 101 0
  cell dickson-1049599-m2-t4-idx0-ts102 dickson 1049599 4 2 10000 3155981005055438761 idx:0 102 0
  cell dickson-1049599-m2-t4-idx0-ts103 dickson 1049599 4 2 10000 3155981005055438761 idx:0 103 0
  cell dickson-1049599-m2-t4-idx0-ts104 dickson 1049599 4 2 10000 3155981005055438761 idx:0 104 0
  cell dickson-1049599-m2-t4-idx1-ts101 dickson 1049599 4 2 10000 3155981005055438761 idx:1 101 0
  cell dickson-1049599-m2-t4-idx1-ts102 dickson 1049599 4 2 10000 3155981005055438761 idx:1 102 0
  cell dickson-1049599-m2-t4-idx1-ts103 dickson 1049599 4 2 10000 3155981005055438761 idx:1 103 0
  cell dickson-1049599-m2-t4-idx1-ts104 dickson 1049599 4 2 10000 3155981005055438761 idx:1 104 0
  cell dickson-1049599-m2-t4-max-ts101 dickson 1049599 4 2 10000 3155981005055438761 max 101 0
  cell dickson-1049599-m2-t4-max-ts102 dickson 1049599 4 2 10000 3155981005055438761 max 102 0
  cell dickson-1049599-m2-t4-max-ts103 dickson 1049599 4 2 10000 3155981005055438761 max 103 0
  cell dickson-1049599-m2-t4-max-ts104 dickson 1049599 4 2 10000 3155981005055438761 max 104 0
  cell dickson-1049599-m2-t6-idx0-ts101 dickson 1049599 6 2 8199 2118746520032967994 idx:0 101 0
  cell dickson-1049599-m2-t6-idx0-ts102 dickson 1049599 6 2 8199 2118746520032967994 idx:0 102 0
  cell dickson-1049599-m2-t6-idx0-ts103 dickson 1049599 6 2 8199 2118746520032967994 idx:0 103 0
  cell dickson-1049599-m2-t6-idx0-ts104 dickson 1049599 6 2 8199 2118746520032967994 idx:0 104 0
  cell dickson-1049599-m2-t6-idx1-ts101 dickson 1049599 6 2 8199 2118746520032967994 idx:1 101 0
  cell dickson-1049599-m2-t6-idx1-ts102 dickson 1049599 6 2 8199 2118746520032967994 idx:1 102 0
  cell dickson-1049599-m2-t6-idx1-ts103 dickson 1049599 6 2 8199 2118746520032967994 idx:1 103 0
  cell dickson-1049599-m2-t6-idx1-ts104 dickson 1049599 6 2 8199 2118746520032967994 idx:1 104 0
  cell dickson-1049599-m2-t6-max-ts101 dickson 1049599 6 2 8199 2118746520032967994 max 101 0
  cell dickson-1049599-m2-t6-max-ts102 dickson 1049599 6 2 8199 2118746520032967994 max 102 0
  cell dickson-1049599-m2-t6-max-ts103 dickson 1049599 6 2 8199 2118746520032967994 max 103 0
  cell dickson-1049599-m2-t6-max-ts104 dickson 1049599 6 2 8199 2118746520032967994 max 104 0
  cell dickson-1049599-m3-t4-idx0-ts101 dickson 1049599 4 3 10000 3155981005055438761 idx:0 101 0
  cell dickson-1049599-m3-t4-idx0-ts102 dickson 1049599 4 3 10000 3155981005055438761 idx:0 102 0
  cell dickson-1049599-m3-t4-idx0-ts103 dickson 1049599 4 3 10000 3155981005055438761 idx:0 103 0
  cell dickson-1049599-m3-t4-idx0-ts104 dickson 1049599 4 3 10000 3155981005055438761 idx:0 104 0
  cell dickson-1049599-m3-t4-idx1-ts101 dickson 1049599 4 3 10000 3155981005055438761 idx:1 101 0
  cell dickson-1049599-m3-t4-idx1-ts102 dickson 1049599 4 3 10000 3155981005055438761 idx:1 102 0
  cell dickson-1049599-m3-t4-idx1-ts103 dickson 1049599 4 3 10000 3155981005055438761 idx:1 103 0
  cell dickson-1049599-m3-t4-idx1-ts104 dickson 1049599 4 3 10000 3155981005055438761 idx:1 104 0
  cell dickson-1049599-m3-t4-max-ts101 dickson 1049599 4 3 10000 3155981005055438761 max 101 0
  cell dickson-1049599-m3-t4-max-ts102 dickson 1049599 4 3 10000 3155981005055438761 max 102 0
  cell dickson-1049599-m3-t4-max-ts103 dickson 1049599 4 3 10000 3155981005055438761 max 103 0
  cell dickson-1049599-m3-t4-max-ts104 dickson 1049599 4 3 10000 3155981005055438761 max 104 0
  cell dickson-16780031-m2-t4-idx0-ts101 dickson 16780031 4 2 10000 5479868948465299270 idx:0 101 0
  cell dickson-16780031-m2-t4-idx0-ts102 dickson 16780031 4 2 10000 5479868948465299270 idx:0 102 0
  cell dickson-16780031-m2-t4-idx0-ts103 dickson 16780031 4 2 10000 5479868948465299270 idx:0 103 0
  cell dickson-16780031-m2-t4-idx0-ts104 dickson 16780031 4 2 10000 5479868948465299270 idx:0 104 0
  cell dickson-16780031-m2-t4-idx1-ts101 dickson 16780031 4 2 10000 5479868948465299270 idx:1 101 0
  cell dickson-16780031-m2-t4-idx1-ts102 dickson 16780031 4 2 10000 5479868948465299270 idx:1 102 0
  cell dickson-16780031-m2-t4-idx1-ts103 dickson 16780031 4 2 10000 5479868948465299270 idx:1 103 0
  cell dickson-16780031-m2-t4-idx1-ts104 dickson 16780031 4 2 10000 5479868948465299270 idx:1 104 0
  cell dickson-16780031-m2-t4-max-ts101 dickson 16780031 4 2 10000 5479868948465299270 max 101 0
  cell dickson-16780031-m2-t4-max-ts102 dickson 16780031 4 2 10000 5479868948465299270 max 102 0
  cell dickson-16780031-m2-t4-max-ts103 dickson 16780031 4 2 10000 5479868948465299270 max 103 0
  cell dickson-16780031-m2-t4-max-ts104 dickson 16780031 4 2 10000 5479868948465299270 max 104 0
  cell dickson-16780031-m2-t6-idx0-ts101 dickson 16780031 6 2 10000 4483130442468001098 idx:0 101 0
  cell dickson-16780031-m2-t6-idx0-ts102 dickson 16780031 6 2 10000 4483130442468001098 idx:0 102 0
  cell dickson-16780031-m2-t6-idx0-ts103 dickson 16780031 6 2 10000 4483130442468001098 idx:0 103 0
  cell dickson-16780031-m2-t6-idx0-ts104 dickson 16780031 6 2 10000 4483130442468001098 idx:0 104 0
  cell dickson-16780031-m2-t6-idx1-ts101 dickson 16780031 6 2 10000 4483130442468001098 idx:1 101 0
  cell dickson-16780031-m2-t6-idx1-ts102 dickson 16780031 6 2 10000 4483130442468001098 idx:1 102 0
  cell dickson-16780031-m2-t6-idx1-ts103 dickson 16780031 6 2 10000 4483130442468001098 idx:1 103 0
  cell dickson-16780031-m2-t6-idx1-ts104 dickson 16780031 6 2 10000 4483130442468001098 idx:1 104 0
  cell dickson-16780031-m2-t6-max-ts101 dickson 16780031 6 2 10000 4483130442468001098 max 101 0
  cell dickson-16780031-m2-t6-max-ts102 dickson 16780031 6 2 10000 4483130442468001098 max 102 0
  cell dickson-16780031-m2-t6-max-ts103 dickson 16780031 6 2 10000 4483130442468001098 max 103 0
  cell dickson-16780031-m2-t6-max-ts104 dickson 16780031 6 2 10000 4483130442468001098 max 104 0
  cell dickson-16780031-m3-t4-idx0-ts101 dickson 16780031 4 3 10000 5479868948465299270 idx:0 101 0
  cell dickson-16780031-m3-t4-idx0-ts102 dickson 16780031 4 3 10000 5479868948465299270 idx:0 102 0
  cell dickson-16780031-m3-t4-idx0-ts103 dickson 16780031 4 3 10000 5479868948465299270 idx:0 103 0
  cell dickson-16780031-m3-t4-idx0-ts104 dickson 16780031 4 3 10000 5479868948465299270 idx:0 104 0
  cell dickson-16780031-m3-t4-idx1-ts101 dickson 16780031 4 3 10000 5479868948465299270 idx:1 101 0
  cell dickson-16780031-m3-t4-idx1-ts102 dickson 16780031 4 3 10000 5479868948465299270 idx:1 102 0
  cell dickson-16780031-m3-t4-idx1-ts103 dickson 16780031 4 3 10000 5479868948465299270 idx:1 103 0
  cell dickson-16780031-m3-t4-idx1-ts104 dickson 16780031 4 3 10000 5479868948465299270 idx:1 104 0
  cell dickson-16780031-m3-t4-max-ts101 dickson 16780031 4 3 10000 5479868948465299270 max 101 0
  cell dickson-16780031-m3-t4-max-ts102 dickson 16780031 4 3 10000 5479868948465299270 max 102 0
  cell dickson-16780031-m3-t4-max-ts103 dickson 16780031 4 3 10000 5479868948465299270 max 103 0
  cell dickson-16780031-m3-t4-max-ts104 dickson 16780031 4 3 10000 5479868948465299270 max 104 0
  ;;
*) echo "REFUSED: unknown stage $STAGE"; exit 2 ;;
esac
echo "STAGE $STAGE calls=$NSTEP nonzero_exit_calls=$BAD cells=$NCELL cells_not_emitted=$EMITFAIL msolve_nonzero=$MSNZ msolve_timeout=$MSTO"
sha256sum "$J"
if [ "$STAGE" = m3 ]; then exit 0; fi
[ "$BAD" -eq 0 ] || exit 1
exit 0
