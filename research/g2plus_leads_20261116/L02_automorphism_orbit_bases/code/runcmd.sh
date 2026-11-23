#!/bin/bash
# usage: runcmd.sh <output-file-to-hash,...> <command...>   (runs from lead dir; logs to raw/COMMANDS.log)
cd /home/user/crypto/research/g2plus_leads_20261116/L02_automorphism_orbit_bases
outs="$1"; shift
start=$(date -u +%FT%TZ)
t0=$(date +%s)
( ulimit -v 2000000; timeout 900 "$@" ) 
rc=$?
t1=$(date +%s)
{
echo "[$start] CMD: $* | exit=$rc | elapsed_s=$((t1-t0))"
for f in ${outs//,/ }; do [ -f "$f" ] && echo "    sha256 $(sha256sum "$f")"; done
} >> raw/COMMANDS.log
if [ $rc -eq 124 ]; then echo "TIMEOUT $* (900 s)" >> raw/TIMEOUTS.txt; fi
exit $rc
