#!/usr/bin/env bash
# Regenerate lean/Certificates.lean and lean/certificates.json from the
# committed band sums named in scripts/lean_certificates.json (PROTOCOL.md,
# "What is proven in Lean", item 3). CI runs this and requires no diff.
set -euo pipefail
cd "$(dirname "$0")/.."
python3 scripts/lean_certificates.py scripts/lean_certificates.json lean/Certificates.lean lean/certificates.json
