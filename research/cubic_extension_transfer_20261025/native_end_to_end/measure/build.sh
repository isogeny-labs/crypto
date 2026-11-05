#!/usr/bin/env bash
# Development/qualified-image build; this script never launches an experiment.
set -euo pipefail
here=$(cd -- "$(dirname -- "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
revision=$(git -C "$root" rev-parse HEAD)
dirty=false
if [[ -n $(git -C "$root" status --porcelain) ]]; then dirty=true; fi
CRYPTO_BUILD_GIT_COMMIT="$revision" CRYPTO_BUILD_GIT_DIRTY="$dirty" \
  "${CARGO:-cargo}" build --release --locked --manifest-path "$here/Cargo.toml"
