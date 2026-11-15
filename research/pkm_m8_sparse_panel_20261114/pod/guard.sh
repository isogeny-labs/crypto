#!/usr/bin/env bash
# Pod-side dead-man guard for the m = 8 panel pod: the pod can never bill past the deadline.   [pcc:unverified]
# Usage: guard.sh DEADLINE_EPOCH GRACE_S WORKDIR        (WORKDIR = /work: run/out/done.json, fetched.flag, guard.log live there)
# Started by launch.sh through `rp.sh exec`; it re-launches itself detached (setsid nohup) and returns at once.
#  * at DEADLINE_EPOCH: creates WORKDIR/run/STOP; job.sh then starts no new row (remaining rows -> not_run, reason deadline)
#    and closes normally (results.tgz, done.json). A row already running is left to finish (its own timeout bounds it).
#  * stops compute (runpodctl stop pod; fallback kill -TERM 1, 30 s, kill -KILL 1) when the first of these holds:
#      (a)/(c) run/out/done.json AND fetched.flag exist (the controller has the results);
#      (b)     now >= DEADLINE_EPOCH + GRACE_S;
#      (d)     fetched.flag exists and now >= DEADLINE_EPOCH (partial fetch at the hard deadline; done.json never came).
#  Writes the reason to WORKDIR/guard.log before stopping. The controller (launch.sh) normally deletes the pod first.
# Env (tests only): GUARD_POLL_S (default 60), GUARD_DRY=1 (log "would kill", never signal anything).
set -uo pipefail
DEADLINE=${1:?DEADLINE_EPOCH}; GRACE=${2:?GRACE_S}; W=${3:?WORKDIR}
case "$DEADLINE$GRACE" in *[!0-9]*) echo "DEADLINE_EPOCH and GRACE_S must be integers" >&2; exit 2;; esac
POLL=${GUARD_POLL_S:-60}; DRY=${GUARD_DRY:-0}
mkdir -p "$W/run"; W=$(cd "$W" && pwd); LOG=$W/guard.log
if [ -z "${GUARD_DAEMON:-}" ]; then   # self-daemonise: survives the end of the exec that started it
  GUARD_DAEMON=1 setsid nohup bash "$(readlink -f "$0")" "$@" >> "$W/guard.out" 2>&1 < /dev/null &
  exit 0
fi
log() { echo "$(date -u +%FT%TZ) guard: $*" >> "$LOG"; }
exec 8> "$W/guard.lock"; flock -n 8 || { log "another guard holds the lock, exiting"; exit 0; }
log "started pid=$$ deadline=$DEADLINE ($(date -u -d "@$DEADLINE" +%FT%TZ)) grace=$GRACE poll=$POLL dry=$DRY"
stopped=0; REASON=
while :; do
  now=$(date +%s)
  if [ $stopped = 0 ] && [ "$now" -ge "$DEADLINE" ]; then touch "$W/run/STOP"; stopped=1; log "deadline reached: created $W/run/STOP (job.sh starts no new row)"; fi
  done_=0; flag=0; [ -e "$W/run/out/done.json" ] && done_=1; [ -e "$W/fetched.flag" ] && flag=1
  if [ $done_ = 1 ] && [ $flag = 1 ]; then REASON="done.json and fetched.flag present (results fetched)"; break; fi
  if [ "$now" -ge $((DEADLINE + GRACE)) ]; then REASON="deadline+grace reached (now=$now >= $((DEADLINE + GRACE)))"; break; fi
  if [ $flag = 1 ] && [ "$now" -ge "$DEADLINE" ]; then REASON="fetched.flag present after the deadline (partial fetch)"; break; fi
  nxt=$((DEADLINE + GRACE)); [ $stopped = 0 ] && nxt=$DEADLINE
  s=$((nxt - now)); [ $s -lt 1 ] && s=1; [ $s -gt "$POLL" ] && s=$POLL
  sleep "$s"
done
log "STOP COMPUTE: $REASON"; sync
if [ "$DRY" = 1 ]; then log "GUARD_DRY: would kill -TERM 1, sleep 30, kill -KILL 1"; exit 0; fi
# Measured on a real pod (HARDENING.md): when PID 1 dies RunPod RESTARTS the container and the pod stays RUNNING (billed). So the
# authoritative stop is `runpodctl stop pod`, with the pod-scoped RUNPOD_API_KEY / RUNPOD_POD_ID that RunPod injects into the pod
# (its value is never read, printed or written here; output is scrubbed). guard.stopped makes a restarted podserver retry the stop.
touch "$W/guard.stopped"
if command -v runpodctl > /dev/null && [ -n "${RUNPOD_POD_ID:-}" ]; then
  for k in 1 2 3; do
    log "runpodctl stop pod (try $k): $(runpodctl stop pod "$RUNPOD_POD_ID" 2>&1 | sed 's/[A-Za-z0-9_-]\{30,\}/<scrubbed>/g' | tr '\n' ' ')"; sync; sleep 20
  done
  log "still running 60 s after runpodctl stop; falling back to killing PID 1"
else log "no runpodctl or RUNPOD_POD_ID: falling back to killing PID 1 (RunPod will restart the container; pod keeps billing)"; fi
kill -TERM 1; sleep 30
log "PID 1 still alive after 30 s: kill -KILL 1, then kill the podserver and everything else"; sync
kill -KILL 1; pkill -KILL -f podserver.py; sleep 5; kill -KILL -1
