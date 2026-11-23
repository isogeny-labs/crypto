#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
out=${1:?new build output directory};mkdir -p "$out/deps"
cache=/workspace/native-deps/cargo-target/release/build
while IFS= read -r file;do ln -sf "$file" "$out/deps/$(basename "$file")";done < <(find "$cache" -type f \( -name '*.rlib' -o -name '*.rmeta' -o -name '*.so' \))
compat="$cache/crypto/4202dfa51538bfcf/out/libcrypto_lib-4202dfa51538bfcf.rlib"
rand="$cache/rand/cf232f7e8e2f457a/out/librand-cf232f7e8e2f457a.rlib"
num="$cache/num-bigint/920d97d93b548abb/out/libnum_bigint-920d97d93b548abb.rlib"
serde="$cache/serde/468b40b659602d93/out/libserde-468b40b659602d93.rlib"
json="$cache/serde_json/7a1f1726470844fa/out/libserde_json-7a1f1726470844fa.rlib"
sha256sum "$here/current-rho-scaling.rs" "$here/../../../src/cryptanalysis/prime_fast.rs" "$compat" "$rand" "$num" "$serde" "$json" > "$out/source-and-compat.sha256"
/workspace/native-deps/rust/bin/rustc --edition=2021 -O -C target-cpu=native -C debuginfo=0 -L "dependency=$out/deps" --extern "crypto_lib=$compat" --extern "rand=$rand" --extern "num_bigint=$num" --extern "serde=$serde" --extern "serde_json=$json" "$here/current-rho-scaling.rs" -o "$out/current-rho"
chmod 0555 "$out/current-rho"
