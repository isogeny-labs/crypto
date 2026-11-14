#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel);raw="$here/runs/${1:?fresh attempt}"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/ENCODE43.sing" "$raw/input.sing";cp "$here/source-data/reference-jets154.sing" "$raw/";cp "$0" "$raw/runner.sh"
sha256sum "$raw/input.sing" "$raw/reference-jets154.sing" "$raw/runner.sh" > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
set +e
(cd "$raw";ulimit -v 4194304;timeout 1200 /home/ec2-user/.cache/koblitz-native-cas/singular/bin/Singular -q input.sing) > "$raw/native.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt"
set -e
[ "$rc" = 0 ];! rg -q '^ *\?' "$raw/native.log";rg -qx ENC43_ALL_CHECKS_PASS "$raw/native.log";echo "PASS ${1}"
