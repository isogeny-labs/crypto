#!/usr/bin/env bash
# Native timing, as PROTOCOL.md declares: `ic price` on rule v3's k0n53 T01
# params, every process under tools/isolated_bench.py --cpus 3 with
# RAYON_NUM_THREADS=1.  An A/A of the baseline x 10 (5 pairs), then 30
# paired A/B rounds with the arm order alternating each round.  Nothing is
# overwritten, so an interrupted run resumes.
#
#   BIN=<dir with ic-{baseline,candidate}> bash run_time.sh
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
runs="$here/runs/time"
params="$repo/research/ic_tool_program/rule/v3/runs/k0n53/T01.params.json"
: "${BIN:?}"
one() { # phase arm round
    local d="$runs/$1" out="$runs/$1/$2-r$3.json"
    [ -e "$out" ] && return 0
    mkdir -p "$d"
    python3 "$repo/tools/isolated_bench.py" run --wait --cpus 3 --max-other-cpu "${MAX_OTHER_CPU:-0.3}" \
        --label "time/$1/$2-r$3" --out "$d/isolation.jsonl" -- env RAYON_NUM_THREADS=1 \
        "$BIN/ic-$2" price --params "$params" --json --out "$out.tmp" > /dev/null
    mv "$out.tmp" "$out"
}
for r in $(seq 1 5); do one aa baseline "$((2 * r - 1))"; one aa baseline "$((2 * r))"; done
for r in $(seq 1 30); do
    if [ $((r % 2)) -eq 1 ]; then one ab baseline "$r"; one ab candidate "$r"
    else one ab candidate "$r"; one ab baseline "$r"; fi
done
echo "runs complete"
