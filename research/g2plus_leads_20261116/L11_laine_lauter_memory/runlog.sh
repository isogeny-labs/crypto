#!/bin/bash
# usage: runlog.sh OUTFILE|- "command..." ; logs command, exit code, sha256 of outputs
cd "$(dirname "$0")"
out="$1"; shift
cmd="$*"
start=$(date -u +%FT%TZ)
if [ "$out" = "-" ]; then bash -c "$cmd"; else bash -c "$cmd" > "$out"; fi
rc=$?
{
echo "[$start] CMD: $cmd (stdout -> $out)"
echo "  exit=$rc"
if [ "$out" != "-" ] && [ -f "$out" ]; then echo "  sha256 $(sha256sum "$out")"; fi
} >> raw/COMMANDS.log
exit $rc
