#!/usr/bin/env bash
# Thin orchestration for the native validator (src/main.rs): build it, rerun
# the three legacy Python execution paths natively, and compare each output
# with the committed legacy output (validation/*.json).
#
#   SESSION_ROOT  directory holding P20/ and P28/eks-results/session (default:
#                 the study directory). recompute records it as session_dir;
#                 the committed legacy outputs name the validator's original
#                 worktree, so pass that path (with byte-identical copies) to
#                 make recompute's session_dir field equal as well.
#   OUT           output directory (default: validation/native/out)
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$here/../.." && pwd)
cd "$study"
out="${OUT:-validation/native/out}"
session_root="${SESSION_ROOT:-$study}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$here/target}"
cargo build --release --offline --manifest-path "$here/Cargo.toml" >&2 \
  || cargo build --release --manifest-path "$here/Cargo.toml" >&2
bin="$CARGO_TARGET_DIR/release/srenew-validate"
mkdir -p "$out"
for b in 20 28; do
  "$bin" recompute "$session_root/P$b/eks-results/session" "P$b" > "$out/recompute-P$b.json"
  "$bin" mechanism "$session_root/P$b/eks-results/session" > "$out/mechanism-P$b.json"
done
# Gate files twice: from the committed recompute/mechanism outputs (tests the
# gate logic alone) and from the native ones (the whole native path).
for src in committed native; do
  rdir=validation; [ "$src" = native ] && rdir="$out"
  "$bin" gate . validation VAL-20261029-4aed57 --recompute-dir "$rdir" \
    --out "$out/gate-from-$src-recompute" > "$out/gate-from-$src-recompute.summary.json"
done
{
  for f in recompute-P20 recompute-P28 mechanism-P20 mechanism-P28; do
    "$bin" cmpjson "validation/$f.json" "$out/$f.json" 1e-12
  done
  for src in committed native; do
    for f in replay-certificate validator-report; do
      "$bin" cmpjson "validation/$f.json" "$out/gate-from-$src-recompute/$f.json" 1e-12
    done
  done
} > "$out/equivalence.jsonl"
cat "$out/equivalence.jsonl"
