#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
mkdir -p build results fixtures
sed -e 's/^fn main() {/fn reference_main() {/' \
  -e 's/use std::collections::HashMap;/type HashMap<K,V> = std::collections::HashMap<K,V,std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>;/' \
  -e 's/HashMap::new()/HashMap::default()/g' \
  -e '/^struct ArmResult {/a\    scalar: Option<u64>,' \
  -e '/if let Some(k) = found {/a\        res.scalar = Some(k);' vendor/quartic_ic.rs > build/quartic_followup.rs
sed 's/HashMap::new()/HashMap::default()/g' quartic_adapter.rs >> build/quartic_followup.rs
"${RUSTC:-rustc}" --edition 2021 -O build/quartic_followup.rs -o build/quartic_followup
"${RUSTC:-rustc}" --edition 2021 --test -O build/quartic_followup.rs -o build/quartic_tests
for source in isolated_native summarize_native; do
  "${CXX:-g++}" -std=c++17 -O2 -Wall -Wextra -Werror -Ivendor "$source.cpp" -o "build/$source"
done
for source in proof_audit theta_identity; do
  "${CXX:-g++}" -std=c++17 -O2 -Wall -Wextra -Werror -Ivendor "$source.cpp" -lcrypto -o "build/$source"
done
historical=../historical/extension-field-experiments/research_lead/results
for spec in '109 repaired_pipeline.json' '157 forced_source_pipeline.json'; do
  read -r prime file <<< "$spec"
  jq -r '[.p,.r,.quartic,.source_field.modulus,.source_curve,.source_generator.x,.source_generator.y,.source_target.x,.source_target.y,.scalar][]' "$historical/$file" > "fixtures/p$prime.fixture"
done
sha256sum vendor/quartic_ic.rs quartic_adapter.rs fixtures/*.fixture build/quartic_followup > results/build-sha256.txt
