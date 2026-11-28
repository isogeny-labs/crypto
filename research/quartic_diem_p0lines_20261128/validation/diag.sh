#!/bin/sh
# Validator: the binary's uncounted per-attempt diagnosis (--quartic-diag, and the core dump
# QL_CORE_DUMP) on the grid curves that hold multi-attempt runs, arms nofb and p0, solver
# seeds 1-6 (lesson L-4c4c2ac6ef29: replay a dev-only mechanism diagnosis on grid rows).
# Flags are the frozen ones of run_main.sh / run_131101.sh plus --quartic-diag, with
# --quartic-arms and --quartic-seeds restricted.  diag_check.py requires every committed
# field to be unchanged by the flag, then reads the per-attempt ranks and the dumped cores.
#   BIN : the frozen binary (checked against binary.sha256's hash)
#   OUT : validator scratch directory (never the study's raw/)
#   PART: main (4099, 16411, 65537) | 131101 ; always 1 thread
set -e
BIN=$1
OUT=$2
PART=$3
mkdir -p "$OUT"
echo "283c1510a9da8d735495a44aafe3685b174029987f10dc10adb0ecffb23723cd  $BIN" | sha256sum -c -
if [ "$PART" = main ]; then
  QL_CORE_DUMP="$OUT/cores_main.jsonl" "$BIN" run --primes 4099,16411,65537 --genera 2 --arms - \
    --curve-seeds 4099=1,16411=1,65537=1 \
    --quartic --quartic-arms 2lp-sfb-sge-nofb,2lp-sfb-sge-p0 --quartic-inhom 4 --quartic-sge 8,1.5,64 \
    --quartic-seeds 4099=205,16411=93/158/252/366/382,65537=57/156/166/369 --quartic-diag \
    --qc 3.0 --max-h 1 --curves 5 --seeds 6 --rho-cap 1 --threads 1 --out "$OUT/diag_main.jsonl"
fi
if [ "$PART" = 131101 ]; then
  QL_CORE_DUMP="$OUT/cores_131101.jsonl" "$BIN" run --primes 131101 --genera 2 --arms - --curve-seeds 131101=1 \
    --quartic --quartic-arms 2lp-sfb-sge-nofb,2lp-sfb-sge-p0 --quartic-inhom 4 --quartic-sge 8,1.5,64 \
    --quartic-seeds 131101=53/177/288 --quartic-diag \
    --qc 3.0 --max-h 1 --curves 5 --seeds 6 --rho-cap 1 --threads 1 --out "$OUT/diag_131101.jsonl"
fi
echo DIAG_DONE "$PART"
