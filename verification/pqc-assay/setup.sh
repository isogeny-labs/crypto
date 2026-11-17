#!/usr/bin/env bash
# Thin tool/source bootstrap; no cryptographic implementation is generated here.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
# shellcheck source=toolchain.env
source "$here/toolchain.env"
cache=${ASSAY_CACHE:-"$here/.cache"}
mkdir -p "$cache"
case "$(uname -s)/$(uname -m)" in
    Linux/x86_64) ;;
    *) echo 'This bootstrap supports Linux x86_64; see README.md for the upstream platforms.' >&2; exit 2 ;;
esac
saw_home=${ASSAY_SAW_HOME:-"$cache/saw-$ASSAY_SAW_VERSION"}
if [[ ! -x "$saw_home/bin/saw" ]]; then
    archive="$cache/$ASSAY_SAW_ASSET"
    curl --fail --location --retry 3 --output "$archive.partial" \
        "https://github.com/GaloisInc/saw-script/releases/download/v$ASSAY_SAW_VERSION/$ASSAY_SAW_ASSET"
    printf '%s  %s\n' "$ASSAY_SAW_SHA256" "$archive.partial" | sha256sum --check -
    mv "$archive.partial" "$archive"
    mkdir -p "$saw_home"
    tar -xzf "$archive" -C "$saw_home" --strip-components=1
fi
"$saw_home/bin/saw" --version 2>&1 | grep -F "$ASSAY_SAW_VERSION ("
upstream="$cache/upstream"
if [[ ! -d "$upstream/.git" ]]; then
    git init --quiet "$upstream"
    git -C "$upstream" remote add origin https://github.com/amarshat/pqc-assay.git
fi
if ! git -C "$upstream" cat-file -e "$ASSAY_UPSTREAM_REV^{commit}" 2>/dev/null; then
    git -C "$upstream" fetch --depth=1 origin "$ASSAY_UPSTREAM_REV"
fi
git -C "$upstream" checkout --quiet --detach "$ASSAY_UPSTREAM_REV"
test "$(git -C "$upstream" rev-parse HEAD)" = "$ASSAY_UPSTREAM_REV"
git -C "$upstream" diff --exit-code HEAD -- model/cryptol/MLDSA_NTT.cry
printf 'SAW: %s\nUpstream: %s\n' "$saw_home" "$ASSAY_UPSTREAM_REV"
