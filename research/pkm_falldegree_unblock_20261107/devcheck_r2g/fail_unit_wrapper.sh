#!/bin/bash
# Stands in for the pilot (BIN). Derived from the review's tools/failwrap.sh. Calls the real pilot ($PILOT_REAL) with the same
# arguments, except that the unit named in $FAIL_UNIT exits 3 without running ("a tool failure"). The same wrapper file is used
# by every job of a test (so binary.sha256 agrees between jobs); only the environment variable FAIL_UNIT differs.
# $WRAP_LOG (a file), if set, gets one line per call: "<unit> fail|run".
all="$*"; u=${all##*--out runs/}; u=${u%.jsonl}
if [ "$u" = "${FAIL_UNIT:-none}" ]; then
  [ -n "${WRAP_LOG:-}" ] && echo "$u fail" >> "$WRAP_LOG"
  echo "wrapper: forced tool failure of $u" >&2; exit 3
fi
[ -n "${WRAP_LOG:-}" ] && echo "$u run" >> "$WRAP_LOG"
exec "$PILOT_REAL" "$@"
