#!/usr/bin/env bash
set -u
cd /workspace/crypto
# Finish the existing counted replay before beginning the next measurement.
tail --pid=1857 -f /dev/null
study=research/ic_algebraic_crossover_20261023
run="$study/pipeline-runs/quartic-smoke-20261025-01"
meta="$study/pipeline-receipts/quartic-smoke-20261025-01-launch"
test ! -e "$run" || exit 2
mkdir -p "$meta"
cp /workspace/ic-local-next-20261025.sh "$meta/runner.sh"
git rev-parse HEAD > "$meta/source-commit.txt"
sha256sum target/release/ecbench "$study/specs/quartic-smoke.json" "$study/PROTOCOL.md" > "$meta/hashes.txt"
date -u +%FT%TZ > "$meta/started.txt"
target/release/ecbench run --wait --spec "$study/specs/quartic-smoke.json" --out "$run" --cpus none --allow-busy > /workspace/ic-quartic-smoke-20261025.log 2>&1
rc=$?
jq -n --argjson rc "$rc" '{exit_code:$rc}' > "$meta/status.json"
target/release/ecbench verify --dir "$run" --replay-all --exit-code --out "$study/pipeline-receipts/quartic-smoke-20261025-01.json" > /workspace/ic-quartic-smoke-replay-20261025.log 2>&1
printf 'quartic replay exit %s\n' "$?"
date -u +%FT%TZ > "$meta/ended.txt"
export RAYON_NUM_THREADS=4 HKY_CHUNK=256
bash /workspace/ic-runner-stable-v4-20261023.sh single "$study/runs/hky-N58-random-d2-20261025-01" hky --input "$study/inputs/fresh/m6-n10-ell3-random.jsonl" --draw 2 --degree 4 --max-cols 600000
bash /workspace/ic-runner-stable-v4-20261023.sh single "$study/runs/hky-N60-random-d0-20261025-01" hky --input "$study/inputs/fresh/m5-n15-ell3-random.jsonl" --draw 0 --degree 4 --max-cols 600000
bash /workspace/ic-runner-stable-v4-20261023.sh single "$study/runs/hky-N100-random-d0-20261025-01" hky --input "$study/inputs/fresh/m10-n10-ell2-random.jsonl" --draw 0 --degree 4 --max-cols 600000 --engine sparse --max-entries 25000000
