#!/bin/bash
set -euo pipefail
test "$#" -eq 3
extract_tree=$(realpath "$1")
eval_tree=$(realpath "$2")
test ! -e "$3"
mkdir -p "$(dirname "$3")"
mkdir "$3"
out=$(cd "$3" && pwd)
study=$(cd "$(dirname "$0")" && pwd)
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
work() {
    mkdir "$out/source"
    cp "$study/evaluate-class.sh" "$study/INTERFACE.md" "$study/check-class-only.sh" "$out/source/"
    python3 - "$extract_tree/cases/public-FG.json" "$out/class-only.json" <<'PY'
import json,sys
from pathlib import Path
data=json.loads(Path(sys.argv[1]).read_text())
Path(sys.argv[2]).write_text(json.dumps({'class_space':data['class_space']},separators=(',',':'))+'\n')
PY
    sha256sum "$out/source/"* "$out/class-only.json" "$extract_tree/cases/public-FG.json" "$extract_tree/calls/public-FG-verify/result.json" "$eval_tree/bin/pushforward" > "$out/pre-call-manifest.sha256"
    bash "$out/source/evaluate-class.sh" "$extract_tree" "$eval_tree" "$out/class-only.json" "$out/api"
    cpu=$(python3 -c 'import os; print(min(os.sched_getaffinity(0)))')
    printf '%s\n' "$cpu" > "$out/selected-cpu.txt"
    set +e
    (ulimit -v 1048576; { time timeout 600 taskset -c "$cpu" "$eval_tree/bin/pushforward" --compare "$out/api/result.json" "$extract_tree/calls/public-FG-verify/result.json" > "$out/comparison.json" 2> "$out/comparison.stderr.txt"; } 2> "$out/comparison-cost.json")
    code=$?
    set -e
    printf '%s\n' "$code" > "$out/COMPARISON_EXIT_CODE.txt"
    test "$code" -eq 0 || return "$code"
    sha256sum -c "$out/pre-call-manifest.sha256" > "$out/unchanged-bytes.txt"
}
set +e
{ time (set -e;work); } 2> "$out/wrapper-cost.json"
code=$?
set -e
printf '%s\n' "$code" > "$out/WRAPPER_EXIT_CODE.txt"
exit "$code"
