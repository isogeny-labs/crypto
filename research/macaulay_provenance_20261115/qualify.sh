#!/usr/bin/env bash
# Orchestration of the prospectively fixed native software checks.
set -euo pipefail
mca_build=$(realpath "${1:?usage: qualify.sh BUILD NEW_OUTDIR}")
mca_root=$(realpath -m "${2:?new output directory required}")
mca_study=$(cd "$(dirname "$0")" && pwd)
mkdir "$mca_root"
export MACAULAY_VERIFIER="$mca_build/verify"
sha256sum "$mca_build/produce" "$mca_build/verify" "$mca_build/compare" "$mca_study"/*.cpp "$mca_study"/*.hpp > "$mca_root/source-binary.sha256"
git -C "$mca_study" rev-parse HEAD > "$mca_root/source-commit.txt"
printf 'case\tdegree\texit\tstatus\n' > "$mca_root/summary.tsv"
mca_bounded(){
    local mca_dir=$1
    shift
    local mca_exit
    if (
        ulimit -v 2097152 || exit 125
        ulimit -t 20 || exit 125
        exec timeout --signal=KILL 20 "$@"
    ) > "$mca_dir/stdout.log" 2> "$mca_dir/stderr.log"; then mca_exit=0; else mca_exit=$?; fi
    printf '%s\n' "$mca_exit" > "$mca_dir/exit.txt"
}
while read -r mca_case mca_degree; do
    mca_dir="$mca_root/$mca_case"
    mkdir "$mca_dir"
    cp "$mca_study/fixtures/$mca_case.ms" "$mca_dir/input.ms"
    mca_bounded "$mca_dir" "$mca_build/produce" -t 1 -g 2 --degree "$mca_degree" -f "$mca_dir/input.ms" -o "$mca_dir/basis.gb"
    mca_exit=$(cat "$mca_dir/exit.txt")
    printf '%s\t%s\t%s\t' "$mca_case" "$mca_degree" "$mca_exit" >> "$mca_root/summary.tsv"
    if [ "$mca_exit" -ne 0 ]; then printf 'failed_positive\n' >> "$mca_root/summary.tsv"; exit 2; fi
    "$mca_build/compare" "$mca_case" "$mca_dir/input.ms" "$mca_dir/basis.gb" > "$mca_dir/compare.log" 2>&1
    printf 'match\n' >> "$mca_root/summary.tsv"
done <<'CASES'
grid 3
nonreduced 3
extension 3
unit 1
x 2
xy 2
CASES
mkdir "$mca_root/negative"
for mca_case in wrong-coefficient forged-unit missing-original incomplete-gb; do
    mca_dir="$mca_root/negative/$mca_case"
    mkdir "$mca_dir"
    case "$mca_case" in
    wrong-coefficient)
        cp "$mca_root/x/input.ms" "$mca_dir/input.ms"
        cp "$mca_root/x/basis.gb" "$mca_dir/basis.gb"
        awk 'NF==3 {$NF=2} {print}' "$mca_root/x/basis.gb.cert" > "$mca_dir/basis.gb.cert"
        mca_expected='input-row identity mismatch';;
    forged-unit)
        cp "$mca_root/x/input.ms" "$mca_dir/input.ms"
        cp "$mca_root/x/basis.gb.cert" "$mca_dir/basis.gb.cert"
        printf '#field characteristic: 5\n#variable order: x\n#monomial order: graded reverse lexicographical\n[1]:\n' > "$mca_dir/basis.gb"
        mca_expected='input-row identity mismatch';;
    missing-original)
        cp "$mca_study/fixtures/xy.ms" "$mca_dir/input.ms"
        printf '#field characteristic: 5\n#variable order: x,y\n#monomial order: graded reverse lexicographical\n[x]:\n' > "$mca_dir/basis.gb"
        printf 'MCAULAY-CERT-1 5 2 2 1 1\n0 0 0\n1 0 1\n' > "$mca_dir/basis.gb.cert"
        mca_expected='original generator does not reduce to zero';;
    incomplete-gb)
        cp "$mca_study/fixtures/non-gb.ms" "$mca_dir/input.ms"
        printf '#field characteristic: 5\n#variable order: x,y\n#monomial order: graded reverse lexicographical\n[x^2-y,x*y-1]:\n' > "$mca_dir/basis.gb"
        printf 'MCAULAY-CERT-1 5 2 3 2 2\n0 0 0\n1 0 0\n1 0 1\n1 1 1\n' > "$mca_dir/basis.gb.cert"
        mca_expected='Buchberger completion failed';;
    esac
    mca_bounded "$mca_dir" "$mca_build/verify" "$mca_dir/input.ms" "$mca_dir/basis.gb" "$mca_dir/basis.gb.cert"
    test "$(cat "$mca_dir/exit.txt")" = 2
    rg -q -F "$mca_expected" "$mca_dir/stderr.log"
done
for mca_degree in 4 5 6; do
    mca_case="norm-d$mca_degree"
    mca_dir="$mca_root/$mca_case"
    mkdir "$mca_dir"
    cp "$mca_study/fixtures/norm-m3-t3-r0-6.ms" "$mca_dir/input.ms"
    mca_bounded "$mca_dir" "$mca_build/produce" -t 1 -g 2 --degree "$mca_degree" -f "$mca_dir/input.ms" -o "$mca_dir/basis.gb"
    mca_exit=$(cat "$mca_dir/exit.txt")
    if [ "$mca_exit" = 0 ]; then
        "$mca_build/compare" "$mca_study/fixtures/norm-m3-t3-r0-6-reference.gb" "$mca_dir/input.ms" "$mca_dir/basis.gb" > "$mca_dir/compare.log" 2>&1
        mca_status=match
    else mca_status=unknown; fi
    printf '%s\t%s\t%s\t%s\n' "$mca_case" "$mca_degree" "$mca_exit" "$mca_status" >> "$mca_root/summary.tsv"
done
cat "$mca_root/summary.tsv"
