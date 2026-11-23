#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
bash "$here/../verify-gzip-manifest.sh" "$here" PACKAGE.sha256
