#!/usr/bin/env bash
# Remaining curves (option 2: session kept active). Wave 1: 16411 curves 172, 2640 (2 threads each).
# Wave 2: 8209 curves 393, 474 (1 thread each) and 16411 curve 180 (2 threads). Suffix r2.
set -uo pipefail
d=research/genus4_lean_rho_20261116
P=$d/percurve.sh
bash $P 16411 172 2 5e8 r2 & bash $P 16411 2640 2 5e8 r2 & wait
bash $P 8209 393 1 1.2e8 r2 & bash $P 8209 474 1 1.2e8 r2 & bash $P 16411 180 2 5e8 r2 & wait
echo waves-done > $d/waves.done
