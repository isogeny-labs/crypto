#!/usr/bin/env bash
# Build the native solver arms' binary (examples/gc_native_solve.rs) for
# groebner_compare/native_worker.py, reproducibly, outside the working tree.
#
# Why not `cargo build` at HEAD: on 2026-11-17 the library at crypto HEAD
# (f8480787ec, and origin/main 17281ed565) does not compile: sparse-merge damage
# (lost closing braces, duplicated match arms, missing module declarations in
# crates/hashed-ecdl-tensor, lost functions in sat.rs, ecbench and others).
# This script therefore exports BASE (default eb01341d3a, the commit of the
# measured benchmark record benchmarks/records/20261112T1831Z-inherited-f4-
# batched-e2e.json, which built `ic` with inherited-f4 and its next kernels) and
# adds, verbatim from SRC (default HEAD):
#   - src/cryptanalysis/f4_trace.rs (Joux-Vitse GF(2) Macaulay record/replay,
#     absent at BASE), declared in koblitz_groebner.rs exactly as at HEAD;
#   - examples/gc_native_solve.rs (this study's driver).
# The engines are BASE's; only f4_trace.rs and the driver come from SRC.
#
#   groebner_compare/build_native.sh OUT_DIR [BASE] [SRC]
# Prints the binary path; writes OUT_DIR/build_identity.json.
set -euo pipefail
out="${1:?usage: build_native.sh OUT_DIR [BASE] [SRC]}"
base="${2:-eb01341d3a}"
src="${3:-HEAD}"
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$out"
tree="$out/tree"
mkdir -p "$tree"
cd "$repo"
paths=$(git show "$base:Cargo.toml" | grep -oE 'path = "[^"]+"' | cut -d'"' -f2 | sort -u \
  | grep -v '^src/\|^crates/\|^examples/\|^benches/' \
  | while read -r p; do git cat-file -e "$base:$p" 2>/dev/null && echo "$p"; done)
git archive "$base" Cargo.toml Cargo.lock src crates examples benches $paths \
  docs/ic/boundary_targets.json docs/ecbench/schema.sql docs/curves/registry.json \
  docs/ic/calibration.json docs/curves/standards/parameters.json | tar -x -C "$tree"
git show "$src:src/cryptanalysis/f4_trace.rs" > "$tree/src/cryptanalysis/f4_trace.rs"
if [ "$src" = "WORKTREE" ] || ! git cat-file -e "$src:examples/gc_native_solve.rs" 2>/dev/null; then
  cp "$repo/examples/gc_native_solve.rs" "$tree/examples/gc_native_solve.rs"
else
  git show "$src:examples/gc_native_solve.rs" > "$tree/examples/gc_native_solve.rs"
fi
# Matched-rho / t_GA adapter of amendment-1 (PA-2), from the working tree.
cp "$repo/examples/gc_rho_match.rs" "$tree/examples/gc_rho_match.rs"
if ! grep -q 'mod f4_trace;' "$tree/src/cryptanalysis/koblitz_groebner.rs"; then
  cat >> "$tree/src/cryptanalysis/koblitz_groebner.rs" <<'EOF'

// Scratch build for research/groebner_solver_shootout_20261116: f4_trace.rs
// copied verbatim from crypto HEAD f8480787ec (same declaration as there).
#[path = "f4_trace.rs"]
mod f4_trace;
pub use f4_trace::{
    matrix_f4_f2_record, matrix_f4_f2_record_counted, matrix_f4_f2_replay,
    matrix_f4_f2_replay_checked, matrix_f4_f2_replay_counted, F4Trace, F4TraceStep, ReplayReject,
    DEFAULT_AUDIT,
};
EOF
fi
start=$(date +%s.%N)
(cd "$tree" && cargo build --release --example gc_native_solve --example gc_rho_match)
end=$(date +%s.%N)
bin="$tree/target/release/examples/gc_native_solve"
python3 - "$out/build_identity.json" "$base" "$src" "$tree" "$bin" "$start" "$end" <<'PY'
import hashlib, json, subprocess, sys
path, base, src, tree, binary, start, end = sys.argv[1:]
def sha(p):
    return hashlib.sha256(open(p, 'rb').read()).hexdigest()
rev = lambda r: subprocess.run(['git', 'rev-parse', r], capture_output=True, text=True).stdout.strip()
ident = {"base_commit": rev(base), "src_commit": rev(src) if src != "WORKTREE" else "WORKTREE",
         "added_files_sha256": {f: sha(f"{tree}/{f}") for f in
                                ("src/cryptanalysis/f4_trace.rs", "examples/gc_native_solve.rs",
                                 "examples/gc_rho_match.rs",
                                 "src/cryptanalysis/koblitz_groebner.rs")},
         "binary": binary, "binary_sha256": sha(binary),
         "rho_binary": binary.replace("gc_native_solve", "gc_rho_match"),
         "rho_binary_sha256": sha(binary.replace("gc_native_solve", "gc_rho_match")),
         "build_wall_seconds": float(end) - float(start),
         "rustc": subprocess.run(['rustc', '--version'], capture_output=True, text=True).stdout.strip(),
         "cargo": subprocess.run(['cargo', '--version'], capture_output=True, text=True).stdout.strip()}
json.dump(ident, open(path, 'w'), indent=2)
print(json.dumps(ident, indent=2))
PY
echo "$bin"
