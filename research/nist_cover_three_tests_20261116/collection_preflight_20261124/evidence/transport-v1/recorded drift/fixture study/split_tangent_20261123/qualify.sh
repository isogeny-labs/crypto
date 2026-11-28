#!/bin/bash
set -euo pipefail
test "$#" -eq 1
origin=$(cd "$(dirname "$0")" && pwd)
mkdir "$1"
mkdir "$1/bin" "$1/source"
cp "$origin/../fixtures/collector-stub.sh" "$1/bin/collector"
cp "$origin/../fixtures/checker-stub.sh" "$1/bin/checker"
cp "$origin/collector.cpp" "$1/source/collector.cpp"
cp "$origin/checker.cpp" "$1/source/checker.cpp"
chmod 0555 "$1/bin/collector" "$1/bin/checker"
