#!/usr/bin/env bash
# Broad rootfind benchmark matrix. Engineering only. [pcc:unverified]
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
EV="$(cd "$(dirname "$0")" && pwd)"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target-kmitm-fast}"
BIN="$CARGO_TARGET_DIR/release/examples"
FLOOR="$BIN/rootfind_floor_wall"
KMITM="$BIN/rootfind_kmitm_wall"
PIPE="$BIN/rootfind_pipeline_e2e"

need() { [[ -x "$1" ]] || { echo "missing binary: $1" >&2; exit 1; }; }
need "$FLOOR"; need "$KMITM"; need "$PIPE"

host() {
  {
    echo "nproc=$(nproc)"
    uname -a
    rustc +1.98.0 --version
    lscpu | rg -i 'Model name|Flags' | head -5 || true
    date -u +%Y-%m-%dT%H:%M:%SZ
  } >"$EV/host.txt"
}

run() {
  local label="$1"; shift
  echo "=== $label ==="
  "$@" >"$EV/${label}.txt" 2>"$EV/${label}.stderr"
  echo "wrote $EV/${label}.txt"
}

host

# --- Oracle wall: floor vs subtract, m=2 and m=3, multi-bit × |Fx| ---
run floor_wall_matrix \
  "$FLOOR" --bits 20,24,28,32 --sizes 64,128,256,512,1024 --ms 2,3 --targets 1024

# --- Oracle wall: kmitm m=3..12 + lanes + thr1/2/4 ---
run kmitm_wall_b20_a14_t64 \
  "$KMITM" --bits 20 --size 14 --targets 64
run kmitm_wall_b20_a64_t128 \
  "$KMITM" --bits 20 --size 64 --targets 128
run kmitm_wall_b24_a14_t64 \
  "$KMITM" --bits 24 --size 14 --targets 64
run kmitm_wall_b24_a64_t128 \
  "$KMITM" --bits 24 --size 64 --targets 128
run kmitm_wall_b24_a128_t128 \
  "$KMITM" --bits 24 --size 128 --targets 128
run kmitm_wall_b28_a14_t64 \
  "$KMITM" --bits 28 --size 14 --targets 64
run kmitm_wall_b28_a64_t128 \
  "$KMITM" --bits 28 --size 64 --targets 128
run kmitm_wall_b28_a256_t64 \
  "$KMITM" --bits 28 --size 256 --targets 64

# --- Pipeline e2e planted-log (fail-closed) ---
run e2e_b20_a14_full \
  "$PIPE" --bits 20 --size 14 --repeats 5 --multi 8 --panel all
run e2e_b20_a64_full \
  "$PIPE" --bits 20 --size 64 --repeats 3 --multi 4 --panel all
run e2e_b24_a64_full \
  "$PIPE" --bits 24 --size 64 --repeats 3 --multi 4 --panel all
run e2e_b24_a256_breakthrough \
  "$PIPE" --bits 24 --size 256 --repeats 5 --panel breakthrough --breakthrough-focus prepare
run e2e_b28_a128_breakthrough \
  "$PIPE" --bits 28 --size 128 --repeats 5 --panel breakthrough --breakthrough-focus prepare
run e2e_b28_a512_breakthrough \
  "$PIPE" --bits 28 --size 512 --repeats 5 --panel breakthrough --breakthrough-focus prepare

echo "matrix complete"
