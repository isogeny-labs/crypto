#!/usr/bin/env bash
# Validator rebuild of genus_ladder at ca72267899 from a git-archive tree (paths.py) in a
# validator-owned tree and target directory.  The root disk had 1.8 GB free, so the target
# directory is a RAM-backed tmpfs reached through a symlink in the validator's scratch dir.
set -u
V=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/val4po
T=/dev/shm/val4po_target
mkdir -p $T && ln -sfn $T $V/target
export CARGO_TARGET_DIR=$V/target
tree=$V/src_ca72267899
find $tree -type f -exec touch {} +
echo "=== start $(date -u +%FT%TZ)"
echo "=== other rustc crypto_lib running: $(pgrep -fc 'rustc --crate-name crypto_lib' || true)"
( cd $tree && cargo build --release --locked --offline --bin genus_ladder ) > $V/build.log 2>&1
rc=$?
echo "=== rc=$rc end $(date -u +%FT%TZ)"
grep "Compiling crypto v0.1.0" $V/build.log | tail -1
grep "Finished" $V/build.log | tail -1
[ $rc -eq 0 ] && mkdir -p $V/bin && cp $CARGO_TARGET_DIR/release/genus_ladder $V/bin/genus_ladder.val && sha256sum $V/bin/genus_ladder.val
