#!/usr/bin/env bash
# A4 fixed panel; run a frozen copy. Verifier/controls reuse qualified binaries.
set -euo pipefail
idx_build=$(realpath "${1:?usage: qualify-indexed.sh INDEX_BUILD AUX_BUILD PC_BUILD NEW_OUTDIR}")
aux_build=$(realpath "${2:?graph-substituted auxiliary build}")
aux_pc=$(realpath "${3:?existing unchanged verifier/compare build}")
aux_root=$(realpath -m "${4:?fresh output root}")
aux_repo=$(git rev-parse --show-toplevel)
aux_study="$aux_repo/research/projective_aux_norm_20261115"
aux_fixtures="$aux_repo/research/macaulay_provenance_20261115/fixtures"
mkdir "$aux_root"
cp "$idx_build/build-source-binary.sha256" "$aux_root/build-source-binary.sha256"
cp "$idx_build/compiler.txt" "$aux_root/compiler.txt"
sha256sum "$0" "$aux_study/qualify-indexed.sh" "$idx_build/produce" \
 "$aux_pc/"{verify,compare} "$aux_build/"{auxiliary,replay} "$aux_study/indexed.cpp" > "$aux_root/pre-call-source-binary.sha256"
git -C "$aux_repo" rev-parse HEAD > "$aux_root/source-commit.txt"
export PC_VERIFIER="$aux_pc/verify"
while read -r aux_case aux_degree; do
 aux_dir="$aux_root/$aux_case"
 mkdir "$aux_dir"
 cp "$aux_fixtures/$aux_case.ms" "$aux_dir/input.ms"
 if (ulimit -v 2097152 || exit 125; ulimit -t 20 || exit 125; exec timeout --signal=KILL 20 "$idx_build/produce" -t 1 -g 2 --degree "$aux_degree" -f "$aux_dir/input.ms" -o "$aux_dir/basis.gb") \
  > "$aux_dir/stdout.log" 2> "$aux_dir/stderr.log"; then aux_exit=0; else aux_exit=$?; fi
 printf '%s\n' "$aux_exit" > "$aux_dir/exit.txt"
 test "$aux_exit" = 0
 if [ "$aux_case" = norm-m3-t3-r0-6 ]; then aux_expected="$aux_fixtures/norm-m3-t3-r0-6-reference.gb"; else aux_expected=$aux_case; fi
 "$aux_pc/compare" "$aux_expected" "$aux_dir/input.ms" "$aux_dir/basis.gb" > "$aux_dir/compare.txt" 2>&1
done <<'CASES'
grid 3
nonreduced 3
extension 3
unit 1
x 2
xy 2
norm-m3-t3-r0-6 4
CASES
export PC_DEGREE_CAP=6
if (ulimit -v 2097152 || exit 125; exec timeout --signal=KILL 1200 "$aux_build/auxiliary" --qualify-eliminated "$idx_build/produce" "$aux_root/models" "$aux_root/summary.tsv") \
 > "$aux_root/stdout.log" 2> "$aux_root/stderr.log"; then aux_exit=0; else aux_exit=$?; fi
printf '%s\n' "$aux_exit" > "$aux_root/exit.txt"
test "$aux_exit" = 0
"$aux_build/replay" "$aux_root/models" "$aux_root/summary.tsv" > "$aux_root/openssl-replay.txt" 2>&1
cat "$aux_root/openssl-replay.txt"
