#!/bin/bash
# Writes iso/BUNDLE.sha256: SHA-256 of every file in the R4 bundle (iso/BUNDLE_PATHS), paths relative to the repo root,
# excluding BUNDLE.sha256 itself. Run from anywhere; commit the result. The runner verifies these after unpacking repo.tar.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); repo=$(cd "$here/../../.." && pwd); cd "$repo"
git ls-files $(cat "$here/BUNDLE_PATHS") | grep -v '/iso/BUNDLE\.sha256$' | sort | xargs sha256sum > "$here/BUNDLE.sha256"
wc -l < "$here/BUNDLE.sha256"
