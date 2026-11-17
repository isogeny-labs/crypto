#!/usr/bin/env bash
# Regenerate lean/Certificates.lean and lean/certificates.json from the
# committed band sums named in scripts/lean_certificates.json (PROTOCOL.md,
# "What is proven in Lean", item 3), with the native generator
# examples/ic_planner_exhaustive_lean_certs.rs. CI runs this and requires no diff.
set -euo pipefail
cd "$(dirname "$0")/.."
cargo run --release --example ic_planner_exhaustive_lean_certs -- \
  scripts/lean_certificates.json lean/Certificates.lean lean/certificates.json
