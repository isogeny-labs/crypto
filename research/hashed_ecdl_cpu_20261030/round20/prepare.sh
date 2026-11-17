#!/usr/bin/env bash
# Build software artifacts only. Does not run a benchmark, freeze or submit.
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
out=${1:?fresh artifact directory required}
mkdir "$out"
mkdir "$out/bundle"
study="$repo/research/hashed_ecdl_cpu_20261030/round20"
for arm in candidate previous; do
    case "$arm" in
      candidate) revision=fab61dde9ada8fafcaa4d15af735f3b422e886d0 ;;
      previous) revision=a6117b9ec7743d879223806b1cc28556ba83903e ;;
    esac
    mkdir "$out/$arm-source"
    git archive "$revision" crates/hashed-ecdl-cpu crates/hashed-ecdl-tensor \
      src/cryptanalysis/hashed_ecdl.rs \
      research/hashed_ecdl_cpu_20261030/panel2/results/cert-v-0-1.txt \
      research/hashed_ecdl_cpu_20261030/panel2/results/cert-v-1-1.txt \
      research/hashed_ecdl_cpu_20261030/panel2/results/cert-v-2-1.txt \
      | tar -x -C "$out/$arm-source"
    (
      cd "$out/$arm-source"
      find . -type f -print0 | sort -z | xargs -0 sha256sum
    ) > "$out/$arm-source.sha256"
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
cp /home/ec2-user/hashed-ecdl-cpu-round19/bundle/baseline "$out/bundle/baseline"
# Legacy basename inverse is the integrated pre-streaming candidate, not a patch.
cp /home/ec2-user/hashed-ecdl-cpu-round19/bundle/candidate "$out/bundle/inverse"
cp /home/ec2-user/hashed-ecdl-cpu-round19/bundle/analyze "$out/bundle/analyze"
for mapping in baseline:baseline inverse:candidate analyze:analyze; do
    name=${mapping%%:*}
    old=${mapping#*:}
    expected=$(jq -r --arg name "$old" '.binaries[$name]' "$repo/research/hashed_ecdl_cpu_20261030/round19/build.json")
    printf '%s  %s\n' "$expected" "$out/bundle/$name" | sha256sum -c -
done
(cd "$out/bundle" && sha256sum baseline candidate previous inverse analyze > binaries.sha256 && sha256sum -c binaries.sha256)
cp "$out/"*-source.sha256 "$out/"*-materialization.json "$out/"*-tests.log "$out/"*-build.log "$study/"
cp "$out/bundle/binaries.sha256" "$study/"
rustc -Vv > "$study/rustc.txt"
printf '%s\n' 'Software preparation complete. No protocol freeze, upload, benchmark or submission.'
