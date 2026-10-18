#!/usr/bin/env bash
# The (4,6) / (4,7) / degree-5-at-ℓ=7 study (PROTOCOL.md). Thin orchestration:
# every computation is examples/dreg_ladder (draws) and examples/dreg_closure.
#
#   run.sh LADDER_BIN CLOSURE_BIN host|gen|measure|control|deg5 [C2]
set -u
LADDER=$1; CLOSURE=$2; STEP=${3:-measure}
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
DRAWS=$HERE/draws; RUNS=$HERE/runs
mkdir -p "$DRAWS" "$RUNS"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-$(nproc)}
SEED=20261018
FRESH=${FRESH:-"4:15:6 4:16:6 4:17:7 4:18:7"}
SCALE=research/dreg_closure_scale_20261016/draws
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }

cgroup_limit() {
  local v
  v=$(cat /sys/fs/cgroup/memory.max 2>/dev/null || cat /sys/fs/cgroup/memory/memory.limit_in_bytes 2>/dev/null || echo max)
  if [ "$v" = max ] || [ -z "$v" ]; then v=$(( $(awk '/^MemTotal:/ {print $2}' /proc/meminfo) * 1024 )); fi
  echo "$v"
}
LIMIT=$(cgroup_limit)
A=$(( LIMIT * 92 / 100 ))

one() { # name file draw to timeout [extra flags]
  local name=$1 file=$2 k=$3 to=$4 tmo=$5; shift 5
  local out=$RUNS/$name.d$k.jsonl
  if [ -s "$out" ]; then log "$name d$k: done already, skipped"; return; fi
  log "$name d$k: start (to=$to A=$A cols<=1100000)"
  local start; start=$(date +%s)
  (cd "$REPO" && prlimit --as=$A timeout "$tmo" /usr/bin/time -v \
     "$CLOSURE" --to "$to" --max-cols 1100000 --draw "$k" "$@" "$file") > "$out" 2> "$RUNS/$name.d$k.log"
  log "$name d$k: exit $? after $(( $(date +%s) - start )) s"
}

case $STEP in
host)
  { echo "date: $(date -u +%FT%TZ)"; echo "commit: $(git -C "$REPO" rev-parse HEAD)"
    echo "ladder sha256: $(sha256sum "$LADDER" | cut -d' ' -f1)"; echo "closure sha256: $(sha256sum "$CLOSURE" | cut -d' ' -f1)"
    echo "rustc: $(rustc --version)"; echo "cgroup limit bytes: $LIMIT; A bytes: $A"
    echo "MemTotal KiB: $(awk '/^MemTotal:/ {print $2}' /proc/meminfo)"; echo "RAYON_NUM_THREADS: $RAYON_NUM_THREADS"
    echo "pod: ${RUNPOD_POD_ID:-none}"; lscpu; } > "$RUNS/host-$(hostname).txt" ;;
gen)
  for c in $FRESH; do IFS=: read -r m n ell <<< "$c"
    [ -s "$DRAWS/m$m-$n-$ell.jsonl" ] && continue
    log "gen m$m-$n-$ell: start"
    (cd "$REPO" && "$LADDER" --cells "$n:$ell:3" --m "$m" --unsat 4 --ffd-max 3 --max-draws 4096 --seed $SEED) \
      > "$DRAWS/m$m-$n-$ell.jsonl" 2> "$DRAWS/m$m-$n-$ell.log"
    log "gen m$m-$n-$ell: exit $? ($(grep -vc '"satisfiable"' "$DRAWS/m$m-$n-$ell.jsonl") unsat)"
  done; sha256sum "$DRAWS"/*.jsonl > "$DRAWS/sha256.txt" ;;
measure)
  # Budget order (PROTOCOL.md §8): the A cells first, two draws each, the two
  # cells side by side (44 + 59 GB projected); then the remaining A draws the
  # same way. B cells only through `FRESH` including them (not in the budget run).
  TMO4=${TMO4:-14400}
  pair() { # cell1 cell2 draw-index
    local c1=$1 c2=$2 i=$3
    for c in $c1 $c2; do IFS=: read -r m n ell <<< "$c"
      f=research/dreg_closure_m4l6_20261018/draws/m$m-$n-$ell.jsonl
      k=$(grep -v '"satisfiable"' "$REPO/$f" | grep -o '"draw":[0-9]*' | cut -d: -f2 | sed -n "$((i+1))p")
      [ -n "$k" ] && one "fresh-m$m-$n-$ell" "$f" "$k" 4 "$TMO4" &
    done; wait; }
  set -- $FRESH
  for i in 0 1 2 3; do pair "$1" "${2:-}" "$i"; done ;;
control)
  for c in $FRESH; do IFS=: read -r m n ell <<< "$c"
    f=research/dreg_closure_m4l6_20261018/draws/m$m-$n-$ell.jsonl
    for k in $(grep '"satisfiable"' "$REPO/$f" | grep -o '"draw":[0-9]*' | cut -d: -f2 | head -1); do
      one "control-m$m-$n-$ell" "$f" "$k" 4 43200 --satisfiable; done; done ;;
deg5)
  # C1 first draw only unless DEG5_DRAWS says otherwise (budget run).
  TMO5=${TMO5:-144000}; NDRAWS=${DEG5_DRAWS:-4}
  cells="19"; [ "${4:-}" = C2 ] && cells="19 21"
  for n in $cells; do
    if [ "$n" = 19 ] && [ "$LIMIT" -lt 200000000000 ]; then log "C1 (19,7): host limit $LIMIT < 200 GB, not attempted"; continue; fi
    if [ "$n" = 21 ] && [ "$LIMIT" -lt 420000000000 ]; then log "C2 (21,7): host limit $LIMIT < 420 GB, not attempted"; continue; fi
    f=$SCALE/m3-$n-7.jsonl
    for k in $(grep -v '"satisfiable"' "$REPO/$f" | grep -o '"draw":[0-9]*' | cut -d: -f2 | head -n "$NDRAWS"); do
      one "deg5-m3-$n-7" "$f" "$k" 5 "$TMO5"; done; done ;;
esac
