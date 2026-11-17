#!/usr/bin/env bash
# The follow-up session PROTOCOL.md declares: 42 rounds, each running
# baseline a, baseline q and candidate c of `ic price` (rule v3, k0n53 T01)
# under tools/isolated_bench.py --cpus 3, RAYON_NUM_THREADS=1, the order
# cycling through the six permutations.  Nothing is overwritten, so an
# interrupted run resumes; launches refused for CPU pressure are retried
# and counted in runs/refusals.count.
#
#   BIN=<dir with ic-{baseline,candidate}> bash run.sh
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../.." && pwd)"
runs="$here/runs"
params="$repo/research/ic_tool_program/rule/v3/runs/k0n53/T01.params.json"
: "${BIN:?}"
(cd "$BIN" && sha256sum -c --quiet "$runs/arms.sha256")
refusals="$runs/refusals.count"
[ -e "$refusals" ] || echo 0 > "$refusals"
perms=("a q c" "a c q" "q a c" "q c a" "c a q" "c q a")
one() { # label round
    local out="$runs/$1-r$2.json" bin
    [ -e "$out" ] && return 0
    case "$1" in a|q) bin="$BIN/ic-baseline" ;; c) bin="$BIN/ic-candidate" ;; esac
    # Only a preflight refusal ("machine is busy", no record written) is
    # retried; any other failure stops the session.
    until python3 "$repo/tools/isolated_bench.py" run --wait --cpus 3 --max-other-cpu 0.3 \
        --label "aa2/$1-r$2" --out "$runs/isolation.jsonl" -- env RAYON_NUM_THREADS=1 \
        "$bin" price --params "$params" --json --out "$out.tmp" > /dev/null 2> "$runs/last.err"; do
        if ! grep -q 'machine is busy' "$runs/last.err"; then
            cat "$runs/last.err" >&2
            echo "aa2/$1-r$2 failed (not a refusal); stopping" >&2
            exit 1
        fi
        cat "$runs/last.err" >> "$runs/refusals.log"
        echo $(( $(cat "$refusals") + 1 )) > "$refusals"
        rm -f "$out.tmp"
        sleep 20
    done
    rm -f "$runs/last.err"
    mv "$out.tmp" "$out"
}
for r in $(seq 1 42); do
    for label in ${perms[$(( (r - 1) % 6 ))]}; do one "$label" "$r"; done
done
echo "runs complete"
