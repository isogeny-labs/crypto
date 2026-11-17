#!/usr/bin/env bash
# Guess-bit study (PREREGISTRATION.md).  Thin orchestration only: every
# computation is examples/dreg_linear_guess, examples/dreg_m4_link_score and
# examples/dreg_m4_guess_bits.
#
#   research/dreg_m4_guess_bits_20261106/run.sh BIN SCORER BITS
#
# Line b (phase 1) or e (phase 2) of a runs/ file is that branch.  A rerun
# resumes after the last complete line.  Any run that leaves no line stops it.
set -u
BIN=$1; SCORER=$2; BITS=$3
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
RUNS=$HERE/runs
mkdir -p "$RUNS/attempts"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-1}
A=research/dreg_closure_driver_20261016/draws/m4-12-5.jsonl
B=research/dreg_closure_driver_20261016/draws/m4-11-5.jsonl
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }
file_of() { case $1 in 24|26|47|52) echo "$A" ;; *) echo "$B" ;; esac; }
RAW=(9 8 7 6 14 13 12 19 18 17)
SUM=("9=4" "8=3" "7=2" "6=1")
LINK=("2:top1" "2:top2" "2:top3" "3:top1" "3:top2" "3:top3")

# args for arm, guess set (space-separated indices), values (bit j of x).
args_of() {
  local arm=$1 set=$2 x=$3 j=0 fix="" sum="" link="" v
  for i in $set; do
    v=$(( x >> j & 1 )); j=$((j + 1))
    if [ "$arm" = R ]; then fix="$fix,${RAW[$i]}=$v"
    elif [ "$i" -lt 4 ]; then sum="$sum,${SUM[$i]}+$v"
    else link="$link,${LINK[$((i - 4))]}=$v"; fi
  done
  [ -n "$fix" ] && printf -- '--fix %s ' "${fix#,}"
  [ -n "$sum" ] && printf -- '--sum %s ' "${sum#,}"
  [ -n "$link" ] && printf -- '--link %s ' "${link#,}"
}

run_file() { # out draw set count
  local out=$1 d=$2 set=$3 count=$4 arm=$5
  touch "$out"
  if [ -s "$out" ] && [ "$(tail -c1 "$out" | od -An -c | tr -d ' ')" != '\n' ]; then
    local stamp; stamp=$(date -u +%Y%m%dT%H%M%SZ); cp "$out" "$RUNS/attempts/$(basename "$out" .jsonl).$stamp.jsonl"
    sed -i '$d' "$out"; log "$(basename "$out"): partial last line kept as an attempt"
  fi
  local start; start=$(wc -l < "$out")
  [ "$start" -ge "$count" ] && return 0
  log "$(basename "$out"): lines $start..$((count - 1))"
  for x in $(seq "$start" $((count - 1))); do
    # shellcheck disable=SC2046
    line=$(cd "$REPO" && prlimit --as=4000000000 timeout 600 "$BIN" --to 3 --max-cols 140000 --check \
             --draw "$d" $(args_of "$arm" "$set" "$x") "$(file_of "$d")" 2>>"${out%.jsonl}.log")
    if [ -z "$line" ]; then log "$(basename "$out") line $x: no record"; exit 1; fi
    printf '%s\n' "$line" >> "$out"
  done
}

# Phase 1.
for d in 24 26 47 52 8 67 107 143; do
  for arm in I R; do run_file "$RUNS/$arm.d$d.jsonl" "$d" "0 1 2 3 4 5 6 7 8 9" 1024 "$arm"; done
done
log "phase 1 done"

# V1, V2.
(cd "$REPO" && "$SCORER" --study dreg_m4_guess_bits_20261106 --control-from dreg_two_guess_20261106 research) \
  > "$HERE/phase1-score.txt" || { log "V1 fails"; exit 1; }
if diff <(grep '^draw ' "$HERE/phase1-score.txt") \
        <(grep '^draw ' "$REPO/research/dreg_m4_link_guess_d3_20261106/score-output.txt") > /dev/null; then
  log "V2: per-draw counts reproduce #3571"
else
  log "V2 fails: per-draw counts differ from #3571"; exit 1
fi

# Phase 2.
(cd "$REPO" && "$BITS" analyse research) > "$HERE/essential.jsonl" || { log "analyse fails"; exit 1; }
while read -r row; do
  arm=$(jq -r .arm <<<"$row"); d=$(jq -r .draw <<<"$row")
  set=$(jq -r '.essential | map(tostring) | join(" ")' <<<"$row")
  size=$(jq -r '.essential | length' <<<"$row")
  if [ "$size" -ge 1 ] && [ "$size" -le 9 ]; then
    run_file "$RUNS/P2.$arm.d$d.jsonl" "$d" "$set" $(( 1 << size )) "$arm"
  fi
done < "$HERE/essential.jsonl"
log "phase 2 done"
