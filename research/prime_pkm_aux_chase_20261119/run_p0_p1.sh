#!/bin/zsh
set -euo pipefail
export RUSTUP_TOOLCHAIN=1.99.1
export PATH="$HOME/.cargo/bin:/opt/homebrew/bin:/usr/bin:/bin:$PATH"
BIN=/Volumes/SSD990-2/cargo-target-lead-1a/release/prime-pkm-aux-chase-20261119
OUT=/Volumes/SSD990-2/wt-3858/crypto-pkm/research/prime_pkm_aux_chase_20261119
exec "$BIN" --suites p0_1c,p1_3a "$OUT/RESULTS.json" > "$OUT/RUN_P0_P1.log" 2>&1
