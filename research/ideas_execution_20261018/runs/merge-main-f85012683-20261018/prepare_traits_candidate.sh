#!/usr/bin/env bash
# Run from the crypto repository root after the merged-source release build.
# This prepares a candidate only; curve_traits check is a mandatory later gate.
set -euo pipefail
export CARGO_HOME=/workspace/toolchains/cargo
export RUSTUP_HOME=/workspace/toolchains/rustup
export PATH=/workspace/toolchains/cargo/bin:$PATH
study=research/ideas_execution_20261018/runs/merge-main-f85012683-20261018
native_lib=target/release/deps/libcrypto_lib-3e0b38c24aa5646f.rlib
serde_lib=target/release/deps/libserde_json-458049281485fb75.rlib
rustc --edition=2021 -O "$study/refresh_crosswalk_digest.rs" \
  -L dependency=target/release/deps --extern "crypto_lib=$native_lib" \
  --extern "serde_json=$serde_lib" -o /tmp/crypto-pr2148-refresh-crosswalk
rustc --edition=2021 -O "$study/verify_merge_traits.rs" \
  -L dependency=target/release/deps --extern "crypto_lib=$native_lib" \
  --extern "serde_json=$serde_lib" -o /tmp/crypto-pr2148-verify-traits
cp "$study/prior/docs__curves__traits.json.main" /tmp/crypto-pr2148-traits-candidate.json
/tmp/crypto-pr2148-refresh-crosswalk /tmp/crypto-pr2148-traits-candidate.json \
  "$study/prior/main-crosswalk.yaml" "$study/crosswalk-equivalence.json"
/tmp/crypto-pr2148-verify-traits \
  "$study/prior/docs__curves__traits.json.feature" \
  "$study/prior/docs__curves__traits.json.main" \
  /tmp/crypto-pr2148-traits-candidate.json > "$study/traits-coverage.json"
sha256sum "$native_lib" "$serde_lib" target/release/curve_traits \
  target/release/ecbench /tmp/crypto-pr2148-refresh-crosswalk \
  /tmp/crypto-pr2148-verify-traits /tmp/crypto-pr2148-traits-candidate.json \
  > "$study/native-binaries.sha256"
