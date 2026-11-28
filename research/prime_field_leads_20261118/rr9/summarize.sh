#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
if [ "$#" -ne 1 ]; then
    printf '%s\n' 'usage: summarize.sh output-directory' >&2
    exit 2
fi
out="$(realpath "$1")"
g++ -Wall -Wextra -Wno-return-type -std=c++20 -O3 verify.cpp -o "$out/verify"
printf 'fixture\tvariant\tsolver\trepeat\tstatus\tsetup_ms\tsolver_ms\twall_ms\tterms\tdegree\tbytes\treplay\n'
for file in "$out"/*.out; do
    name="$(basename "$file" .out)"
    IFS=. read -r fixture variant solver repeat <<< "$name"
    case "$fixture" in
        neg1) x=625037715; y=2100627251;;
        neg2) x=1923055775; y=307731458;;
        pos) x=1911579686; y=548820096;;
        *) continue;;
    esac
    prefix="$out/$name"
    status="$(cat "$prefix.status")"
    replay='not completed'
    if [ "$status" -eq 0 ]; then
        replay="$("$out/verify" "$file" "$x" "$y" "$variant")"
    fi
    setup="$(awk '$1=="setup_ms"{print $2}' "$file")"
    solve="$(awk '$1=="solver_ms"{print $2}' "$file")"
    wall="$(awk '$1=="real"{s=$2;sub(/s$/,"",s);split(s,a,"m");printf "%.0f",60000*a[1]+1000*a[2]}' "$prefix.time")"
    terms="$(awk '$1=="witness_terms"{print $2}' "$file")"
    degree="$(awk '$1=="witness_degree"{print $2}' "$file")"
    bytes="$(wc -c < "$file")"
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
        "$fixture" "$variant" "$solver" "$repeat" "$status" "$setup" "$solve" "$wall" "$terms" "$degree" "$bytes" "$replay"
done
if [ -f "$out/neg1.reduced.liftstd.1.out" ] && [ "$(cat "$out/neg1.reduced.liftstd.1.status")" -eq 0 ]; then
    if diagnostic="$("$out/verify" "$out/neg1.reduced.liftstd.1.out" 625037715 2100627251 reduced mutate 2>&1)"; then
        printf '%s\n' 'mutated reduced certificate accepted' >&2
        exit 1
    fi
    test "$diagnostic" = "identity does not equal one"
    printf 'negative control rejected: %s\n' "$diagnostic" >&2
fi
