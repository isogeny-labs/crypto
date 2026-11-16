#!/bin/sh
# Cross-check derived storage with shell integer arithmetic and jq.
set -eu

study=research/rootfind_paged_pair_index_20261116
census=research/rootfind_pair_collision_census_20261116/reports/run-20261116T0711Z

for n in 13 19 23; do
    raw="$census/k$n.json"
    a=$(jq -er '.base_abscissae' "$raw")
    entries=$(jq -er '.unique_keys' "$raw")
    attempts=$(jq -er '.insert_attempts' "$raw")
    directory=$(( (1 << n) / 16 + 1 ))
    capacity=$(( a * a + 1 ))
    if [ "$capacity" -gt "$directory" ]; then
        capacity=$directory
    fi
    requested=$(( 4 * directory + 128 * capacity ))
    flat=$(( 8 * ((1 << n) + 1) ))
    stored_requested=$(jq -er --argjson n "$n" '.rows[] | select(.field_degree == $n) | .paged_requested_backing_bytes' "$study/storage.json")
    stored_flat=$(jq -er --argjson n "$n" '.rows[] | select(.field_degree == $n) | .flat_requested_backing_bytes' "$study/storage.json")
    stored_entries=$(jq -er --argjson n "$n" '.rows[] | select(.field_degree == $n) | .first_witness_keys' "$study/storage.json")
    test "$entries" -le "$attempts"
    test "$requested" -eq "$stored_requested"
    test "$flat" -eq "$stored_flat"
    test "$entries" -eq "$stored_entries"
    printf 'K%s: a=%s entries=%s directory=%s pages=%s paged=%s flat=%s\n' \
        "$n" "$a" "$entries" "$directory" "$capacity" "$requested" "$flat"
done
