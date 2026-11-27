#!/usr/bin/env bash
# Unprivileged native build only. Never publishes an image or launches a run.
set -euo pipefail
[[ $# == 8 ]] || {
  echo 'usage: build-pair.sh BASE_DIR BASE_SHA CANDIDATE_DIR CANDIDATE_SHA DRIVER_DIR DRIVER_SHA OPTIONS_JSON NEW_OUTPUT_DIR' >&2
  exit 2
}
base_dir=$(realpath "$1")
base_sha=$2
candidate_dir=$(realpath "$3")
candidate_sha=$4
driver_dir=$(realpath "$5")
driver_sha=$6
options=$(realpath "$7")
output=$(realpath -m "$8")
overlay="$driver_dir/research/unified_point_index_20261127/panel-bridge-20261127/native-metrics.patch"
for sha in "$base_sha" "$candidate_sha" "$driver_sha"; do
  [[ $sha =~ ^[a-f0-9]{40}$ ]] || { echo "invalid source SHA: $sha" >&2; exit 2; }
done
for pair in "$base_dir:$base_sha" "$candidate_dir:$candidate_sha" "$driver_dir:$driver_sha"; do
  dir=${pair%:*}
  sha=${pair#*:}
  [[ $(git -C "$dir" rev-parse HEAD) == "$sha" ]] || { echo "source revision mismatch: $dir" >&2; exit 2; }
  git -C "$dir" diff --quiet HEAD || { echo "dirty source: $dir" >&2; exit 2; }
  [[ -z $(git -C "$dir" ls-files --others --exclude-standard) ]] || { echo "untracked source: $dir" >&2; exit 2; }
  [[ -f $dir/crates/hashed-ecdl-tensor/Cargo.toml ]] || { echo "incomplete native checkout: $dir" >&2; exit 2; }
done
[[ -f $options && ! -L $options ]] || { echo 'missing or symlinked options JSON' >&2; exit 2; }
[[ -s $overlay && ! -L $overlay ]] || { echo 'missing or symlinked native metrics overlay' >&2; exit 2; }
overlay_hash=$(sha256sum "$overlay" | cut -d' ' -f1)
[[ $options == "$driver_dir/research/unified_point_index_20261127/panel-bridge-20261127/optimized-options.json" ]] || {
  echo 'options must come from the pinned driver source' >&2
  exit 2
}
jq -e 'type == "object"' "$options" >/dev/null
options_hash=$(sha256sum "$options" | cut -d' ' -f1)
[[ ! -e $output && ! -L $output ]] || { echo 'output already exists' >&2; exit 2; }
for dir in "$base_dir" "$candidate_dir" "$driver_dir"; do
  [[ $output != "$dir" && $output != "$dir/"* ]] || { echo 'build output must be outside every source checkout' >&2; exit 2; }
done
mkdir "$output"
export CARGO_BUILD_JOBS=2 RUSTFLAGS='-C target-cpu=x86-64'
target=x86_64-unknown-linux-musl
build() {
  local source=$1 bin=$2 label=$3
  local target_dir="$output/build-$label"
  local copy="$output/source-$label"
  local manifest="$source/crates/hashed-ecdl-tensor/Cargo.toml"
  if [[ $label != panel ]]; then
    mkdir -p "$copy/crates"
    cp -a "$source/crates/hashed-ecdl-tensor" "$copy/crates/"
    (cd "$copy" && git apply --unidiff-zero --check "$overlay" && git apply --unidiff-zero "$overlay")
    for file in src/bin/hashed-ecdl.rs src/collector_metrics.rs src/metrics.rs tests/optimized_cli.rs; do
      cmp "$copy/crates/hashed-ecdl-tensor/$file" "$driver_dir/crates/hashed-ecdl-tensor/$file" || {
        echo "native metrics overlay differs from driver source: $label/$file" >&2
        exit 1
      }
    done
    manifest="$copy/crates/hashed-ecdl-tensor/Cargo.toml"
  fi
  if [[ $label != panel ]]; then
    CARGO_TARGET_DIR="$target_dir" cargo +1.78.0 test --locked --release \
      --manifest-path "$manifest" --target "$target" --test optimized_cli \
      > "$output/$label.test.log" 2>&1
  fi
  CARGO_TARGET_DIR="$target_dir" cargo +1.78.0 build --locked --release \
    --manifest-path "$manifest" \
    --target "$target" --bin "$bin" > "$output/$label.build.log" 2>&1
  install -m755 "$target_dir/$target/release/$bin" "$output/$label"
  readelf -h "$output/$label" | grep -q 'Advanced Micro Devices X86-64'
  if readelf -l "$output/$label" | grep -q INTERP; then
    echo "dynamically linked binary: $label" >&2
    exit 1
  fi
  rm -r "$target_dir"
  if [[ $label != panel ]]; then rm -r "$copy"; fi
}
build "$base_dir" hashed-ecdl baseline
build "$candidate_dir" hashed-ecdl candidate
build "$driver_dir" hashed-ecdl-panel panel
for pair in "$base_dir:$base_sha" "$candidate_dir:$candidate_sha" "$driver_dir:$driver_sha"; do
  dir=${pair%:*}
  sha=${pair#*:}
  [[ $(git -C "$dir" rev-parse HEAD) == "$sha" ]] || { echo "source revision changed during build: $dir" >&2; exit 1; }
  git -C "$dir" diff --quiet HEAD || { echo "source changed during build: $dir" >&2; exit 1; }
  [[ -z $(git -C "$dir" ls-files --others --exclude-standard) ]] || { echo "new untracked source during build: $dir" >&2; exit 1; }
done
[[ $(sha256sum "$options" | cut -d' ' -f1) == "$options_hash" ]] || {
  echo 'optimized options changed during build' >&2
  exit 1
}
install -m644 "$options" "$output/optimized-options.json"
[[ $(sha256sum "$overlay" | cut -d' ' -f1) == "$overlay_hash" ]] || {
  echo 'native metrics overlay changed during build' >&2
  exit 1
}
(cd "$output" && sha256sum baseline candidate panel optimized-options.json > SHA256SUMS)
jq -n --arg baseline "$base_sha" --arg candidate "$candidate_sha" --arg driver "$driver_sha" \
  --arg target "$target" --arg rustc "$(rustc +1.78.0 -Vv)" \
  --arg baseline_hash "$(sha256sum "$output/baseline" | cut -d' ' -f1)" \
  --arg candidate_hash "$(sha256sum "$output/candidate" | cut -d' ' -f1)" \
  --arg panel_hash "$(sha256sum "$output/panel" | cut -d' ' -f1)" \
  --arg options_hash "$(sha256sum "$output/optimized-options.json" | cut -d' ' -f1)" \
  --arg overlay_hash "$overlay_hash" \
  '{schema:"point-index.build-pair/v2",baseline_revision:$baseline,candidate_revision:$candidate,
    driver_revision:$driver,target:$target,rustc:$rustc,rustflags:"-C target-cpu=x86-64",
    baseline_sha256:$baseline_hash,candidate_sha256:$candidate_hash,panel_sha256:$panel_hash,
    optimized_options_sha256:$options_hash,measurement_overlay_sha256:$overlay_hash,
    calibrated_total_operations:null,experiment_submitted:false}' > "$output/build-info.json"
echo "Native pair bundle built and hashed in $output; no experiment submitted."
