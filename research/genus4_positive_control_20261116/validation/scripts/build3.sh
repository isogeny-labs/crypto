#!/usr/bin/env bash
# Build BIN from the git-archive tree of COMMIT in the validator's own target dir, memory-aware:
# start only when no foreign crypto_lib *test* compile runs and MemAvailable >= 4 GiB; kill our own
# compile (never another session's) if a foreign test compile appears or MemAvailable < 3 GiB, and
# retry.  Sources touched first (L-ca1840e60dc0).  Usage: build3.sh COMMIT BIN [notouch]
set -u
G=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/g4val
export CARGO_TARGET_DIR=$G/target
c=$1; bin=$2; tree=$G/src_$c
mkdir -p $G/bin
avail_mb() { echo $(( $(awk '/MemAvailable/{print $2}' /proc/meminfo) / 1024 )); }
foreign_test() { for p in $(pgrep -f "rustc --crate-name crypto_lib"); do [ "$(readlink /proc/$p/cwd)" != "$tree" ] && tr '\0' ' ' < /proc/$p/cmdline | grep -q -- "--test" && return 0; done; return 1; }
[ "${3:-}" = notouch ] || find $tree -type f -exec touch {} +
for attempt in $(seq 1 30); do
  while foreign_test || [ $(avail_mb) -lt 4096 ]; do sleep 10; done
  echo "=== build $c $bin attempt $attempt start $(date -u +%FT%TZ) avail=$(avail_mb)MB"
  ( cd $tree && exec cargo build --release --locked --offline -j2 --bin $bin ) >> $G/build_${c}_$bin.log 2>&1 &
  cp=$!; yielded=0
  while kill -0 $cp 2>/dev/null; do
    if foreign_test || [ $(avail_mb) -lt 3072 ]; then
      pkill -P $cp 2>/dev/null; kill $cp 2>/dev/null
      for p in $(pgrep -f "rustc --crate-name"); do [ "$(readlink /proc/$p/cwd)" = "$tree" ] && kill $p; done
      wait $cp 2>/dev/null; yielded=1; echo "    yielded at $(date -u +%FT%TZ) avail=$(avail_mb)MB"; break
    fi
    sleep 3
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
echo "gave up"; exit 1
