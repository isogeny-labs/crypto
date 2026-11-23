#!/usr/bin/env bash
set -uo pipefail
cd /workspace/crypto || exit 1
export PATH=/workspace/toolchains/cargo/bin:$PATH
export CARGO_HOME=/workspace/toolchains/cargo
export RUSTUP_HOME=/workspace/toolchains/rustup
export CARGO_TARGET_DIR=/workspace/crypto/target
export RAYON_NUM_THREADS=8
out=/workspace/pkm-scalar-certificates-20261105/g2-counters
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
run_check example-tests cargo test --release --locked --example pkm_oracle_program -j2 -- --test-threads=1 || exit $?
run_check clippy cargo clippy --release --locked --example pkm_oracle_program --no-deps -j2 -- -D warnings || exit $?
run_check rustfmt rustfmt --edition 2021 --check examples/pkm_oracle_program.rs examples/pkm_oracle_program/pipeline.rs examples/pkm_oracle_program/reference.rs examples/pkm_oracle_program/tests.rs || exit $?
run_check cli-build cargo build --release --locked --example pkm_oracle_program -j2 || exit $?
[ ! -e /workspace/pkm-proof-control-no-input.json ] || exit 90
run_check cli-gate "$CARGO_TARGET_DIR/release/examples/pkm_oracle_program" run /workspace/pkm-proof-control-no-input.json
[ "$(cat "$out/cli-gate.exit")" = 1 ] || exit 90
rg -q 'checkpoint gate refused experiment or replay' "$out/cli-gate.log" || exit 90
exit 0
