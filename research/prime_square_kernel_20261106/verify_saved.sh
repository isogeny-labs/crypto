#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:?provide output directory}
mkdir -p "$out"
gzip -dc "$here/runs/local-20261107-main/panel.csv.gz" > "$out/main.csv"
gzip -dc "$here/runs/local-20261107-replay/panel.csv.gz" > "$out/replay.csv"
g++ -std=c++20 -O2 -Wall -Wextra -Werror "$here/analyze.cpp" -lgmpxx -lgmp -o "$out/analyze"
"$out/analyze" --unit
"$out/analyze" "$out/main.csv" "$out/replay.csv" "$out/recomputed"
cmp "$out/recomputed/summary.json" "$here/reports/local-20261107/summary.json"
cmp "$out/recomputed/per-level.csv" "$here/reports/local-20261107/per-level.csv"
