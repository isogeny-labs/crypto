#!/bin/bash
cd /Volumes/SSD990-2/s2-build; export TMPDIR=$PWD/tmp WDSAT_BIN=$PWD/bins
P=target/release/examples/s2_split_probe
envof() { case $1 in
 base) echo "";; memo0) echo "IC_TEMPLATE_MEMO=0";; nolin) echo "KIC_LINEAR_ELIM=0";;
 noinh) echo "KIC_F4_INHERIT=0 KIC_LINEAR_ELIM=1 SOLVER_SPLIT_RULE=highest";; splitlow) echo "SOLVER_SPLIT_RULE=lowest";;
 chainlayout) echo "KIC_CHAIN_ORDER=layout";; occmult) echo "KIC_F4_MULTIPLIERS=occurring";;
 alloff) echo "IC_TEMPLATE_MEMO=0 KIC_F4_INHERIT=0 KIC_CHAIN_ORDER=layout KIC_F4_MULTIPLIERS=occurring";; esac; }
for lc in "5 11 1 k1n11" "6 13 0 k0n13" "7 15 0 k0n15"; do set -- $lc; l=$1; n=$2; a=$3; c=$4
 for x in base memo0 nolin noinh splitlow chainlayout occmult alloff; do
  ( env $(envof $x) timeout 2400 $P $n $a $l 150 20261101 lab=1 wda=0 wdb=0 > probes_abl/$c.l$l.$x.jsonl 2> probes_abl/$c.l$l.$x.err ) &
 done
done
wait; echo done > probes_abl/DONE
