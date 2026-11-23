#!/usr/bin/env bash
set -eu
cd "$(dirname "$0")"

g++ -Wall -Wextra -pedantic -std=c++20 -O3 branch_certificate.cpp -o branch_certificate
g++ -Wall -Wextra -pedantic -Wno-return-type -std=c++20 -O3 verify_branch.cpp -o verify_branch

for fixture in 'neg1 625037715 2100627251' 'neg2 1923055775 307731458' 'pos 1911579686 548820096'; do
    read -r name x y <<< "$fixture"
    ./branch_certificate "$x" "$y" 301 "$name.rrb" | tee "$name.generate.log"
    ./verify_branch "$name.rrb" "$x" "$y" 301 | tee "$name.replay.log"
done

cp neg1.rrb corrupted-coefficient.rrb
printf '\x00\x00\x00\x00' | dd of=corrupted-coefficient.rrb bs=1 seek=11 conv=notrunc status=none
cp neg1.rrb missing-leaf.rrb
truncate -s -34 missing-leaf.rrb
printf '\xff\x00\x00\x00' | dd of=missing-leaf.rrb bs=1 seek=5 conv=notrunc status=none
for artifact in corrupted-coefficient missing-leaf; do
    if ./verify_branch "$artifact.rrb" 625037715 2100627251 1 > "$artifact.replay.log" 2>&1; then
        echo "$artifact was incorrectly accepted" >&2
        exit 1
    fi
    cat "$artifact.replay.log"
done
