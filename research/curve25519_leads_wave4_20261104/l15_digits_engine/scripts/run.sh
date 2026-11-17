#!/usr/bin/env bash
# L15 (PROTOCOL.md, frozen before data): seven f4cost cells, sequential.
set -euo pipefail
cd "$(dirname "$0")/.."
B=${BIN:-/home/user/ctarget-l15/release/curve25519_family}
mkdir -p data
sha256sum "$B" > data/binary.sha256
D5='digits@113971:289589;761384;273167;963514;602728'
D6='digits@51134:293482;1015139;374525;914144;700453;505371'
cell() { # name form t G
  local n=$1 f=$2 t=$3 g=$4
  "$B" f4cost 20 "$f" "$t" "$g" 100 $((2026111500 + t)) 3600 2 full 0 30000000 > "data/$n.json" 2> "data/$n.stderr"
  echo "$(date -u +%FT%TZ) $n rc=$?" >> data/run.log
}
cell ctl_bits_lo0_t5_g3 bits 5 3
cell trans_t5_g3 bits@815271 5 3
cell trans_t5_g4 bits@815271 5 4
cell digits_t5_g3 "$D5" 5 3
cell digits_t5_g4 "$D5" 5 4
cell trans_t6_g5 bits@889850 6 5
cell digits_t6_g5 "$D6" 6 5
(cd data && sha256sum *.json binary.sha256 > SHA256SUMS)
echo DONE >> data/run.log
