#!/bin/bash
# runlog.sh LOGFILE "CMDSTRING" OUTFILE...   : runs CMDSTRING via bash -c, logs command, exit code, sha256 of outputs
log="$1"; cmd="$2"; shift 2
start=$(date -u +%FT%TZ)
bash -c "$cmd"; ec=$?
{
  echo "[$start] CMD: $cmd"
  echo "  exit=$ec"
  for o in "$@"; do [ -f "$o" ] && echo "  sha256 $(sha256sum "$o" | cut -d' ' -f1)  $o"; done
} >> "$log"
exit $ec
