#!/bin/bash
# Stage B as resumable units, revision 3 (Amendment R2-G, sections 3 and 7).
# Run as a FILE:  /bin/bash <this file> [arguments]   ($0 is the file; its sha256 is recorded in host.txt; if $0 is not a
# readable file, as under bash -c, the runner stops with exit 2). Arguments, in any order:
#   --list                  (first argument) print the unit list, "unit ladder t" per line, exit 0
#   --argv <ladder> <T>     (first arguments) print the pilot flags of that unit (without --out), exit
#   PREV                    result prefix of an earlier unit job, verbatim that job's ISOLAB_RESULTS_URI (ends in "/");
#                           outside test mode it must match ^s3://.+/$ (else exit 16). A PREV given twice: exit 16.
#   close:<ladder>:<T>      close the ladder from level T on (units with t >= T are logged "skip <unit> closed-by-rule").
#                           Accepted only if unit <ladder>-t<T> is lost (below) in these PREVs, else exit 16. One per ladder.
#   allow:<unit>:<K>        K >= 3, <unit> in the list: the unit counts as lost only when in flight in K or more PREVs
#                           (default 2). One per unit; otherwise exit 16.
# Every PREV must hold progress.txt, host.txt, binary.sha256 and, for each done unit, runs/<unit>.{argv,log,jsonl}
# (the .jsonl ends with its {"summary" line and the .argv equals this job's argv for the unit), else exit 12. Exit 12 also if
# a PREV's testmode:/pilot_extra:/runner_sha256: line or binary hashes differ from this job's, if a PREV's prev: line names a job
# that is not among the PREVs, or if a unit is done in two PREVs. host.txt closed_by_rule: and allowed: (this job's arguments plus
# those of every PREV) are carried forward, so a closure or allowance given once holds in every later job.
# Per unit, over the PREVs: done = "end <u> exit 0"; failed = PREVs with "end <u> exit N", N != 0; in flight = PREVs with "start <u>"
# and neither an "end <u>" nor an "abort <u>" line. A unit that is not done and failed in 2 or more PREVs closes its ladder from
# its level on (log "closed <ladder> at t=<T> tool-failure"). A unit that is not done, not at or above a closed level of its
# ladder (stop line, close:, tool failure) and in flight in K or more PREVs is lost: lost_units.txt, exit 75.
#
# Environment:
#   ISOLAB_RESULTS_URI  required (not for --list / --argv), unset or empty: exit 2; used verbatim (production: ends in "/")
#   RAYON_NUM_THREADS (12)   NODE_NAME POD_UID ISOLAB_JOB_ID (recorded in host.txt)
#   TEST-ONLY (local tests; refused with exit 15 unless S3CP or S3SYNC is set, which marks test mode):
#     STUDY_DIR (/work/study)  BIN (/usr/local/bin/pkm_tower_pilot)  G1BIN (/usr/local/bin/hky_fp_tower_check)
#     S3CP / S3SYNC (aws s3 cp / sync; replaced by local-directory shims in tests)
#     PILOT_EXTRA (extra pilot flags appended to every call)   UNIT_FILTER (regex restricting the unit list)
#     LOCK_WAIT (seconds a lock wait lasts, 900)
# Tools used (the job image has no others): bash awk basename cat cp cut date dirname grep head ls mkdir mktemp mv nproc rm
#   rmdir sed sha256sum sleep sort stat tail tee timeout touch tr uname wc find xargs aws. No flock: uploads are serialised
#   by a mkdir lock ($S/.up.lock.d); a lock wait of LOCK_WAIT seconds fails and counts as an upload failure. Every S3 command
#   reads /dev/null; the unit-list loops read the list on fd 3, so nothing inside a loop can consume it.
# Order: read the PREVs and compute every closure and lost unit; write and upload binary.sha256 and host.txt (exit 13 if that
#   fails); start the syncer; G1; log tool-failure closures; exit 75 if a unit is lost; the units.
# Per unit: runs/<unit>.argv is written, "start" is uploaded, the pilot runs, files are uploaded, then the "end" line. Every
#   upload of progress.txt is preceded by units_done.txt (the units with an "end <u> exit 0" line of this job's progress.txt)
#   in the same locked step, so S3 never holds an "end <u> exit 0" line without the unit in units_done.txt. Before every
#   exit 13 inside the unit loop an "abort <unit> <reason>" line is appended (start-upload-failed, files-upload-failed,
#   end-upload-failed); the exit handler uploads it.
# Exit handler: creates $S/.stop (the syncer checks it before each upload), kills the pilot, releases a lock that the main
#   shell itself holds, waits at most 20 s for the lock, removes the lock, kills the syncer (and its sleep), writes the
#   summaries and uploads everything once more.
# Exit: 0 list exhausted and every unit done or at/above a closed level; 2 cannot make work dir, ISOLAB_RESULTS_URI unset or
#       empty, or $0 not a readable file; 10 G1 failed; 11 unit count differs from 25 ladders / 91 units (49 hky-B in 13
#       ladders, 42 d-B in 12 ladders); 12 an earlier job is unreadable, inconsistent or from another run (see above); 13 an
#       upload failed or a lock wait ran out (the runner stops); 14 the list ended with a unit neither done nor at/above a
#       closed level; 15 test-only variable set outside test mode (or LOCK_WAIT not a whole number); 16 bad argument; 20 a pilot
#       exited non-zero; 75 a unit was lost (lost_units.txt); 143 TERM/INT. (Exit 17 belongs to the job request's start command.)
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
# pilot_words <ladder> <T> <unit>: sets PW, the exact words of the unit's pilot call
pilot_words() {
  local x
  ladder_argv "$1" "$2" || return 1
  PW=("${ARGV[@]}")
  set -f; for x in $PILOT_EXTRA; do PW+=("$x"); done; set +f
  PW+=(--out "runs/$3.jsonl")
}

if [ "${1:-}" = "--list" ]; then units; exit 0; fi
if [ "${1:-}" = "--argv" ]; then ladder_argv "${2:-}" "${3:-}" && echo "${ARGV[*]}"; exit $?; fi

# --- test-only guard, before anything touches S3 and before defaults ---
TESTMODE=0
if [ -n "${S3CP:-}" ] || [ -n "${S3SYNC:-}" ]; then TESTMODE=1; fi
if [ "$TESTMODE" = 0 ]; then
  for v in PILOT_EXTRA UNIT_FILTER BIN G1BIN STUDY_DIR LOCK_WAIT; do
    if [ -n "${!v:-}" ]; then echo "refused: $v is set but S3CP/S3SYNC are not (test mode off); $v is for local tests only" >&2; exit 15; fi
  done
fi
LOCK_WAIT=${LOCK_WAIT:-900}
case $LOCK_WAIT in ''|*[!0-9]*) echo "LOCK_WAIT must be a whole number of seconds" >&2; exit 15;; esac
[ -f "$0" ] || { echo "run the runner as a file: /bin/bash <file> <arguments>" >&2; exit 2; }
RSHA=$(sha256sum "$0" | cut -d' ' -f1)

# --- unit list and its count assertion ---
ALL=$(units)
nl=$(ladders | wc -l); nu=$(printf '%s\n' "$ALL" | wc -l)
nlh=$(ladders | grep -c '^B-'); nuh=$(printf '%s\n' "$ALL" | grep -c '^B-')
nld=$(ladders | grep -c '^DB-'); nud=$(printf '%s\n' "$ALL" | grep -c '^DB-')
if [ "$nl/$nu/$nlh/$nuh/$nld/$nud" != "25/91/13/49/12/42" ]; then
  echo "unit count mismatch: ladders/units/hkyB-ladders/hkyB-units/dB-ladders/dB-units = $nl/$nu/$nlh/$nuh/$nld/$nud, expected 25/91/13/49/12/42" >&2
  exit 11
fi
FULL=$(printf '%s\n' "$ALL" | awk '{print $1}')
if [ -n "${UNIT_FILTER:-}" ]; then ALL=$(printf '%s\n' "$ALL" | awk -v re="$UNIT_FILTER" '$1 ~ re'); fi

declare -A INALL RULE ALLOW DONE_IN FAIL INFL CLOSED CLOSED_SRC TFC JOBDONE
while read -r u lad t; do [ -n "$u" ] && INALL[$u]=1; done <<< "$ALL"
# chk_close <word>: close:<ladder>:<T> -> CL_L CL_T ; chk_allow <word>: allow:<unit>:<K> -> AL_U AL_K (exact spelling only)
chk_close() {
  local x extra tmx
  IFS=: read -r x CL_L CL_T extra <<< "$1"
  [ "$1" = "close:$CL_L:$CL_T" ] || return 1
  tmx=$(ladders | awk -v l="$CL_L" '$1 == l {print $8}')
  [ -n "$tmx" ] && [[ $CL_T =~ ^[1-9][0-9]{0,2}$ ]] && [ "$CL_T" -ge 2 ] && [ "$CL_T" -le "$tmx" ]
}
chk_allow() {
  local x extra
  IFS=: read -r x AL_U AL_K extra <<< "$1"
  [ "$1" = "allow:$AL_U:$AL_K" ] && [[ $AL_K =~ ^[1-9][0-9]{0,2}$ ]] && [ "$AL_K" -ge 3 ] && printf '%s\n' "$FULL" | grep -qxF -- "$AL_U"
}
# --- arguments: validated before anything touches S3 ---
PREVS=(); OWNC=()
for a in "$@"; do
  case "$a" in
    close:*)
      chk_close "$a" || { echo "bad argument $a: need close:<ladder>:<T> with an existing ladder and 2 <= T <= its top level" >&2; exit 16; }
      [ -z "${RULE[$CL_L]:-}" ] || { echo "bad argument $a: a second close: for ladder $CL_L" >&2; exit 16; }
      RULE[$CL_L]=$CL_T; OWNC+=("$CL_L:$CL_T") ;;
    allow:*)
      chk_allow "$a" || { echo "bad argument $a: need allow:<unit>:<K> with a unit of the list and K >= 3" >&2; exit 16; }
      [ -z "${ALLOW[$AL_U]:-}" ] || { echo "bad argument $a: a second allow: for unit $AL_U" >&2; exit 16; }
      ALLOW[$AL_U]=$AL_K ;;
    *)
      if [ -z "$a" ] || { [ "$TESTMODE" = 0 ] && ! [[ $a =~ ^s3://.+/$ ]]; }; then echo "bad argument '$a': a PREV must match ^s3://.+/$ (the earlier job's ISOLAB_RESULTS_URI)" >&2; exit 16; fi
      for j in "${!PREVS[@]}"; do [ "${PREVS[$j]}" != "$a" ] || { echo "bad argument $a: PREV given twice" >&2; exit 16; }; done
      PREVS+=("$a") ;;
  esac
done

if [ -z "${ISOLAB_RESULTS_URI:-}" ]; then echo "ISOLAB_RESULTS_URI is unset or empty" >&2; exit 2; fi
U=$ISOLAB_RESULTS_URI
S=${STUDY_DIR:-/work/study}
BIN=${BIN:-/usr/local/bin/pkm_tower_pilot}
G1BIN=${G1BIN:-/usr/local/bin/hky_fp_tower_check}
S3CP=${S3CP:-aws s3 cp}
S3SYNC=${S3SYNC:-aws s3 sync}
PILOT_EXTRA=${PILOT_EXTRA:-}
mkdir -p "$S/runs" "$S/gates" "$S/prev" && cd "$S" || exit 2
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-12}
touch progress.txt
log() { echo "$(date -u +%FT%TZ) $*" >> "$S/progress.txt"; }

# uploads are serialised by a mkdir lock so a stale background upload cannot overwrite a newer one; a wait is bounded
lock() {
  local n=0
  until mkdir "$S/.up.lock.d" 2>/dev/null; do n=$((n + 1)); [ "$n" -gt "$LOCK_WAIT" ] && return 1; sleep 1; done
  HELD=1
}
unlock() { HELD=0; rmdir "$S/.up.lock.d" 2>/dev/null; }
# units_done.txt: the units of this ALL list with an "end <u> exit 0" line in this job's progress.txt
write_done() {
  awk 'NR == FNR { if ($2 == "end" && $4 == "exit" && $5 == "0") d[$3] = 1; next } $1 in d { print $1 }' "$S/progress.txt" - <<< "$ALL" > "$S/units_done.tmp" &&
    mv "$S/units_done.tmp" "$S/units_done.txt"
}
up_runs() { # unit files and gates; progress.txt is a separate step; fails if either sync fails
  local r=0
  $S3SYNC "$S/runs" "$U/runs" --only-show-errors < /dev/null || r=1
  $S3SYNC "$S/gates" "$U/gates" --only-show-errors < /dev/null || r=1
  return $r
}
up_progress() { # units_done.txt first, so that no "end" line is ever in S3 without its unit in units_done.txt
  write_done && $S3CP "$S/units_done.txt" "$U/units_done.txt" --only-show-errors < /dev/null &&
    $S3CP "$S/progress.txt" "$U/progress.txt" --only-show-errors < /dev/null
}
up()                { local r; lock || return 1; up_runs && up_progress; r=$?; unlock; return $r; }
up_runs_locked()     { local r; lock || return 1; up_runs; r=$?; unlock; return $r; }
up_progress_locked() { local r; lock || return 1; up_progress; r=$?; unlock; return $r; }

LOST=(); TFORD=()
ppid=""; syncer=""; HELD=0   # HELD: this shell holds the lock (a TERM can arrive inside a locked step)
write_summaries() {
  write_done
  { for l in "${!CLOSED[@]}"; do echo "$l t=${CLOSED[$l]} ${CLOSED_SRC[$l]}"; done
    for l in "${!RULE[@]}"; do echo "$l t=${RULE[$l]} by-rule"; done
    for l in "${!TFC[@]}"; do echo "$l t=${TFC[$l]} tool-failure"; done; } | LC_ALL=C sort > "$S/ladders_closed.txt"
}
final() {
  local rc=$? f n=0
  trap - EXIT TERM INT
  : > "$S/.stop"
  [ -n "$ppid" ] && kill "$ppid" 2>/dev/null
  [ "$HELD" = 1 ] && unlock
  until mkdir "$S/.up.lock.d" 2>/dev/null; do n=$((n + 1)); [ "$n" -ge 20 ] && break; sleep 1; done
  rmdir "$S/.up.lock.d" 2>/dev/null     # released, or removed regardless after 20 s
  [ -n "$syncer" ] && kill "$syncer" 2>/dev/null
  write_summaries
  up || { [ "$rc" = 0 ] && rc=13; }
  for f in units_done.txt ladders_closed.txt lost_units.txt binary.sha256 host.txt; do
    [ -f "$S/$f" ] && { $S3CP "$S/$f" "$U/$f" --only-show-errors < /dev/null || { [ "$rc" = 0 ] && rc=13; }; }
  done
  exit $rc
}
trap final EXIT
trap 'exit 143' TERM INT

sha256sum "$BIN" "$G1BIN" /usr/local/bin/prime_fall_degree_tools > "$S/binary.sha256" 2>&1

# --- earlier jobs: read everything, check it, count; no upload and no unit before this is complete ---
bad12() { echo "$*" >&2; exit 12; }
own_line() { case $1 in testmode) echo "testmode: $TESTMODE";; pilot_extra) echo "pilot_extra: $PILOT_EXTRA";; *) echo "runner_sha256: $RSHA";; esac; }
OWNBIN=$(awk '{print $1}' "$S/binary.sha256")
for i in "${!PREVS[@]}"; do
  pv=${PREVS[$i]}; d="$S/prev/$i"; mkdir -p "$d/runs"
  for f in progress.txt host.txt binary.sha256; do
    $S3CP "$pv/$f" "$d/$f" --only-show-errors < /dev/null || bad12 "cannot read $pv/$f"
  done
  for k in testmode pilot_extra runner_sha256; do
    [ "$(grep -m1 "^$k:" "$d/host.txt")" = "$(own_line $k)" ] || bad12 "$pv/host.txt: its $k: line differs from this job's"
  done
  [ "$(awk '{print $1}' "$d/binary.sha256")" = "$OWNBIN" ] || bad12 "$pv/binary.sha256 differs from this job's binary hashes"
  for k in prev closed_by_rule allowed; do grep -q "^$k:" "$d/host.txt" || bad12 "$pv/host.txt has no $k: line"; done
  for w in $(grep -m1 '^prev:' "$d/host.txt" | cut -d' ' -f2-); do
    ok=0; for j in "${!PREVS[@]}"; do [ "${PREVS[$j]}" != "$w" ] || ok=1; done
    [ "$ok" = 1 ] || bad12 "$pv was run after $w, which is not among the PREVs: an earlier job was left out"
  done
  for w in $(grep -m1 '^closed_by_rule:' "$d/host.txt" | cut -d' ' -f2-); do
    chk_close "$w" || bad12 "$pv/host.txt: bad closure word $w"
    if [ -z "${RULE[$CL_L]:-}" ] || [ "$CL_T" -lt "${RULE[$CL_L]}" ]; then RULE[$CL_L]=$CL_T; fi
  done
  for w in $(grep -m1 '^allowed:' "$d/host.txt" | cut -d' ' -f2-); do
    chk_allow "$w" || bad12 "$pv/host.txt: bad allowance word $w"
    if [ -z "${ALLOW[$AL_U]:-}" ] || [ "$AL_K" -gt "${ALLOW[$AL_U]}" ]; then ALLOW[$AL_U]=$AL_K; fi
  done
  # one line "<unit> D|F|I" per unit: done, failed (end with non-zero exit), in flight (start, no end, no abort)
  while read -r u st; do
    [ -n "${INALL[$u]:-}" ] || continue
    case $st in
      D) [ -z "${DONE_IN[$u]:-}" ] || bad12 "unit $u is done in more than one PREV"
         DONE_IN[$u]=$i ;;
      F) FAIL[$u]=$(( ${FAIL[$u]:-0} + 1 )) ;;
      I) INFL[$u]=$(( ${INFL[$u]:-0} + 1 )) ;;
    esac
  done < <(awk '$2 == "start" { s[$3] = 1 }  $2 == "abort" { a[$3] = 1 }  $2 == "end" { e[$3] = 1 }
                $2 == "end" && $4 == "exit" && $5 == "0" { d[$3] = 1 }
                END { for (u in d) print u, "D"
                      for (u in e) if (!(u in d)) print u, "F"
                      for (u in s) if (!(u in e) && !(u in a)) print u, "I" }' "$d/progress.txt")
done
# every done unit: its argv record, log and rows must be readable; argv as computed here; rows file complete (final summary line)
while read -r u lad t <&3; do
  [ -n "$u" ] || continue
  i=${DONE_IN[$u]:-}
  [ -n "$i" ] || continue
  pv=${PREVS[$i]}; d="$S/prev/$i"
  for ext in argv log jsonl; do
    $S3CP "$pv/runs/$u.$ext" "$d/runs/$u.$ext" --only-show-errors < /dev/null || bad12 "cannot read $pv/runs/$u.$ext"
  done
  pilot_words "$lad" "$t" "$u" || exit 11
  [ "$(cat "$d/runs/$u.argv")" = "${PW[*]}" ] || bad12 "$pv/runs/$u.argv differs from the argv this job computes for $u"
  tail -n1 "$d/runs/$u.jsonl" | grep -q '^{"summary"' || bad12 "$pv/runs/$u.jsonl has no final summary line"
done 3<<< "$ALL"
# closures: stop lines of done units, then tool failures (list order); closed_at: a unit at or above a closed level of its ladder
while read -r u lad t <&3; do
  [ -n "$u" ] || continue
  i=${DONE_IN[$u]:-}
  [ -n "$i" ] || continue
  if grep -q 'timed out; stopping' "$S/prev/$i/runs/$u.log" 2>/dev/null; then
    if [ -z "${CLOSED[$lad]:-}" ] || [ "$t" -lt "${CLOSED[$lad]}" ]; then CLOSED[$lad]=$t; CLOSED_SRC[$lad]="prev $i"; fi
  fi
done 3<<< "$ALL"
while read -r u lad t <&3; do
  [ -n "$u" ] || continue
  if [ -z "${DONE_IN[$u]:-}" ] && [ "${FAIL[$u]:-0}" -ge 2 ] && [ -z "${TFC[$lad]:-}" ]; then TFC[$lad]=$t; TFORD+=("$lad"); fi
done 3<<< "$ALL"
closed_at() {
  { [ -n "${CLOSED[$1]:-}" ] && [ "$2" -ge "${CLOSED[$1]}" ]; } || { [ -n "${RULE[$1]:-}" ] && [ "$2" -ge "${RULE[$1]}" ]; } ||
    { [ -n "${TFC[$1]:-}" ] && [ "$2" -ge "${TFC[$1]}" ]; }
}
# close:<ladder>:<T> arguments of this job need a lost unit <ladder>-t<T> (not done, in flight in K or more PREVs)
for j in "${!OWNC[@]}"; do
  IFS=: read -r cl ct <<< "${OWNC[$j]}"
  if [ -n "${DONE_IN[$cl-t$ct]:-}" ] || [ "${INFL[$cl-t$ct]:-0}" -lt "${ALLOW[$cl-t$ct]:-2}" ]; then
    echo "bad argument close:$cl:$ct: unit $cl-t$ct is not lost in these PREVs (not done and in flight in ${ALLOW[$cl-t$ct]:-2} or more)" >&2; exit 16
  fi
done
while read -r u lad t <&3; do
  [ -n "$u" ] || continue
  [ -z "${DONE_IN[$u]:-}" ] || continue
  closed_at "$lad" "$t" && continue
  if [ "${INFL[$u]:-0}" -ge "${ALLOW[$u]:-2}" ]; then LOST+=("$u"); fi
done 3<<< "$ALL"

# --- binary.sha256 and host.txt: complete now, uploaded before G1 and before any unit ---
cw=$(for l in "${!RULE[@]}"; do echo "close:$l:${RULE[$l]}"; done | LC_ALL=C sort | tr '\n' ' ')
aw=$(for l in "${!ALLOW[@]}"; do echo "allow:$l:${ALLOW[$l]}"; done | LC_ALL=C sort | tr '\n' ' ')
tw=$(for l in "${!TFC[@]}"; do echo "$l:${TFC[$l]}"; done | LC_ALL=C sort | tr '\n' ' ')
{ echo "argv: $*"; echo "nproc: $(nproc)"; echo "rayon: $RAYON_NUM_THREADS"
  grep -m1 'model name' /proc/cpuinfo; grep MemTotal /proc/meminfo; uname -r; date -u +%FT%TZ
  echo "prev: ${PREVS[*]:-}"; echo "closed_by_rule: ${cw% }"; echo "allowed: ${aw% }"; echo "closed_tool_failure: ${tw% }"
  echo "testmode: $TESTMODE"; echo "pilot_extra: $PILOT_EXTRA"; echo "unit_filter: ${UNIT_FILTER:-}"; echo "lock_wait: $LOCK_WAIT"
  echo "runner_sha256: $RSHA"; echo "bash: $BASH_VERSION"; echo "node: ${NODE_NAME:-}"; echo "pod_uid: ${POD_UID:-}"
  echo "job: ${ISOLAB_JOB_ID:-}"; echo "seed: 1347112276 (the pilot's default; recorded, not passed)"
  [ -f /opt/study/build-info.json ] && cat /opt/study/build-info.json; } > "$S/host.txt"
$S3CP "$S/binary.sha256" "$U/binary.sha256" --only-show-errors < /dev/null || exit 13
$S3CP "$S/host.txt" "$U/host.txt" --only-show-errors < /dev/null || exit 13
( trap 'kill "$sp" 2>/dev/null; exit 0' TERM; sp=""   # on TERM the syncer takes its sleep with it
  while :; do sleep 300 & sp=$!; wait "$sp"; [ -e "$S/.stop" ] && break; up; done ) &
syncer=$!

# --- G1 first ---
log "start g1"; "$G1BIN" > gates/g1.log 2>&1 < /dev/null; rc=$?
grep -q 'G1: 6 cases pass' gates/g1.log || { [ "$rc" = 0 ] && rc=1; }
log "end g1 exit $rc"
[ "$rc" = 0 ] || exit 10
for j in "${!TFORD[@]}"; do log "closed ${TFORD[$j]} at t=${TFC[${TFORD[$j]}]} tool-failure"; done
if [ "${#LOST[@]}" -gt 0 ]; then
  : > "$S/lost_units.txt"
  for j in "${!LOST[@]}"; do echo "${LOST[$j]}" >> "$S/lost_units.txt"; log "lost ${LOST[$j]}"; echo "lost ${LOST[$j]}"; done
  exit 75
fi

# --- units ---
while read -r u lad t <&3; do
  [ -n "$u" ] || continue
  if [ -n "${DONE_IN[$u]:-}" ]; then log "skip $u done-in ${DONE_IN[$u]}"; continue; fi
  if [ -n "${RULE[$lad]:-}" ] && [ "$t" -ge "${RULE[$lad]}" ]; then log "skip $u closed-by-rule"; continue; fi
  if [ -n "${TFC[$lad]:-}" ] && [ "$t" -ge "${TFC[$lad]}" ]; then log "skip $u closed-tool-failure"; continue; fi
  if [ -n "${CLOSED[$lad]:-}" ]; then log "skip $u ladder-closed"; continue; fi
  pilot_words "$lad" "$t" "$u" || exit 11
  rm -f runs/"$u".*
  echo "${PW[*]}" > "runs/$u.argv"
  log "start $u"; up_progress_locked || { log "abort $u start-upload-failed"; exit 13; }
  "$BIN" "${PW[@]}" > "runs/$u.out" 2> "runs/$u.log" < /dev/null &
  ppid=$!; wait "$ppid"; rc=$?; ppid=""
  up_runs_locked || { log "abort $u files-upload-failed"; exit 13; }            # unit files first
  log "end $u exit $rc"; up_progress_locked || { log "abort $u end-upload-failed"; exit 13; }
  if [ "$rc" != 0 ]; then exit 20; fi
  JOBDONE[$u]=1
  if grep -q 'timed out; stopping' "runs/$u.log"; then
    CLOSED[$lad]=$t; CLOSED_SRC[$lad]="this-job"
    log "closed $lad at t=$t"; up_progress_locked || { log "abort $u end-upload-failed"; exit 13; }
  fi
done 3<<< "$ALL"

# --- completeness: every unit done, or at/above the level where its ladder closed ---
while read -r u lad t <&3; do
  [ -n "$u" ] || continue
  if [ -n "${DONE_IN[$u]:-}" ] || [ -n "${JOBDONE[$u]:-}" ]; then continue; fi
  if [ -n "${CLOSED[$lad]:-}" ] && [ "$t" -gt "${CLOSED[$lad]}" ]; then continue; fi
  if [ -n "${RULE[$lad]:-}" ] && [ "$t" -ge "${RULE[$lad]}" ]; then continue; fi
  if [ -n "${TFC[$lad]:-}" ] && [ "$t" -ge "${TFC[$lad]}" ]; then continue; fi
  echo "incomplete: $u is neither done nor at or above a closed ladder level" >&2; exit 14
done 3<<< "$ALL"
exit 0
