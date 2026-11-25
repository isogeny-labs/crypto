#!/usr/bin/env bash
set -euo pipefail
R=/tmp/ecdlp-leaf-guard-20261123
LEAN=/tmp/ecdlp-proof-check-20261123/lean-4.35.0-linux/bin/lean-kernel-original
cd "$R/source-owner"
"$LEAN" -j1 -M0 -o "$R/lib/CryptoLeanCore/TaperedPrecision.olean" CryptoLeanCore/TaperedPrecision.lean
cd "$R/frozen-consumer"
"$LEAN" -j1 -M0 -o "$R/lib/LeafGuard.olean" LeafGuard.lean
cd /tmp/ecdlp-port-mass-publish-20261123
bash tools/lean/proof-gate.sh --out "$R/consumer-gate" research/hashed_ecdl_leaf_guard_20261123/formal/LeafGuard.lean
cd /tmp/crypto-lean-precision-20261123
bash /tmp/ecdlp-port-mass-publish-20261123/tools/lean/proof-gate.sh --out "$R/owner-gate" CryptoLeanCore/TaperedPrecision.lean
cd "$R"
/tmp/ecdlp-proof-check-20261123/lean-4.35.0-linux/bin/leanchecker --fresh -v LeafGuard
