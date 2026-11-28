#!/usr/bin/env bash
# Fresh standalone proof qualification against an explicitly pinned existing Mathlib cache.
set -euo pipefail
OUT_ARG=$1
MATHLIB_ROOT=$2
MATHLIB_DEPENDENCY_PATH=$3
REPO=$(git rev-parse --show-toplevel)
STUDY="$REPO/research/hashed_ecdl_precision_history_20261123"
CORE="$REPO/research/hashed_ecdl_precision_20261123/formal/CryptoLeanCore/TaperedPrecision.lean"
test "$(git -C "$MATHLIB_ROOT" rev-parse HEAD)" = 56a005855ea17e0b0958f73ec801ae140b0a1679
case "$MATHLIB_DEPENDENCY_PATH" in *'/checks/'*) exit 2;; esac
mkdir "$OUT_ARG"
OUT=$(cd "$OUT_ARG" && pwd)
mkdir -p "$OUT/input/CryptoLeanCore" "$OUT/lib/CryptoLeanCore"
lean --version > "$OUT/lean-version.txt"
rg -q 'version 4\.35\.0,' "$OUT/lean-version.txt"
lean --print-prefix > "$OUT/lean-prefix.txt"
IFS= read -r LEAN_PREFIX < "$OUT/lean-prefix.txt"
export LEAN_PATH="$OUT/lib:$MATHLIB_DEPENDENCY_PATH:$LEAN_PREFIX/lib/lean"
printf '%s\n' "$LEAN_PATH" > "$OUT/lean-path.txt"
printf '%s  %s\n' 92e4300c71c4b1570ab193df4de22fe31b04a9d7ca0f0212191169555951989a "$CORE" | sha256sum -c - > "$OUT/core-source-check.txt"
cp "$CORE" "$OUT/input/CryptoLeanCore/TaperedPrecision.lean"
cp "$STUDY/formal/PrecisionHistory.lean" "$OUT/input/PrecisionHistory.lean"
cp "$STUDY/formal/PrecisionHistoryControls.lean" "$OUT/input/PrecisionHistoryControls.lean"
cp "$STUDY/reproduction/Axioms.source" "$OUT/input/Axioms.lean"
chmod a-w "$OUT/input/CryptoLeanCore/TaperedPrecision.lean" "$OUT/input/PrecisionHistory.lean" "$OUT/input/PrecisionHistoryControls.lean" "$OUT/input/Axioms.lean"
cd "$OUT/input"
sha256sum CryptoLeanCore/TaperedPrecision.lean PrecisionHistory.lean PrecisionHistoryControls.lean Axioms.lean > "$OUT/input-before.sha256"
lean -j1 -o "$OUT/lib/CryptoLeanCore/TaperedPrecision.olean" CryptoLeanCore/TaperedPrecision.lean > "$OUT/core-build.stdout" 2> "$OUT/core-build.stderr"
lean -j1 -o "$OUT/lib/PrecisionHistory.olean" PrecisionHistory.lean > "$OUT/history-build.stdout" 2> "$OUT/history-build.stderr"
lean -j1 -o "$OUT/lib/PrecisionHistoryControls.olean" PrecisionHistoryControls.lean > "$OUT/controls-build.stdout" 2> "$OUT/controls-build.stderr"
lean -j1 Axioms.lean > "$OUT/axioms.stdout" 2> "$OUT/axioms.stderr"
sha256sum -c "$OUT/input-before.sha256" > "$OUT/input-after-check.txt"
cd "$REPO"
python3 tools/pcc/pcc.py verify pcc/claims/PCC-20261123-dc026e.json --json > "$OUT/pcc-verdict.json"
bash tools/lean/proof-gate.sh --out "$OUT/control-gate-receipts" "$STUDY/formal/PrecisionHistoryControls.lean" > "$OUT/control-gate.stdout" 2> "$OUT/control-gate.stderr"
printf 'Fresh finite-history qualification passed; eight controls, no solver runs.\n'
