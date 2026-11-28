#!/usr/bin/env bash
# Re-run every check of the independent-validation lane.
# Usage: run_all.sh REPO_ROOT REV SCRATCH_DIR [--repro]
#   REV          frozen commit validated (6016f362e8e1c14fc8eddb080f642c72d953e694)
#   SCRATCH_DIR  empty-or-new directory for binaries and reproduction outputs
#   --repro      also rebuild the descent-finder mini-build and the producer
#                bootstrap (CARGO_TARGET_DIR=/home/user/cargo-target-round2/independent-validation),
#                rerun traced_descent_check on both streams and descent_breakdown,
#                and compare with the committed outputs byte for byte.
# Writes results/*.txt next to this script. Read-only on producer files.
set -euo pipefail
R=$(cd "${1:?REPO_ROOT}" && pwd); REV=${2:?REV}; S=${3:?SCRATCH_DIR}; REPRO=${4:-}
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=$here/results
mkdir -p "$S/bin" "$out"
L=$R/research/extension_ic_graph_20261115
N=$L/nagao-e2e; F=$L/runpod-deferred-pair-20261117-50; D=$L/leads-20261118/descent-finder
cd "$R"
echo "validated_rev=$REV head=$(git rev-parse HEAD) rustc=$(rustc --version)" > "$out/environment.txt"
uname -srm >> "$out/environment.txt"
rustc -O --edition=2021 "$here/validate.rs" -o "$S/bin/validate"
V=$S/bin/validate

# every recorded scalar, from every retained record that carries one
scalars=$( { jq -r '.runs[].scalar' $N/results/summary.json
             jq -r '.actual_scalar' $F/*.summary.jsonl
             jq -r '.outcomes[].actual_scalar' $F/run-study.json
             jq -r '.actual_scalar' $F/qualified-original.json
             jq -r '.actual_scalar, (.recovered_scalar // empty), (.signature.scalar // empty)' $F/*.raw.json
             jq -r '.expected_scalar' $F/fixture.json; } | sort -u )
echo "distinct recorded scalars: $(echo "$scalars" | wc -l) ($(echo $scalars))" > "$out/ec.txt"
for pub in $N/public.json $F/public.json; do
  echo "== $pub" >> "$out/ec.txt"
  "$V" ec "$pub" $scalars >> "$out/ec.txt"
done
"$V" nagao $N/results/summary.json $N/results/flow.json > "$out/nagao-stats.txt"
"$V" run50 $F > "$out/run50.txt"
"$V" descent $F $D > "$out/descent.txt"
for s in seed389-n3415 seed401-n1506; do
  zcat $D/runs/traced-$s.rows.jsonl.gz | "$V" rows $D/runs/traced-$s.summary.json > "$out/rows-$s.txt"
done
bash "$here/check_hashes.sh" "$R" "$REV" > "$out/hashes.tsv"

if [[ $REPRO == --repro ]]; then
  T=/home/user/cargo-target-round2/independent-validation
  A=$S/asm; rm -rf "$A"; mkdir -p "$A/scripts" "$A/none"
  sed -e "s#/home/user/crypto/.claude/worktrees/wf_8b7b71c2-376-2#$R#g" -e "s#/dev/shm/frozen#$A/frozen#g" $D/mini-build/fetch_frozen.sh > "$A/scripts/fetch_frozen.sh"
  sed -e "s#/home/user/crypto/.claude/worktrees/wf_8b7b71c2-376-2#$R#g" -e "s#/dev/shm/frozen#$A/frozen#g" -e "s#/dev/shm/mini2#$A/mini2#g" -e "s#/dev/shm/mini/Cargo.toml#$A/none/Cargo.toml#g" $D/mini-build/assemble.py > "$A/scripts/assemble.py"
  bash "$A/scripts/fetch_frozen.sh" > "$A/fetch.log" 2>&1
  (cd "$A" && python3 -I "$A/scripts/assemble.py" > /dev/null)
  {
    echo "# reassembled tree vs BUILD_MANIFEST 'assembled build tree'"
    (cd "$A/mini2" && sha256sum $(find src -type f | sort)) | sort -k2 > "$A/assembled.sha256"
    awk '/## assembled/{f=1;next} /^## /{f=0} f && NF==2 {print}' $D/BUILD_MANIFEST.txt | sort -k2 > "$A/manifest.sha256"
    join -1 2 -2 2 "$A/assembled.sha256" "$A/manifest.sha256" | awk '{print ($2==$3?"MATCH":"MISMATCH"), $1}'
    echo "Cargo.toml with bin path /dev/shm/df-lane/traced_descent_check.rs: $(sed 's#TRACED_CHECK_PATH#/dev/shm/df-lane/traced_descent_check.rs#' $D/mini-build/Cargo.toml.in | sha256sum | cut -c1-64)"
    sed "s#TRACED_CHECK_PATH#$D/traced_descent_check.rs#" $D/mini-build/Cargo.toml.in > "$A/mini2/Cargo.toml"
    (cd "$A/mini2" && CARGO_TARGET_DIR=$T cargo build --offline --release -q)
    echo "offline-resolved Cargo.lock: $(sha256sum "$A/mini2/Cargo.lock" | cut -c1-64)"
    echo "manifest Cargo.lock/Cargo.toml: $(grep -E 'Cargo\.(lock|toml)$' $D/BUILD_MANIFEST.txt | tr '\n' ' ')"
    mkdir -p "$S/rerun"; rm -f "$S/rerun/"*
    "$T/release/traced_descent_check" $F/public.json 389 3415 "$S/rerun/rows389.jsonl" > "$S/rerun/out389.json"
    "$T/release/traced_descent_check" $N/public.json 401 1506 "$S/rerun/rows401.jsonl" > "$S/rerun/out401.json"
    echo "# rerun outputs vs committed (RAW.sha256)"
    for f in out389.json out401.json rows389.jsonl rows401.jsonl; do echo "$(sha256sum "$S/rerun/$f" | cut -c1-64) $f"; done
    cat $D/runs/RAW.sha256 | grep -E 'run3415|run401'
    rustc -O --edition=2021 $D/descent_breakdown.rs -o "$S/bin/descent_breakdown"
    "$S/bin/descent_breakdown" research/extension_ic_graph_20261115/runpod-deferred-pair-20261117-50 > "$S/rerun/breakdown.json"
    cmp "$S/rerun/breakdown.json" $D/breakdown.json && echo "MATCH breakdown.json byte-equal on rerun"
    (cd "$here/bootstrap-replay" && CARGO_TARGET_DIR=$T cargo build --offline --release -q)
    echo "# producer bootstrap replay (rand 0.8.8 StdRng seed 0xE2E_1009) vs recorded"
    for c in 0 1 2; do
      args=$(jq -r ".comparisons[$c].paired_ratios[]" $N/results/summary.json | tr '\n' ' ')
      echo "$(jq -r ".comparisons[$c] | \"\(.numerator)/\(.denominator) recorded \(.bootstrap95[0]) \(.bootstrap95[1])\"" $N/results/summary.json) replay $("$T/release/nagao_bootstrap_replay" $args)"
    done
  } > "$out/reproduction.txt" 2>&1
fi
grep -h '^SUMMARY' "$out"/*.txt
cut -f1 "$out/hashes.tsv" | sort | uniq -c
