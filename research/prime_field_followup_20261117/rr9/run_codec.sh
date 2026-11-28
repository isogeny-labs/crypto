#!/usr/bin/env bash
set -eu
cd "$(dirname "$0")"
g++ -Wall -Wextra -pedantic -Wno-return-type -std=c++20 -O3 identity_codec.cpp -o identity_codec
for fixture in 'neg1 625037715 2100627251' 'neg2 1923055775 307731458'; do
    read -r name x y <<< "$fixture"
    for presentation in 0 1; do
        source="signfree_module/liftstd.v5-$name-$presentation.sing.1.out"
        target="$name-$presentation.rri"
        ./identity_codec encode "$x" "$y" "$presentation" "$source" "$target"
        ./identity_codec verify "$x" "$y" "$presentation" "$target"
        wc -c "$source" "$target"
    done
done
cp neg1-0.rri tampered.rri
printf '\x01\x00\x00\x00' | dd of=tampered.rri bs=1 seek=433 conv=notrunc status=none
cp neg1-0.rri truncated.rri
truncate -s -1 truncated.rri
for artifact in tampered truncated; do
    if ./identity_codec verify 625037715 2100627251 0 "$artifact.rri" > "$artifact.identity.replay.log" 2>&1; then
        echo "$artifact was incorrectly accepted" >&2
        exit 1
    fi
    cat "$artifact.identity.replay.log"
done
