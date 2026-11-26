#!/bin/sh
# Completes the m = 5 rows of the frozen grid below the cells grid_down ran
# (alpha 0.02 .. 0.08), so no grid cell is skipped. Same binary as run_rest.sh.
set -eu
cd "$(dirname "$0")"
B=./target/run-bin
for a in 2 4 6; do $B cell S3 32 5 $a; done
for a in 2 4 6 8; do $B cell S3 48 5 $a; done
for a in 2 4 6 8; do $B cell S3 64 5 $a; done
echo FILLDONE
