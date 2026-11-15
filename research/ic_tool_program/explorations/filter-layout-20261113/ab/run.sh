#!/usr/bin/env bash
# PROTOCOL.md's session: 42 rounds; in each, k0n53 then k0n61, labels in
# permutation (r-1) mod 6 of [a q t, a t q, q a t, q t a, t a q, t q a].
# a, q: KIC_FILTER_LAYOUT=low; t: the default (top). Resumes; busy-gate
# refusals are retried and counted.   BIN=<ic> ISO=<isolated_bench> bash run.sh
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; repo="$(cd "$here/../../../../.." && pwd)"
runs="$here/runs"; : "${BIN:?}" "${ISO:?}"
echo "$(sha256sum "$BIN" | cut -d' ' -f1)  ic" | cmp - "${ARMS:-$runs/arms.sha256}"
suite="$repo/research/ic_tool_program/suite/v1"
refusals="$runs/refusals.count"; [ -e "$refusals" ] || echo 0 > "$refusals"
perms=("a q t" "a t q" "q a t" "q t a" "t a q" "t q a")
one() { # label row round
    local out="$runs/$2/$1-r$3.json" seed
    [ -e "$out" ] && return 0
    mkdir -p "$runs/$2"
    seed=$(jq -r --arg id "$2-M1-T01" '.rows[] | select(.id == $id) | .rho_seed' "$suite/SUITE.json")
    local env=(RAYON_NUM_THREADS=1)
    case "$1" in a|q) env+=(KIC_FILTER_LAYOUT=low) ;; esac
    until env "${env[@]}" "$ISO" run --wait --cpus 2 --out "$runs/isolation.jsonl" --label "fl/$2/$1-r$3" -- \
        "$BIN" price --params "$suite/params/S/$2/M1-T01.json" --json --out "$out.tmp" \
        --single-target --rho-seed "$seed" > /dev/null 2> "$runs/last.err"; do
        if ! grep -qi 'busy\|refus' "$runs/last.err"; then
            cat "$runs/last.err" >&2; echo "fl/$2/$1-r$3 failed (not a refusal); stopping" >&2; exit 1
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
