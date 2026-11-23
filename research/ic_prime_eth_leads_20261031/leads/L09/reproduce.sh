#!/bin/sh
# L09 exact commands (run from this directory; TMPDIR must be writable; cargo offline, no installs)
set -e
L=$(pwd)
# 1. prime PDP CNFs (patched copy of research/prime_pdp_sat_sweep_20261031/sweeper; adds a `dump` subcommand)
( cd sweeper-dump && CARGO_TARGET_DIR=$TMPDIR/l09-sweeper-target cargo build --release --offline )
B=$TMPDIR/l09-sweeper-target/release/prime_pdp_sat_sweep
mkdir -p inst
for b in 16 24 32; do for m in 2 3; do for e in bits-grouplaw bits-s3; do for x in 0 1; do
  l=$(( (b+m-1)/m ))
  $B dump --bits $b --l $l --m $m --encoding $e --xor $x --out inst/pdp_b${b}_m${m}_${e}_xor${x}.cnf
done; done; done; done
gzip -9 inst/*.cnf
# 2. Bochum m=2 network (patched copy of cryptanalysis/bindings/rust/hashed-ecdl-tensor; adds Network::census_json + examples/btn_dump.rs)
( cd rust-dump && CARGO_TARGET_DIR=$TMPDIR/l09-crate-target cargo build --release --offline --example btn_dump )
mkdir -p graphs
python3 gen_prime_curves.py graphs 6,8,10,12,16
# 3. table
python3 tests/test_btn_census.py
python3 run_table.py table2
python3 compact_table.py table2.json > table_compact2.md
python3 fits.py table2.json
