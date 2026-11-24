#!/bin/sh
# Validator replay of research/quartic_diem_la_20261123 with the frozen binary.
# Flags are the frozen ones of run_main.sh / run_131101.sh, with only
# --quartic-seeds (and, for 131101, --curves to match) restricted; --seeds 2 is kept
# so every replayed (curve, solver seed) has both arms.  Output goes to OUT
# (validator scratch), never to raw/.
#   BIN: hash-checked copy of genus_ladder.qla (binary.sha256: adddfbfa...d6d1f6)
#   OUT: output directory
#   PART: main | 131101
#   SEEDS131101: slash-separated curve seeds at 131101 (PART=131101 only)
set -e
BIN=$1
OUT=$2
PART=$3
mkdir -p "$OUT"
echo "adddfbfa0bf2cdfd115a1fff510fa29753411c60c940dc0b5ca916b0e5d6d1f6  $BIN" | sha256sum -c -
if [ "$PART" = main ]; then
  # All 15 main-grid curves (a superset of the dispatch's sample), 2 threads as frozen.
  "$BIN" run --primes 4099,16411,65537 --genera 2 --arms - \
    --curve-seeds 4099=1,16411=1,65537=1 \
    --quartic --quartic-arms 2lp-sfb,2lp-sfb-sge --quartic-inhom 4 --quartic-sge 8,1.5,64 \
    --quartic-seeds 4099=35/205/220/257/295,16411=93/158/252/366/382,65537=13/57/156/166/369 \
    --qc 3.0 --max-h 1 --curves 5 --seeds 2 --rho-cap 1 --threads 2 --out "$OUT/replay_main.jsonl"
fi
if [ "$PART" = 131101 ]; then
  S=$4
  N=$(echo "$S" | tr '/' '\n' | wc -l)
  "$BIN" run --primes 131101 --genera 2 --arms - --curve-seeds 131101=1 \
    --quartic --quartic-arms 2lp-sfb,2lp-sfb-sge --quartic-inhom 4 --quartic-sge 8,1.5,64 \
    --quartic-seeds 131101=$S \
    --qc 3.0 --max-h 1 --curves $N --seeds 2 --rho-cap 1 --threads 1 --out "$OUT/replay_131101.jsonl"
fi
echo REPLAY_DONE "$PART"
