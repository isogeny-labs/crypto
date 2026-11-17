#!/bin/bash
# usage: replay_at_commit.sh <commit> <session-dir (current tree)> <outdir> <label>
# Builds ecbench at <commit> in a sparse worktree (shared target dir), runs `verify --replay-all` on the
# session, writes the receipt and a one-line summary. Read-only on the research tree.
set -u
c="$1"; sess="$2"; out="$3"; label="$4"
repo=$(git rev-parse --show-toplevel)
wt=${TMPDIR:-/tmp}/bisect-wt-$label
git worktree prune
[ -e "$wt" ] && { echo "$wt exists; pick another label"; exit 2; }
git worktree add -q --no-checkout "$wt" "$c" || exit 2
dirs=$(git show "$c:Cargo.toml" | sed -n 's/.*path *= *"\([^"]*\)".*/\1/p' | sed 's#/[^/]*$##' | sort -u)
keep="src benches examples tests tools crates docs store scripts"
for d in $dirs; do [ "$d" = "src/bin" ] || keep="$keep $d"; done
git -C "$wt" sparse-checkout init --cone
git -C "$wt" sparse-checkout set $keep
git -C "$wt" checkout -q "$c" 2>&1 | tail -1
# Optional compile repair for a window in which main did not build: PATCH_FROM="<commit>:<path>" replaces that one
# file by its version at <commit> (the repaired one).  Recorded in the summary line as "repaired".
if [ -n "${PATCH_FROM:-}" ]; then git -C "$wt" checkout "${PATCH_FROM%%:*}" -- "${PATCH_FROM#*:}"; fi
mkdir -p "$out"; out=$(realpath "$out"); sess=$(realpath "$sess")
( cd "$wt" && CARGO_TARGET_DIR="$repo/target" cargo build --release --bin ecbench 2> "$out/$label.build.log" ) || { echo "$label build_failed"; exit 3; }
"$repo/target/release/ecbench" verify --dir "$sess" --replay-all --out "$out/$label.receipt.json" > /dev/null 2> "$out/$label.stderr"
jq -c --arg l "$label" --arg c "$c" --arg r "${PATCH_FROM:-}" '{label:$l,commit:$c,repaired:$r,ok,replays:(.replays|length),reproduced:([.replays[]|select(.reproduced)]|length)}' "$out/$label.receipt.json" | tee -a "$out/summary.jsonl"
git worktree remove --force "$wt"
