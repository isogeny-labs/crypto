#!/usr/bin/env bash
# Build the optlevel binary if it is missing or older than its sources, then run it.
#   tools/optlevel/run.sh probe | calibrate | measure | score | plot | controls | ...
# Thin build/run orchestration only; everything else is in the Rust crate beside this file.
# OPTLEVEL_BIN names a prebuilt binary to run instead (a host without cargo).
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
bin=${OPTLEVEL_BIN:-$here/target/release/optlevel}
if [ -z "${OPTLEVEL_BIN:-}" ]; then
  if [ ! -x "$bin" ] || [ -n "$(find "$here/src" "$here/kernels" "$here/schema" "$here/Cargo.toml" "$here/Cargo.lock" -newer "$bin" -print -quit)" ]; then
    cargo build --release --locked --manifest-path "$here/Cargo.toml" --target-dir "$here/target" >&2
  fi
fi
exec "$bin" "$@"
