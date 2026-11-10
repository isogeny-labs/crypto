#!/bin/bash
# Stage B as resumable units (DRAFT Amendment R2-G, sections 3 and 7).
# Passed as the string argument of: /bin/bash -c "<this script>" stageb <PREV...>
# Every positional argument after $0 is the S3 URI prefix of an EARLIER job's results (possibly none).
# First argument "--list": print the unit list ("unit ladder t" per line) and exit 0.
# First argument "--argv <ladder> <T>": print the pilot flags of that unit (without --out) and exit.
#
# Environment (all overridable for local tests):
#   ISOLAB_RESULTS_URI  required (not for --list / --argv)
#   STUDY_DIR   (/work/study)  BIN (/usr/local/bin/pkm_tower_pilot)  G1BIN (/usr/local/bin/hky_fp_tower_check)
#   S3CP/S3SYNC/S3LS (aws s3 cp / sync / ls)   RAYON_NUM_THREADS (12)
#   PILOT_EXTRA  extra pilot flags appended to every call. TESTS ONLY; unset in production.
#   UNIT_FILTER  regex restricting the unit list. TESTS ONLY; unset in production.
# Exit: 0 list exhausted; 2 cannot make work dir; 10 G1 failed; 11 unit count differs from 25 ladders / 91 units
#       (49 hky-B in 13 ladders, 42 d-B in 12 ladders); 12 a PREV prefix has no readable progress.txt;
#       20 a pilot exited non-zero; 75 a unit was in flight in two or more PREV jobs (lost_units.txt).
#
# Final argv of each ladder, from run.sh lines hky-B / d-B (T = the unit's level; P0=786433, P1=2013265921):
#   common  = --p $P --ladder-t none --planted 0 --random 2
#   hky     = --engine hky --hky-cmax 7 --budget 1800 --hky-max-cols 2000000 --hky-max-entries 300000000
#   f4      = --engine tower --budget 1800 --max-nnz 2000000000
#   lv      = --t-min T --max-t-m3 T --max-t-m4 T --max-t T
#   tail    = $PILOT_EXTRA --out runs/<unit>.jsonl   (stdout runs/<unit>.out, stderr runs/<unit>.log)
#   B-m{3,4,5,6}-tower              common(P1) hky --kinds kummer  --m M --controls tower lv    t=2..8 (m=3) / 2..6
#   B-m{3,4,5,6}-tower-p0-kummer    common(P0) hky --kinds kummer  --m M --controls tower lv    t=2..tmax0(M)=5,4,3,2
#   B-m{3,4,5,6}-tower-p0-isogeny   common(P0) hky --kinds isogeny --m M --controls tower lv    t=2..tmax0(M)
#   B-m3-null                       common(P1) hky --kinds kummer  --m 3 --controls null --null-chain lv   t=2..8
#   DB-m{3,4,5,6}-tower             common(P1) f4  --kinds kummer  --m M --controls tower lv    t=2..8 (m=3) / 2..6
#   DB-m{3,4,5,6}-tower-p0-kummer   common(P0) f4  --kinds kummer  --m M --controls tower lv    t=2..tmax0(M)
#   DB-m{3,4,5,6}-tower-p0-isogeny  common(P0) f4  --kinds isogeny --m M --controls tower lv    t=2..tmax0(M)
# (`--list` and `--argv <ladder> <T>` print the exact unit list and argv.)
set -u
P0=786433
P1=2013265921
tmax0() { case $1 in 2) echo 8;; 3) echo 5;; 4) echo 4;; 5) echo 3;; *) echo 2;; esac; }
HKY=(--engine hky --hky-cmax 7 --budget 1800 --hky-max-cols 2000000 --hky-max-entries 300000000)
F4=(--engine tower --budget 1800 --max-nnz 2000000000)

# ladders: name cfg prime kind m control chain tmax   (run.sh order)
ladders() {
  local m tm t0
  for m in 3 4 5 6; do
    tm=6; [ "$m" = 3 ] && tm=8
    t0=$(tmax0 $m)
    echo "B-m$m-tower hky P1 kummer $m tower - $tm"
    echo "B-m$m-tower-p0-kummer hky P0 kummer $m tower - $t0"
    echo "B-m$m-tower-p0-isogeny hky P0 isogeny $m tower - $t0"
  done
  echo "B-m3-null hky P1 kummer 3 null chain 8"
  for m in 3 4 5 6; do
    tm=6; [ "$m" = 3 ] && tm=8
    t0=$(tmax0 $m)
    echo "DB-m$m-tower f4 P1 kummer $m tower - $tm"
    echo "DB-m$m-tower-p0-kummer f4 P0 kummer $m tower - $t0"
    echo "DB-m$m-tower-p0-isogeny f4 P0 isogeny $m tower - $t0"
  done
}
# units: "unit ladder t" lines, t = 2..tmax of each ladder
units() {
  local lad cfg pr kind m ctl chain tmax t
  while read -r lad cfg pr kind m ctl chain tmax; do
    for ((t = 2; t <= tmax; t++)); do echo "$lad-t$t $lad $t"; done
  done < <(ladders)
}
# ladder_argv <ladder> <T>: sets ARGV
ladder_argv() {
  local lad cfg pr kind m ctl chain tmax p
  read -r lad cfg pr kind m ctl chain tmax < <(ladders | awk -v l="$1" '$1 == l')
  [ -n "$lad" ] || return 1
  p=$P1; [ "$pr" = P0 ] && p=$P0
  ARGV=(--p "$p" --ladder-t none --planted 0 --random 2)
  if [ "$cfg" = hky ]; then ARGV+=("${HKY[@]}"); else ARGV+=("${F4[@]}"); fi
  ARGV+=(--kinds "$kind" --m "$m" --controls "$ctl")
  [ "$chain" = chain ] && ARGV+=(--null-chain)
  ARGV+=(--t-min "$2" --max-t-m3 "$2" --max-t-m4 "$2" --max-t "$2")
}

if [ "${1:-}" = "--list" ]; then units; exit 0; fi
if [ "${1:-}" = "--argv" ]; then ladder_argv "$2" "$3" && echo "${ARGV[*]}"; exit $?; fi

# --- unit list and its count assertion (before anything else) ---
ALL=$(units)
nl=$(ladders | wc -l); nu=$(printf '%s\n' "$ALL" | wc -l)
nlh=$(ladders | grep -c '^B-'); nuh=$(printf '%s\n' "$ALL" | grep -c '^B-')
nld=$(ladders | grep -c '^DB-'); nud=$(printf '%s\n' "$ALL" | grep -c '^DB-')
if [ "$nl/$nu/$nlh/$nuh/$nld/$nud" != "25/91/13/49/12/42" ]; then
  echo "unit count mismatch: ladders/units/hkyB-ladders/hkyB-units/dB-ladders/dB-units = $nl/$nu/$nlh/$nuh/$nld/$nud, expected 25/91/13/49/12/42" >&2
  exit 11
fi
if [ -n "${UNIT_FILTER:-}" ]; then ALL=$(printf '%s\n' "$ALL" | awk -v re="$UNIT_FILTER" '$1 ~ re'); fi

: "${ISOLAB_RESULTS_URI:?}"
S=${STUDY_DIR:-/work/study}
BIN=${BIN:-/usr/local/bin/pkm_tower_pilot}
G1BIN=${G1BIN:-/usr/local/bin/hky_fp_tower_check}
S3CP=${S3CP:-aws s3 cp}
S3SYNC=${S3SYNC:-aws s3 sync}
S3LS=${S3LS:-aws s3 ls}
PILOT_EXTRA=${PILOT_EXTRA:-}
URI=${ISOLAB_RESULTS_URI%/}
mkdir -p "$S/runs" "$S/gates" "$S/prev" && cd "$S" || exit 2
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-12}
touch progress.txt
log() { echo "$(date -u +%FT%TZ) $*" >> "$S/progress.txt"; }

# uploads are serialised by a lock so a stale background upload cannot overwrite a newer one
LOCK=$S/.up.lock
up_runs() { # unit files and gates; progress.txt is a separate step
  $S3SYNC "$S/runs" "$URI/runs" --only-show-errors
  $S3SYNC "$S/gates" "$URI/gates" --only-show-errors
}
up_progress() { $S3CP "$S/progress.txt" "$URI/progress.txt" --only-show-errors 2>/dev/null; }
up() { ( flock 9; up_runs; up_progress ) 9> "$LOCK"; }
up_runs_locked() { ( flock 9; up_runs ) 9> "$LOCK"; }
up_progress_locked() { ( flock 9; up_progress ) 9> "$LOCK"; }

declare -A DONE_IN CLOSED CLOSED_SRC INFL JOBDONE
ppid=""; syncer=""
write_summaries() {
  : > "$S/units_done.txt"
  for u in "${!JOBDONE[@]}"; do echo "$u"; done | sort -V >> "$S/units_done.txt"
  : > "$S/ladders_closed.txt"
  for l in "${!CLOSED[@]}"; do echo "$l t=${CLOSED[$l]} ${CLOSED_SRC[$l]}"; done | sort >> "$S/ladders_closed.txt"
}
final() {
  local rc=$?
  trap - EXIT TERM INT
  [ -n "$syncer" ] && kill "$syncer" 2>/dev/null
  [ -n "$ppid" ] && kill "$ppid" 2>/dev/null
  write_summaries
  up
  for f in units_done.txt ladders_closed.txt lost_units.txt binary.sha256 host.txt; do
    [ -f "$S/$f" ] && $S3CP "$S/$f" "$URI/$f" --only-show-errors
  done
  exit $rc
}
trap final EXIT
trap 'exit 143' TERM INT

sha256sum "$BIN" "$G1BIN" /usr/local/bin/prime_fall_degree_tools > "$S/binary.sha256" 2>&1
{ echo "argv: stageb $*"; echo "nproc: $(nproc)"; echo "rayon: $RAYON_NUM_THREADS"
  grep -m1 'model name' /proc/cpuinfo; grep MemTotal /proc/meminfo; uname -r; date -u +%FT%TZ
  echo "prev: $*"; } > "$S/host.txt"
$S3CP "$S/binary.sha256" "$URI/binary.sha256" --only-show-errors
$S3CP "$S/host.txt" "$URI/host.txt" --only-show-errors
( while sleep 300; do up; done ) &
syncer=$!

# --- G1 first ---
log "start g1"; "$G1BIN" > gates/g1.log 2>&1 < /dev/null; rc=$?
grep -q 'G1: 6 cases pass' gates/g1.log || { [ "$rc" = 0 ] && rc=1; }
log "end g1 exit $rc"
[ "$rc" = 0 ] || exit 10

# --- prior state ---
has_end()   { awk -v u="$2" '$2=="end" && $3==u {f=1} END{exit !f}' "$1"; }
has_start() { awk -v u="$2" '$2=="start" && $3==u {f=1} END{exit !f}' "$1"; }
done_in()   { awk -v u="$2" '$2=="end" && $3==u && $4=="exit" && $5=="0" {f=1} END{exit !f}' "$1"; }
PREVS=("$@")
for i in "${!PREVS[@]}"; do
  pv=${PREVS[$i]%/}; d="$S/prev/$i"; mkdir -p "$d/runs"
  $S3CP "$pv/progress.txt" "$d/progress.txt" --only-show-errors < /dev/null || { echo "cannot read $pv/progress.txt" >&2; exit 12; }
  $S3LS "$pv/runs/" < /dev/null 2>/dev/null | awk '{print $NF}' | grep '\.log$' > "$d/loglist.txt"
  while read -r f; do
    $S3CP "$pv/runs/$f" "$d/runs/$f" --only-show-errors < /dev/null
  done < "$d/loglist.txt"
  while read -r u lad t; do
    if done_in "$d/progress.txt" "$u"; then
      [ -z "${DONE_IN[$u]:-}" ] && DONE_IN[$u]=$i
    elif has_start "$d/progress.txt" "$u" && ! has_end "$d/progress.txt" "$u"; then
      INFL[$u]=$(( ${INFL[$u]:-0} + 1 ))
    fi
  done <<< "$ALL"
done
: > "$S/lost_units.txt"
while read -r u lad t; do
  if [ -n "$u" ] && [ -z "${DONE_IN[$u]:-}" ] && [ "${INFL[$u]:-0}" -ge 2 ]; then
    echo "$u" >> "$S/lost_units.txt"; log "lost $u"; echo "lost $u"
  fi
done <<< "$ALL"
if [ -s "$S/lost_units.txt" ]; then exit 75; fi
rm -f "$S/lost_units.txt"
while read -r u lad t; do   # closures recorded by done units of earlier jobs
  [ -n "$u" ] || continue
  i=${DONE_IN[$u]:-}
  [ -n "$i" ] || continue
  if grep -q 'timed out; stopping' "$S/prev/$i/runs/$u.log" 2>/dev/null; then
    if [ -z "${CLOSED[$lad]:-}" ] || [ "$t" -lt "${CLOSED[$lad]}" ]; then CLOSED[$lad]=$t; CLOSED_SRC[$lad]="prev $i"; fi
  fi
done <<< "$ALL"

# --- units ---
while read -r u lad t; do
  [ -n "$u" ] || continue
  if [ -n "${DONE_IN[$u]:-}" ]; then log "skip $u done-in ${DONE_IN[$u]}"; continue; fi
  if [ -n "${CLOSED[$lad]:-}" ]; then log "skip $u ladder-closed"; continue; fi
  ladder_argv "$lad" "$t" || exit 11
  rm -f runs/"$u".*
  log "start $u"; up_progress_locked
  # shellcheck disable=SC2086
  "$BIN" "${ARGV[@]}" $PILOT_EXTRA --out "runs/$u.jsonl" > "runs/$u.out" 2> "runs/$u.log" < /dev/null &
  ppid=$!; wait "$ppid"; rc=$?; ppid=""
  up_runs_locked                       # unit files first
  log "end $u exit $rc"; up_progress_locked
  if [ "$rc" != 0 ]; then exit 20; fi
  JOBDONE[$u]=1
  if grep -q 'timed out; stopping' "runs/$u.log"; then
    CLOSED[$lad]=$t; CLOSED_SRC[$lad]="this-job"
    log "closed $lad at t=$t"; up_progress_locked
  fi
done <<< "$ALL"
exit 0
