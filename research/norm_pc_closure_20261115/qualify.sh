#!/usr/bin/env bash
# Thin orchestration of the fixed native software-correctness panel.
set -euo pipefail
pc_build=$(realpath "${1:?usage: qualify.sh BUILD NEW_OUTDIR}")
pc_root=$(realpath -m "${2:?new output directory required}")
pc_study=$(cd "$(dirname "$0")" && pwd)
pc_fixtures="$pc_study/../macaulay_provenance_20261115/fixtures"
mkdir "$pc_root"
export PC_VERIFIER="$pc_build/verify"
sha256sum "$pc_build/"{produce,verify,compare} "$pc_study/"*.cpp "$pc_study/"*.hpp > "$pc_root/source-binary.sha256"
git -C "$pc_study" rev-parse HEAD > "$pc_root/source-commit.txt"
printf 'case\tdegree\texit\tstatus\n' > "$pc_root/summary.tsv"
pc_bounded(){
 local pc_dir=$1
 shift
 local pc_exit
 if (ulimit -v 2097152 || exit 125; ulimit -t 20 || exit 125; exec timeout --signal=KILL 20 "$@") \
   > "$pc_dir/stdout.log" 2> "$pc_dir/stderr.log"; then pc_exit=0; else pc_exit=$?; fi
 printf '%s\n' "$pc_exit" > "$pc_dir/exit.txt"
}
while read -r pc_case pc_degree; do
 pc_dir="$pc_root/$pc_case"
 mkdir "$pc_dir"
 cp "$pc_fixtures/$pc_case.ms" "$pc_dir/input.ms"
 pc_bounded "$pc_dir" "$pc_build/produce" -t 1 -g 2 --degree "$pc_degree" -f "$pc_dir/input.ms" -o "$pc_dir/basis.gb"
 pc_exit=$(cat "$pc_dir/exit.txt")
 printf '%s\t%s\t%s\t' "$pc_case" "$pc_degree" "$pc_exit" >> "$pc_root/summary.tsv"
 if [ "$pc_exit" -ne 0 ]; then printf 'failed_positive\n' >> "$pc_root/summary.tsv"; exit 2; fi
 "$pc_build/compare" "$pc_case" "$pc_dir/input.ms" "$pc_dir/basis.gb" > "$pc_dir/compare.log" 2>&1
 printf 'match\n' >> "$pc_root/summary.tsv"
done <<'CASES'
grid 3
nonreduced 3
extension 3
unit 1
x 2
xy 2
CASES
mkdir "$pc_root/negative"
for pc_case in wrong-coefficient forward-reference incomplete-gb missing-original; do
 pc_dir="$pc_root/negative/$pc_case"
 mkdir "$pc_dir"
 case "$pc_case" in
 wrong-coefficient)
  cp "$pc_root/x/"{input.ms,basis.gb} "$pc_dir/"
  printf 'PC-DAG-CERT-1 5 1 2 2 1\nI 0\nL 1 0 2\nF 1\n' > "$pc_dir/basis.gb.cert"
  pc_expected='final polynomial identity mismatch';;
 forward-reference)
  cp "$pc_root/x/"{input.ms,basis.gb} "$pc_dir/"
  printf 'PC-DAG-CERT-1 5 1 2 1 1\nM 0 0\nF 0\n' > "$pc_dir/basis.gb.cert"
  pc_expected='invalid backward reference or variable';;
 incomplete-gb)
  cp "$pc_fixtures/non-gb.ms" "$pc_dir/input.ms"
  printf '#field characteristic: 5\n#variable order: x,y\n#monomial order: graded reverse lexicographical\n[x^2-y,x*y-1]:\n' > "$pc_dir/basis.gb"
  printf 'PC-DAG-CERT-1 5 2 3 2 2\nI 0\nI 1\nF 0 1\n' > "$pc_dir/basis.gb.cert"
  pc_expected='Buchberger completion failed';;
 missing-original)
  cp "$pc_fixtures/xy.ms" "$pc_dir/input.ms"
  printf '#field characteristic: 5\n#variable order: x,y\n#monomial order: graded reverse lexicographical\n[x]:\n' > "$pc_dir/basis.gb"
  printf 'PC-DAG-CERT-1 5 2 2 1 1\nI 0\nF 0\n' > "$pc_dir/basis.gb.cert"
  pc_expected='original generator does not reduce to zero';;
 esac
 pc_bounded "$pc_dir" "$pc_build/verify" "$pc_dir/input.ms" "$pc_dir/basis.gb" "$pc_dir/basis.gb.cert"
 test "$(cat "$pc_dir/exit.txt")" = 2
 rg -q -F "$pc_expected" "$pc_dir/stderr.log"
 printf '%s\t-\t2\trejected\n' "$pc_case" >> "$pc_root/summary.tsv"
done
for pc_degree in 4 5 6; do
 pc_case="norm-d$pc_degree"
 pc_dir="$pc_root/$pc_case"
 mkdir "$pc_dir"
 cp "$pc_fixtures/norm-m3-t3-r0-6.ms" "$pc_dir/input.ms"
 pc_bounded "$pc_dir" "$pc_build/produce" -t 1 -g 2 --degree "$pc_degree" -f "$pc_dir/input.ms" -o "$pc_dir/basis.gb"
 pc_exit=$(cat "$pc_dir/exit.txt")
 if [ "$pc_exit" = 0 ]; then
  "$pc_build/compare" "$pc_fixtures/norm-m3-t3-r0-6-reference.gb" "$pc_dir/input.ms" "$pc_dir/basis.gb" > "$pc_dir/compare.log" 2>&1
  pc_status=match
 else pc_status=unknown; fi
 printf '%s\t%s\t%s\t%s\n' "$pc_case" "$pc_degree" "$pc_exit" "$pc_status" >> "$pc_root/summary.tsv"
done
cat "$pc_root/summary.tsv"
