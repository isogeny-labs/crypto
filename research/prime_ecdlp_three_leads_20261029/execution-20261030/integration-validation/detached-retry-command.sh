#!/usr/bin/env bash
set -uo pipefail
cd /workspace/crypto || exit 1
export PATH=/workspace/toolchains/cargo/bin:$PATH
export CARGO_HOME=/workspace/toolchains/cargo
export RUSTUP_HOME=/workspace/toolchains/rustup
export CARGO_TARGET_DIR=/workspace/crypto/target
export RAYON_NUM_THREADS=8
out=/workspace/pr2949-integration-20261030/retry-20261030T0838Z
finish_supervisor() {
  local result=$?
  printf '%s\n' "$result" > "$out/supervisor.exit.tmp"
  mv "$out/supervisor.exit.tmp" "$out/supervisor.exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/supervisor.finished"
}
trap finish_supervisor EXIT
run_check() {
  local name=$1
  shift
  while IFS= read -r binding; do
    expected=${binding%% *}
    path=${binding#* }
    actual=$(git rev-parse "HEAD:$path") || return 86
    [ "$actual" = "$expected" ] || { printf 'source binding changed: %s\n' "$path" >&2; return 86; }
  done < "$out/source-bindings.txt"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/$name.started"
  "$@" > "$out/$name.log" 2>&1
  local result=$?
  printf '%s\n' "$result" > "$out/$name.exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/$name.finished"
  printf '%s exit=%s\n' "$name" "$result"
  return "$result"
}
run_check native-tests cargo test --release --locked --example pkm_oracle_program --example pkm_symmetric_complete --example hecke_target_router -j2 -- --test-threads=1 || exit $?
run_check clippy cargo clippy --release --locked --example pkm_oracle_program --example pkm_symmetric_complete --example hecke_target_router --no-deps -j2 -- -D warnings || exit $?
run_check rustfmt rustfmt --edition 2021 --check examples/pkm_oracle_program.rs examples/pkm_symmetric_complete.rs examples/hecke_target_router.rs || exit $?
run_check cli-build cargo build --release --locked --example pkm_oracle_program -j2 || exit $?
run_check cli-gate "$CARGO_TARGET_DIR/release/examples/pkm_oracle_program" run /workspace/pkm-do-not-exist-request-20261030.json
[ "$(cat "$out/cli-gate.exit")" = 1 ] || exit 1
run_check backlog-build cargo build --release --locked --bin backlog -j2 || exit $?
run_check backlog-validate "$CARGO_TARGET_DIR/release/backlog" validate XP-20261029-pkm-verified-oracle-program || exit $?
run_check library-build cargo test --release --lib --locked -j2 --no-run || exit $?
run_check library-test timeout 2400 cargo test --release --lib --locked -j2 -- --test-threads=1
result=$?
printf '%s\n' "$result" > "$out/native-validation.exit"
exit "$result"
