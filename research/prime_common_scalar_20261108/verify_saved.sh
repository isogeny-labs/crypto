#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
out=${1:?provide a fresh output directory}
mkdir "$out"
cd "$root"
rg 'check.cpp|analyze.cpp|run.sh' "$here/source-manifest.sha256" | sha256sum -c - > "$out/source-pins.log"
g++ -std=c++20 -O2 -Wall -Wextra -Werror "$here/check.cpp" -lgmpxx -lgmp -o "$out/check"
g++ -std=c++20 -O2 -Wall -Wextra -Werror "$here/analyze.cpp" -lgmpxx -lgmp -o "$out/analyze"
"$out/check" --unit > "$out/unit.log"
"$out/analyze" --unit >> "$out/unit.log"
xz -dc "$here/runs/0/panel.csv.xz" > "$out/main.csv"
xz -dc "$here/runs/1/panel.csv.xz" > "$out/replay.csv"
"$out/analyze" "$out/main.csv" "$out/replay.csv" "$out/reports" > "$out/analysis.log"
cmp "$out/reports/summary.json" "$here/reports/summary.json"
cmp "$out/reports/per-level.csv" "$here/reports/per-level.csv"
echo 'Saved-data reconstruction and grouped recount PASS; no new panel or independent signature.'
