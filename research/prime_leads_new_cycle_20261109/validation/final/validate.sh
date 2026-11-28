#!/usr/bin/env bash
set -uo pipefail
cd /workspace/crypto-leads-cycle-20261109 || exit 90
export PATH=/workspace/toolchains/cargo/bin:/workspace/toolchains/lean-4.34.1-linux/bin:$PATH
export CARGO_HOME=/workspace/toolchains/cargo RUSTUP_HOME=/workspace/toolchains/rustup
out=/workspace/prime-leads-cycle-20261109/native-final
study=research/prime_leads_new_cycle_20261109
mkdir -p "$out"
finish() { local code=$?; printf '%s\n' "$code" > "$out/supervisor.exit"; date -u +%FT%TZ > "$out/supervisor.finished"; }
trap finish EXIT
guard() {
  while read -r expected path; do
    [ "$(git rev-parse "HEAD:$path")" = "$expected" ] || return 86
    git diff --quiet HEAD -- "$path" || return 86
  done < "$study/validation/final/source-bindings.txt"
  [ -z "$(git status --porcelain --untracked-files=no)" ] || return 86
}
phase() {
  local name=$1; shift
  date -u +%FT%TZ > "$out/$name.started"
  guard > "$out/$name.guard.log" 2>&1
  local code=$?
  if [ "$code" = 0 ]; then "$@" > "$out/$name.log" 2>&1; code=$?; fi
  printf '%s\n' "$code" > "$out/$name.exit"
  date -u +%FT%TZ > "$out/$name.finished"
  return "$code"
}
phase rust-build timeout 300 rustc --edition=2021 --test "$study/controls.rs" -o "$out/controls-tests" || exit $?
phase controls timeout 60 "$out/controls-tests" --test-threads=1 --nocapture || exit $?
phase strict-clippy timeout 300 clippy-driver --edition=2021 --test "$study/controls.rs" -o "$out/controls-clippy" -D warnings || exit $?
phase cycle2-lib rustc --edition=2021 --crate-type=rlib --crate-name lead_controls "$study/controls.rs" -o "$out/liblead_controls.rlib" || exit $?
phase cycle2-build rustc --edition=2021 --test "$study/anchor_cycle2.rs" --extern "lead_controls=$out/liblead_controls.rlib" -o "$out/anchor-tests" || exit $?
phase cycle2-controls timeout 60 "$out/anchor-tests" --test-threads=1 --nocapture || exit $?
phase cycle2-clippy clippy-driver --edition=2021 --test "$study/anchor_cycle2.rs" --extern "lead_controls=$out/liblead_controls.rlib" -o "$out/anchor-clippy" -D warnings || exit $?
phase format rustfmt --edition 2021 --check "$study/controls.rs" "$study/anchor_cycle2.rs" || exit $?
phase lean-build bash -c 'cd research/prime_leads_new_cycle_20261109/lean && lake build BinaryPrimitiveGuard' || exit $?
phase lean-gate tools/lean/proof-gate.sh --out "$out/gate" "$study/lean" || exit $?
phase lean-kernel bash -c 'cd research/prime_leads_new_cycle_20261109/lean && lake env leanchecker BinaryPrimitiveGuard' || exit $?
sha256sum "$out/controls-tests" "$out/controls-clippy" "$out/anchor-tests" "$out/anchor-clippy" "$out/liblead_controls.rlib" > "$out/binaries.sha256"
exit 0
