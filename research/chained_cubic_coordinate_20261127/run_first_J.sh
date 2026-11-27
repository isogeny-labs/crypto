#!/usr/bin/env bash
# Thin native caller; Python performs only file/Git/hash/JSON custody.
set -euo pipefail
source_commit=${1:?published source commit}
out=${2:?fresh absolute RAM output directory}
compiler_temp=${3:?fresh absolute RAM compiler temporary directory}
study_dir=$(cd "$(dirname "$0")" && pwd)
repo_dir=$(git -C "$study_dir" rev-parse --show-toplevel)
study_rel=research/chained_cubic_coordinate_20261127
export PATH="/home/agent/.cargo/bin:$PATH"
export RAYON_NUM_THREADS=1 CARGO_BUILD_JOBS=1
export CHECKPOINT_SKIP="Fixed integer lifts of already qualified public cubic identities with unchanged canonical IDC1v1 instrument; deterministic correctness only, see PROTOCOL_J.md."
unset CLIPPY_ARGS RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER CARGO_ENCODED_RUSTFLAGS
export RUSTUP_TOOLCHAIN=1.99.1
export CARGO_HOME=/dev/shm/ct27-H-cargo-home
export RUSTC=/home/agent/.rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin/rustc
export RUSTFLAGS="-Dwarnings -Copt-level=3 -Coverflow-checks=yes -Cdebug-assertions=yes -Cdebuginfo=0 -Ccodegen-units=1"
umask 077
mkdir "$out" "$compiler_temp"
export TMPDIR="$compiler_temp"
metadata="$study_dir/custody_metadata_J.py"
finish() {
 local status=$? guard_status=0
 trap - EXIT
 python3 "$metadata" finish "$out" "$status" > "$out/final-guards.stdout" 2> "$out/final-guards.stderr" || guard_status=$?
 if [[ "$status" -ne 0 ]]; then exit "$status"; fi
 exit "$guard_status"
}
trap finish EXIT
cd "$repo_dir"
python3 "$metadata" init "$source_commit" "$out" "$compiler_temp" > "$out/init.stdout" 2> "$out/init.stderr"
copied="$out/source/$study_rel"
phase() {
 local name=$1 status=0
 shift
 printf '%q ' "$@" > "$out/$name.argv"
 printf '\n' >> "$out/$name.argv"
 ( ulimit -c 0; ulimit -v 2097152; timeout --signal=TERM --kill-after=5s 120s "$@" ) > "$out/$name.stdout" 2> "$out/$name.stderr" || status=$?
 python3 "$metadata" phase "$out" "$name" "$status"
 if [[ "$status" -ne 0 ]]; then return "$status"; fi
}
phase rustc-version rustup run 1.99.1 rustc --version
phase cargo-version rustup run 1.99.1 cargo --version
phase clippy-version rustup run 1.99.1 clippy-driver --version
phase rustfmt-version rustup run 1.99.1 rustfmt --version
phase format rustup run 1.99.1 rustfmt --edition 2021 --config skip_children=true --check "$copied/identity_record_j/src/main.rs" "$copied/identity_record_j/inputs/canonical_sha256_rust199.rs"
phase compile rustup run 1.99.1 cargo build --manifest-path "$copied/identity_record_j/Cargo.toml" --release --locked --offline --verbose --jobs 1 --target-dir "$out/target"
phase clippy rustup run 1.99.1 cargo clippy --manifest-path "$copied/identity_record_j/Cargo.toml" --release --locked --offline --verbose --jobs 1 --target-dir "$out/target" -- -Dwarnings
cp "$out/target/release/chained-cubic-identity-record-j" "$out/qualifier"
chmod 0555 "$out/qualifier"
python3 "$metadata" pre "$out" > "$out/pre-call.stdout" 2> "$out/pre-call.stderr"
phase actual-integer-lift-identity "$out/qualifier" "$out/data"
