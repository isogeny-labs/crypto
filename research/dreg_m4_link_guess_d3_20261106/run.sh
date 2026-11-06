#!/usr/bin/env bash
# m = 4 per-link invariant guess at degree 3 (PREREGISTRATION.md).  Thin
# orchestration only: every computation is examples/dreg_linear_guess.
#
#   research/dreg_m4_link_guess_d3_20261106/run.sh BIN
#
# Line b of runs/<ARM>.d<draw>.jsonl is branch b.  A rerun resumes after the
# last complete line.  A run that leaves no line stops the script non-zero.
set -u
BIN=$1
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
RUNS=$HERE/runs
mkdir -p "$RUNS/attempts"
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-4}
A=research/dreg_closure_driver_20261016/draws/m4-12-5.jsonl
B=research/dreg_closure_driver_20261016/draws/m4-11-5.jsonl
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }
file_of() { case $1 in 24|26|47|52) echo "$A" ;; *) echo "$B" ;; esac; }

# Identity control N (m = 3, degree 4).
if [ ! -s "$RUNS/N.d5.jsonl" ]; then
  log "N.d5: start"
  (cd "$REPO" && prlimit --as=15000000000 timeout 7200 "$BIN" --to 4 --max-cols 140000 --check \
     --draw 5 --sum 15=7+0 --fix 23=0,24=0 research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl) \
     > "$RUNS/N.d5.jsonl" 2> "$RUNS/N.d5.log" || { log "N.d5: failed"; exit 1; }
  log "N.d5: done"
fi

args_of() { # arm b
  local b=$2; local c=(); for i in 0 1 2 3 4 5 6 7 8 9; do c[$i]=$(( b >> i & 1 )); done
  if [ "$1" = I ]; then
    echo "--sum 9=4+${c[0]},8=3+${c[1]},7=2+${c[2]},6=1+${c[3]} --link 2:top1=${c[4]},2:top2=${c[5]},2:top3=${c[6]},3:top1=${c[7]},3:top2=${c[8]},3:top3=${c[9]}"
  else
    echo "--fix 9=${c[0]},8=${c[1]},7=${c[2]},6=${c[3]},14=${c[4]},13=${c[5]},12=${c[6]},19=${c[7]},18=${c[8]},17=${c[9]}"
  fi
}

for d in 24 26 47 52 8 67 107 143; do
  for arm in I R; do
    out=$RUNS/$arm.d$d.jsonl
    touch "$out"
    # Drop a trailing partial line, keeping it as an attempt.
    if [ -s "$out" ] && [ "$(tail -c1 "$out" | od -An -c | tr -d ' ')" != '\n' ]; then
      stamp=$(date -u +%Y%m%dT%H%M%SZ); cp "$out" "$RUNS/attempts/$arm.d$d.$stamp.jsonl"
      sed -i '$d' "$out"; log "$arm.d$d: partial last line kept as attempts/$arm.d$d.$stamp.jsonl"
    fi
    start=$(wc -l < "$out")
    [ "$start" -ge 1024 ] && continue
    log "$arm.d$d: branches $start..1023"
    for b in $(seq "$start" 1023); do
      # shellcheck disable=SC2046
      line=$(cd "$REPO" && prlimit --as=8000000000 timeout 600 "$BIN" --to 3 --max-cols 140000 --check \
               --draw "$d" $(args_of "$arm" "$b") "$(file_of "$d")" 2>>"$RUNS/$arm.d$d.log")
      if [ -z "$line" ]; then log "$arm.d$d.b$b: no record"; exit 1; fi
      printf '%s\n' "$line" >> "$out"
    done
    log "$arm.d$d: done"
  done
done
