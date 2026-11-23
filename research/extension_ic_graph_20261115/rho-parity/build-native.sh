#!/usr/bin/env bash
# Build against an explicitly pinned, separately built native library.
set -euo pipefail
[[ $# == 2 ]] || { echo 'usage: build-native.sh BASELINE_CHECKOUT NEW_BUILD_DIR' >&2; exit 2; }
root=$(cd "$1" && pwd)
[[ $(git -C "$root" rev-parse HEAD) == 232925556a6e39d4589932c60e75a1bf24f9e049 ]] || exit 2
[[ -z $(git -C "$root" status --porcelain -- src Cargo.toml Cargo.lock build.rs) ]] || exit 2
mkdir "$2"
out=$(cd "$2" && pwd)
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
source /home/agent/.cargo/env
mapfile -t libraries < <(find "$root/target/release/build" -path '*/out/libcrypto_lib-*.rlib')
[[ ${#libraries[@]} == 1 ]] || { echo 'Build the pinned baseline with cargo build --offline --release --lib --locked first.' >&2; exit 2; }
args=(--edition=2021 -C opt-level=3 --extern "crypto_lib=${libraries[0]}")
for dep in rand rayon serde serde_json; do
    mapfile -t matches < <(find "$root/target/release/build/$dep" -path "*/out/lib${dep}-*.rlib")
    [[ ${#matches[@]} == 1 ]] || { echo "Ambiguous dependency: $dep" >&2; exit 2; }
    args+=(--extern "$dep=${matches[0]}")
done
while IFS= read -r dir; do args+=(-L "dependency=$dir" -L "native=$dir"); done < <(find "$root/target/release/build" -type d -name out)
rustc -Vv > "$out/compiler.txt"
printf '%s\n' "${args[@]}" > "$out/rustc-args.txt"
sha256sum "${libraries[0]}" > "$out/library.sha256"
rustc "${args[@]}" "$here/qualify.rs" -o "$out/qualify"
rustc "${args[@]}" --test "$here/qualify.rs" -o "$out/tests"
rustc "${args[@]}" "$here/analyze.rs" -o "$out/analyze"
sha256sum "$out/qualify" "$out/tests" "$out/analyze" > "$out/binaries.sha256"
