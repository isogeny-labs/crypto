#!/bin/sh
# Reproduce both sweeps.  Prime runs from the crypto-autoresearcher checkout;
# the analytic budget runs from a cryptanalysis worktree at the recorded commit.
set -eu
# Run in a scratch copy: the --out files are appended to, not replaced.
HERE=$(cd "$(dirname "$0")" && pwd)
OUTDIR="${OUTDIR:-$HERE}"
mkdir -p "$OUTDIR/prime"

# ---- prime, measured (crypto-autoresearcher 480187a0d1, Python 3.12.8) ----
cd "${AUTORESEARCHER:-/Volumes/SSD990-2/merge-work/crypto-autoresearcher}"

python3 -m crypto_autoresearcher.index_calculus sweep \
    --bits 12 14 16 18 --m 2 3 4 5 6 7 8 9 --fb small_x \
    --curves 3 --rho-curves 3 --reps 500 --workers 1 --engine enumerate \
    --quiet --json --out "$OUTDIR"/prime/sweep.jsonl > "$OUTDIR"/prime/sweep.json
python3 -m crypto_autoresearcher.index_calculus sweep \
    --bits 12 14 16 18 --m 2 3 4 5 6 7 8 9 --fb small_x \
    --curves 3 --rho-curves 3 --reps 500 --workers 1 --engine mitm \
    --quiet --json --out "$OUTDIR"/prime/sweep-mitm.jsonl > "$OUTDIR"/prime/sweep-mitm.json

# The 32-bit extension: curves 0 and 1 from `sweep`, curve 2 separately.
python3 -m crypto_autoresearcher.index_calculus sweep \
    --bits 32 --m 5 6 7 8 9 --fb small_x --curves 2 --rho-curves 2 \
    --reps 500 --workers 1 --engine mitm \
    --quiet --json --out "$OUTDIR"/prime/sweep-mitm-32.jsonl > /dev/null
python3 "$HERE"/scripts/run_curve.py \
    --bits 32 --curve 2 --m 5 6 7 8 9 --engine mitm \
    --out "$OUTDIR"/prime/sweep-mitm-32.jsonl

python3 "$HERE"/scripts/dump_factor_bases.py \
    --rows "$OUTDIR"/prime/sweep.jsonl "$OUTDIR"/prime/sweep-mitm.jsonl "$OUTDIR"/prime/sweep-mitm-32.jsonl \
    --out "${FACTOR_BASE_ROOT:-/Volumes/data/ic-arity-sweep-20261027}"

# ---- binary/Koblitz, analytic (cryptanalysis 0fa835a2a13e) ----
# Build ca-ic in a worktree of the suite, then:
#   ca-ic budget --standard --max-arity 9 --json --out budget/standard/koblitz.json
#   ca-ic budget --standard --max-arity 9 --no-frobenius --json \
#       --out budget/standard/binary-generic.json
#   for n in 11 13 15 17 19 23 29 31 37 39 41; do
#       ca-ic budget --degree "$n" --max-arity 9 --json \
#           --out "budget/toy/koblitz/n$n.json"; done
#   for n in 15 18 21 24 27; do
#       ca-ic budget --degree "$n" --max-arity 9 --no-frobenius --json \
#           --out "budget/toy/binary/n$n.json"; done
