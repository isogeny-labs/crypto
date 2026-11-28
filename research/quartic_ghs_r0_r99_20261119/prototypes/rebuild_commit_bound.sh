#!/usr/bin/env bash
set -euo pipefail

script_dir=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo=$(git -C "$script_dir" rev-parse --show-toplevel)
revision=95297ce1d098e124e72d45effc07fe8c640de82b
tree=aaf30505039899dfce2125579cf1256d308ecd1b
expected_toolchain=be69adad9137eaa0bdac1dd07e2fe286b1831fc8bad193ea27806ca0c7c8e7c6
expected_binary=cbd81182cd5bf6d91ebb53e45d3b353644fae48372eaddc856415808124f28b5
expected_stdout=a375a9767b166ca9b90914835eef192a5f7cd2acba5a09d7b5f16a008ecd5e38
expected_stderr=e04b10c45ecf697666f0172d84c5d1e3eb79af1c255b4cc4e3b9b648ffb228fd

paths=(
  research/quartic_ghs_strong_rho_20261114/main.rs
  research/quartic_ghs_strong_rho_20261114/strong_rho.rs
  research/quartic_n4_20261019/common.rs
  research/quartic_n4_20261019/ghs.rs
  research/quartic_n4v2_20261020/rho2.rs
  src/cryptanalysis/plane_quartic_pic.rs
)
expected_sources=(
  957557d731a983e58887b9c51a89d5ef107f1162084b4689dc8c94a332dc19fd
  b1d223a1e8d812d55250edc42b8e5afe48cf2544aec0aec161ca6568085f0464
  832fd28810909194875a9438e164ec1b45dcefbaa83257b4e3fad3095ef865d6
  652b4760c993e0daa2687837d81f5bd49447dd5d1a562425c55302b4df66abf2
  e18d2802e72ff71e8bc20e5c834b7085124c5aa5d756b9e49773b97f519fd7ec
  e6cf9d5337f1ba3d6b2bc22b1d4f29f4093831817721452e1702a055cf3d1c37
)

actual_tree=$(git -C "$repo" show -s --format=%T "$revision")
[[ "$actual_tree" == "$tree" ]] || {
  printf 'tree mismatch: expected %s, got %s\n' "$tree" "$actual_tree" >&2
  exit 1
}

toolchain=$(rustc --version --verbose | sha256sum | cut -d' ' -f1)
[[ "$toolchain" == "$expected_toolchain" ]] || {
  printf 'toolchain mismatch: expected %s, got %s\n' "$expected_toolchain" "$toolchain" >&2
  exit 1
}

scratch_a=$(mktemp -d /tmp/ghs-r0-r99-rebuild-a.XXXXXX)
scratch_b=$(mktemp -d /tmp/ghs-r0-r99-rebuild-b.XXXXXX)
cleanup() {
  rm -rf -- "$scratch_a" "$scratch_b"
}
trap cleanup EXIT

for scratch in "$scratch_a" "$scratch_b"; do
  git -C "$repo" archive "$revision" "${paths[@]}" | tar -x -C "$scratch"
  for index in "${!paths[@]}"; do
    actual=$(sha256sum "$scratch/${paths[$index]}" | cut -d' ' -f1)
    [[ "$actual" == "${expected_sources[$index]}" ]] || {
      printf 'source mismatch: %s\n' "${paths[$index]}" >&2
      exit 1
    }
  done

  GHS_SOURCE_REVISION="$revision" rustc --edition 2021 \
    -C opt-level=3 \
    -C codegen-units=1 \
    -C strip=debuginfo \
    --remap-path-prefix "$scratch=/source" \
    "$scratch/research/quartic_ghs_strong_rho_20261114/main.rs" \
    -o "$scratch/ghs-strong-rho"
done

hash_a=$(sha256sum "$scratch_a/ghs-strong-rho" | cut -d' ' -f1)
hash_b=$(sha256sum "$scratch_b/ghs-strong-rho" | cut -d' ' -f1)
[[ "$hash_a" == "$expected_binary" && "$hash_b" == "$expected_binary" ]] || {
  printf 'binary mismatch: expected %s, got %s and %s\n' \
    "$expected_binary" "$hash_a" "$hash_b" >&2
  exit 1
}
cmp -s "$scratch_a/ghs-strong-rho" "$scratch_b/ghs-strong-rho"

"$scratch_a/ghs-strong-rho" selftest \
  > "$scratch_a/selftest.stdout" \
  2> "$scratch_a/selftest.stderr"
stdout_hash=$(sha256sum "$scratch_a/selftest.stdout" | cut -d' ' -f1)
stderr_hash=$(sha256sum "$scratch_a/selftest.stderr" | cut -d' ' -f1)
[[ "$stdout_hash" == "$expected_stdout" && "$stderr_hash" == "$expected_stderr" ]] || {
  printf 'selftest receipt mismatch\n' >&2
  exit 1
}
grep -Fxq 'SELFTEST OK' "$scratch_a/selftest.stdout"

printf 'source_commit=%s\n' "$revision"
printf 'source_tree=%s\n' "$tree"
printf 'source_files_verified=%s\n' "${#paths[@]}"
printf 'toolchain_sha256=%s\n' "$toolchain"
printf 'archive_rebuilds=2\n'
printf 'binary_sha256=%s\n' "$hash_a"
printf 'binary_bytes=%s\n' "$(stat -c %s "$scratch_a/ghs-strong-rho")"
printf 'cross_path_binary_equal=true\n'
printf 'selftest_stdout_sha256=%s\n' "$stdout_hash"
printf 'selftest_stderr_sha256=%s\n' "$stderr_hash"
printf 'SELFTEST OK\n'
