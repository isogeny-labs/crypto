#!/usr/bin/env bash
# Package only published source and immutable Rust executables, no credentials.
set -euo pipefail
umask 077
[[ $# == 1 ]] || { echo 'usage: prepare-portfolio-runpod.sh NEW-STAGING-DIR' >&2; exit 2; }
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo=$(/usr/bin/git -C "$here" rev-parse --show-toplevel)
cd "$repo"
[[ -z $(/usr/bin/git status --porcelain --untracked-files=normal) ]] || { echo 'Publish clean source/spec first.' >&2; exit 2; }
sha256sum -c "$here/validation/portfolio34-source.sha256" >/dev/null
sha256sum -c "$here/validation/portfolio34-build.sha256" >/dev/null
mkdir "$1"
stage=$(cd "$1" && pwd)
mkdir "$stage/bundle"
bundle=$stage/bundle
cp target/release/examples/jv_sieve_two_large_prime_dlp_compare "$bundle/driver"
cp "$here/native_analysis/target/release/portfolio-analysis" "$bundle/analyzer"
cp "$here/run-portfolio-task.sh" "$here/portfolio-plan.json" "$bundle/"
cp "$here/validation/runpod34-allocation.json" "$bundle/allocation.json"
cp .resume/RESUME.md "$bundle/RESUME-before.md"
/usr/bin/git rev-parse HEAD > "$bundle/source-revision.txt"
"$HOME/.cargo/bin/rustc" -Vv > "$bundle/build-rustc.txt"
/usr/bin/git ls-files -z -- Cargo.toml Cargo.lock src/lib.rs src/cryptanalysis/jv\* src/cryptanalysis/gaudry\* src/cryptanalysis/f4_fp\* src/cryptanalysis/large_prime_filter\* src/cryptanalysis/residual_walk.rs src/cryptanalysis/bitops.rs src/cryptanalysis/ic_measurement.rs src/prime_hyperelliptic examples/jv_sieve_graph_compare.rs examples/jv_sieve_two_large_prime_dlp_compare.rs research/extension_ic_graph_20261115 | tar --null -T - -cf "$bundle/source-checkpoint.tar"
(cd "$bundle" && sha256sum driver analyzer run-portfolio-task.sh portfolio-plan.json allocation.json RESUME-before.md source-revision.txt build-rustc.txt source-checkpoint.tar) > "$bundle/bundle.sha256"
tar -czf "$stage/bundle.tar.gz" -C "$bundle" .
sha256sum "$stage/bundle.tar.gz" > "$stage/bundle-tar.sha256"
echo "Immutable credential-free measurement bundle: $stage"
