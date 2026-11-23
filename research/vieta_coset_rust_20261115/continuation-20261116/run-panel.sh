#!/usr/bin/env bash
set -uo pipefail
mode=$1
out=$2
qualifier=$3
mkdir -p "$out"
ulimit -v 8388608
bad=0
if [ "$mode" = preflight ]; then
    cases=("word62 32 2 0 poly-preflight")
else
    cases=("word62 8192 6 2 poly-main" "word62 16384 6 2 poly-main" "p224 512 6 2 poly-main" "p224 2048 6 2 poly-main" "p224 8192 2 0 poly-supplemental")
fi
for item in "${cases[@]}"; do
    read -r curve size rounds warmups panel <<< "$item"
    started=$(date -u +%FT%TZ)
    if VIETA_ISOLATED_RUN=1 VIETA_PANEL="$panel" timeout --signal=TERM --kill-after=5s 1600 "$qualifier" bench "$curve" "$size" "$rounds" "$warmups" 30 v1 > "$out/$curve-m$size.jsonl" 2> "$out/$curve-m$size.stderr"; then
        status=0
    else
        status=$?
        bad=1
    fi
    printf '{"kind":"configuration-exit","curve":"%s","m":%s,"panel":"%s","exit":%s,"started":"%s","finished":"%s"}\n' "$curve" "$size" "$panel" "$status" "$started" "$(date -u +%FT%TZ)" >> "$out/driver.jsonl"
done
exit "$bad"
