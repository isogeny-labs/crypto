#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=${1:?fresh attempt};raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/FIBER51.rs" "$raw/input.rs";cp "$0" "$raw/runner.sh"
sha256sum "$raw/input.rs" "$raw/runner.sh" > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
rustc --version > "$raw/version.txt"
set +e
(cd "$raw";ulimit -v 4194304;timeout 1200 bash -c 'rustc --edition=2021 -C codegen-units=1 -o fiber51 input.rs && ./fiber51') > "$raw/native.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";set -e
if [ "$rc" != 0 ] || ! rg -qx FIBER51_ALL_CHECKS_PASS "$raw/native.log";then echo "FAIL $attempt native=$rc";exit 1;fi
echo "PASS $attempt"
