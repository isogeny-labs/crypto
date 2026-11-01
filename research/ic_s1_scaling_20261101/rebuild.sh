#!/bin/bash
B=/Volumes/SSD990-2/s1-build/s1b-20261101
cd $B/tree
export TMPDIR=$B/tmp CARGO_TARGET_DIR=$B/target WDSAT_BIN_DEFAULT=$B/bins WDSAT_TMP_DEFAULT=$B/tmp
cargo build --release --offline --bin ecbench --example wdsat_probe > $B/build.log 2>&1
echo "BUILD-DONE rc=$?" >> $B/build.log
