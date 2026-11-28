#!/bin/bash
set -euo pipefail
test "$#" -eq 4
extract_tree=$(realpath "$1")
eval_tree=$(realpath "$2")
supplied=$(realpath "$3")
test ! -e "$4"
mkdir -p "$(dirname "$4")"
mkdir "$4"
out=$(cd "$4" && pwd)
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
work() {
    cp "$supplied" "$out/class.json"
    cp "${BASH_SOURCE[0]}" "$out/evaluate-class.sh"
    cpu=$(python3 -c 'import os; print(min(os.sched_getaffinity(0)))')
    printf '%s\n' "$cpu" > "$out/selected-cpu.txt"
    sha256sum "$out/class.json" "$out/evaluate-class.sh" "$extract_tree/source/"* "$extract_tree/input/"* "$extract_tree/bin/extract" "$eval_tree/source/"* "$eval_tree/bin/pushforward" > "$out/pre-call-manifest.sha256"
    for stage in extract pushforward; do
        set +e
        if test "$stage" = extract; then
            (ulimit -v 1048576; { time timeout 600 taskset -c "$cpu" "$extract_tree/bin/extract" "$extract_tree/input/cover.json" "$extract_tree/input/quartic.json" "$extract_tree/input/relations.json" "$extract_tree/input/formal.json" "$extract_tree/input/actual.json" "$out/class.json" > "$out/extracted.json" 2> "$out/extract.stderr.txt"; } 2> "$out/extract-cost.json")
        else
            (ulimit -v 1048576; { time timeout 600 taskset -c "$cpu" "$eval_tree/bin/pushforward" "$extract_tree/input/quartic.json" "$extract_tree/input/backmap.json" "$out/extracted.json" "$extract_tree/input/pool.json" "$extract_tree/input/formal.json" > "$out/result.json" 2> "$out/pushforward.stderr.txt"; } 2> "$out/pushforward-cost.json")
        fi
        code=$?
        set -e
        printf '%s\n' "$code" > "$out/$stage-EXIT_CODE.txt"
        test "$code" -eq 0 || return "$code"
    done
    sha256sum -c "$out/pre-call-manifest.sha256" > "$out/unchanged-bytes.txt"
}
set +e
{ time (set -e;work); } 2> "$out/wrapper-cost.json"
code=$?
set -e
printf '%s\n' "$code" > "$out/WRAPPER_EXIT_CODE.txt"
exit "$code"
