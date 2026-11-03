#!/usr/bin/env bash
# Recheck the upstream model-to-math leg, including regenerated-lift identity.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
# shellcheck source=toolchain.env
source "$here/toolchain.env"
cache=${ASSAY_CACHE:-"$here/.cache"}
if [[ ! -d "$cache/upstream/.git" ]]; then
    mkdir -p "$cache/upstream"
    git init --quiet "$cache/upstream"
    git -C "$cache/upstream" remote add origin https://github.com/amarshat/pqc-assay.git
fi
if ! git -C "$cache/upstream" cat-file -e "$ASSAY_UPSTREAM_REV^{commit}" 2>/dev/null; then
    git -C "$cache/upstream" fetch --depth=1 origin "$ASSAY_UPSTREAM_REV"
fi
git -C "$cache/upstream" checkout --quiet --detach "$ASSAY_UPSTREAM_REV"
cd "$cache/upstream"
test "$(git rev-parse HEAD)" = "$ASSAY_UPSTREAM_REV"
git diff --exit-code HEAD
SKIP_PROVERIF=1 ./scripts/setup.sh
./scripts/setup_isabelle_cryptol.sh
make lift-check isabelle
