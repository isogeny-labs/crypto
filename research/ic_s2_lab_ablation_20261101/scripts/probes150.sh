#!/bin/bash
cd /Volumes/SSD990-2/s2-build; export TMPDIR=$PWD/tmp WDSAT_BIN=$PWD/bins
P=target/release/examples/s2_split_probe
for lc in "5 11 1 k1n11" "8 11 1 k1n11" "6 13 0 k0n13" "8 13 0 k0n13" "7 15 0 k0n15" "10 15 0 k0n15"; do set -- $lc
  l=$1; n=$2; a=$3; c=$4
  ( timeout 3000 $P $n $a $l 150 20261101 lab=1 wda=1 wdb=0 > probes150/$c.l$l.laba.jsonl 2> probes150/$c.l$l.laba.err ) &
  ( timeout 3000 $P $n $a $l 150 20261101 lab=0 wda=0 wdb=1 > probes150/$c.l$l.b.jsonl 2> probes150/$c.l$l.b.err ) &
done
wait; echo PROBES-DONE > probes150/DONE
