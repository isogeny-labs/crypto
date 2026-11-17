#!/usr/bin/env bash
# Links the census against the repository's compiled crypto_lib (built by
# `cargo build --release --example jv_cover` at main 5f3b5c209a; jv_cover.rs differs
# from the study's 0c2258ead2 only by rustfmt line breaks). Output binary in target/.
set -eu
R=/home/user/crypto; D=$R/target/release; OUT=$R/target/rt_fbline_redteam
mkdir -p $OUT
LIB=$(ls $D/deps/libcrypto_lib-42993a31e73be122.rlib)
RAND=$(ls $D/deps/librand-995139fa871833e1.rlib)
rustc --edition 2021 -C opt-level=3 -C target-cpu=native "$(dirname "$0")/src/main.rs" -o $OUT/sieve_transfer_pilot \
  -L dependency=$D/deps --extern crypto_lib=$LIB --extern rand=$RAND \
  -L native=$D/build/ring-c0fdbe03416acbc1/out -L native=$D/build/blake3-0755614ee3b1c653/out \
  -L native=$D/build/libsqlite3-sys-149b4cc2a9bd6b30/out
