#!/bin/sh
# Exact census runs for lead R6.8 (native Rust; Python only plots afterwards).
# Usage: from the repository root,
#   export CARGO_TARGET_DIR=<target> CARGO_BUILD_JOBS=1
#   sh research/isogeny_fibers_20261010/round6/r6_8_transport_oracle/run.sh
set -eu
OUT=research/isogeny_fibers_20261010/round6/r6_8_transport_oracle
EX="${EX_BIN:-cargo +1.98 run --release --example r6_8_transport_oracle --}"
for SIZE in 72 144; do
  $EX --family j0 --p 195709 --b 3 --size $SIZE --seed 7 > "$OUT/census_j0_p18_s$SIZE.json"
  $EX --family j0 --p 493159 --b 5 --size $SIZE --seed 7 > "$OUT/census_j0_p19_s$SIZE.json"
  $EX --family j0 --p 903163 --b 4 --size $SIZE --seed 7 > "$OUT/census_j0_p20_s$SIZE.json"
  $EX --family 6r --pmin 1048576 --size $SIZE --seed 7 > "$OUT/census_6r_p20_s$SIZE.json"
done
