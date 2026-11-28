#!/usr/bin/env bash
set -euo pipefail

pplus_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
pplus_rustc=${PPLUS_RUSTC:-/workspace/scratch/857f9e575d1c/tmp/vitse-rust-toolchain/installed/bin/rustc}
pplus_evidence="$pplus_dir/evidence/attempt-02"
mkdir -p "$pplus_dir/evidence"
mkdir "$pplus_evidence"
mkdir "$pplus_dir/build"
cd "$pplus_dir"

node custody.mjs source
"$pplus_rustc" --version --verbose > "$pplus_evidence/compiler-version.txt"
node custody.mjs freeze "$pplus_rustc"
chmod a-w PROTOCOL.md production.rs qualifier.rs source-manifest.json custody.mjs run.sh

cat > "$pplus_evidence/commands.txt" <<'COMMANDS'
Build: rustc --edition=2024 --crate-name pplus_selector_qualifier -C opt-level=2 qualifier.rs -o build/pplus-selector-build
Normal: evidence/attempt-02/pplus-selector-qualifier
Expected rejection: evidence/attempt-02/pplus-selector-qualifier --mutate-first-root
Expected rejection: evidence/attempt-02/pplus-selector-qualifier --mutate-norm-one-lift
Build timeout: 60 seconds. Each native invocation timeout: 30 seconds.
COMMANDS

set +e
timeout 60 "$pplus_rustc" --edition=2024 --crate-name pplus_selector_qualifier -C opt-level=2 qualifier.rs -o "$pplus_dir/build/pplus-selector-build" > "$pplus_evidence/compile.stdout" 2> "$pplus_evidence/compile.stderr"
pplus_code=$?
set -e
printf '%s\n' "$pplus_code" > "$pplus_evidence/compile.exit"
test "$pplus_code" -eq 0

cp "$pplus_dir/build/pplus-selector-build" "$pplus_evidence/pplus-selector-qualifier"
chmod 555 "$pplus_evidence/pplus-selector-qualifier"
node custody.mjs binary

set +e
timeout 30 "$pplus_evidence/pplus-selector-qualifier" > "$pplus_evidence/normal.stdout" 2> "$pplus_evidence/normal.stderr"
pplus_code=$?
set -e
printf '%s\n' "$pplus_code" > "$pplus_evidence/normal.exit"
test "$pplus_code" -eq 0

set +e
timeout 30 "$pplus_evidence/pplus-selector-qualifier" --mutate-first-root > "$pplus_evidence/mutated-root.stdout" 2> "$pplus_evidence/mutated-root.stderr"
pplus_code=$?
set -e
printf '%s\n' "$pplus_code" > "$pplus_evidence/mutated-root.exit"
test "$pplus_code" -ne 0
test "$pplus_code" -ne 124
test "$pplus_code" -ne 137

set +e
timeout 30 "$pplus_evidence/pplus-selector-qualifier" --mutate-norm-one-lift > "$pplus_evidence/mutated-lift.stdout" 2> "$pplus_evidence/mutated-lift.stderr"
pplus_code=$?
set -e
printf '%s\n' "$pplus_code" > "$pplus_evidence/mutated-lift.exit"
test "$pplus_code" -ne 0
test "$pplus_code" -ne 124
test "$pplus_code" -ne 137

node custody.mjs finish
printf '%s\n' 'Accepted native attempt-02 finished; raw Rust JSON and custody receipts are in evidence/attempt-02.'
