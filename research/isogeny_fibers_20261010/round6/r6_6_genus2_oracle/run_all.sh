#!/bin/sh
# R6.6 runs (single-threaded, shared host). Build first:
#   export CARGO_TARGET_DIR=…/scratchpad/r6target CARGO_BUILD_JOBS=1
#   cargo build --release --example r6_6_genus2_oracle
set -e
B=${CARGO_TARGET_DIR:?}/release/examples/r6_6_genus2_oracle
D=$(dirname "$0")/raw
mkdir -p "$D"
# C2: f = x^5 + b splits completely, J[2] rational, h = 16 (Richelot available)
for pb in 1051:1042 2131:32 4111:2172; do
  p=${pb%%:*}; b=${pb##*:}
  $B --p $p --b $b --size 80 --reps 40 --seed 1 --arms plain,A,R,G,plainP,phi_plain,phi_A \
     --json "$D/census_c2_p${p}_s80.json" 2> "$D/census_c2_p${p}_s80.log"
done
# C1: b square, f has no rational root, h = 5 (no rational (2,2)-kernel)
for pb in 1051:49 2131:4 4111:3984; do
  p=${pb%%:*}; b=${pb##*:}
  $B --p $p --b $b --size 80 --reps 40 --seed 2 --arms plain,A,G \
     --json "$D/census_c1_p${p}_s80.json" 2> "$D/census_c1_p${p}_s80.log"
done
