#!/usr/bin/env bash
# Fixed-curve direct IC development checks; no timing or scaling claims.
set -euo pipefail
export LC_ALL=C
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:-/tmp/direct-ic-development-check}
mkdir -p "$out"
sha256sum "$here/PROTOCOL.md" > "$out/direct-protocol.sha256"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror "$here/direct_ic.cpp" -o "$out/direct_ic"
g++ --version | head -1 > "$out/direct-compiler.txt"
sha256sum "$here/direct_ic.cpp" "$here/norm_solver.cpp" "$here/point_oracle.cpp" "$out/direct_ic" > "$out/direct-build.sha256"
"$out/direct_ic" --selftest "$out" > "$out/direct-checks.tsv" 2> "$out/direct-selftest.log"
test "$(wc -l < "$out/direct-checks.tsv")" -eq 19
test "$(wc -l < "$out/direct-relations.tsv")" -ge 73
test "$(wc -l < "$out/direct-kernels.tsv")" -ge 19
printf 'fixture_expected\tQx\tQy\tseed\trecovered\tattempts\tpdp_calls\tcoefficient_pairs\taccepted_rows\tbatches\tzero_denominator_batches\tfinal_C_rank\tverified\n' > "$out/direct-cli.tsv"
awk -F '\t' 'NR>1 {print $1,$2,$3}' "$out/direct-checks.tsv" > "$out/direct-cli-fixtures.txt"
count=0
while read -r expected qx qy; do
    # Only Q coordinates and a fixed development seed cross the executable API.
    "$out/direct_ic" --solve "$qx" "$qy" 1234 > "$out/direct-cli-case.tsv"
    recovered=$(awk -F '\t' '$1=="result" {print $2}' "$out/direct-cli-case.tsv")
    test "$recovered" -eq "$expected"
    awk -F '\t' -v e="$expected" -v x="$qx" -v y="$qy" 'BEGIN {OFS="\t"} $1=="result" {print e,x,y,1234,$2,$3,$4,$5,$6,$7,$8,$9,$10}' "$out/direct-cli-case.tsv" >> "$out/direct-cli.tsv"
    count=$((count+1))
done < "$out/direct-cli-fixtures.txt"
test "$count" -eq 18
negative=0
for args in '1 1 1234' '17 2 1234' '0 6 witness' '0 6'; do
    read -r -a values <<< "$args"
    if "$out/direct_ic" --solve "${values[@]}" > "$out/direct-negative.stdout" 2> "$out/direct-negative.stderr"; then
        echo "ERROR: invalid DLP input accepted: $args" >&2;exit 1
    else
        code=$?;test "$code" -eq 2
        printf 'PASS direct negative args=%s exit=%s reason=%s\n' "$args" "$code" "$(cat "$out/direct-negative.stderr")"
        negative=$((negative+1))
    fi
done
test "$negative" -eq 4
cat "$out/direct-selftest.log"
printf 'PASS separate_CLI_targets=%s seed=1234 expected_scalar_arguments=0 negative_CLI=%s\n' "$count" "$negative"
printf 'scope=fixed_F17_direct_IC_correctness; cold_setup_and_all_misses_retained; timings=not_collected; scaling_claim=none\n'
