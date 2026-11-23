#!/bin/bash
# Build ecbench from ONE source archive on a plain Linux CPU box (no Docker): release, locked.
#   pod/build.sh <source.tgz> [workdir=/work]
# <source.tgz> is `git archive --format=tar.gz` of the commit under test, plus this study directory (the main session
# makes it with mksrc.sh of research/wallfix_runpod_20261114/ or plain `git archive HEAD`).  The binary's sha256 is the
# code identity: it is printed, written to bin/BINARY.sha256 and recorded by ecbench in every session.
set -eu
SRC=${1:?source archive}; WORK=${2:-/work}
mkdir -p "$WORK" && cd "$WORK"
[ -f /root/.cargo/env ] && . /root/.cargo/env || true
if ! command -v cargo >/dev/null; then
  command -v apt-get >/dev/null && apt-get install -y --no-install-recommends build-essential pkg-config ca-certificates curl nodejs jq >/dev/null
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable
  . /root/.cargo/env
fi
command -v node >/dev/null || { echo "node (>= 18) is needed for the spec and analysis scripts"; exit 98; }
mkdir -p bin s && rm -rf s/* && tar xzf "$SRC" -C s && find s -exec touch {} +
export CARGO_TARGET_DIR="$WORK/target" CARGO_INCREMENTAL=0
(cd s && cargo build --release --locked --bin ecbench) > build.log 2>&1 && rc=0 || rc=$?
echo "build rc=$rc"
[ $rc -eq 0 ] || { tail -30 build.log; exit $rc; }
grep -q 'Compiling crypto ' build.log || { echo "did not compile the crypto crate"; exit 99; }
cp target/release/ecbench bin/ecbench && rustc -vV > bin/rustc.txt && (cd bin && sha256sum ecbench | tee BINARY.sha256)
