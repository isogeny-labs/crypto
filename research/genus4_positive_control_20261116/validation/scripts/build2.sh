#!/usr/bin/env bash
# Build BIN from the git-archive tree of COMMIT in the validator's own target dir, yielding to other
# sessions: wait until no foreign crypto_lib compile runs; if one starts while ours compiles
# crypto_lib, kill ours and retry (two concurrent crypto_lib release compiles OOM this host).
# Sources are touched first (lesson L-ca1840e60dc0).  Usage: build2.sh COMMIT BIN [notouch]
set -u
G=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/g4val
export CARGO_TARGET_DIR=$G/target
c=$1; bin=$2; tree=$G/src_$c
mkdir -p $G/bin
foreign() { for p in $(pgrep -f "rustc --crate-name crypto_lib"); do [ "$(readlink /proc/$p/cwd)" != "$tree" ] && return 0; done; return 1; }
mine() { for p in $(pgrep -f "rustc --crate-name crypto_lib"); do [ "$(readlink /proc/$p/cwd)" = "$tree" ] && return 0; done; return 1; }
[ "${3:-}" = notouch ] || find $tree -type f -exec touch {} +
for attempt in 1 2 3 4 5 6 7 8; do
  while foreign; do sleep 15; done
  echo "=== build $c $bin attempt $attempt start $(date -u +%FT%TZ)"
  ( cd $tree && exec cargo build --release --locked --offline -j2 --bin $bin ) >> $G/build_${c}_$bin.log 2>&1 &
  cp=$!
  yielded=0
  while kill -0 $cp 2>/dev/null; do
    if mine && foreign; then kill $cp; wait $cp 2>/dev/null; yielded=1; echo "    yielded to a foreign crypto_lib compile at $(date -u +%FT%TZ)"; break; fi
    sleep 5
  done
  [ $yielded = 1 ] && continue
  wait $cp; rc=$?
  echo "=== build $c $bin rc=$rc end $(date -u +%FT%TZ)"
  grep "Compiling crypto v0.1.0" $G/build_${c}_$bin.log | tail -1 || echo "(crypto not recompiled in this invocation)"
  [ $rc -eq 0 ] || { tail -20 $G/build_${c}_$bin.log; exit 1; }
  cp $CARGO_TARGET_DIR/release/$bin $G/bin/$bin.$c
  sha256sum $G/bin/$bin.$c
  exit 0
done
echo "gave up after 8 attempts"; exit 1
