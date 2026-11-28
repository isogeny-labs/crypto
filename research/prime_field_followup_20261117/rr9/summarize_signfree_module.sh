#!/usr/bin/env bash
set -eu
cd "$(dirname "$0")"
printf 'method\tfixture\tpresentation\trep\tbasis_ms\tlift_ms\twall_ms\tmultiplier_terms\tbytes\tstatus\n'
for out in signfree_module/*.out; do
    prefix="${out%.out}"
    name="$(basename "$prefix")"
    IFS=. read -r method target suffix rep <<< "$name"
    target="${target#v5-}"
    fixture="${target%-*}"
    presentation="${target##*-}"
    basis="$(awk '/^basis_ms /{print $2}' "$out")"
    lift="$(awk '/^lift_ms /{print $2}' "$out")"
    wall="$(awk '$1=="real"{s=$2;sub(/s$/,"",s);split(s,a,"m");printf "%.0f",60000*a[1]+1000*a[2]}' "$prefix.time")"
    terms="$(awk '/^unit replay: /{print $3}' "$prefix.replay")"
    size="$(wc -c < "$out")"
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$method" "$fixture" "$presentation" "$rep" "$basis" "$lift" "$wall" "$terms" "$size" "$(cat "$prefix.status")"
done
