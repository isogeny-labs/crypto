#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
if [ "$#" -ne 2 ]; then
    printf '%s\n' 'usage: run.sh /absolute/path/to/Singular output-directory' >&2
    exit 2
fi
singular="$(realpath "$1")"
out="$2"
mkdir -p "$out"
nreps="${RR9_REPS:-3}"
for fixture in 'neg1 625037715 2100627251' 'neg2 1923055775 307731458' 'pos 1911579686 548820096'; do
    read -r name x y <<< "$fixture"
    for variant in full reduced; do
        for solver in std liftstd; do
            prefix="$out/$name.$variant.$solver"
            {
                printf '%s\n' 'ring full=2147483647,(a0,a1,a2,a3,v0,v1,v2),dp;' 'option(noredefine);' 'system("--ticks-per-sec",1000);'
                printf 'number rx=%s; number ry=%s; int setup_start=rtimer;\n' "$x" "$y"
                cat equations.sing
                if [ "$variant" = reduced ]; then
                    cat reduce.sing
                fi
                printf '%s\n' 'print("setup_ms "+string(rtimer-setup_start));' 'int solve_start=rtimer;'
                if [ "$solver" = liftstd ]; then
                    printf '%s\n' 'int certificate_mode=1; matrix T; ideal G=liftstd(I,T);'
                else
                    printf '%s\n' 'int certificate_mode=0; ideal G=std(I);'
                fi
                cat finish.sing
                printf '%s\n' 'quit;'
            } > "$prefix.sing"
            for ((repeat=1;repeat<=nreps;repeat++)); do
                status=0
                (ulimit -v 786432; time timeout 30 "$singular" -q "$prefix.sing") > "$prefix.$repeat.out" 2> "$prefix.$repeat.time" || status=$?
                if grep -Eq '^ *\?' "$prefix.$repeat.out"; then
                    status=1
                fi
                printf '%s\n' "$status" > "$prefix.$repeat.status"
            done
        done
    done
done
