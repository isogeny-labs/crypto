#!/usr/bin/env bash
# C2 (x arm only): the existing F4 engine on the bits system, m = 2, G = 0, 10 targets.
# Calibrates the M2 proxy's D_ref for x against the engine's solving_degree_max.
set -euo pipefail
cd "$(dirname "$0")/.."
OUT=${OUT:-data}
BIN=${BIN:-/home/user/bin-archive/curve25519_family-5abc70ba}
sha256sum "$BIN" > "$OUT/engine_binary.sha256"
for k in ${KS:-16 18 20}; do for T in ${TS:-4 5}; do
  timeout ${TMO:-900} "$BIN" f4cost $k bits $T 0 ${NT:-10} $((2026112200 + 10*k + T)) 900 2 full 0 30000000 \
    > "$OUT/engine_k${k}_T${T}.json" 2> "$OUT/engine_k${k}_T${T}.stderr" || echo "engine k=$k T=$T exit $?" >> "$OUT/engine.log"
done; done
