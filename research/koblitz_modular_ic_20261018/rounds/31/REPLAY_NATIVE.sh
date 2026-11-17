#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=${1:?fresh attempt};program=${2:?native bin};raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp -a "$here/native" "$raw/source";cp "$0" "$raw/runner.sh"
(cd "$raw/source";rg --files | sort | xargs sha256sum) > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
date -u +%FT%TZ > "$raw/started-utc.txt"
set +e
(ulimit -v 4194304;timeout 1200 env CARGO_TARGET_DIR=/var/tmp/koblitz-r31-target CARGO_BUILD_JOBS=1 cargo build --offline --locked --release --manifest-path "$raw/source/Cargo.toml" --bin "$program") > "$raw/compile.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/compile.exit"
if [ "$rc" = 0 ];then (ulimit -v 4194304;timeout 1200 "/var/tmp/koblitz-r31-target/release/$program" "$raw/certificates") > "$raw/native.log" 2>&1;rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";fi
date -u +%FT%TZ > "$raw/finished-utc.txt"
set -e
[ "$rc" = 0 ];rg -qx "${program^^}_ALL_CHECKS_PASS" "$raw/native.log"
echo "PASS $attempt"
