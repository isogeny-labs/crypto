#!/bin/bash
# build.sh: build ecbench for each source tarball, same toolchain, shared target dir, binaries to /work/bin.
# git archive stamps files with the commit time, older than artifacts already in the shared target dir,
# so cargo's mtime fingerprint would reuse the previous tree's binary (it did, in run 1). Touch the tree
# after unpacking and require that cargo actually compiled the crypto crate.
set -u; . /root/.cargo/env; cd /work; mkdir -p bin
export CARGO_TARGET_DIR=/work/target CARGO_INCREMENTAL=0
for t in src-5382b439bc src-43e68a1d56 src-6745ba0a5f src-94c682e021 src-2c1dc30e16 src-21f21a45a3; do
  rm -rf /work/s && mkdir -p /work/s && tar xzf $t.tgz -C /work/s && find /work/s -exec touch {} +
  s=$(date +%s); (cd /work/s && cargo build --release --locked --bin ecbench) > build-$t.log 2>&1; rc=$?
  echo "$t rc=$rc $(( $(date +%s)-s ))s"
  grep -q 'Compiling crypto ' build-$t.log || { echo "$t did not compile the crypto crate: refusing a stale binary"; rc=99; }
  [ $rc -eq 0 ] && cp /work/target/release/ecbench bin/ecbench-${t#src-} || tail -20 build-$t.log
done
sha256sum bin/* > bin/SHA256SUMS; cat bin/SHA256SUMS; rustc -vV > bin/rustc.txt
