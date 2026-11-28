#!/usr/bin/env bash
# Package only published source and immutable Rust executables, no credentials.
set -euo pipefail
umask 077
[[ $# == 2 ]] || { echo 'usage: prepare-quotient-runpod.sh NEW-STAGING-DIR ORIGINALS-DIR' >&2; exit 2; }
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo=$(/usr/bin/git -C "$here" rev-parse --show-toplevel)
cd "$repo"
[[ -z $(/usr/bin/git status --porcelain --untracked-files=normal) ]] || { echo 'Publish clean source/spec first.' >&2; exit 2; }
sha256sum -c "$here/validation/quotient-source.sha256" >/dev/null
sha256sum -c "$here/validation/quotient-build.sha256" >/dev/null
mkdir "$1"
stage=$(cd "$1" && pwd)
mkdir "$stage/bundle"
bundle=$stage/bundle
mkdir "$bundle/originals"
while IFS= read -r mode; do
  [[ $(sha256sum "$2/$mode.raw.json" | cut -d " " -f1) == $(jq -er --arg mode "$mode" '.input_sha256[$mode]' "$here/quotient-plan.json") ]] || { echo "Immutable original SHA mismatch."; exit 2; }
  cp "$2/$mode.raw.json" "$bundle/originals/"
done < <(jq -er ".mode_order[]" "$here/quotient-plan.json")
cp "$here/native_analysis/target/release/quotient-bench" "$bundle/driver"
cp "$here/native_analysis/target/release/quotient-analysis" "$bundle/analyzer"
cp "$here/run-quotient-task.sh" "$bundle/run-task.sh"
cp "$here/quotient-plan.json" "$bundle/"
cp "$here/validation/runpod38-allocation.json" "$bundle/allocation.json"
cp .resume/RESUME.md "$bundle/RESUME-before.md"
/usr/bin/git rev-parse HEAD > "$bundle/source-revision.txt"
"$HOME/.cargo/bin/rustc" -Vv > "$bundle/build-rustc.txt"
/usr/bin/git ls-files -z -- Cargo.toml Cargo.lock src/lib.rs src/cryptanalysis/jv\* src/cryptanalysis/gaudry\* src/cryptanalysis/f4_fp\* src/cryptanalysis/large_prime_filter\* src/cryptanalysis/residual_walk.rs src/cryptanalysis/bitops.rs src/cryptanalysis/ic_measurement.rs src/prime_hyperelliptic examples/jv_sieve_graph_compare.rs examples/jv_sieve_two_large_prime_dlp_compare.rs research/extension_ic_graph_20261115 | tar --null -T - -cf "$bundle/source-checkpoint.tar"
(cd "$bundle" && sha256sum driver analyzer originals/*.raw.json run-task.sh quotient-plan.json allocation.json RESUME-before.md source-revision.txt build-rustc.txt source-checkpoint.tar) > "$bundle/bundle.sha256"
tar -czf "$stage/bundle.tar.gz" -C "$bundle" .
sha256sum "$stage/bundle.tar.gz" > "$stage/bundle-tar.sha256"
echo "Immutable credential-free measurement bundle: $stage"
