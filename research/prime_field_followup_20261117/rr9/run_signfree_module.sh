#!/usr/bin/env bash
set -eu
cd "$(dirname "$0")"

singular_bin="${SINGULAR_BIN:-Singular}"
mkdir -p signfree_module

for method in liftstd std slimgb; do
    for source in fixtures/v5-*.sing; do
        name="$(basename "$source" .sing)"
        target="signfree_module/$method.$name.sing"
        identify="ideal G=liftstd(I,T);"
        lift=""
        if [ "$method" != liftstd ]; then
            identify="ideal G=$method(I);"
            lift="ideal unitIdeal=1; T=lift(I,unitIdeal); G[1]=1;"
        fi
        awk -v identify="$identify" -v lift="$lift" '
            /ideal G=liftstd\(I,T\);/ {
                print "system(\"--ticks-per-sec\",1000); int basisStart=rtimer;"
                print identify
                print "print(\"basis_ms \"+string(rtimer-basisStart));"
                next
            }
            /poly replay=0;/ {
                print "int liftStart=rtimer;"
                print lift
                print "print(\"lift_ms \"+string(rtimer-liftStart));"
            }
            {print}
        ' "$source" > "$target"
        for rep in 1 2 3; do
            prefix="$target.$rep"
            status=0
            ( ulimit -v 786432; time timeout 30 "$singular_bin" -q "$target" ) > "$prefix.out" 2> "$prefix.time" || status=$?
            printf '%s\n' "$status" > "$prefix.status"
        done
    done
done

g++ -Wall -Wextra -pedantic -std=c++20 -O3 prior_verify.cpp -o prior_verify
for artifact in signfree_module/*.out; do
    name="$(basename "$artifact")"
    presentation="$(echo "$name" | cut -d. -f2 | rev | cut -d- -f1)"
    case "$name" in
        *.v5-neg1-*) x=625037715; y=2100627251;;
        *.v5-neg2-*) x=1923055775; y=307731458;;
        *.v5-pos-*) x=1911579686; y=548820096;;
    esac
    mode_status=0
    if [ "$(cat "${artifact%.out}.status")" -eq 0 ] && ! grep -q '^[[:space:]]*?' "$artifact"; then
        (ulimit -c 0; ./prior_verify "$artifact" "$x" "$y" "$presentation") > "${artifact%.out}.replay" 2>&1 || mode_status=$?
    else
        mode_status=1
    fi
    printf '%s\t%s\n' "$artifact" "$mode_status"
done | tee signfree_module.replays.tsv
awk -F '\t' '$2 != 0 {failed=1} END {exit failed || NR != 54}' signfree_module.replays.tsv
