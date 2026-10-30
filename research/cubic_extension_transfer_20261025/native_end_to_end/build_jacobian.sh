#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
mkdir -p build
printf '%s\n' '#![allow(dead_code)]' > build/jacobian_native.rs
sed -e 's/^fn main() {/fn reference_main() {/' \
  -e 's/use std::collections::HashMap;/type HashMap<K,V> = std::collections::HashMap<K,V,std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>;/' \
  -e 's/HashMap::new()/HashMap::default()/g' \
  -e '/^struct ArmResult {/a\    scalar: Option<u64>,' \
  -e '/if let Some(k) = found {/a\        res.scalar = Some(k);' ../native_followup/vendor/quartic_ic.rs >> build/jacobian_native.rs
cat jacobian_bridge.rs >> build/jacobian_native.rs
"${RUSTC:-rustc}" --edition 2021 -O build/jacobian_native.rs -o build/jacobian_order
"${RUSTC:-rustc}" --edition 2021 -O --crate-type cdylib build/jacobian_native.rs -o build/libcubic_jacobian.so
