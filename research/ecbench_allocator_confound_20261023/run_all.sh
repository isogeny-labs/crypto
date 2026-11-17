#!/bin/sh
# The exact session commands of the study (PREREGISTRATION.md "Design").  ABBA per spec.
# BIN_BASE / BIN_FIX are copies of the two release binaries (hashes in BINARIES.txt).
set -u
R=research/ecbench_allocator_confound_20261023
SMOKE=docs/ecbench/specs/smoke-prime.json
CLAIM=$R/specs/claim-koblitz-r10.json
run() { # name binary spec
  [ -e "$R/sessions/$1" ] && { echo "exists $1"; return; }
  date -u +"%FT%TZ start $1 load=$(cut -d' ' -f1-3 /proc/loadavg) cargobuild=$(flock -n /home/user/wt/.cargo.lock true && echo idle || echo BUSY)"
  "$2" run --wait --allow-busy --cpus auto --quiet --spec "$3" --out "$R/sessions/$1"; echo "rc=$? $1"
}
for k in smoke claim; do
  if [ $k = smoke ]; then SP=$SMOKE; else SP=$CLAIM; fi
  run base-$k     "$BIN_BASE" $SP
  run fix-$k      "$BIN_FIX"  $SP
  run fix-$k-rev  "$BIN_FIX"  $SP
  run base-$k-rev "$BIN_BASE" $SP
done
