#!/usr/bin/env bash
# Portable regression only. Local original execution receipts remain distinct.
set -euo pipefail
out=${1:?fresh output directory}
study_dir=$(cd "$(dirname "$0")" && pwd)
repo_dir=$(git -C "$study_dir" rev-parse --show-toplevel)
study_rel=research/chained_cm19_resolvents_20261124
export RAYON_NUM_THREADS=1 CARGO_BUILD_JOBS=1
export LEAN_NUM_THREADS=1 LEAN_STACK_SIZE_KB=16384 LEAN_MAIN_USE_THREAD=0 MIMALLOC_ARENA_RESERVE=65536
export CHECKPOINT_SKIP="Fixed actual quartic, resolvent, kernel and genus-two proof correctness regression, no experiment or timing comparison; exact source/input checksums verified before calls."
unset RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CLIPPY_ARGS LEAN_SYSROOT LEAN_PATH LEAN_OPTS
umask 077
mkdir "$out" "$out/temp"
export TMPDIR="$out/temp"
metadata="$study_dir/ci_custody_metadata_E.py"
finish() {
 local status=$? guard_status=0
 trap - EXIT
 python3 "$metadata" finish "$out" "$status" > "$out/final-guards.stdout" 2> "$out/final-guards.stderr" || guard_status=$?
 if [[ "$status" -ne 0 ]]; then exit "$status"; fi
 exit "$guard_status"
}
trap finish EXIT
cd "$repo_dir"
python3 "$metadata" prepare "$out" > "$out/init.stdout" 2> "$out/init.stderr"
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
phase rustfmt-version rustup run 1.99.1 rustfmt --version
phase clippy-version rustup run 1.99.1 clippy-driver --version
phase format rustup run 1.99.1 rustfmt --edition 2021 --config skip_children=true --check "$copied/main_a.rs" "$copied/poly_a.rs" "$copied/field_a.rs" "$copied/main_b.rs" "$copied/ring_b.rs" "$copied/models_b.rs" "$copied/incidence_b.rs" "$copied/cm_b.rs" "$copied/main_c.rs" "$copied/main_d.rs" "$copied/poly_d.rs" "$copied/cubic_d.rs" "$copied/cm_d.rs"
for unit in a b c d; do
 phase "compile-$unit" rustup run 1.99.1 rustc --edition=2021 --crate-name "chained_cm19_resolvents_ci_$unit" -Dwarnings -Copt-level=3 -Coverflow-checks=yes -Cdebug-assertions=yes -Cdebuginfo=0 -Ccodegen-units=1 "$copied/main_$unit.rs" -o "$out/native-built-$unit"
 phase "clippy-$unit" rustup run 1.99.1 clippy-driver --edition=2021 --crate-name "chained_cm19_resolvents_ci_$unit" -Dwarnings --emit=metadata "$copied/main_$unit.rs" -o "$out/clippy-$unit.rmeta"
 cp "$out/native-built-$unit" "$out/qualifier-$unit"
 chmod 0555 "$out/qualifier-$unit"
done
python3 "$metadata" pre "$out" > "$out/pre-call.stdout" 2> "$out/pre-call.stderr"
phase primitive-quartic-a "$out/qualifier-a"
phase resolvents-incidence-b "$out/qualifier-b"
phase split-kernel-c "$out/qualifier-c"
phase principal-genus2-d "$out/qualifier-d"
