#!/bin/bash
# Runs the whole suite on the final runner and writes the verbatim outputs.
# Usage: run_suite.sh <output file> [progress dir]       (env: BOOT_REQUEST=<request.json> to start the runner as a job does;
#        WROOT=<scratch root> for the test work directories)
# Paths are shortened in the output ($T = the scratch root, $LAYER = the frozen binaries); nothing else is changed.
# T5k (a kill check) is not part of the suite.
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
SP=${SP:-/tmp/claude-0/-home-user/354dbe26-6a93-506a-a404-2fc6ef137fdc/scratchpad}
T=${WROOT:-$SP/r2g_test}; LAYER=${LAYER:-$SP/r2g_layer/usr/local/bin}
out=${1:?output file}; pdir=${2:-}
: > "$out"
for t in T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T24 T25 T26; do
  { echo "##### $t"; "$HERE/test_driver.sh" $t 2>&1; } | sed "s#$T#\$T#g; s#$LAYER#\$LAYER#g; s#$SP#\$SCRATCH#g" >> "$out"
  if [ -n "$pdir" ]; then mkdir -p "$pdir"
    for d in "$T/$t"/w-*; do [ -f "$d/progress.txt" ] || continue; j=${d##*/w-}
      cp "$d/progress.txt" "$pdir/${t}_$j.progress.txt"
      for f in ladders_closed.txt lost_units.txt; do [ -s "$d/$f" ] && cp "$d/$f" "$pdir/${t}_$j.$f"; done
    done
  fi
done
grep '^RESULT ' "$out" > "$out.res"; { echo "=== summary"; cat "$out.res"; } >> "$out"; rm -f "$out.res"
