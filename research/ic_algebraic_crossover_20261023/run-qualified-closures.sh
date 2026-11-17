#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
inputs="$study/pipeline-receipts/qualified-queries-20261025"
meta="$study/pipeline-receipts/qualified-closures-20261025"
bin=/workspace/ic-frozen-20261025/ic_algebraic_followup-qualified-closure
test ! -e "$meta"
mkdir -p "$meta"
git rev-parse HEAD > "$meta/source-commit.txt"
sha256sum "$bin" "$study/QUALIFIED_CLOSURE_PROTOCOL.md" "$inputs/summary.json" > "$meta/hashes.txt"
date -u +%FT%TZ > "$meta/started.txt"
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
for cell in '3 22 8' '3 25 9' '4 17 6' '4 18 6'; do
  read -r m n ell <<< "$cell"
  for family in prefix geometric geomtraceu random; do
    unit="m$m-n$n-ell$ell-$family"
    input="$inputs/$unit/output.jsonl"
    test "$(jq -r .exit_code "$inputs/$unit/reference/status.json")" = 0
    test "$(jq -r .exit_code "$inputs/$unit/replay/status.json")" = 0
    while read -r draw selection; do
      for degree in 2 3; do
        folder="$meta/$unit-d$draw-D$degree"
        mkdir -p "$folder"
        command=("$bin" guess --input "$input" --draw "$draw" --vars none --degree "$degree" --max-cols 40000 --first-branch 0 --end-branch 1)
        if [ "$selection" = positive ]; then command+=(--sat-control true); fi
        printf '%q ' "${command[@]}" > "$folder/command.txt"
        printf '\n' >> "$folder/command.txt"
        sha256sum "$input" > "$folder/input-hash.txt"
        date -u +%FT%TZ > "$folder/started.txt"
        set +e
        (ulimit -v 2097152; /workspace/.tools/usr/bin/time -f 'wall_seconds=%e\nmax_rss_kib=%M\nexit=%x' -o "$folder/resources.txt" timeout -k 30 180 "${command[@]}") > "$folder/output.jsonl" 2> "$folder/stderr.txt"
        rc=$?
        set -e
        printf '{"exit_code":%s,"address_space_bytes":2147483648,"timeout_seconds":180,"selection":"%s"}\n' "$rc" "$selection" > "$folder/status.json"
        date -u +%FT%TZ > "$folder/ended.txt"
        printf '%s exit=%s\n' "$folder" "$rc"
      done
    done < <(jq -r 'select(.followup_selection != null) | [.draw,.followup_selection] | @tsv' "$input")
  done
done
date -u +%FT%TZ > "$meta/ended.txt"
printf '%s\n' 'All selected degree-two/three attempts terminated. Native raw audit, independent closure replay, graphs/PDF and inspection required; all degree-four/guessing obligations remain open.' > "$meta/REVIEW_REQUIRED.txt"
