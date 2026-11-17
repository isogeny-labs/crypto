#!/usr/bin/env bash
# Lead step 3 (local part): D family (D, DL1, DL) with 128-bit ell, memory mode, seeds 1-3, at
# k = 23 and 24; the same pipeline at k = 17..22 on this host for a same-host exponent fit; and the
# frozen calibration (v2 calib, three times per size). Sequential, single-threaded, 20-min cap/job.
set -u
D=$(cd "$(dirname "$0")" && pwd)
cd "$D"
W=raw/wide
mkdir -p $W
export V2W_COMPACT=1
for q in 8388571 16777183; do
  for s in 1 2 3; do
    t=ic_${q}_${s}_D
    [ -s $W/c_$t.res.json ] || python3 runjob.py $W/c_$t 1200 ./bin/v2w ic $q $s D 3.0 2.0 4 4 >/dev/null
  done
done
for q in 131071 262111 524287 1048507 2097091 4194271; do
  for s in 1 2 3; do
    t=ic_${q}_${s}_D
    [ -s $W/c_$t.res.json ] || python3 runjob.py $W/c_$t 1200 ./bin/v2w ic $q $s D 3.0 2.0 4 4 >/dev/null
  done
done
for q in 131071 262111 524287 1048507 2097091 4194271 8388571 16777183; do
  for c in 1 2 3; do
    [ -s $W/calib_${q}_$c.res.json ] || python3 runjob.py $W/calib_${q}_$c 1200 ./bin/v2w calib $q $c >/dev/null
  done
done
echo "wide done $(date -u +%FT%TZ)" > raw/WIDE_DONE
