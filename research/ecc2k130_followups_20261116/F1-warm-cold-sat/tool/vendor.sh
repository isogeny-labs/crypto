#!/bin/sh
# Copy cadical-sys 0.8.0 from the cargo registry to the lane scratch directory and apply the
# F1 bridge patch (saved-phase getter/seeder; CaDiCaL sources untouched). Thin orchestration only.
set -eu
SP=/tmp/claude-0/-home-user/e01afaa4-f4f8-52ad-ba5a-6c307eb9b47e/scratchpad/lanes/F1
SRC=$(ls -d "$HOME"/.cargo/registry/src/*/cadical-sys-0.8.0 | head -1)
mkdir -p "$SP/vendor"
rm -rf "$SP/vendor/cadical-sys"
cp -r "$SRC" "$SP/vendor/cadical-sys"
cd "$SP/vendor/cadical-sys"
sed -i 's/\r$//' src/bridge.rs src/cadical_bridge.hpp src/lib.rs
patch -p0 < "$(dirname "$0")/cadical-sys-f1.patch"
