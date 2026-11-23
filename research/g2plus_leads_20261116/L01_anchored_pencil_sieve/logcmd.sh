#!/bin/bash
# usage: logcmd.sh <outfile-or-> <command...> ; appends to raw/COMMANDS.log
out="$1"; shift
start=$(date -u +%FT%TZ)
"$@"
rc=$?
{
  echo "[$start] CMD: $* ; exit=$rc"
  if [ "$out" != "-" ] && [ -f "$out" ]; then sha256sum "$out" | sed 's/^/    /'; fi
} >> raw/COMMANDS.log
exit $rc
