#!/usr/bin/env bash
set -euo pipefail
R=/tmp/ecdlp-port-mass-20261123
export PATH="/tmp/ecdlp-precision-audit-20261123/bin:$PATH"
cd /tmp/ecdlp-native-crypto-20261123
bash tools/lean/proof-gate.sh --out "$R/10-study-gate-details" research/hashed_ecdl_port_mass_20261123/formal/PortControls.lean
cd /tmp/crypto-lean-precision-20261123
bash /tmp/ecdlp-native-crypto-20261123/tools/lean/proof-gate.sh --out "$R/10-shared-gate-details" CryptoLean/Combinatorics/PortMass.lean
cd "$R"
/tmp/ecdlp-proof-check-20261123/lean-4.35.0-linux/bin/leanchecker --fresh -v PortControls
