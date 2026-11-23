#!/usr/bin/env bash
# PROTOCOL.md (frozen 27482d63a0): seeds 0..1999, k in {16..24}, t in {4..7}.
set -euo pipefail
cd "$(dirname "$0")/.."
B=${BIN:-/home/user/ctarget/release/curve25519_family}
for k in 16 18 20 22 24; do
  ( for t in 4 5 6 7; do "$B" iso-census $k $t 0 2000; done > data/k$k.jsonl ) &
  [ $k = 22 ] && wait
done
wait
sha256sum data/*.jsonl > data/SHA256SUMS
