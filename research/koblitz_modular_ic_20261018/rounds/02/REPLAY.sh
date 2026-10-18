#!/usr/bin/env bash
# Replay native research in a fresh directory. Requires the extracted raw tree.
set -euo pipefail
round_dir=$(cd "$(dirname "$0")" && pwd)
study_dir=$(cd "$round_dir/../.." && pwd)
expected=${1:-"$round_dir/runs"}
replay_dir=${2:-$(mktemp -d)}
manifest="$study_dir/native/Cargo.toml"
mkdir -p "$replay_dir"
cargo build --release --locked --manifest-path "$manifest"
runner="$study_dir/native/target/release/koblitz-modular-carriers"
"$runner" --round02 "$replay_dir"
"$runner" --delpezzo "$replay_dir/lc2-delpezzo.json"
"$runner" --unbalanced "$replay_dir/lc1b-unbalanced.json"
"$runner" --gluing-modules "$replay_dir"
"$runner" --klein-transfer "$replay_dir/kt0-klein-transfer-replay.json"
"$runner" --klein-transfer-wide "$replay_dir/kt1-klein-transfer-wide.json"
# The historical first KT0 output covered only G and 2G. Keep that input
# for byte-for-byte replay of MR2, whose solver ignores the class labels.
cp "$expected/kt0-klein-transfer.json" "$replay_dir/kt0-klein-transfer.json"
"$runner" --klein-lines "$replay_dir"
"$runner" --klein-lines-wide "$replay_dir"
"$runner" --verify-klein "$replay_dir/kt1-klein-transfer-wide.json"
"$runner" --verify-klein-relations "$replay_dir/mr2b-klein-wide-relations.json" "$replay_dir/kt1-klein-transfer-wide.json"
for name in lc0-bielliptic lc0b-closed-branch lc1-bidegree34 lc1b-unbalanced lc2-delpezzo mr0-orbit-rank mr1-eisenstein-screen gl0-resultants gl1-frobenius-modules kt1-klein-transfer-wide mr2-klein-line-relations-v2 mr2b-klein-wide-relations; do
    cmp "$expected/$name.json" "$replay_dir/$name.json"
done
cmp "$expected/kt0-klein-transfer-v3.json" "$replay_dir/kt0-klein-transfer-replay.json"
printf 'PASS: thirteen native output files replay byte for byte; certificate verifiers pass.\n'
