#!/usr/bin/env bash
# L5 measured runs (seed 20261114), frozen grid of PROTOCOL.md §3. Sequential, 1 core.
set -euo pipefail
SP=/tmp/claude-0/-home-user/e01afaa4-f4f8-52ad-ba5a-6c307eb9b47e/scratchpad
B=$SP/lanes/L5/target/release/ffcop
D=$(cd "$(dirname "$0")" && pwd)/raw
S=20261114
ARMS=s3p,s3u,genp,genu,gsp,gsu,e3p,e3u,k2p
run() { local out=$1; shift; echo "[$(date -u +%H:%M:%S)] $out: $*"; timeout 7200 prlimit --as=2000000000 "$B" "$@" > "$D/$out"; }
for n in 13 19 29 37; do lm=$(( (n+1)/2 + 3 )); N=$([ $n = 13 ] && echo 500 || ([ $n = 19 ] && echo 300 || ([ $n = 29 ] && echo 200 || echo 60))); run exist_n$n.jsonl exist $n $lm $N $S; done
for n in 13 19 29 37 53 61 131; do
  hi=$(( (n+1)/2 + 1 )); [ $hi -gt 36 ] && hi=36
  run lat_n$n.jsonl lattice $n 1:5 2:$hi 10 $S $ARMS
done
run lat_n131_half.jsonl lattice 131 1:5 66 10 $S $ARMS
for d in 2 3; do for n in 29 61 131; do hi=$(( (n+d-1)/d + 1 )); [ $hi -gt 58 ] && hi=58; run uni_d${d}_n$n.jsonl uni $n $d 1:4 2:$hi 5 $S; done; done
for n in 29 61 131; do run kobaug_n$n.jsonl kobaug $n 2,3 $((n/8)),$((n/7)),$((n/6)),$((n/6+1)) 3 $S; done
run aa_lat_n61_k3_a.jsonl lattice 61 3 2:32 10 $S $ARMS
run aa_lat_n61_k3_b.jsonl lattice 61 3 2:32 10 $S $ARMS
for n in 61 131; do run highk_n$n.jsonl lattice $n 6:8 $(( (n+7)/8 )):$(( (n+4)/5 )) 3 $S s3p,gsp,genp; done
run exist_n53.jsonl exist 53 24 10 $S
echo DONE
