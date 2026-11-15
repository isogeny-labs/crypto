#!/usr/bin/env bash
# Prepare third_party/cryptominisat-rs (gitignored) from the crates.io tarball of
# `cryptominisat 5.8.0-rust1` plus the patch in this directory. The crates.io
# registry is the one channel the lab's network policy allows; the patch adds
# conflict/decision/propagation counters and a conflict limit to the C shim and
# the Rust API, disables the sqlite statistics path whose helper files the
# tarball omits, and forces <cstdint> for current GCC. Run from anywhere.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
dst="$here/../cryptominisat-rs"
src="$(ls -d "${CARGO_HOME:-$HOME/.cargo}"/registry/src/*/cryptominisat-5.8.0 2>/dev/null | head -1 || true)"
if [ -z "$src" ]; then
  echo "cryptominisat-5.8.0 is not in the cargo registry yet; run 'cargo fetch' in tools/b0-ext-solvers first" >&2
  exit 1
fi
rm -rf "$dst"; cp -r "$src" "$dst"; rm -f "$dst/.cargo-ok"
patch -d "$dst" -p1 --quiet < "$here/0001-stats-limits-nosqlite-cstdint.patch"
echo "prepared $dst from $src"
