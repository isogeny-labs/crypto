#!/bin/bash
# pilot: msolve -t 4 -v 2 -P 1, 3600 s timeout, 8 GB address-space cap, one call at a time
E=/home/user/crypto/research/pkm_m8_sparse_panel_20261114/smoke/pilot-t2r8-s777/export
P=/tmp/claude-0/-home-user/c2970330-965d-56e2-8425-0faf587c0f40/scratchpad/pilot
for T in d000470 d000000; do
  /usr/bin/time -v -o $P/$T.time timeout 3600 prlimit --as=$((8*1024*1024*1024)) msolve -t 4 -v 2 -P 1 -f $E/$T.ms -o $P/$T.P1.out > $P/$T.P1.stdout 2> $P/$T.P1.stderr
  echo "$T rc=$?" >> $P/status.txt
done
echo finished >> $P/status.txt
