#!/usr/bin/env bash
# Restore byte-exact historical test inputs from the source Git object database.
# No cloud operations; an existing differing file is an error, never overwritten.
set -euo pipefail
study_root=$(cd -- "$(dirname -- "$0")/../../.." && pwd)
cd -- "$study_root"
tmp_fixture=$(mktemp)
trap 'rm -f -- "$tmp_fixture"' EXIT
hydrate() {
    local blob=$1 destination=$2
    git cat-file blob "$blob" > "$tmp_fixture"
    if [[ -e "$destination" ]]; then
        cmp -- "$tmp_fixture" "$destination"
    else
        mkdir -p -- "$(dirname -- "$destination")"
        (set -o noclobber; cat -- "$tmp_fixture" > "$destination")
    fi
}
hydrate 641784a764be689aa3ce4088cb4871e9e9972d58 research/ecbench_all_candidates_20261003/sessions/prime/records.jsonl
hydrate 2be8ee24c8d38f7004ca8ba4fec347a7e07cdb9e research/ecbench_all_candidates_20261003/sessions/koblitz/records.jsonl
hydrate c137804b32063875d9d05bbcfd09818456aadcf5 research/ic_planner_validation_20261020/sessions/r24/records.jsonl
