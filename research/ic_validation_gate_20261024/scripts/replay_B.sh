#!/bin/sh
# Replay of Table B in a fresh process, resumable (skips finished outputs). Run from the repo root.
# usage: replay_B.sh <ic-binary> <outdir>
# Frozen configuration = the archive's config block (seed 123212651130, 3 repeats, ladders as listed).
set -e
IC="$1"; OUT="$2"; mkdir -p "$OUT"
[ -s "$OUT/prime.json" ] || RAYON_NUM_THREADS=1 "$IC" boundary --regime prime --prime-bits 10,12,14,16,18,20,22,24 \
  --repeats 3 --seed 123212651130 --json --out "$OUT/prime.json" > /dev/null 2> "$OUT/prime.err"
[ -s "$OUT/char2.json" ] || RAYON_NUM_THREADS=1 "$IC" boundary --regime char2 --char2-degrees 15,18,21,24,27 \
  --repeats 3 --seed 123212651130 --json --out "$OUT/char2.json" > /dev/null 2> "$OUT/char2.err"
# The current binary writes the matched (negation, A = 2) and the frozen plain walks into the ledger's
# own `rho` list, interleaved, in the same process as the IC variants.  Separately, re-run the archive's
# re-pricing of the archived ledger on this binary: it replays the frozen walk on the recorded seeds
# (identity check) and recomputes the matched reference.
[ -s "$OUT/archive.reprice.json" ] || RAYON_NUM_THREADS=1 "$IC" rho --reprice docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json --out "$OUT/archive.reprice.json" > /dev/null 2> "$OUT/archive.reprice.err"
echo done
