#!/bin/bash
# Build the PKM pricing instrument and ecbench from ONE source archive on a plain Linux CPU box (no Docker).
# Needs: bash, tar, gcc/cc, a rust toolchain (rustup; the repo's rust-toolchain pin is honoured), sha256sum.
#   WORK=/work SRC=/work/src-<commit>.tgz  bash build.sh
# SRC is `git archive <commit>` of the commit that carries research/pkm_priced_20261115/ and the instrument; its sha256
# is the code identity.  Locked, release, one target dir.  Writes $WORK/bin/{pkm_symmetric_tower,ecbench}, build.log, ids.txt.
set -u; WORK=${WORK:-/work}; SRC=${SRC:?path to the git-archive tgz}
[ -f /root/.cargo/env ] && . /root/.cargo/env; [ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
mkdir -p "$WORK/bin" "$WORK/s" && cd "$WORK" && rm -rf s/* && tar xzf "$SRC" -C s && find s -exec touch {} +
export CARGO_TARGET_DIR=$WORK/target CARGO_INCREMENTAL=0
(cd s && cargo build --release --locked --example pkm_symmetric_tower --bin ecbench) > build.log 2>&1; rc=$?
echo "build rc=$rc"; [ $rc -eq 0 ] || { tail -20 build.log; exit $rc; }
grep -q 'Compiling crypto' build.log || { echo "did not compile the crypto crate"; exit 99; }
cp target/release/examples/pkm_symmetric_tower bin/ && cp target/release/ecbench bin/
{ echo "src_sha256 $(sha256sum "$SRC" | cut -d' ' -f1)"; sha256sum bin/pkm_symmetric_tower bin/ecbench; rustc -vV; } > ids.txt
cat ids.txt | head -4
