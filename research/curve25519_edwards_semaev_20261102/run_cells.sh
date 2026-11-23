#!/usr/bin/env bash
# Thin orchestration for the 16 frozen cells (PROTOCOL.md + Amendment 1).
# Sequential; interleaved form order; frozen binary only.
set -u
D=$(cd "$(dirname "$0")" && pwd)
BIN=/home/user/frozen-bin/curve25519_family_lead5
mkdir -p "$D/cells"
ulimit -v 3500000
for t in 4 5 6 7; do
  for g in 0 3; do
    for form in bits edwards-bits; do
      c="$D/cells/${form}_t${t}_g${g}"
      seed=$((2026111500 + t))
      { echo "start $(date -u +%FT%TZ)"; uptime; } > "$c.status"
      timeout 900 "$BIN" f4cost 20 "$form" "$t" "$g" 100 "$seed" 600 2 full 0 30000000 \
        > "$c.json" 2> "$c.stderr"
      rc=$?
      { echo "end $(date -u +%FT%TZ) rc=$rc"; uptime; } >> "$c.status"
    done
  done
done
echo done > "$D/cells/ALL_DONE"
