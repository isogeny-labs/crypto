#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
out=${1:?built current rho directory}
cache=/workspace/native-deps/cargo-target/release/build
/workspace/native-deps/rust/bin/rustc --edition=2021 -O --test -C debuginfo=0 -L "dependency=$out/deps" --extern "crypto_lib=$cache/crypto/4202dfa51538bfcf/out/libcrypto_lib-4202dfa51538bfcf.rlib" --extern "rand=$cache/rand/cf232f7e8e2f457a/out/librand-cf232f7e8e2f457a.rlib" --extern "num_bigint=$cache/num-bigint/920d97d93b548abb/out/libnum_bigint-920d97d93b548abb.rlib" --extern "serde=$cache/serde/468b40b659602d93/out/libserde-468b40b659602d93.rlib" --extern "serde_json=$cache/serde_json/7a1f1726470844fa/out/libserde_json-7a1f1726470844fa.rlib" "$here/current-rho.rs" -o "$out/current-rho-tests"
"$out/current-rho-tests" --test-threads=1
