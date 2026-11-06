#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
mkdir -p build
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic src/simulate.cpp -o build/simulate
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic src/audit.cpp -o build/audit
build/simulate --self-test
build/simulate --out "${1:-run-synthetic}"
build/audit "${1:-run-synthetic}"
python3 tools/render.py "${1:-run-synthetic}"
