#!/bin/sh
# Validator replay of research/quartic_diem_ladder_20261121 with the frozen binary,
# one thread, the frozen flags of run_main.sh / run_131101_*.sh, curve seeds listed
# directly (same admission path make_instance(q, s, s, 1), no search).
#   BIN: hash-checked copy of genus_ladder.qsfb (sha256 f0243cf8...e38b)
#   OUT: output directory (validator scratch)
# 131101: all five curves.  Other primes: one curve each, drawn with
# random.Random(20261123).choice over the five admitted curves (rho runs at p <= 16411).
set -e
BIN=$1
OUT=$2
mkdir -p "$OUT"
echo "f0243cf85a90ddb630f7ad868a4375fffae1189aff959a71da3c901a49c9e38b  $BIN" | sha256sum -c -
COMMON="--genera 2 --arms - --quartic --quartic-arms 2lp-new,2lp-sfb --quartic-inhom 4 --qc 3.0 --max-h 1 --seeds 2 --rho-cap 2e8 --threads 1"
"$BIN" run --primes 131101 --curve-seeds 131101=1 --quartic-seeds 131101=53/177/193 --curves 3 $COMMON --out "$OUT/replay_131101_odd.jsonl"
"$BIN" run --primes 131101 --curve-seeds 131101=1 --quartic-seeds 131101=228/288 --curves 2 $COMMON --out "$OUT/replay_131101_even.jsonl"
"$BIN" run --primes 1031,2053,4099,8209,16411,32771,65537 \
  --curve-seeds 1031=1,2053=1,4099=1,8209=1,16411=1,32771=1,65537=1 \
  --quartic-seeds 1031=317,2053=105,4099=205,8209=16,16411=382,32771=32,65537=369 \
  --curves 1 $COMMON --out "$OUT/replay_main_sample.jsonl"
echo REPLAY_DONE
