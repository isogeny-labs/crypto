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
# Keep original binaries intact. Remove debug/symbol tables from this fresh copy.
sha256sum "$nist_context/root/opt/nist/bin/"* > "$nist_context/receipts/UNSTRIPPED.sha256"
for nist_binary in journal_check sampler_resume journal_tests known-orders; do
    strip --strip-unneeded "$nist_context/root/opt/nist/bin/$nist_binary"
done
# Small explicit userspace, not an AWS CLI image or a host-filesystem copy.
# ldd is used only on these trusted installed tools; every copied file is hashed.
nist_shell_tools=(/bin/bash /bin/true /usr/bin/env /usr/bin/sha256sum \
    /usr/bin/realpath /usr/bin/stat /usr/bin/id /usr/bin/mkdir /usr/bin/mktemp \
    /usr/bin/cat /usr/bin/find /usr/bin/wc /usr/bin/dirname)
ldd "${nist_shell_tools[@]}" > "$nist_context/receipts/shell-ldd.txt"
if rg -q 'not found' "$nist_context/receipts/shell-ldd.txt"; then
    echo 'Shell runtime dependency missing' >&2; exit 1;
fi
awk '$2 == "=>" && substr($3,1,1) == "/" {print $3} substr($1,1,1) == "/" && substr($2,1,1) == "(" {print $1}' \
    "$nist_context/receipts/shell-ldd.txt" | sort -u > "$nist_context/receipts/shell-libraries.txt"
mapfile -t nist_shell_libraries < "$nist_context/receipts/shell-libraries.txt"
[[ ${#nist_shell_libraries[@]} -gt 0 ]] || { echo 'No ELF loader/runtime found' >&2; exit 1; }
for nist_runtime_file in "${nist_shell_tools[@]}" "${nist_shell_libraries[@]}"; do
    [[ $nist_runtime_file == /* && -f $nist_runtime_file ]] || exit 1
    mkdir -p "$nist_context/root$(dirname "$nist_runtime_file")"
    cp --dereference -- "$nist_runtime_file" "$nist_context/root$nist_runtime_file"
done
sha256sum "${nist_shell_tools[@]}" "${nist_shell_libraries[@]}" > "$nist_context/receipts/HOST-RUNTIME.sha256"
mkdir -p "$nist_context/root/etc" "$nist_context/root/work/compute" "$nist_context/root/tmp"
printf 'root:x:0:0:root:/:/bin/bash\nnist:x:1000:1000:development:/work:/bin/bash\n' > "$nist_context/root/etc/passwd"
printf 'root:x:0:\nnist:x:1000:\n' > "$nist_context/root/etc/group"
# Old private staging modes are not a container ABI: normalize this fresh copy.
find "$nist_context/root" -type d -exec chmod 755 {} +
find "$nist_context/root" -type f -exec chmod 644 {} +
chmod 755 "$nist_context/root/opt/nist/bin/"*
for nist_runtime_file in "${nist_shell_tools[@]}"; do
    chmod 755 "$nist_context/root$nist_runtime_file"
done
# The dynamic ELF loader also needs execute permission.
for nist_runtime_file in "${nist_shell_libraries[@]}"; do
    chmod 755 "$nist_context/root$nist_runtime_file"
done
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
