#!/usr/bin/env bash
# Thin compile/run control only. All scientific assertions and counters are Rust.
set -euo pipefail
STUDY16=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
SOURCE16=${1:?exact public source commit required}
HOLD16=${2:?fresh custody path required}
TEMP16=${3:?fresh compiler temporary path required}
export PATH="/home/agent/.cargo/bin:$PATH"
export TMPDIR="$TEMP16" CLIPPY_CONF_DIR="$HOLD16/source" RUST_TEST_THREADS=1
CLEAN16=(env -i "PATH=$PATH" "HOME=$HOME" "TMPDIR=$TMPDIR" "CLIPPY_CONF_DIR=$CLIPPY_CONF_DIR" "RUST_TEST_THREADS=1")
for key16 in LANG LC_ALL TZ LD_LIBRARY_PATH RUSTUP_HOME CARGO_HOME; do
    if [[ -v $key16 ]]; then CLEAN16+=("$key16=${!key16}"); fi
done
META16="$STUDY16/custody_metadata16B.py"
finish16() {
    local initial16=$?
    trap - EXIT
    local final16=0
    if [[ -f "$HOLD16/CONTEXT16.json" ]]; then
        "${CLEAN16[@]}" python3 "$META16" finish --hold "$HOLD16" || final16=$?
    fi
    if ((initial16 != 0)); then exit "$initial16"; fi
    exit "$final16"
}
trap finish16 EXIT
"${CLEAN16[@]}" python3 "$META16" init --hold "$HOLD16" --temp "$TEMP16" --source-commit "$SOURCE16"
cd -- "$HOLD16/source"
phase16() {
    local name16=$1 expected16=$2 code16
    shift 2
    "${CLEAN16[@]}" python3 "$META16" record-start --hold "$HOLD16" -- "$name16" "$expected16" "$@"
    set +e
    "${CLEAN16[@]}" "$@" > "$HOLD16/$name16.stdout" 2> "$HOLD16/$name16.stderr"
    code16=$?
    set -e
    "${CLEAN16[@]}" python3 "$META16" record-end --hold "$HOLD16" -- "$name16" "$code16"
    printf '%s: exit %s, expected %s\n' "$name16" "$code16" "$expected16"
    [[ $code16 -eq $expected16 ]]
}
phase16 rustc-version 0 rustup run 1.99.1 rustc --version
phase16 clippy-version 0 rustup run 1.99.1 clippy-driver --version
phase16 rustfmt-version 0 rustup run 1.99.1 rustfmt --version
phase16 clippy-positive 0 timeout --signal=TERM --kill-after=5s 120s rustup run 1.99.1 clippy-driver --crate-name sentinel_positive --edition=2021 --emit=metadata -D clippy::eq_op config/clippy-positive.rs -o "$HOLD16/clippy-positive.rmeta"
phase16 clippy-negative 1 timeout --signal=TERM --kill-after=5s 120s rustup run 1.99.1 clippy-driver --crate-name sentinel_negative --edition=2021 --emit=metadata -D clippy::eq_op config/clippy-negative.rs -o "$HOLD16/clippy-negative.rmeta"
"${CLEAN16[@]}" python3 "$META16" frontend --hold "$HOLD16"
BIGINT16=("$HOLD16/deps"/libnum_bigint-*.rlib)
TRAITS16=("$HOLD16/deps"/libnum_traits-*.rlib)
[[ ${#BIGINT16[@]} -eq 1 && ${#TRAITS16[@]} -eq 1 ]]
COMMON16=(--crate-name chained_primefield_cost_16 --test --edition=2021 src/lib.rs -L "dependency=$HOLD16/deps" --extern "num_bigint=${BIGINT16[0]}" --extern "num_traits=${TRAITS16[0]}" -C opt-level=3 -C codegen-units=1 -C strip=symbols)
phase16 compile-only 0 timeout --signal=TERM --kill-after=5s 120s rustup run 1.99.1 rustc "${COMMON16[@]}" -o "$HOLD16/compiler-output"
phase16 strict-clippy 0 timeout --signal=TERM --kill-after=5s 120s rustup run 1.99.1 clippy-driver "${COMMON16[@]}" --emit=metadata -D warnings -o "$HOLD16/strict-clippy.rmeta"
phase16 format 0 rustup run 1.99.1 rustfmt --check --edition 2021 src/lib.rs vendor/fp_poly.rs
"${CLEAN16[@]}" python3 "$META16" pre-call --hold "$HOLD16"
phase16 list-tests 0 "$HOLD16/immutable-qualifier" --list
"${CLEAN16[@]}" python3 "$META16" test-inventory --hold "$HOLD16"
phase16 first-native-unit 0 timeout --signal=TERM --kill-after=5s 120s "$HOLD16/immutable-qualifier" nonsplit::map::tests_14::frozen_prime_cost_16 --exact --nocapture --test-threads=1
