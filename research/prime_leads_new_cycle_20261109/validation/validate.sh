#!/usr/bin/env bash
set -uo pipefail
cd /workspace/crypto-leads-cycle-20261109 || exit 90
export PATH=/workspace/toolchains/cargo/bin:/workspace/toolchains/lean-4.34.1-linux/bin:$PATH
export CARGO_HOME=/workspace/toolchains/cargo RUSTUP_HOME=/workspace/toolchains/rustup
out=/workspace/prime-leads-cycle-20261109/native
study=research/prime_leads_new_cycle_20261109
mkdir -p "$out"
finish() { local code=$?; printf '%s\n' "$code" > "$out/supervisor.exit"; date -u +%FT%TZ > "$out/supervisor.finished"; }
trap finish EXIT
guard() {
  while read -r expected path; do
    [ "$(git rev-parse "HEAD:$path")" = "$expected" ] || return 86
    git diff --quiet HEAD -- "$path" || return 86
  done < "$study/validation/source-bindings.txt"
  [ -z "$(git status --porcelain --untracked-files=no)" ] || return 86
}
phase() {
  local name=$1; shift
  date -u +%FT%TZ > "$out/$name.started"
  guard > "$out/$name.guard.log" 2>&1 || return 86
  "$@" > "$out/$name.log" 2>&1
  local code=$?
  printf '%s\n' "$code" > "$out/$name.exit"
  date -u +%FT%TZ > "$out/$name.finished"
  return "$code"
}
phase rust-build rustc --edition=2021 --test "$study/controls.rs" -o "$out/controls-tests" || exit $?
phase controls "$out/controls-tests" --test-threads=1 --nocapture || exit $?
phase strict-clippy clippy-driver --edition=2021 --test "$study/controls.rs" -o "$out/controls-clippy" -D warnings || exit $?
phase format rustfmt --edition 2021 --check "$study/controls.rs" || exit $?
phase lean-build bash -c 'cd research/prime_leads_new_cycle_20261109/lean && lake build BinaryPrimitiveGuard' || exit $?
phase lean-gate tools/lean/proof-gate.sh --out "$out/gate" "$study/lean" || exit $?
sha256sum "$out/controls-tests" "$out/controls-clippy" > "$out/binaries.sha256"
exit 0
