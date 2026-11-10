#!/bin/bash
# Fault-injecting wrapper around s3shim.sh (the committed local stand-in for `aws s3 cp|sync`). Derived from the review's
# tools/shim2.sh. It only looks at `cp` calls that UPLOAD a progress.txt (destination s3://.../progress.txt), counted in $CNT
# (a file, one per job; 1-based). Switches (environment):
#   FAILN="2 5"            those progress.txt uploads fail (exit 1, nothing copied)
#   SLOW_RE=<regex>        when the progress.txt being uploaded has a last line matching it: touch $SLOW_MARK, sleep $SLOW_S
#                          (default 3) BEFORE the upload (a window in which the test can send a signal)
#   SNAP_DIR=<dir>         before each such upload n, save what S3 holds now: $SNAP_DIR/<n>.units_done, <n>.progress, and the
#                          last line of the file about to be uploaded: <n>.lastline
# Everything else is passed to s3shim.sh unchanged (its own switches SHIM_READS_STDIN, SHIM_FAIL_PROGRESS,
# SHIM_FAIL_SYNC_RUNS, SHIM_LOG still work).
op=$1
if [ "$op" = cp ] && [ "$(basename "${3:-}")" = progress.txt ] && [ "${3#s3://}" != "$3" ]; then
  n=$(( $(cat "$CNT" 2>/dev/null || echo 0) + 1 )); echo $n > "$CNT"
  if [ -n "${SNAP_DIR:-}" ]; then
    d=$FAKE_S3/${3#s3://}; d=${d%/*}; mkdir -p "$SNAP_DIR"
    cp -f "$d/units_done.txt" "$SNAP_DIR/$n.units_done" 2>/dev/null; cp -f "$d/progress.txt" "$SNAP_DIR/$n.progress" 2>/dev/null
    tail -n1 "$2" > "$SNAP_DIR/$n.lastline"
  fi
  for f in ${FAILN:-}; do [ "$f" = "$n" ] && { echo "shim_faults: injected failure on progress upload #$n" >&2; exit 1; }; done
  if [ -n "${SLOW_RE:-}" ] && tail -n1 "$2" | grep -qE "$SLOW_RE"; then touch "$SLOW_MARK"; sleep "${SLOW_S:-3}"; fi
fi
exec "$(dirname "$0")/s3shim.sh" "$@"
