#!/usr/bin/env bash
# Called only after the guarded driver is terminal, including failed attempts.
set -euo pipefail
ROOT=$1
DRIVER=$2
mkdir -p "$ROOT/main"
if [ ! -e "$ROOT/main/config.json" ]; then
  cp --no-clobber "$ROOT/main-config.json" "$ROOT/main/config.json"
  printf '%s\n' 'Finalizer copied frozen config after driver did not produce one; no run/cell is reconstructed.' > "$ROOT/main/finalizer-config-reconstruction.txt"
fi
exec "$DRIVER" analyze "$ROOT/main" 10
