#!/usr/bin/env bash
# PROTOCOL.md's session: 42 rounds; in each, k0n53 then k0n61, labels a q c
# (a, q: KIC_SCAN_PAIRED=0; c: paired) in permutation (r-1) mod 6.  Nothing
# is overwritten, so an interrupted session resumes; busy-gate refusals are
# retried and counted.   BIN=<path to ic> ISO=<path to isolated_bench> bash run.sh
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; repo="$(cd "$here/../../../../.." && pwd)"
runs="$here/runs"; : "${BIN:?}" "${ISO:?}"
echo "$(sha256sum "$BIN" | cut -d' ' -f1)  ic" | cmp - "$runs/arms.sha256"
suite="$repo/research/ic_tool_program/suite/v1"
refusals="$runs/refusals.count"; [ -e "$refusals" ] || echo 0 > "$refusals"
perms=("a q c" "a c q" "q a c" "q c a" "c a q" "c q a")
one() { # label row round
    local out="$runs/$2/$1-r$3.json" seed
    [ -e "$out" ] && return 0
    mkdir -p "$runs/$2"
    seed=$(jq -r --arg id "$2-M1-T01" '.rows[] | select(.id == $id) | .rho_seed' "$suite/SUITE.json")
    local env=(RAYON_NUM_THREADS=1); [ "$1" != c ] && env+=(KIC_SCAN_PAIRED=0)
    until env "${env[@]}" "$ISO" run --wait --cpus 2 --out "$runs/isolation.jsonl" --label "l2ab/$2/$1-r$3" -- \
        "$BIN" price --params "$suite/params/S/$2/M1-T01.json" --json --out "$out.tmp" \
        --single-target --rho-seed "$seed" > /dev/null 2> "$runs/last.err"; do
        if ! grep -qi 'busy\|refus' "$runs/last.err"; then
            cat "$runs/last.err" >&2; echo "l2ab/$2/$1-r$3 failed (not a refusal); stopping" >&2; exit 1
        fi
        cat "$runs/last.err" >> "$runs/refusals.log"; echo $(( $(cat "$refusals") + 1 )) > "$refusals"
        rm -f "$out.tmp"; sleep 15
    done
    rm -f "$runs/last.err"; mv "$out.tmp" "$out"
}
for r in $(seq 1 42); do
    for row in k0n53 k0n61; do
        for label in ${perms[$(( (r - 1) % 6 ))]}; do one "$label" "$row" "$r"; done
    done
done
echo "runs complete"
