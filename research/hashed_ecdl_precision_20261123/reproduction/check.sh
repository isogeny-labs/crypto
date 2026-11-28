#!/usr/bin/env bash
# Thin, fresh-directory qualification using Lean/Std only.
set -euo pipefail
OUT_ARG=$1
REPO=$(git rev-parse --show-toplevel)
STUDY="$REPO/research/hashed_ecdl_precision_20261123"
mkdir "$OUT_ARG"
OUT=$(cd "$OUT_ARG" && pwd)
mkdir -p "$OUT/input/CryptoLeanCore" "$OUT/lib/CryptoLeanCore"
lean --version > "$OUT/lean-version.txt"
rg -q 'version 4\.35\.0,' "$OUT/lean-version.txt"
lean --print-prefix > "$OUT/lean-prefix.txt"
IFS= read -r LEAN_PREFIX < "$OUT/lean-prefix.txt"
export LEAN_PATH="$OUT/lib:$LEAN_PREFIX/lib/lean"
printf '%s  %s\n' 92e4300c71c4b1570ab193df4de22fe31b04a9d7ca0f0212191169555951989a "$STUDY/formal/CryptoLeanCore/TaperedPrecision.lean" | sha256sum -c - > "$OUT/shared-source-check.txt"
cp "$STUDY/formal/CryptoLeanCore/TaperedPrecision.lean" "$OUT/input/CryptoLeanCore/TaperedPrecision.lean"
cp "$STUDY/formal/PrecisionControls.lean" "$OUT/input/PrecisionControls.lean"
cp "$STUDY/reproduction/Axioms.source" "$OUT/input/Axioms.lean"
chmod a-w "$OUT/input/CryptoLeanCore/TaperedPrecision.lean" "$OUT/input/PrecisionControls.lean" "$OUT/input/Axioms.lean"
cd "$OUT/input"
sha256sum CryptoLeanCore/TaperedPrecision.lean PrecisionControls.lean Axioms.lean > "$OUT/input-before.sha256"
lean -j1 -o "$OUT/lib/CryptoLeanCore/TaperedPrecision.olean" CryptoLeanCore/TaperedPrecision.lean > "$OUT/core-build.stdout" 2> "$OUT/core-build.stderr"
lean -j1 -o "$OUT/lib/PrecisionControls.olean" PrecisionControls.lean > "$OUT/controls-build.stdout" 2> "$OUT/controls-build.stderr"
lean -j1 Axioms.lean > "$OUT/axioms.stdout" 2> "$OUT/axioms.stderr"
sha256sum -c "$OUT/input-before.sha256" > "$OUT/input-after-check.txt"
cd "$REPO"
python3 tools/pcc/pcc.py verify pcc/claims/PCC-20261123-75fa8c.json --json > "$OUT/pcc-verdict.json"
bash tools/lean/proof-gate.sh --out "$OUT/control-gate-receipts" research/hashed_ecdl_precision_20261123/formal/PrecisionControls.lean > "$OUT/control-gate.stdout" 2> "$OUT/control-gate.stderr"
printf 'Fresh Std-only qualification passed; seven scalar controls, no solver runs.\n'
