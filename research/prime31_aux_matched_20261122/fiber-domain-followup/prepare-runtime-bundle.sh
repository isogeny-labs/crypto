#!/usr/bin/env bash
# Construct metadata/source payloads from existing binaries. Never builds/runs a cell.
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
exec python3 "$here/prepare_runtime_bundle.py" "$@"
