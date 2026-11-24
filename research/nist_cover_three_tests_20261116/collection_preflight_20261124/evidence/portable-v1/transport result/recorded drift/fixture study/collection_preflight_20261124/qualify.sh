#!/bin/bash
set -euo pipefail
test "$#" -eq 1
origin=$(cd "$(dirname "$0")" && pwd)
mkdir "$1"
mkdir "$1/bin" "$1/source"
cp "$origin/../fixtures/preflight" "$1/bin/preflight"
cp "$origin/preflight.cpp" "$origin/typed_json.hpp" "$origin/pins.hpp" "$1/source/"
chmod 0555 "$1/bin/preflight"
