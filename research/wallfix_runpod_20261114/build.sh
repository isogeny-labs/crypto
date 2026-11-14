#!/bin/bash
# build.sh: build ecbench for each source tarball, same toolchain, shared target dir, binaries to /work/bin
set -u; . /root/.cargo/env; cd /work; mkdir -p bin
export CARGO_TARGET_DIR=/work/target CARGO_INCREMENTAL=0
for t in src-5382b439bc src-43e68a1d56 src-6745ba0a5f src-94c682e021 src-2c1dc30e16 src-21f21a45a3; do
  rm -rf /work/s && mkdir -p /work/s && tar xzf $t.tgz -C /work/s
  s=$(date +%s); (cd /work/s && cargo build --release --locked --bin ecbench) > build-$t.log 2>&1; rc=$?
  echo "$t rc=$rc $(( $(date +%s)-s ))s"
  [ $rc -eq 0 ] && cp /work/target/release/ecbench bin/ecbench-${t#src-} || tail -20 build-$t.log
done
sha256sum bin/* > bin/SHA256SUMS; cat bin/SHA256SUMS; rustc -vV > bin/rustc.txt
