#!/bin/bash
# matched rho sessions (single thread, L0); target source for the IC example
cd "$(dirname "$0")"; BIN=/home/user/crypto/target/release/ecbench; sha256sum $BIN > BINARY.sha256
for b in 21 24 27; do for r in 1 2 3; do n=B$b-r$r; [ -f sessions/$n/session.json ] && continue; rm -rf sessions/$n
 RAYON_NUM_THREADS=1 $BIN run --wait --allow-busy --spec specs/$n.json --out sessions/$n --cpus none > raw_$n.log 2>&1; echo "$n rc=$?"; done; done
