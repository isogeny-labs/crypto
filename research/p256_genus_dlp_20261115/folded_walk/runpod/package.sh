#!/usr/bin/env bash
# Build a portable correctness replay payload before renting any machine.
set -euo pipefail
if [ "$#" -ne 1 ]; then echo 'usage: package.sh FRESH_OUTPUT_DIRECTORY' >&2; exit 2; fi
study="$(cd "$(dirname "$0")/.." && pwd)"
root="$(git -C "$study" rev-parse --show-toplevel)"
mkdir "$1"
out="$(realpath "$1")"
cd "$root"
cargo build --release --locked --manifest-path "$study/Cargo.toml"
cp "$study/target/release/p256-folded-walk" "$out/p256-folded-walk"
cp "$study/runpod/pod-run.sh" "$out/pod-run.sh"
cp "$study/results/toy.json" "$out/expected-toy.json"
cp "$study/results/trace.json" "$out/expected-trace.json"
git rev-parse HEAD > "$out/source-revision.txt"
git status --porcelain > "$out/source-state.txt"
rustc --version --verbose > "$out/toolchain.txt"
study_root=research/p256_genus_dlp_20261115
rg --files "$study_root/native" "$study_root/classification" "$study_root/transfer_search" "$study_root/folded_walk" \
  -g '*.rs' -g Cargo.toml -g Cargo.lock > "$out/source-files.txt"
printf '%s\n' src/cryptanalysis/identity_certificate.rs src/cryptanalysis/smith_kernel_coverage.rs src/cryptanalysis/smith_prop24.rs >> "$out/source-files.txt"
printf '%s\n' "$study_root/historical/results/toy-zeta.json" docs/identity-certificates/schoenhage-10mult.json >> "$out/source-files.txt"
tar -czf "$out/source.tar.gz" -T "$out/source-files.txt"
(cd "$out" && sha256sum p256-folded-walk pod-run.sh expected-toy.json expected-trace.json source.tar.gz source-revision.txt source-state.txt toolchain.txt source-files.txt > SHA256SUMS)
printf 'Payload prepared: %s\n' "$out"
