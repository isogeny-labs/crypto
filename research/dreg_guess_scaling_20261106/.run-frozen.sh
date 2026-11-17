#!/usr/bin/env bash
# Thin orchestration for PREREGISTRATION.md. Every computation is done by
# examples/dreg_ladder and examples/dreg_linear_guess (--engine pe).
#
#   research/dreg_guess_scaling_20261106/run.sh gateA|gen|measure
#
# One process per run; a run with a non-empty output is skipped, and an
# empty earlier attempt is kept under runs/attempts/.
set -u
STEP=${1:-measure}
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
BIN=$REPO/target/release/examples/dreg_linear_guess
LADDER=$REPO/target/release/examples/dreg_ladder
RUNS=$HERE/runs
F=research/dreg_guess_scaling_20261106/draws/m3-25-9.jsonl
mkdir -p "$RUNS/attempts"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-4}
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }
sha256sum "$BIN" "$LADDER" | sed "s#$REPO/##" > "$RUNS/binary.sha256"

one() { # name drawfile args...
  local name=$1 file=$2; shift 2
  local out=$RUNS/$name.jsonl
  [ -s "$out" ] && return 0
  if [ -e "$out" ]; then
    local stamp; stamp=$(date -u +%Y%m%dT%H%M%SZ)
    mv "$out" "$RUNS/attempts/$name.$stamp.jsonl"
    [ -e "$RUNS/$name.log" ] && mv "$RUNS/$name.log" "$RUNS/attempts/$name.$stamp.log"
  fi
  log "$name: start $*"
  local start; start=$(date +%s)
  (cd "$REPO" && prlimit --as=15000000000 timeout 7200 "$BIN" --engine pe --batch 64 \
     --to 4 --max-cols 230000 --check "$@" "$file") > "$out" 2> "$RUNS/$name.log"
  log "$name: exit $? after $(( $(date +%s) - start )) s"
}

saturated_unrefuted() { grep -q '"degree":4,"xl_refuted":false,"closure_refuted":false,"saturated":true' "$1" 2>/dev/null; }
refuted4() { grep -q '"degree":4,"xl_refuted":false,"closure_refuted":true' "$1" 2>/dev/null; }

registered() { grep '"solutions":0' "$REPO/$F" | grep -o '"draw":[0-9]*' | cut -d: -f2 | head -4; }

case $STEP in
gateA)
  one A1 research/dreg_closure_guess_20261022/draws/m3-22-8.jsonl --draw 0 --fix 7=0,15=0,23=0,24=0
  one A2 research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl --draw 0 --sum 15=7+0 --fix 23=0,24=1 ;;
gen)
  (cd "$REPO" && "$LADDER" --cells 25:9:4 --m 3 --ffd-max 3 --seed 20261106 --max-draws 64) \
    > "$HERE/draws/m3-25-9.jsonl" 2> "$HERE/draws/gen.log" ;;
measure)
  for d in $(registered); do
    one "P.d$d" "$F" --draw "$d" --fix 6=0,7=0,8=0,15=0,16=0,17=0,24=0,25=0,26=0
  done
  for d in $(registered); do
    for b in $(seq 0 31); do
      a=$((b & 1)); bb=$((b >> 1 & 1)); c=$((b >> 2 & 1)); dd=$((b >> 3 & 1)); e=$((b >> 4 & 1))
      one "S5.d$d.b$b" "$F" --draw "$d" --sum "17=8+$a,16=7+$bb" --fix "26=$c,27=$dd,28=$e"
      if ! refuted4 "$RUNS/S5.d$d.b$b.jsonl"; then
        log "S5.d$d: stops at b$b (not refuted at degree 4)"; break
      fi
    done
  done ;;
summand)  # PREREG_SUMMAND.md
  for spec in "7:research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl:0 3 5 7" \
              "8:research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl:0 3 5 6" \
              "9:$F:$(registered | tr '\n' ' ')"; do
    ell=${spec%%:*}; rest=${spec#*:}; file=${rest%%:*}; draws=${rest#*:}
    for d in $draws; do
      for h in 0 85 170 255; do
        fix=$(for i in $(seq 0 $((ell - 1))); do printf '%d=%d,' "$i" $(( (h >> i) & 1 )); done)
        one "W.l$ell.d$d.h$h" "$file" --draw "$d" --fix "${fix%,}"
      done
    done
  done ;;
esac
