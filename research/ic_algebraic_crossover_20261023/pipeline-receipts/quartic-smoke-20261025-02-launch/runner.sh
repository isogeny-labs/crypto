#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
study=research/ic_algebraic_crossover_20261023
for task_input in "$study/specs/quartic-smoke.json" "$study/inputs/guess-ell8.jsonl" "$study/run-local.sh" "$study/inputs/fresh/m6-n10-ell3-random.jsonl" "$study/inputs/fresh/m5-n15-ell3-random.jsonl" "$study/inputs/fresh/m10-n10-ell2-random.jsonl"; do test -r "$task_input"; done
run="$study/pipeline-runs/quartic-smoke-20261025-02"
meta="$study/pipeline-receipts/quartic-smoke-20261025-02-launch"
test ! -e "$run"
mkdir -p "$meta"
cp /workspace/ic-local-next-v2-20261025.sh "$meta/runner.sh"
git rev-parse HEAD > "$meta/source-commit.txt"
sha256sum target/release/ecbench "$study/specs/quartic-smoke.json" "$study/PROTOCOL.md" > "$meta/hashes.txt"
date -u +%FT%TZ > "$meta/started.txt"
set +e
target/release/ecbench run --wait --spec "$study/specs/quartic-smoke.json" --out "$run" --cpus none --allow-busy > /workspace/ic-quartic-smoke-v2-20261025.log 2>&1
rc=$?
set -e
jq -n --argjson rc "$rc" '{exit_code:$rc}' > "$meta/status.json"
set +e
target/release/ecbench verify --dir "$run" --replay-all --exit-code --out "$study/pipeline-receipts/quartic-smoke-20261025-02.json" > /workspace/ic-quartic-smoke-v2-replay-20261025.log 2>&1
rc=$?
set -e
printf 'quartic replay exit %s\n' "$rc"
date -u +%FT%TZ > "$meta/ended.txt"
export RAYON_NUM_THREADS=4 HKY_CHUNK=256
bash /workspace/ic-runner-stable-v4-20261023.sh single "$study/runs/hky-N58-random-d2-20261025-02" hky --input "$study/inputs/fresh/m6-n10-ell3-random.jsonl" --draw 2 --degree 4 --max-cols 600000
bash /workspace/ic-runner-stable-v4-20261023.sh single "$study/runs/hky-N60-random-d0-20261025-02" hky --input "$study/inputs/fresh/m5-n15-ell3-random.jsonl" --draw 0 --degree 4 --max-cols 600000
bash /workspace/ic-runner-stable-v4-20261023.sh single "$study/runs/hky-N100-random-d0-20261025-02" hky --input "$study/inputs/fresh/m10-n10-ell2-random.jsonl" --draw 0 --degree 4 --max-cols 600000 --engine sparse --max-entries 25000000
