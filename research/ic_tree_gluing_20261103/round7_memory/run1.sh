#!/bin/bash
# usage: run1.sh NAME msolve-args...   ; 12 GiB address-space cap, 7200 s timeout, samples peak VmHWM
NAME=$1; shift
R=/home/user/crypto/research/ic_tree_gluing_20261103/round7_memory
F=/tmp/claude-0/-home-user/8900182d-dd1d-52b2-9106-78048929cee5/scratchpad/r3/m5_t4_k0_T.ms
S=/tmp/claude-0/-home-user/8900182d-dd1d-52b2-9106-78048929cee5/scratchpad/mem
ulimit -v 12582912
t0=$(date +%s)
( timeout 7200 msolve -f $F -o $S/$NAME.out "$@" -v 2 > $R/logs/$NAME.log 2>&1; echo "exit=$?" > $S/$NAME.exit ) &
sleep 2
peak=0
while pgrep -x msolve >/dev/null; do
  p=$(pgrep -x msolve | head -1); h=$(awk '/VmHWM/{print $2}' /proc/$p/status 2>/dev/null); [ -n "$h" ] && [ "$h" -gt "$peak" ] && peak=$h
  sleep 5
done
wait
echo "$NAME args=$* $(cat $S/$NAME.exit) wall=$(( $(date +%s)-t0 )) peakRSS_kB=$peak" >> $R/logs/summary.txt
