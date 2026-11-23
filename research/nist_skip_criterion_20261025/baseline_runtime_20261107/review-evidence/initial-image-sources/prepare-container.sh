#!/usr/bin/env bash
# Credential-free local staging; no registry push, cloud build or random sampling.
set -euo pipefail
[[ $# == 2 && $1 == /* && $2 == /* ]] || {
    echo 'Usage: prepare-container.sh /ABS/EXISTING/PREPARATION /ABS/FRESH/CONTEXT' >&2; exit 2;
}
nist_here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
nist_repo=$(cd -- "$nist_here/../../.." && pwd)
nist_prepared=$1
nist_context=$2
nist_pari=${PARI_ROOT:-/workspace/tools/sage_skeleton_env}
nist_cxx=${CXX:-c++}
# Verify the exact source/library/layer inventories from the retained static build.
(cd "$nist_repo" && sha256sum -c "$nist_prepared/receipts/SOURCE.sha256")
sha256sum -c "$nist_prepared/receipts/LIBRARIES.sha256"
(cd "$nist_prepared/layer" && sha256sum -c "$nist_prepared/receipts/LAYER.sha256")
[[ -z $(find "$nist_prepared/layer" ! -type f ! -type d -print -quit) ]] || {
    echo 'Nonregular layer payload refused' >&2; exit 2;
}
diff -u <(sed -E 's/^[0-9a-f]{64}  //' "$nist_prepared/receipts/LAYER.sha256" | sort) \
    <(cd "$nist_prepared/layer" && find . -type f | sort)
mkdir -- "$nist_context"
mkdir -- "$nist_context/receipts"
cp -a -- "$nist_prepared/layer" "$nist_context/root"
read -r -a nist_dependencies <<< "$(pkg-config --static --libs libcrypto gmpxx gmp)"
"$nist_cxx" --version > "$nist_context/receipts/compiler.txt"
"$nist_cxx" -O2 -std=c++17 -Wall -Wextra -Werror -pedantic -static -I"$nist_pari/include" \
    "$nist_here/known-orders.cpp" -o "$nist_context/root/opt/nist/bin/known-orders" \
    "$nist_pari/lib/libpari.a" "${nist_dependencies[@]}" -lm -pthread
mkdir -p "$nist_context/root/opt/nist/source/baseline_runtime_20261107"
cp -- "$nist_here/known-orders.cpp" "$nist_here/test-launcher.sh" \
    "$nist_context/root/opt/nist/source/baseline_runtime_20261107/"
cp -- "$nist_here/run-field" "$nist_context/root/opt/nist/bin/run-field"
# Old private staging modes are not a container ABI: normalize this fresh copy.
find "$nist_context/root" -type d -exec chmod 755 {} +
find "$nist_context/root" -type f -exec chmod 644 {} +
chmod 755 "$nist_context/root/opt/nist/bin/"*
cp -- "$nist_here/Dockerfile" "$nist_context/Dockerfile"
(
    cd "$nist_context/root"
    find . -type f -print0 | sort -z | xargs -0 sha256sum
) > "$nist_context/receipts/PAYLOAD.sha256"
cp -- "$nist_context/receipts/PAYLOAD.sha256" "$nist_context/root/opt/nist/PAYLOAD.sha256"
chmod 644 "$nist_context/root/opt/nist/PAYLOAD.sha256"
readelf -lW "$nist_context/root/opt/nist/bin/known-orders" > "$nist_context/receipts/known-orders.elf-programs.txt"
readelf -dW "$nist_context/root/opt/nist/bin/known-orders" > "$nist_context/receipts/known-orders.elf-dynamic.txt"
if rg -q 'INTERP|NEEDED' "$nist_context/receipts/known-orders.elf-"*.txt; then
    echo 'Nonstatic known-order control refused' >&2; exit 1;
fi
printf 'Fresh local context staged: %s; no image published or sample generated.\n' "$nist_context"
