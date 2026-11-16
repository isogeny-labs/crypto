#!/usr/bin/env bash
# The (4,6) / (4,7) / degree-5-at-ℓ=7 study (PROTOCOL.md). Thin orchestration:
# every computation is examples/dreg_ladder (draws) and examples/dreg_closure.
#
#   run.sh LADDER_BIN CLOSURE_BIN host|gen|measure|control|deg5 [C2]
#   run.sh LADDER_BIN CLOSURE_BIN gen-a3|measure-a3|tally-a3      (PROTOCOL.md §11, amendment A3)
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
  local mt; mt=$(( $(awk '/^MemTotal:/ {print $2}' /proc/meminfo) * 1024 ))
  # "max" (v2), empty, or v1's 2^63 "unlimited" all mean no limit (A3: the
  # last overflowed A before; it never occurred on the pods, which set one).
  if [ "$v" = max ] || [ -z "$v" ] || [ "$v" -gt "$mt" ]; then v=$mt; fi
  echo "$v"
}
LIMIT=$(cgroup_limit)
A=$(( LIMIT * 92 / 100 ))

oomkills() { awk '/^oom_kill / {print $2; f=1} END {if (!f) print 0}' /sys/fs/cgroup/memory.events 2>/dev/null || echo 0; }

one() { # name file draw to timeout [extra flags]
  local name=$1 file=$2 k=$3 to=$4 tmo=$5; shift 5
  local out=$RUNS/$name.d$k.jsonl
  if [ -s "$out" ]; then log "$name d$k: done already, skipped"; return; fi
  log "$name d$k: start (to=$to A=$A cols<=1100000)"
  local start; start=$(date +%s)
  # A3 §11.8: the record is written to .part and kept only once the closure has
  # ended on its own (or by its timeout, exit 124). Exit 130/143/137 without the
  # timeout means the host stopped it (Spot loss, job cancel); the .part is
  # dropped and a resumed run does that closure again from the start. A 137
  # with a new cgroup oom_kill is the memory outcome and is recorded.
  local rc=0 oom0; oom0=$(oomkills)
  (cd "$REPO" && prlimit --as=$A timeout "$tmo" /usr/bin/time -v \
     "$CLOSURE" --to "$to" --max-cols 1100000 --draw "$k" "$@" "$file") > "$out.part" 2> "$RUNS/$name.d$k.log" || rc=$?
  case $rc in
    137) if [ "$(oomkills)" -gt "$oom0" ]; then :; else
           log "$name d$k: interrupted (exit $rc) after $(( $(date +%s) - start )) s, not recorded"; rm -f "$out.part"; return; fi ;;
    130|143) log "$name d$k: interrupted (exit $rc) after $(( $(date +%s) - start )) s, not recorded"; rm -f "$out.part"; return ;;
  esac
  mv "$out.part" "$out"
  log "$name d$k: exit $rc after $(( $(date +%s) - start )) s"
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
  # DEG5_MIN_LIMIT: the smallest cgroup limit C1 runs on (A3, §11.3: 280 GB).
  TMO5=${TMO5:-144000}; NDRAWS=${DEG5_DRAWS:-4}; MIN5=${DEG5_MIN_LIMIT:-200000000000}
  cells="19"; [ "${4:-}" = C2 ] && cells="19 21"
  for n in $cells; do
    if [ "$n" = 19 ] && [ "$LIMIT" -lt "$MIN5" ]; then log "C1 (19,7): host limit $LIMIT < $MIN5, not attempted"; continue; fi
    if [ "$n" = 21 ] && [ "$LIMIT" -lt 420000000000 ]; then log "C2 (21,7): host limit $LIMIT < 420 GB, not attempted"; continue; fi
    f=$SCALE/m3-$n-7.jsonl
    for k in $(grep -v '"satisfiable"' "$REPO/$f" | grep -o '"draw":[0-9]*' | cut -d: -f2 | head -n "$NDRAWS"); do
      one "deg5-m3-$n-7" "$f" "$k" 5 "$TMO5"; done; done ;;
gen-a3)
  # A3 (§11.2): the fixed fresh range [A3_FIRST, A3_END) of the committed
  # stream, in GEN_JOBS shard processes. First the replay control: draws
  # [0, A3_REPLAY) regenerated the same way must equal the committed lines
  # (all fields but secs); a mismatch stops here and nothing fresh is made.
  IFS=: read -r m n ell <<< "${A3_CELL:-4:15:6}"
  J=${GEN_JOBS:-$(nproc)}; F=${A3_FIRST:-251}; E=${A3_END:-3251}; R=${A3_REPLAY:-$J}
  committed=$DRAWS/m$m-$n-$ell.jsonl; fresh=$DRAWS/a3-m$m-$n-$ell.jsonl; P=$DRAWS/a3-shards; mkdir -p "$P"
  nosecs() { sed -E 's/"secs":[0-9.]+,?//'; }
  bydraw() { sed -E 's/^(.*"draw":([0-9]+)[,}].*)$/\2\t\1/' | sort -n -k1,1 | cut -f2-; }
  shards() { # tag first end
    local i; for i in $(seq 0 $((J - 1))); do
      (cd "$REPO" && "$LADDER" --cells "$n:$ell:3" --m "$m" --ffd-max 3 --seed $SEED --shard "$i/$J" --first-draw "$2" --max-draws "$3") \
        > "$P/$1.s$i.jsonl" 2> "$P/$1.s$i.log" &
    done; wait; cat "$P/$1".s*.jsonl | bydraw; }
  if [ -s "$fresh" ]; then log "gen-a3: $fresh exists, skipped"; exit 0; fi
  log "gen-a3 replay control: draws 0..$((R - 1)) in $J shards"
  shards replay 0 "$R" > "$P/replay.jsonl"
  if ! cmp -s <(nosecs < "$P/replay.jsonl") <(head -n "$R" "$committed" | nosecs); then
    log "gen-a3 REPLAY_MISMATCH: shards do not reproduce $committed; no fresh draws made"; exit 3; fi
  log "gen-a3 replay control: $R/$R lines identical to the committed file"
  log "gen-a3 fresh: draws $F..$((E - 1)) in $J shards"
  shards fresh "$F" "$E" > "$fresh.part"
  got=$(grep -c . "$fresh.part"); [ "$got" = $((E - F)) ] || { log "gen-a3: $got lines, expected $((E - F))"; exit 4; }
  mv "$fresh.part" "$fresh"
  log "gen-a3 fresh: $got draws, $(grep -vc '"satisfiable"' "$fresh") unsatisfiable, sha256 $(sha256sum "$fresh" | cut -d' ' -f1)"
  (cd "$DRAWS" && sha256sum a3-*.jsonl > a3-sha256.txt) ;;
measure-a3)
  # A3 (§11.2): the first A3_UNSAT unsatisfiable fresh draws in draw order,
  # plus the first satisfiable fresh draw as the control, at degree 4, at
  # most A3_PAR closures at once, each under A / A3_PAR bytes of address space.
  IFS=: read -r m n ell <<< "${A3_CELL:-4:15:6}"
  f=research/dreg_closure_m4l6_20261018/draws/a3-m$m-$n-$ell.jsonl
  (cd "$DRAWS" && sha256sum -c --quiet a3-sha256.txt) || { log "measure-a3: draws do not match a3-sha256.txt"; exit 3; }
  PAR=${A3_PAR:-$(( A / 39000000000 ))}; [ "$PAR" -ge 1 ] || PAR=1
  A=$(( A / PAR )); TMO4=${TMO4:-14400}
  log "measure-a3: $PAR at once, A=$A per process"
  jobs_=(); for k in $(grep -v '"satisfiable"' "$REPO/$f" | grep -o '"draw":[0-9]*' | cut -d: -f2 | head -n "${A3_UNSAT:-16}"); do jobs_+=("u $k"); done
  k=$(grep '"satisfiable"' "$REPO/$f" | grep -o '"draw":[0-9]*' | cut -d: -f2 | head -1); [ -n "$k" ] && jobs_+=("c $k")
  for jb in "${jobs_[@]}"; do set -- $jb
    while [ "$(jobs -rp | wc -l)" -ge "$PAR" ]; do wait -n; done
    if [ "$1" = u ]; then one "a3-m$m-$n-$ell" "$f" "$2" 4 "$TMO4" &
    else one "a3-control-m$m-$n-$ell" "$f" "$2" 4 "$TMO4" --satisfiable & fi
    sleep 5
  done; wait ;;
tally-a3)
  # Mechanical count for §11.4; the reading is written by hand from it.
  IFS=: read -r m n ell <<< "${A3_CELL:-4:15:6}"
  { printf 'draw\tkind\tclosure_degree\tsaturated_at_4\texit\n'
    for o in "$RUNS"/a3-m$m-$n-$ell.d*.jsonl "$RUNS"/a3-control-m$m-$n-$ell.d*.jsonl; do [ -e "$o" ] || continue
      k=${o##*.d}; k=${k%.jsonl}; kind=unsat; case $o in *control*) kind=control;; esac
      cd_=$(grep -o '"closure_degree":[0-9a-z]*' "$o" | cut -d: -f2); sat=$(grep -o '"degree":4,[^}]*"saturated":[a-z]*' "$o" | grep -o '"saturated":[a-z]*' | cut -d: -f2)
      nm=$(basename "$o"); nm=${nm%.d*}; ex=$(grep "$nm d$k: exit" "$RUNS/queue.log" | tail -1 | grep -o 'exit [0-9]*' | cut -d' ' -f2)
      [ "$kind" = control ] && cd_=$(grep -o '"refuted_satisfiable":[a-z]*' "$o" | cut -d: -f2)
      printf '%s\t%s\t%s\t%s\t%s\n' "$k" "$kind" "${cd_:-none}" "${sat:-none}" "${ex:-none}"
    done; } | tee "$RUNS/a3-tally.tsv" ;;
esac
