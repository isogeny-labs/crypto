#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
mkdir -p build
printf '%s\n' '#![allow(dead_code)]' > build/source_native.rs
sed -e 's/^fn main() {/fn reference_main() {/' \
  -e 's/use std::collections::HashMap;/type HashMap<K,V> = std::collections::HashMap<K,V,std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>;/' \
  -e 's/HashMap::new()/HashMap::default()/g' ../native_followup/vendor/quartic_ic.rs >> build/source_native.rs
# Reuse only the pinned native parser, cubic arithmetic and rho. Exclude the
# former synthetic target generator and every ten-line/known-log reader.
sed -n '/^fn signed_terms(/,/^fn fixed_instance(/{ /^fn fixed_instance(/!p; }' ../native_followup/quartic_adapter.rs >> build/source_native.rs
sed -n '/^#\[derive(Clone,Copy,Debug,PartialEq,Eq)\]/,/^fn source_fixture(/{ /^fn source_fixture(/!p; }' ../native_followup/quartic_adapter.rs >> build/source_native.rs
sed -n '/^fn source_rho(/,/^fn main(/{ /^fn main(/!p; }' ../native_followup/quartic_adapter.rs | sed 's/HashMap::new()/HashMap::default()/g' >> build/source_native.rs
cat source_adapter.rs >> build/source_native.rs
"${RUSTC:-rustc}" --edition 2021 -O build/source_native.rs -o build/source_public
