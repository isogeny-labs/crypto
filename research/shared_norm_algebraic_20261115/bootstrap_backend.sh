#!/usr/bin/env bash
# User-local native package installation. No system package database changes.
set -euo pipefail
shared_norm_prefix=$(realpath -m "${1:?usage: bootstrap_backend.sh PREFIX}")
shared_norm_packages="$shared_norm_prefix/packages"
mkdir -p "$shared_norm_packages"
cat > "$shared_norm_packages/SHA256SUMS" <<'EOF'
59376c36dbfbaf155dc2725d49b3d23bb2f3f08d8423c47629466e58caf27d66  msolve.deb
bb85466f8b42e5de3d944a02309b5ea836908d873c0958997e3d0ff04fb05ecf  libmsolve.deb
81ac3b20d8a1f125100c84b10b3825a9344080575f88769b553dfe5b008b9e4a  libflint.deb
484b6507b41a1669c8d58562eb53afeeb3955dbdf8e47d3ba4ba63aa14109928  libflint-dev.deb
EOF
while read -r shared_norm_name shared_norm_path; do
    if [ ! -f "$shared_norm_packages/$shared_norm_name" ]; then
        curl --proto '=https' --tlsv1.2 -LfsS --max-time 60 \
            "https://deb.debian.org/debian/$shared_norm_path" \
            -o "$shared_norm_packages/$shared_norm_name"
    fi
done <<'EOF'
msolve.deb pool/main/m/msolve/msolve_0.7.5-1_amd64.deb
libmsolve.deb pool/main/m/msolve/libmsolve0_0.7.5-1_amd64.deb
libflint.deb pool/main/f/flint/libflint19_3.1.3-2_amd64.deb
libflint-dev.deb pool/main/f/flint/libflint-dev_3.1.3-2_amd64.deb
EOF
(cd "$shared_norm_packages" && sha256sum -c SHA256SUMS)
for shared_norm_package in "$shared_norm_packages"/*.deb; do
    dpkg-deb -x "$shared_norm_package" "$shared_norm_prefix"
done
printf 'Native msolve/FLINT installed under %s\n' "$shared_norm_prefix"
