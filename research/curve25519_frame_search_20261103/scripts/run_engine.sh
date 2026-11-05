#!/usr/bin/env bash
# P5 (PROTOCOL.md): F4 engine, k = 20, bits vs bits@L* (L* from confirm/k20.json), G = 3, 100 targets.
# Extra (reported, not preregistered): t = 5, G = 0.
set -euo pipefail
cd "$(dirname "$0")/.."
BIN=${BIN:-/home/user/ctarget/release/curve25519_family}
sha256sum "$BIN" > engine/binary.sha256
L5=976503; L6=908172
cell() { local form=$1 t=$2 g=$3 tag=$4
  timeout 1800 "$BIN" f4cost 20 "$form" $t $g 100 $((2026111500+t)) 900 2 full 0 30000000 > engine/${tag}_t${t}_g${g}.json 2> engine/${tag}_t${t}_g${g}.stderr; }
cell bits 5 3 bits; cell bits@$L5 5 3 frame
cell bits 6 3 bits; cell bits@$L6 6 3 frame
cell bits 5 0 bits; cell bits@$L5 5 0 frame
sha256sum engine/*.json > engine/SHA256SUMS
