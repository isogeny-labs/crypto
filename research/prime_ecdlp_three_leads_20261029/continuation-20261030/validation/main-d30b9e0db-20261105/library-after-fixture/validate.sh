#!/usr/bin/env bash
set -uo pipefail
cd /workspace/crypto || exit 1
export PATH=/workspace/toolchains/cargo/bin:$PATH
export CARGO_HOME=/workspace/toolchains/cargo
export RUSTUP_HOME=/workspace/toolchains/rustup
export CARGO_TARGET_DIR=/workspace/crypto/target
export RAYON_NUM_THREADS=8
out=/workspace/pkm-pr3452-main-20261105/library-after-fixture
finish() {
  local result=$?
  printf '%s\n' "$result" > "$out/supervisor.exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/supervisor.finished"
}
trap finish EXIT
run_check() {
  local name=$1
  shift
  while IFS= read -r binding; do
    expected=${binding%% *}
    path=${binding#* }
    actual=$(git rev-parse "HEAD:$path") || return 86
    [ "$actual" = "$expected" ] || return 86
    git diff --quiet HEAD -- "$path" || return 86
  done < "$out/source-bindings.txt"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/$name.started"
  "$@" > "$out/$name.log" 2>&1
  local result=$?
  printf '%s\n' "$result" > "$out/$name.exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/$name.finished"
  printf '%s exit=%s\n' "$name" "$result"
  return "$result"
}
run_check library-build cargo test --release --lib --locked -j2 --no-run || exit $?
run_check library-test timeout 2400 cargo test --release --lib --locked -j2 -- --test-threads=1
exit $?
