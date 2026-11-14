#!/bin/bash
E=/home/user/crypto/research/pkm_m8_sparse_panel_20261114/smoke/pilot-t2r8-s777/export
P=/tmp/claude-0/-home-user/c2970330-965d-56e2-8425-0faf587c0f40/scratchpad/pilot
T=d000470
/usr/bin/time -v -o $P/$T.cap14.time timeout 3600 prlimit --as=$((14*1024*1024*1024)) msolve -t 4 -v 2 -P 1 -f $E/$T.ms -o $P/$T.cap14.P1.out > $P/$T.cap14.P1.stdout 2> $P/$T.cap14.P1.stderr
echo "$T cap14 rc=$?" >> $P/status2.txt
