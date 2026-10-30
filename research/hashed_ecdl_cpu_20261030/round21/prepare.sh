#!/usr/bin/env bash
# Software preparation only; never executes a measured panel or uploads data.
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
out=${1:?fresh artifact directory required}
study="$repo/research/hashed_ecdl_cpu_20261030/round21"
mkdir "$out"
mkdir "$out/bundle"
for arm in candidate previous inverse; do
    case "$arm" in
      candidate) revision=5ad00c65dc4f06d66c8f09abb9ca4a265783c20d ;;
      previous) revision=2f08282ec3f5ee9b16f61795421e4871a1282800 ;;
      inverse) revision=5b37f957e00d2347210f9bcadaeeac09a895ce8f ;;
    esac
    mkdir "$out/$arm-source"
    git archive "$revision" crates/hashed-ecdl-cpu crates/hashed-ecdl-tensor \
      src/cryptanalysis/hashed_ecdl.rs \
      research/hashed_ecdl_cpu_20261030/panel2/results/cert-v-0-1.txt \
      research/hashed_ecdl_cpu_20261030/panel2/results/cert-v-1-1.txt \
      research/hashed_ecdl_cpu_20261030/panel2/results/cert-v-2-1.txt \
      | tar -x -C "$out/$arm-source"
    (cd "$out/$arm-source"; find . -type f -print0 | sort -z | xargs -0 sha256sum) > "$out/$arm-source.sha256"
    export CARGO_TARGET_DIR="$out/target"
    manifest="$out/$arm-source/crates/hashed-ecdl-cpu/Cargo.toml"
    cargo test --offline --locked --release --manifest-path "$manifest" --lib > "$out/$arm-tests.log" 2>&1
    cargo build --offline --locked --release --manifest-path "$manifest" --example complete > "$out/$arm-build.log" 2>&1
    cp "$out/target/release/examples/complete" "$out/bundle/$arm"
    jq -n --arg revision "$revision" --arg arm "$arm" \
      --arg binary "$(sha256sum "$out/bundle/$arm" | cut -d ' ' -f1)" \
      --arg sources "$(sha256sum "$out/$arm-source.sha256" | cut -d ' ' -f1)" \
      '{repository_commit:$revision,integrated:true,binary:$arm,binary_sha256:$binary,source_manifest_sha256:$sources}' \
      > "$out/$arm-materialization.json"
done
for name in baseline analyze; do
    cp "/home/ec2-user/hashed-ecdl-cpu-round20/bundle/$name" "$out/bundle/$name"
    expected=$(jq -r --arg name "$name" '.binaries[$name]' "$repo/research/hashed_ecdl_cpu_20261030/round20/build.json")
    printf '%s  %s\n' "$expected" "$out/bundle/$name" | sha256sum -c -
done
(cd "$out/bundle"; sha256sum baseline candidate previous inverse analyze > binaries.sha256; sha256sum -c binaries.sha256)
cp "$out/"*-source.sha256 "$out/"*-materialization.json "$out/"*-tests.log "$out/"*-build.log "$study/"
cp "$out/bundle/binaries.sha256" "$study/"
rustc -Vv > "$study/rustc.txt"
printf '%s\n' 'Software preparation complete; no freeze, upload, panel execution or submission.'
