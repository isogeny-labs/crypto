#!/usr/bin/env bash
set -uo pipefail
cd /workspace/crypto || exit 1
export PATH=/workspace/toolchains/cargo/bin:$PATH
export CARGO_HOME=/workspace/toolchains/cargo
export RUSTUP_HOME=/workspace/toolchains/rustup
out=/workspace/pkm-recorded-child-20261105/native
root=research/prime_ecdlp_three_leads_20261029/continuation-20261030
export PKM_CHILD_FIXTURE="$out/fixture"
finish() {
  local result=$?
  printf '%s\n' "$result" > "$out/supervisor.exit"
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
  local result=$?
  printf '%s\n' "$result" > "$out/$name.exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/$name.finished"
  printf '%s exit=%s\n' "$name" "$result"
  return "$result"
}
phase toolchain rustc --version || exit $?
phase fixture-build rustc --edition 2021 -D warnings "$root/prerequisites/child_fixture.rs" -o "$out/fixture" || exit $?
phase test-build rustc --edition 2021 --test -D warnings "$root/prerequisites/recorded_child.rs" -o "$out/tests" || exit $?
phase tests "$out/tests" --test-threads=1 || exit $?
phase clippy clippy-driver --edition 2021 --test -D warnings "$root/prerequisites/recorded_child.rs" -o "$out/clippy-tests" || exit $?
phase fixture-clippy clippy-driver --edition 2021 -D warnings "$root/prerequisites/child_fixture.rs" -o "$out/clippy-fixture" || exit $?
phase format rustfmt --edition 2021 --config skip_children=true --check "$root/prerequisites/recorded_child.rs" "$root/prerequisites/child_observation.rs" || exit $?
exit 0
