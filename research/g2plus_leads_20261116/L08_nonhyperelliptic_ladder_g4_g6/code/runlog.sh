#!/bin/bash
# usage: runlog.sh <outfile> <command...>  ; logs command, exit code, sha256 of outfile (stdout captured if outfile is selftest-style via redirect by caller)
OUT="$1"; shift
START=$(date -u +%FT%TZ)
timeout 600 "$@" > "${OUT}.stdout" 2> "${OUT}.stderr"
RC=$?
echo "[$START] CMD: timeout 600 $* ; EXIT: $RC" >> raw/COMMANDS.log
for f in "$OUT" "${OUT}.stdout" "${OUT}.stderr"; do [ -f "$f" ] && echo "    sha256 $(sha256sum "$f" | cut -d' ' -f1)  $f" >> raw/COMMANDS.log; done
[ $RC -eq 124 ] && echo "$* 124" >> raw/timeouts.txt
exit $RC
