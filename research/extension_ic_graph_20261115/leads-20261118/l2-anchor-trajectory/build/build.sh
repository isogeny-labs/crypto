#!/usr/bin/env bash
# Scoped build for the l2-anchor-trajectory lane. Reuses the l1-steps-2-3 fetch and
# assemble steps unchanged (read-only), then adds one patched copy of Run50's
# deferred collector (patch_l2.py) as crypto_lib::run50::l2_deferred_pair_dlp.
# Usage: SCRATCH=/dir OUT=/dir build.sh
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
lane=$(cd "$here/.." && pwd)
l1=$(cd "$lane/../l1-steps-2-3" && pwd)
: "${SCRATCH:?set SCRATCH}" "${OUT:?set OUT}"
export RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN:-1.98}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/home/user/cargo-target-round2}
export CARGO_INCREMENTAL=0
avail=$(df --output=avail -BG / | tail -1 | tr -dc 0-9)
if [ "$avail" -lt 5 ]; then echo "refusing to build: ${avail}G free on /"; exit 3; fi
SCRATCH="$SCRATCH" bash "$l1/build/fetch.sh"
python3 "$l1/build/assemble.py" "$SCRATCH"
R="$SCRATCH/mini23/src/run50"
python3 "$here/patch_l2.py" "$R/jv_extension_deferred_pair_dlp.rs" "$R/jv_l2_deferred_pair_dlp.rs"
printf '#[path = "jv_l2_deferred_pair_dlp.rs"]\npub mod l2_deferred_pair_dlp;\n' >> "$R/mod.rs"
sed "s#LANE#$lane#g" "$here/Cargo.toml.in" > "$SCRATCH/mini23/Cargo.toml"
cp "$l1/build/Cargo.lock" "$SCRATCH/mini23/Cargo.lock"
mkdir -p "$OUT"
cd "$SCRATCH/mini23"
cargo build --offline --release -j 1 --bins 2>&1 | grep -E "^(error|warning: unused)|^\s+-->|Finished|error\[" | head -80
for b in l2_collect l2_model; do cp "$CARGO_TARGET_DIR/release/$b" "$OUT/$b"; done
sha256sum src/run50/*.rs Cargo.toml Cargo.lock "$lane"/*.rs "$here"/*.py "$OUT"/l2_collect "$OUT"/l2_model
rustc --version; cargo --version
