#!/bin/bash
# usage: run1.sh ARM ORDER ; one run, 1 core, 3 GiB
D=/home/user/crypto/research/ic_tree_gluing_20261103/round4
S=/tmp/claude-0/-home-user/8900182d-dd1d-52b2-9106-78048929cee5/scratchpad
arm=$1; ord=$2
if [ $arm = F ]; then ms=$S/r2/m4_t4_k0_F.ms; else ms=$S/r3/m4_t4_k0_G.ms; fi
log=$D/logs/weighted/${arm}_${ord}.log
ulimit -v 3145728
t0=$(date +%s.%N)
MS=$ms ARM=$arm ORDER=$ord taskset -c ${CPU:-0} timeout 1800 Singular -q $D/weighted.sing > $log 2>&1
rc=$?
t1=$(date +%s.%N)
echo "RC $rc WALL $(echo "$t1 - $t0" | bc)" >> $log
