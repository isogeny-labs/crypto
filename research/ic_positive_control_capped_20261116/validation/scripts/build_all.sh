#!/usr/bin/env bash
# Build the frozen ic_positive_control binary from each frozen commit's extracted tree,
# sharing one validator-owned CARGO_TARGET_DIR (registry deps reused, crate rebuilt per tree).
set -u
V=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/validator
export CARGO_TARGET_DIR=$V/target
mkdir -p $V/bin
for c in "$@"; do
  echo "=== build $c start $(date -u +%FT%TZ)"
  # The four trees share one target dir and cargo's path-package metadata hash does not
  # depend on the tree location; git archive mtimes are commit times, so without a touch
  # cargo judges another tree's artifacts fresh (seen 15:50: 0.17 s "builds").
  find $V/src_$c -type f -exec touch {} +
  ( cd $V/src_$c && cargo build --release --locked --offline --bin ic_positive_control ) > $V/build_$c.log 2>&1
  grep -q "Compiling crypto v0.1.0 ($V/src_$c)" $V/build_$c.log || { echo "=== build $c did not compile src_$c: refusing to copy"; continue; }
  rc=$?
  echo "=== build $c rc=$rc end $(date -u +%FT%TZ)"
  if [ $rc -eq 0 ]; then cp $CARGO_TARGET_DIR/release/ic_positive_control $V/bin/ic_positive_control.$c; sha256sum $V/bin/ic_positive_control.$c; fi
done
