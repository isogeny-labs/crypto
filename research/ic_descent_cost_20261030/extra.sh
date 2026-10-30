#!/bin/bash
R=$(dirname "$0")/run.sh
for spec in "41 12 16 32 1" "41 96 16 32 1" "53 20 16 32 1" "53 80 16 32 1" "61 28 16 32 1" "61 112 16 32 1" "53 280 0 32 1" "41 125 0 32 1" "61 500 0 32 1"; do $R $spec; done
echo EXTRADONE
