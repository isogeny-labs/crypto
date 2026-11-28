#!/usr/bin/env bash
# Build genus_ladder from the git-archive tree of COMMIT in the validator's own target dir.
# Waits for any other crypto_lib compile (shared 16 GB host), touches sources (L-ca1840e60dc0).
set -u
G=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/g4val
export CARGO_TARGET_DIR=$G/target
c=$1
mkdir -p $G/bin
while pgrep -f "rustc --crate-name crypto_lib" >/dev/null; do sleep 20; done
find $G/src_$c -type f -exec touch {} +
echo "=== build $c start $(date -u +%FT%TZ)"
( cd $G/src_$c && cargo build --release --locked --offline -j2 --bin genus_ladder ) > $G/build_$c.log 2>&1
rc=$?
echo "=== build $c rc=$rc end $(date -u +%FT%TZ)"
grep "Compiling crypto v0.1.0" $G/build_$c.log || echo "(crypto not recompiled in this invocation)"
[ $rc -eq 0 ] || { tail -30 $G/build_$c.log; exit 1; }
cp $CARGO_TARGET_DIR/release/genus_ladder $G/bin/genus_ladder.$c
sha256sum $G/bin/genus_ladder.$c
