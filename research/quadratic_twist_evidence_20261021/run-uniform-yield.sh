#!/usr/bin/env bash
# Native uniform-yield record/replay glue. This file has not run the study.
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: timeout 3600 bash research/quadratic_twist_evidence_20261021/run-uniform-yield.sh FRESH_DIRECTORY [EXPECTED_DIRECTORY]

Requires an approved, current branch-scoped resume checkpoint and a secure
checkpoint token. Invokes the unmodified repository checkpoint gate before
building or executing anything. Does not provision compute or upload results.

Without EXPECTED_DIRECTORY, records the first native attempt. With it, also
compares canonical cases, audit receipt and source-input bytes exactly.
The native receipt does not discharge the Lean/probability or full-cost gates.
EOF
}

fail() { printf 'uniform runner: %s\n' "$*" >&2; exit 2; }
if [[ ${1:-} == --help && $# == 1 ]]; then usage; exit 0; fi
if (( $# < 1 || $# > 2 )); then usage >&2; exit 2; fi

root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
crate=$study/native/uniform-yield-audit
out=$(realpath -m -- "$1")
expected=
if (( $# == 2 )); then expected=$(realpath -e -- "$2"); fi
[[ ! -e $out ]] || fail 'output directory must be fresh'
case "$out/" in "$root/"*) fail 'use a fresh scratch directory outside the checkout';; esac
if [[ -n $expected ]]; then
  [[ -d $expected/cases && -f $expected/audit.json && -f $expected/INPUTS.sha256 ]] ||
    fail 'expected record needs cases/, audit.json and INPUTS.sha256'
fi

# The approval for this task covers exactly this repository/branch prefix.
# Detached HEAD would select a different checkpoint prefix, so refuse it.
branch=$(git symbolic-ref --quiet --short HEAD) || fail 'bind the approved branch before launch'
[[ $branch == codex/quadratic-twist-evidence-import ]] || fail 'branch is outside the approved checkpoint scope'
origin=$(git config --get remote.origin.url)
repo=${origin##*/}; repo=${repo%.git}
[[ $repo == crypto ]] || fail 'repository is outside the approved checkpoint scope'
[[ ${CHECKPOINT_BUCKET:-setec-astronomy-artifacts} == setec-astronomy-artifacts ]] || fail 'unexpected checkpoint bucket'
[[ ${CHECKPOINT_PREFIX:-checkpoints} == checkpoints ]] || fail 'unexpected checkpoint prefix'
token_file=${CHECKPOINT_TOKEN_FILE:-$HOME/.config/checkpoint/upload-token}
[[ -n ${CHECKPOINT_UPLOAD_TOKEN:-} || -s $token_file ]] || fail 'secure checkpoint token is not configured; no AWS credential fallback is permitted'
[[ -f scripts/checkpoint.sh && -f .resume/RESUME.md ]] || fail 'current official gate and complete RESUME are required'
current_gate=$(git rev-parse refs/remotes/origin/main:scripts/checkpoint.sh)
[[ $(git hash-object scripts/checkpoint.sh) == "$current_gate" ]] ||
  fail 'checkout checkpoint script differs from fetched main; review current instructions and reconcile the official tool first'
git diff --quiet HEAD -- || fail 'commit the frozen inputs and runner before launch'

inputs=(
  "$study/run-uniform-yield.sh"
  "$study/UNIFORM_YIELD_20261025_PROTOCOL.md"
  "$study/UNIFORM_YIELD_20261026_IMPLEMENTATION.md"
  "$study/UNIFORM_YIELD_20261028_RUNNER.md"
  "$study/native/uniform-yield-v2.cpp"
  "$study/native/phase-adaptive-prior.hpp"
  "$study/native/reduced-s3-prior.hpp"
  "$study/native/full.cpp" "$study/native/json.hpp"
  "$crate/main.rs" "$crate/Cargo.toml" "$crate/Cargo.lock"
  "$study/lean-cm8/CM8Transfer.lean" "$study/lean/S3System.lean"
  src/cryptanalysis/identity_certificate.rs src/hash/sha256.rs
  docs/identity-certificates/schoenhage-10mult.json
  scripts/checkpoint.sh
)
git ls-files --error-unmatch -- "${inputs[@]}" scripts/checkpoint.sh >/dev/null
for input in "${inputs[@]}"; do [[ -f $input ]] || fail "materialize tracked input: $input"; done

mkdir -p -- "$out"
runner_start=$SECONDS
run_capped() {
  local phase=$1; shift
  local remaining=$((3600 - SECONDS + runner_start)) cap=600 result
  (( remaining > 0 )) || { printf 'iteration envelope exhausted\n' >&2; return 124; }
  (( remaining >= cap )) || cap=$remaining
  printf '%q ' "$@" > "$out/$phase.command"
  printf '\n' >> "$out/$phase.command"
  if timeout "$cap" "$@" > "$out/$phase.log" 2>&1; then result=0; else result=$?; fi
  printf '%s\n' "$result" > "$out/$phase.exit"
  return "$result"
}
finish() {
  local result=$?
  trap - EXIT
  printf '%s\n' "$result" > "$out/runner.exit"
  if [[ -f $out/INPUTS.sha256 ]]; then
    if ! sha256sum -c "$out/INPUTS.sha256" > "$out/final-source-check.log" 2>&1; then
      printf 'source changed during the attempt\n' >&2
      (( result != 0 )) || result=2
      printf '%s\n' "$result" > "$out/runner.exit"
    fi
  fi
  if (( result == 0 )); then
    printf '%s\n' 'PASS: native producer/audit/control contract only; formal probability, full-library, registry and complete-work obligations remain separate.'
  fi
  printf '%s\n' 'Review the retained output, update RESUME, and use the official checkpoint gate for the finish/failure checkpoint.' >&2
  exit "$result"
}
trap finish EXIT

sha256sum "${inputs[@]}" > "$out/INPUTS.sha256"
git rev-parse HEAD > "$out/input-head.txt"
git hash-object scripts/checkpoint.sh > "$out/checkpoint-script.blob"
# No CHECKPOINT_SKIP, alternate uploader, local success marker or AWS probe.
run_capped checkpoint-gate bash scripts/checkpoint.sh gate
if [[ -n $expected ]]; then cmp "$expected/INPUTS.sha256" "$out/INPUTS.sha256"; fi

run_capped producer-build c++ -std=c++17 -O2 -Wall -Wextra -Werror \
  "$study/native/uniform-yield-v2.cpp" -lcrypto -lgmpxx -lgmp -o "$out/producer"
run_capped auditor-build env CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR="$out/cargo-target" \
  cargo build --release --locked --offline --manifest-path "$crate/Cargo.toml"
auditor=$out/cargo-target/release/quadratic-twist-uniform-yield-audit
run_capped producer "$out/producer" "$root" "$out/cases"
run_capped audit "$auditor" audit "$root" "$out/cases" "$out/audit.json"
if [[ -n $expected ]]; then
  diff -ru "$expected/cases" "$out/cases" > "$out/cases-diff.log"
  cmp "$expected/audit.json" "$out/audit.json"
fi

mkdir "$out/controls"
controls=(false-row false-degree false-witness missing-arm missing-target
  false-transition false-rational missing-query false-log false-scalar
  false-source false-frontier missing-failed-query)
for control in "${controls[@]}"; do
  control_dir=$out/controls/$control
  cp -a "$out/cases" "$control_dir"
  run_capped "control-$control-mutate" "$auditor" corrupt "$control" "$control_dir"
  outcome=$(cat "$out/control-$control-mutate.log")
  if [[ $outcome == null-no-failed-query ]]; then
    [[ $control == missing-failed-query ]] || fail 'unexpected null control'
    grep -Eq '"missing_failed_query_control"[[:space:]]*:[[:space:]]*"null-no-failed-query"' "$out/audit.json" ||
      fail 'null control disagrees with the independent audit receipt'
    diff -ru "$out/cases" "$control_dir" > "$out/control-$control-null-diff.log"
    printf '%s\n' 'null-no-failed-query; no rejection claimed' > "$out/controls/$control.status"
    continue
  fi
  [[ $outcome == corrupted-and-rehashed ]] || fail "unexpected corruption outcome: $control"
  if diff -rq "$out/cases" "$control_dir" > "$out/control-$control-payload-diff.log"; then
    fail "control did not change a payload: $control"
  else
    diff_result=$?
    [[ $diff_result == 1 ]] || fail "could not compare control payload: $control"
  fi
  receipt=$out/controls/$control-receipt.json
  if run_capped "control-$control-audit" env RUST_BACKTRACE=0 \
    "$auditor" audit "$root" "$control_dir" "$receipt"; then
    fail "false control accepted: $control"
  else
    result=$?
  fi
  [[ $result == 101 && ! -e $receipt ]] || fail "unexpected rejection/receipt: $control"
  grep -q 'false uniform' "$out/control-$control-audit.log" || fail "unexpected rejection cause: $control"
  printf '%s\n' 'rejected-101-no-receipt' > "$out/controls/$control.status"
done

run_capped backend-tests env CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR="$out/cargo-target" \
  cargo test --release --locked --offline --manifest-path "$crate/Cargo.toml"
find "$out/cases" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/OUTPUTS.sha256"
sha256sum "$out/audit.json" >> "$out/OUTPUTS.sha256"
