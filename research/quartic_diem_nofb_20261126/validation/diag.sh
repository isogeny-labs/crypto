#!/bin/sh
# Validator: the binary's uncounted per-attempt rank diagnosis (--quartic-diag) on every
# curve with a fallback pair, both arms, all four solver seeds (lesson L-4c4c2ac6ef29).
# Flags are the frozen ones of run_main.sh / run_131101.sh plus --quartic-diag, with
# --quartic-seeds restricted to the fallback curves.  diag_check.py requires every
# committed field to be unchanged by the flag, then reads the per-attempt ranks.
#   BIN : the frozen binary (checked against binary.sha256's hash)
#   OUT : validator scratch directory (never the study's raw/)
#   PART: main (16411, 65537; 1 thread) | 131101 (THREADS threads, default 1)
set -e
BIN=$1
OUT=$2
PART=$3
THREADS=${4:-1}
mkdir -p "$OUT"
echo "55ebb4404a070356bbd35612737aa31f00a9f0edfa8e72ab26b4a02287631602  $BIN" | sha256sum -c -
if [ "$PART" = main ]; then
  "$BIN" run --primes 16411,65537 --genera 2 --arms - --curve-seeds 16411=1,65537=1 \
    --quartic --quartic-arms 2lp-sfb-sge,2lp-sfb-sge-nofb --quartic-inhom 4 --quartic-sge 8,1.5,64 \
    --quartic-seeds 16411=93/158/252/366/382,65537=156/369 --quartic-diag \
    --qc 3.0 --max-h 1 --curves 5 --seeds 4 --rho-cap 1 --threads 1 --out "$OUT/diag_main.jsonl"
fi
if [ "$PART" = 131101 ]; then
  "$BIN" run --primes 131101 --genera 2 --arms - --curve-seeds 131101=1 \
    --quartic --quartic-arms 2lp-sfb-sge,2lp-sfb-sge-nofb --quartic-inhom 4 --quartic-sge 8,1.5,64 \
    --quartic-seeds 131101=53/177/228/288 --quartic-diag \
    --qc 3.0 --max-h 1 --curves 5 --seeds 4 --rho-cap 1 --threads "$THREADS" --out "$OUT/diag_131101.jsonl"
fi
echo DIAG_DONE "$PART"
