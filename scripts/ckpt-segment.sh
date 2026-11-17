#!/usr/bin/env bash
# Run one segment of a resumable job: restore its checkpoint from S3, run the
# command with DREG_CKPT_DIR pointing at it, push every new generation to S3
# while it runs, and stop it cleanly (SIGTERM, so it saves) at the segment
# deadline or when this script itself is told to stop (a Spot drain, a job
# cancel).  See src/cryptanalysis/closure_checkpoint.rs for the contract.
#
#   ckpt-segment.sh S3_URI WORK_DIR SEGMENT_SECONDS -- COMMAND [ARGS...]
#
# Exit status:
#   0   the command finished: the job is done
#   75  the command saved its state and stopped (segment deadline or signal):
#       run another segment (EX_TEMPFAIL)
#   other  the command failed; the checkpoint is left as it is
#
# S3_URI is the job's own checkpoint prefix.  The command gets DREG_CKPT_DIR
# (and DREG_CKPT_SECONDS if the caller set it).  CKPT_SYNC_SECONDS sets how
# often new generations are pushed (default 120).
set -uo pipefail
here=$(cd "$(dirname "$0")" && pwd)
s3=$1; work=$2; seg=$3; shift 3
[ "${1:-}" = -- ] && shift
[ $# -gt 0 ] || { echo "usage: $0 S3_URI WORK_DIR SEGMENT_SECONDS -- COMMAND..." >&2; exit 2; }
log() { echo "$(date -u +%FT%TZ) ckpt-segment: $*" >&2; }

ckpt=$work/ckpt
mkdir -p "$ckpt"
"$here/ckpt-s3-sync.sh" restore "$s3" "$ckpt" || { log "restore failed"; exit 1; }

"$here/ckpt-s3-sync.sh" watch "$ckpt" "$s3" "${CKPT_SYNC_SECONDS:-120}" &
watcher=$!

# The command in its own session, so a signal reaches everything it started.
sidfile=$work/segment.sid
rm -f "$sidfile"
rm -f "$sidfile.rc"
# The wrapper survives the session-wide TERM (a handler, unlike SIG_IGN, is not
# inherited by the command) so that it can record the command's status.
DREG_CKPT_DIR=$ckpt setsid -w bash -c 'trap : TERM INT; echo $$ > "$0"; "${@:2}"; echo $? > "$0.rc"' "$sidfile" - "$@" &
runner=$!
stopping=""
stop_command() {
  stopping=${1:-signal}
  [ -s "$sidfile" ] && pkill -TERM -s "$(cat "$sidfile")" 2>/dev/null
  return 0
}
trap 'stop_command signal' TERM INT
( sleep "$seg"; kill -USR1 $$ 2>/dev/null ) & timer=$!
trap 'stop_command deadline' USR1

# A trapped signal interrupts `wait`, so wait until the wrapper has really
# ended and read the command's status from the file it leaves.
while kill -0 "$runner" 2>/dev/null; do wait "$runner" 2>/dev/null; done
rc=$(cat "$sidfile.rc" 2>/dev/null || echo 1)
kill "$timer" 2>/dev/null
log "command exited $rc${stopping:+ after $stopping}"

# One last push of whatever the command committed, then stop the watcher.
kill -TERM "$watcher" 2>/dev/null; wait "$watcher" 2>/dev/null
"$here/ckpt-s3-sync.sh" push "$ckpt" "$s3" || { log "final push failed; the checkpoint may be behind"; exit 1; }

case $rc in
  0) exit 0 ;;
  143|130) log "state saved; another segment continues it"; exit 75 ;;
  *) exit "$rc" ;;
esac
