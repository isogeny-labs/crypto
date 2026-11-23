#!/usr/bin/env bash
set -uo pipefail
cd /workspace/crypto || exit 1
export PATH=/workspace/toolchains/cargo/bin:$PATH
export CARGO_HOME=/workspace/toolchains/cargo
export RUSTUP_HOME=/workspace/toolchains/rustup
export CARGO_TARGET_DIR=/workspace/crypto/target
export RAYON_NUM_THREADS=8
out=/workspace/pkm-native-windows-20261106/native
finish() {
  local code=$?
  printf '%s\n' "$code" > "$out/supervisor.exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/supervisor.finished"
}
trap finish EXIT
phase() {
  local name=$1
  shift
  while IFS= read -r binding; do
    expected=${binding%% *}
    path=${binding#* }
    actual=$(git rev-parse "HEAD:$path") || return 86
    [ "$actual" = "$expected" ] || return 86
    git diff --quiet HEAD -- "$path" || return 86
  done < "$out/source-bindings.txt"
  [ -z "$(git status --porcelain --untracked-files=no)" ] || return 86
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/$name.started"
  "$@" > "$out/$name.log" 2>&1
  local code=$?
  printf '%s\n' "$code" > "$out/$name.exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/$name.finished"
  printf '%s exit=%s\n' "$name" "$code"
  return "$code"
}
phase toolchain rustc --version || exit $?
phase example-tests cargo test --release --locked --example pkm_oracle_program -j2 -- --test-threads=1 || exit $?
phase format rustfmt --edition 2021 --check examples/pkm_oracle_program.rs examples/pkm_oracle_program/native_work.rs examples/pkm_oracle_program/tests.rs || exit $?
phase cli-build cargo build --release --locked --example pkm_oracle_program -j2 || exit $?
[ ! -e /workspace/pkm-native-windows-nonexistent-input.json ] || exit 90
phase cli-gate "$CARGO_TARGET_DIR/release/examples/pkm_oracle_program" verify-toy-dlp /workspace/pkm-native-windows-nonexistent-input.json /workspace/pkm-native-windows-nonexistent-input.json
[ "$(cat "$out/cli-gate.exit")" = 1 ] || exit 90
rg -q 'checkpoint gate refused experiment or replay' "$out/cli-gate.log" || exit 90
if rg -q 'pkm.toy-dlp-replay-failure/v1' "$out/cli-gate.log"; then exit 90; fi
phase scoped-clippy cargo clippy --release --locked --example pkm_oracle_program --no-deps -j2 -- -D warnings
exit $?
