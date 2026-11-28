#!/bin/sh
# Validator replay of selected cells with the recorded binary, one thread.
# Usage: replay.sh BINARY OUTDIR
set -u
B=$1
O=$2
sha256sum "$B" > "$O/binary.sha256.replay"
run() { # name p seed
  nice -n 10 "$B" run --primes "$2" --genera 2 --arms - --curve-seeds "$2"=1 \
    --quartic --quartic-arms 2lp,2lp-alg,2lp-kick,2lp-new --quartic-inhom 4 \
    --quartic-seeds "$2"="$3" --qc 3.0 --max-h 1 --curves 1 --seeds 2 \
    --rho-cap 1000 --threads 1 --out "$O/$1.jsonl" > "$O/$1.log" 2>&1
  echo "EXIT $? $(date -u +%FT%TZ)" >> "$O/$1.log"
}
run r1031_244 1031 244
run r4099_220 4099 220
run r8209_114 8209 114
run r16411_93 16411 93
run r32771_217 32771 217
run r65537_156 65537 156
run r65537_13 65537 13
echo ALLDONE > "$O/ALLDONE"
