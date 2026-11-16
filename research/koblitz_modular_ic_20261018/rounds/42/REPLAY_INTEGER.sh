#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=$1;raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp -r "$here/integer-native" "$raw/native";cp "$here/runs/residual42-01/output/residual-systems.json" "$raw/";cp "$0" "$raw/runner.sh"
sha256sum "$raw/native/Cargo.toml" "$raw/native/Cargo.lock" "$raw/native/src/main.rs" "$raw/residual-systems.json" "$raw/runner.sh" > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 env CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR="$raw/build" cargo build --offline --locked --release --manifest-path "$raw/native/Cargo.toml") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/build/release/koblitz-modular-carriers" "$raw/residual-systems.json" "$raw/output") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
[ "$rc" = 0 ];rg -qx INTEGER42_ALL_CHECKS_PASS "$raw/native.log";echo "PASS $attempt"
