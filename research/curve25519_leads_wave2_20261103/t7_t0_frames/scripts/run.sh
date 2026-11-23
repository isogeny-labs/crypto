#!/usr/bin/env bash
# T7 (PROTOCOL.md): frame selection for the t0-bits base at k = 20.
# Executes exactly the planned runs, strictly sequentially (one engine process
# at a time), into ../data/.  Phase 1: census (3 x 1000 f4cost calls with
# targets = 0).  Phase 2: selection (scripts/select.py, no engine call).
# Phase 3: 28 engine cells.  Resumable: finished census rows and finished
# engine cells are skipped.  No git, no rebuild, no other writes.
set -u
D=$(cd "$(dirname "$0")/.." && pwd)
S="$D/scripts"; O="$D/data"
BIN=/home/user/ctarget/release/curve25519_family
SHA=7a438b1d8ce9d7a0b8049a4331599727e9574ea4201f9c7957befa3184ba98be
LO_SHA=84dd67a906ad91ab48f4d333c335b3da6d0c540ffa6588a106dcf2086b54a5c5
mkdir -p "$O/engine"

# F1: frozen instrument and frozen lo list, or nothing runs.
got=$(sha256sum "$BIN" | cut -d' ' -f1)
[ "$got" = "$SHA" ] || { echo "binary sha256 $got != $SHA; refusing (F1)" >&2; exit 2; }
got=$(sha256sum "$S/lo_list.txt" | cut -d' ' -f1)
[ "$got" = "$LO_SHA" ] || { echo "lo_list sha256 $got != $LO_SHA; refusing (F1)" >&2; exit 2; }
sha256sum "$BIN" > "$O/binary.sha256"
ulimit -v 3500000
log() { echo "$(date -u +%FT%TZ) $*" >> "$O/run.log"; }
log "start; $(uptime)"

# ---- Phase 1: census.  f4cost 20 t0-bits@LO <t> 3 0 1 3600 2 full 0 30000000
# targets = 0: base build + prepare + exact m = 2 coverage only, no F4 work.
for t in 4 5 6; do
  f="$O/census_t$t.jsonl"; touch "$f"
  while read -r lo; do
    grep -q "\"lo\":$lo," "$f" && continue
    row=$(env -u F4COST_PLANTED timeout 4000 "$BIN" f4cost 20 "t0-bits@$lo" "$t" 3 0 1 3600 2 full 0 30000000 2>>"$O/census_t$t.stderr")
    rc=$?
    if [ $rc -eq 0 ] && [ -n "$row" ]; then echo "$row" >> "$f"
    else echo "t=$t lo=$lo rc=$rc" >> "$O/census_errors.log"; fi
  done < "$S/lo_list.txt"
  log "census t=$t rows=$(wc -l < "$f"); $(uptime)"
done
sha256sum "$O"/census_t*.jsonl > "$O/census.SHA256SUMS"

# ---- Phase 2: selection (frozen rule; P0 anchors; F1/F2 exits).
sel=$(python3 -I "$S/select.py"); rc=$?
echo "$sel" > "$O/selection.txt"
if [ $rc -ne 0 ]; then log "select.py rc=$rc; engine phase not run (F1/F2)"; exit $rc; fi
L4=$(awk '$1=="T4"{print $2}' <<<"$sel")
L5=$(awk '$1=="T5"{print $2}' <<<"$sel")
L6=$(awk '$1=="T6"{print $2}' <<<"$sel")
log "selected L*: t4=$L4 t5=$L5 t6=$L6"

# ---- Phase 3: engine cells.  100 random targets, seed 2026111500 + t (the
# seed of the frame-search, torsion Addendum A and isogeny-seed engine cells,
# so those cells replay exactly), 20 planted targets, cap 3600 s.
export F4COST_PLANTED=20
cell() { # tag form t G
  local c="$O/engine/$1"
  if [ -s "$c.json" ] && grep -q '"targets_done"' "$c.json"; then return; fi
  { echo "start $(date -u +%FT%TZ)"; uptime; } > "$c.status"
  timeout 4000 "$BIN" f4cost 20 "$2" "$3" "$4" 100 $((2026111500 + $3)) 3600 2 full 0 30000000 \
    > "$c.json" 2> "$c.stderr"
  echo "end $(date -u +%FT%TZ) rc=$?" >> "$c.status"; uptime >> "$c.status"
}
# Primary block: t in {4,5}, G in {0,3}; six arms per cell, fixed order.
# Reference frames/seeds frozen from existing files (PROTOCOL.md, Design):
#   bits@797820 (t=4), bits@976503 (t=5); isogeny@595 (t=4), isogeny@1550 (t=5); isogeny@1093.
for t in 4 5; do
  if [ $t -eq 4 ]; then LS=$L4; LB=797820; IS=595; else LS=$L5; LB=976503; IS=1550; fi
  for g in 0 3; do
    cell t0lo0_t${t}_g$g   "t0-bits@0"     $t $g
    cell t0sel_t${t}_g$g   "t0-bits@$LS"   $t $g
    cell bitslo0_t${t}_g$g "bits"          $t $g
    cell bitsbest_t${t}_g$g "bits@$LB"     $t $g
    cell isosel_t${t}_g$g  "isogeny@$IS"   $t $g
    cell iso1093_t${t}_g$g "isogeny@1093"  $t $g
    log "cell t=$t G=$g done; $(uptime)"
  done
done
# Secondary block: t = 6, G = 4 (the isogeny@1093 incumbent cell).
cell t0lo0_t6_g4    "t0-bits@0"    6 4
cell t0sel_t6_g4    "t0-bits@$L6"  6 4
cell bitsbest_t6_g4 "bits@908172"  6 4
cell iso1093_t6_g4  "isogeny@1093" 6 4
log "cell t=6 G=4 done; $(uptime)"
sha256sum "$O"/engine/*.json > "$O/engine/SHA256SUMS"
log "all done"
echo done > "$O/ALL_DONE"
