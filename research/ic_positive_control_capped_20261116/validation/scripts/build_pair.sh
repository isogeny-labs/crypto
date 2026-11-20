#!/usr/bin/env bash
# Build the frozen ic_positive_control and the validator's val_dump from ONE
# compilation of a tree's crypto_lib, in the shared validator target dir.
# Sources are touched first: the trees share cargo's path-package metadata hash,
# and git-archive mtimes are commit times, so otherwise another tree's lib is
# judged fresh. Usage: build_pair.sh COMMIT [notouch]
set -u
V=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/validator
export CARGO_TARGET_DIR=$V/target
c=$1
while pgrep -f "rustc --crate-name crypto_lib" >/dev/null; do sleep 15; done
python3 -I $V/scripts/make_dumper.py $V/src_$c
[ "${2:-}" = notouch ] || find $V/src_$c -type f -exec touch {} +
echo "=== pair $c start $(date -u +%FT%TZ)"
( cd $V/src_$c && cargo build --release --locked --offline --bin ic_positive_control --bin val_dump ) > $V/build_pair_$c.log 2>&1
rc=$?
echo "=== pair $c rc=$rc end $(date -u +%FT%TZ)"
grep "Compiling crypto v0.1.0" $V/build_pair_$c.log || echo "(crypto not recompiled in this invocation)"
[ $rc -eq 0 ] || exit 1
cp $CARGO_TARGET_DIR/release/val_dump $V/bin/val_dump.$c
if [ "${2:-}" = notouch ]; then
  cmp -s $CARGO_TARGET_DIR/release/ic_positive_control $V/bin/ic_positive_control.$c && echo "frozen binary unchanged by the pair build"
else
  cp $CARGO_TARGET_DIR/release/ic_positive_control $V/bin/ic_positive_control.$c
fi
sha256sum $V/bin/ic_positive_control.$c $V/bin/val_dump.$c
