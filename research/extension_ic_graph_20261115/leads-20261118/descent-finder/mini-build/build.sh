#!/usr/bin/env bash
# Rebuild the descent-finder local control from scratch.
#
# Why a mini build: a full `cargo build --lib` of the crate does not fit on
# the 2026-11-18 host (root volume at 0-500 MB free; the first attempt filled
# it). This assembles only the Nagao descent closure: the frozen library
# commit fcda1e22's f4_fp/zero_dim_la/jv_bitops/bitops/ops sources unchanged,
# the lane's nagao-e2e jv_cover overlay (NagaoSolver callback), and six helper
# shims of byte-copied items (assemble.py lists them; ops.rs drops one unused
# function). It is a declared hybrid boundary, not the frozen library build
# and not a current-main Cargo build.
#
# Usage: build.sh OUT_DIR   (OUT_DIR on a volume with ~300 MB free)
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
lane=$(cd "$here/.." && pwd)
out=${1:?usage: build.sh OUT_DIR}
# fetch_frozen.sh and assemble.py write /dev/shm/frozen and /dev/shm/mini2.
bash "$here/fetch_frozen.sh"
python3 "$here/assemble.py"
sed "s#TRACED_CHECK_PATH#$lane/traced_descent_check.rs#" "$here/Cargo.toml.in" > /dev/shm/mini2/Cargo.toml
cp "$here/Cargo.lock" /dev/shm/mini2/Cargo.lock
(cd /dev/shm/mini2 && CARGO_TARGET_DIR="$out" cargo build --offline --locked --release)
rustc -O --edition=2021 "$lane/descent_breakdown.rs" -o "$out/descent_breakdown"
sha256sum "$out/release/traced_descent_check" "$out/descent_breakdown"
