#!/usr/bin/env bash
set -eu
cd -- "$(dirname -- "$0")"
attempt="evidence/${1:-attempt-01}"
rust_compiler="${PPLUS_RUSTC:-/workspace/scratch/857f9e575d1c/tmp/vitse-rust-toolchain/installed/bin/rustc}"
mkdir -p -- "$attempt"
if test -e "$attempt/PRE_BUILD.sha256"; then
    echo "Refusing to overwrite a frozen attempt: $attempt" >&2
    exit 2
fi
test "$(sha256sum production.rs | cut -d ' ' -f1)" = c6dff02843fceab1cf221465056598e6cd94d3d29a350b72eed85bee4c02c56c
cp -- PROTOCOL.md line_fft.rs production.rs production-source-manifest.json qualify.sh "$attempt/"
sha256sum "$attempt/PROTOCOL.md" "$attempt/line_fft.rs" "$attempt/production.rs" "$attempt/production-source-manifest.json" "$attempt/qualify.sh" > "$attempt/PRE_BUILD.sha256"
"$rust_compiler" --version > "$attempt/compiler-version.txt"
sha256sum "$rust_compiler" > "$attempt/compiler.sha256"
printf '%s\n' 'rustc --edition=2024 --crate-name pplus_line_fft -C opt-level=2 line_fft.rs -o line-fft-qualifier' > "$attempt/compile-command.txt"
set +e
timeout 60 "$rust_compiler" --edition=2024 --crate-name pplus_line_fft -C opt-level=2 "$attempt/line_fft.rs" -o "$attempt/line-fft-qualifier" > "$attempt/compile.stdout" 2> "$attempt/compile.stderr"
compile_status=$?
set -e
printf '%s\n' "$compile_status" > "$attempt/compile.exit-code"
if test "$compile_status" -ne 0; then exit "$compile_status"; fi
sha256sum "$attempt/line-fft-qualifier" > "$attempt/PRE_EXECUTION.sha256"
printf '%s\n' 'timeout 60 ./line-fft-qualifier' > "$attempt/run-command.txt"
set +e
timeout 60 "$attempt/line-fft-qualifier" > "$attempt/native-output.jsonl" 2> "$attempt/native.stderr"
run_status=$?
set -e
printf '%s\n' "$run_status" > "$attempt/run.exit-code"
sha256sum -c "$attempt/PRE_BUILD.sha256" > "$attempt/source-custody-check.txt"
sha256sum -c "$attempt/PRE_EXECUTION.sha256" > "$attempt/binary-custody-check.txt"
sha256sum "$attempt/native-output.jsonl" "$attempt/native.stderr" "$attempt/run.exit-code" > "$attempt/POST_EXECUTION.sha256"
cat -- "$attempt/native-output.jsonl"
exit "$run_status"
