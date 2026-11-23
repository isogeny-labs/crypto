#!/bin/bash
# usage: runlog.sh "<command>" outfile1 outfile2 ...
cd /home/user/crypto/research/g2plus_leads_20261116/L12_bielliptic_quartic_orbits
cmd="$1"; shift
bash -c "$cmd"; rc=$?
{
echo "### $(date -u +%FT%TZ)  CMD: $cmd"
echo "EXIT: $rc"
for f in "$@"; do [ -f "$f" ] && sha256sum "$f"; done
} >> raw/COMMANDS.log
exit $rc
