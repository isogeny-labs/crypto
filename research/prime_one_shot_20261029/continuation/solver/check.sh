#!/usr/bin/env bash
# Fixed correctness fixtures. No clocks, benchmarks, degree fits or EKS runs.
set -euo pipefail
export LC_ALL=C
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:-/tmp/unknown-norm-development-check}
mkdir -p "$out"
sha256sum "$here/PROTOCOL.md" > "$out/protocol.sha256"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror "$here/norm_solver.cpp" -o "$out/norm_solver"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror "$here/point_oracle.cpp" -o "$out/point_oracle"
g++ --version | head -1 > "$out/compiler.txt"
sha256sum "$here/norm_solver.cpp" "$here/point_oracle.cpp" "$out/norm_solver" "$out/point_oracle" > "$out/build.sha256"
"$out/point_oracle" --fixture-points > "$out/targets.txt"
test "$(wc -l < "$out/targets.txt")" -eq 18
printf 'case\tt\tc\ttarget_x\ttarget_y\tmodels\tcoefficient_pairs\tstatus\tusable_abscissas_after_target_exclusion\n' > "$out/checks.tsv"
printf 'case\tmodel\n' > "$out/candidate-models.tsv"
printf 'case\tmodel\n' > "$out/oracle-models.tsv"
case_no=0
sat=0
unsat=0
for domain in '1 1' '2 1' '3 1' '3 16' '4 1'; do
    read -r depth terminal <<< "$domain"
    while read -r target_x target_y; do
        case_no=$((case_no+1))
        args=(17 2 2 "$depth" "$terminal" "$target_x" "$target_y")
        "$out/norm_solver" "${args[@]}" > "$out/candidate.txt"
        "$out/point_oracle" "${args[@]}" > "$out/oracle.txt"
        awk -F '\t' '$1=="model" {print $2}' "$out/candidate.txt" | sort > "$out/candidate-models.tmp"
        awk -F '\t' '$1=="model" {print $2}' "$out/oracle.txt" | sort > "$out/oracle-models.tmp"
        cmp "$out/candidate-models.tmp" "$out/oracle-models.tmp"
        models=$(awk -F '\t' '$1=="summary" {print $2}' "$out/candidate.txt")
        oracle_models=$(awk -F '\t' '$1=="summary" {print $2}' "$out/oracle.txt")
        pairs=$(awk -F '\t' '$1=="summary" {print $3}' "$out/candidate.txt")
        abscissas=$(awk -F '\t' '$1=="summary" {print $3}' "$out/oracle.txt")
        test "$models" -eq "$oracle_models"
        test "$pairs" -eq 272
        test "$models" -eq "$(wc -l < "$out/candidate-models.tmp")"
        if test "$models" -gt 0; then status=SAT;sat=$((sat+1));else status=UNSAT;unsat=$((unsat+1));fi
        printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$case_no" "$depth" "$terminal" "$target_x" "$target_y" "$models" "$pairs" "$status" "$abscissas" >> "$out/checks.tsv"
        awk -v id="$case_no" 'BEGIN{OFS="\t"} {print id,$0}' "$out/candidate-models.tmp" >> "$out/candidate-models.tsv"
        awk -v id="$case_no" 'BEGIN{OFS="\t"} {print id,$0}' "$out/oracle-models.tmp" >> "$out/oracle-models.tsv"
    done < "$out/targets.txt"
done
test "$case_no" -eq 90
test "$sat" -gt 0
test "$unsat" -gt 0
cmp "$out/candidate-models.tsv" "$out/oracle-models.tsv"
negative=0
for invalid in '17 2 2 4 0 0 6' '17 2 2 4 2 0 6' '17 2 2 5 1 0 6' '17 2 2 4 1 1 1' '15 2 2 1 1 0 6' '17 0 0 4 1 0 0' '17 0 1 4 1 0 1' '5 3 2 1 1 1 1'; do
    read -r -a args <<< "$invalid"
    for program in norm_solver point_oracle; do
        if "$out/$program" "${args[@]}" > "$out/negative.stdout" 2> "$out/negative.stderr"; then
            echo "ERROR: $program accepted invalid case: $invalid" >&2;exit 1
        else
            code=$?;test "$code" -eq 2
            printf 'PASS negative %s args=%s exit=%s reason=%s\n' "$program" "$invalid" "$code" "$(cat "$out/negative.stderr")"
            negative=$((negative+1))
        fi
    done
done
for program in norm_solver point_oracle; do
    if "$out/$program" 17 2 2 4 1 0 6 witness.tsv > "$out/negative.stdout" 2> "$out/negative.stderr"; then
        echo "ERROR: $program accepted extra witness input" >&2;exit 1
    else
        code=$?;test "$code" -eq 2;negative=$((negative+1))
        printf 'PASS negative %s extra_witness_input exit=%s reason=%s\n' "$program" "$code" "$(cat "$out/negative.stderr")"
    fi
done
test "$negative" -eq 18
printf 'PASS complete_model_sets=%s SAT=%s UNSAT=%s negative_cli=%s solver_input_witnesses=0\n' "$case_no" "$sat" "$unsat" "$negative"
printf 'scope=one_curve_F17_fixed_correctness; timings=not_collected; scaling_claim=none\n'
