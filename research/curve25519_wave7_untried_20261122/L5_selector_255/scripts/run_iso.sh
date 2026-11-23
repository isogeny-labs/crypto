#!/usr/bin/env bash
# Arm I (PROTOCOL.md): isogeny-tower seed selector vs t, via the archived engine's iso-census
# (the isogeny-seed study's instrument; prime-isogeny:t=T,seed=S; J by [r]-images; exact covered).
# The binary builds only family-rule miniatures: k = 20 (A = 926) and k = 24 (A = 318, curve k24r).
# Usage: run_iso.sh <outdir> <seed_hi> <k:t> [<k:t> ...]     e.g. run_iso.sh data/iso 1000 20:4 20:5 24:9
set -euo pipefail
B=${BIN:-/home/user/bin-archive/curve25519_family-5abc70ba}
out=$1; hi=$2; shift 2
mkdir -p "$out"
sha256sum "$B" > "$out/binary.sha256"
for kt in "$@"; do
  k=${kt%%:*}; t=${kt##*:}
  s=$(date +%s)
  "$B" iso-census "$k" "$t" 0 "$hi" > "$out/iso_k${k}_t${t}.jsonl"
  echo "{\"event\":\"iso_cell\",\"k\":$k,\"t\":$t,\"seeds\":$hi,\"seconds\":$(( $(date +%s) - s ))}"
done
