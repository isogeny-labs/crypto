#!/bin/bash
# Build ecbench from the one source archive (git archive of origin/main 7b3c057e1c via mksrc.sh), release, locked.
set -u; . /root/.cargo/env; cd /work; mkdir -p bin s && rm -rf s/* && tar xzf src-7b3c057e1c.tgz -C s && find s -exec touch {} +
export CARGO_TARGET_DIR=/work/target CARGO_INCREMENTAL=0
(cd s && cargo build --release --locked --bin ecbench) > build.log 2>&1; rc=$?
echo "build rc=$rc"; grep -q 'Compiling crypto ' build.log || { echo "did not compile the crypto crate"; exit 99; }
[ $rc -eq 0 ] && cp target/release/ecbench bin/ecbench && sha256sum bin/ecbench && rustc -vV > bin/rustc.txt
