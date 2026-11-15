#!/usr/bin/env bash
# E32 (note §9.34): the handover ladder.  Restart-safe runner (replaces the
# first-session run_ladder.py, same logic: AGENTS.md forbids Python orchestration).
#
# Ascending p, three curves per size (seeds 1-3; a seed without a curve is
# replaced by the next unused seed), each curve `e32_ladder ic` (arm B, solver
# chain, Karatsuba) then `e32_ladder walk` (E27 walk (iii)).  Only the pinned
# binaries in $E32BIN run.  At most 3 cores: taskset -c 1-3, RAYON_NUM_THREADS=4
# (fixes the pipeline's target stream, §9.19).
#
# Budget (frozen in §9.34): 14 h wall for all runs (seconds kept in spent_s);
# an ic run still going after 4 h is stopped and marked censored; a size is not
# started if the budget left is under 3x the previous size's median curve time
# x p_next/p_prev; p = 4337 only if >= 3 h remain and >= 8 GB are available.
set -u
cd "$(dirname "$0")/../.."
OUT=experiments/36_handover_ladder
E32BIN=/tmp/claude-0/-home-user/1f75bcdd-3a63-5794-929b-19635cbd5274/scratchpad/e32bin
LOG=$OUT/run.log
SPENT=$OUT/spent_s
BUDGET=$((14 * 3600))
CAP=$((4 * 3600))
export RAYON_NUM_THREADS=4
[ -f "$SPENT" ] || echo 0 >"$SPENT"

log() { echo "$(date -u +%FT%TZ) $*" >>"$LOG"; }
spent() { cat "$SPENT"; }
add_spent() { echo $(($(spent) + $1)) >"$SPENT"; }

# run_one <out.json> <timeout> <args...>: prints ok | censored | nocurve | fail
run_one() {
  local out=$1 to=$2
  shift 2
  log "run taskset -c 1-3 $E32BIN/e32_ladder $* --json $out.part"
  local t0=$(date +%s)
  timeout "$to" taskset -c 1-3 "$E32BIN/e32_ladder" "$@" --json "$out.part" >>"$LOG" 2>&1
  local rc=$?
  local dt=$(($(date +%s) - t0))
  add_spent "$dt"
  echo "$dt" >"$out.secs"
  if [ $rc -eq 0 ]; then
    mv "$out.part" "$out"
    echo ok
  elif [ $rc -eq 124 ]; then
    touch "${out%.json}.censored"
    echo censored
  elif tail -c 3000 "$LOG" | grep -q -e "no full-2-torsion" -e "E25's curve"; then
    touch "${out%.json}.nocurve"
    echo nocurve
  else
    echo fail
  fi
}

# do_curve p seed: prints ok | censored | nocurve | fail
do_curve() {
  local p=$1 s=$2 key="p$1s$2"
  local ic=$OUT/ic_$key.json walk=$OUT/walk_$key.json rr=1
  case "$p" in 101 | 149 | 197 | 269 | 397) [ "$s" -le 2 ] && rr=16 ;; esac
  [ -e "$OUT/ic_$key.censored" ] && { echo censored; return; }
  [ -e "$OUT/ic_$key.nocurve" ] && { echo nocurve; return; }
  if [ ! -s "$ic" ]; then
    local st
    st=$(run_one "$ic" "$CAP" ic --p "$p" --seed "$s" --rho-runs "$rr" --max-residuals 40000000)
    [ "$st" = ok ] || { echo "$st"; return; }
  fi
  if [ ! -s "$walk" ]; then
    local st
    st=$(run_one "$walk" "$CAP" walk --p "$p" --seed "$s" --walks 16 --pool 16)
    [ "$st" = ok ] || { echo "walk-$st"; return; }
  fi
  echo ok
}

secs_of() { # total seconds of a finished curve
  local k=$1 t=0 f
  for f in "$OUT/ic_$k.json.secs" "$OUT/walk_$k.json.secs"; do [ -f "$f" ] && t=$((t + $(cat "$f"))); done
  echo "$t"
}

prev_p=0
prev_med=0
for p in 101 149 197 269 397 557 773 1093 1549 2213 3089 4337; do
  left=$((BUDGET - $(spent)))
  if [ "$p" = 4337 ]; then
    mem=$(awk '/MemAvailable/ {print int($2/1048576)}' /proc/meminfo)
    if [ "$left" -lt $((3 * 3600)) ] || [ "$mem" -lt 8 ]; then
      log "stop: p=$p not run (budget left ${left} s, MemAvailable ${mem} GB)"
      break
    fi
  elif [ "$prev_p" -gt 0 ]; then
    need=$((3 * prev_med * p / prev_p))
    if [ ! -s "$OUT/walk_p${p}s3.json" ] && [ "$left" -lt "$need" ]; then
      log "stop: p=$p not started (budget left ${left} s < projected ${need} s)"
      break
    fi
  fi
  got=0
  s=1
  times=()
  while [ $got -lt 3 ] && [ $s -le 12 ]; do
    st=$(do_curve "$p" "$s")
    case "$st" in
      ok) got=$((got + 1)); times+=("$(secs_of "p${p}s$s")") ;;
      censored) got=$((got + 1)); log "p=$p seed=$s censored (cap ${CAP} s)" ;;
      nocurve) log "p=$p seed=$s: no curve; replaced by the next seed" ;;
      *) log "p=$p seed=$s: $st; stopping for inspection"; exit 2 ;;
    esac
    s=$((s + 1))
  done
  if [ ${#times[@]} -gt 0 ]; then
    prev_med=$(printf '%s\n' "${times[@]}" | sort -n | awk '{a[NR]=$1} END {print a[int((NR+1)/2)]}')
    prev_p=$p
  fi
  log "size p=$p done ($got curves); spent $(spent) s"
done
log "finished"
