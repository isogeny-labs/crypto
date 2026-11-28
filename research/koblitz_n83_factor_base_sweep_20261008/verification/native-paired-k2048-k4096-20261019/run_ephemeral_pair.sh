#!/usr/bin/env bash
set -euo pipefail

study=research/koblitz_n83_factor_base_sweep_20261008
pair="$study/verification/native-paired-k2048-k4096-20261019"
bucket=ephemeral-runs-104186811093-us-east-1
input_prefix=jobs/20261019-015400-n83-k4096-phases-5438e8/input
image_ref=python:3.11-slim@sha256:e88e9763f943ec1834f992a4b51e0f24500486803e8bc534e5767af9ea65f6ce
out="$EPHEMERAL_OUT/native-paired"
mkdir -p "$out/input" "$out/arms"
source_commit=$(git rev-parse HEAD)
test "$(uname -m)" = x86_64

aws s3api get-object --bucket "$bucket" \
  --key "$input_prefix/aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a.jsonl.gz" \
  --region us-east-1 "$out/input/k2048.jsonl.gz" > "$out/input/get-k2048.json"
aws s3api get-object --bucket "$bucket" \
  --key "$input_prefix/ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad.jsonl.gz" \
  --region us-east-1 "$out/input/k4096.jsonl.gz" > "$out/input/get-k4096.json"
python3 "$pair/prepare_inputs.py" \
  --object-k2048 "$out/input/k2048.jsonl.gz" \
  --object-k4096 "$out/input/k4096.jsonl.gz" \
  --out "$out" --commit "$source_commit" --clean-source \
  > "$out/preflight.stdout"
rm "$out/input/k2048.jsonl.gz" "$out/input/k4096.jsonl.gz"

cargo build --release --locked --example koblitz_n83_factor_base_export
binary="$PWD/target/release/examples/koblitz_n83_factor_base_export"
sha256sum "$binary" > "$out/binary.sha256"
if ! command -v docker >/dev/null 2>&1; then
  sudo dnf -y install docker | tee "$out/docker-install.log"
fi
sudo systemctl start docker
sudo docker pull "$image_ref" | tee "$out/docker-pull.log"
sudo docker image inspect "$image_ref" --format '{{.Id}}' \
  | tee "$out/container-image-id.txt"

for arm in a1-k2048 b1-k4096 b2-k4096 a2-k2048; do
  k=${arm##*-k}
  arm_out="$out/arms/$arm"
  mkdir -p "$arm_out"
  start_ns=$(date +%s%N)
  set +e
  timeout --signal=TERM --kill-after=10s 620s sudo docker run \
    --name "n83-paired-$arm" --rm \
    --network none --cpus 1 --memory 4g --memory-swap 4g \
    -e N83_CHAIN_PHASE_TRACE=1 -e ICV1_FROZEN_SOURCE_DIR=/source \
    -v "$binary:/worker:ro" \
    -v "$out/source-snapshot:/source:ro" \
    -v "$out/panel-k$k:/panel:ro" \
    -v "$out/fixtures:/fixture:ro" \
    -v "$arm_out:/out" \
    "$image_ref" /worker primary-orbit-chain-cold \
    /panel /fixture "$k" public_x_hash 2026100801 5 1 \
    150000 2000000 32 1000 600 4096 /out/worker \
    > "$arm_out/worker.stdout" 2> "$arm_out/worker.stderr"
  code=$?
  set -e
  end_ns=$(date +%s%N)
  sudo docker rm -f "n83-paired-$arm" >/dev/null 2>&1 || true
  printf '%s\n' "$code" > "$arm_out/exit-code.txt"
  printf '%s\n' "$(((end_ns - start_ns) / 1000000))" > "$arm_out/container-wall-ms.txt"
  if test "$code" -ne 0 && test "$code" -ne 124; then
    exit "$code"
  fi
done

python3 - "$out" "$source_commit" <<'PY'
import json
from pathlib import Path
import sys

root, commit = Path(sys.argv[1]), sys.argv[2]
arms = []
for label in ('a1-k2048', 'b1-k4096', 'b2-k4096', 'a2-k2048'):
    folder = root / 'arms' / label
    summary = folder / 'worker/summary.json'
    cap = folder / 'worker/cap.json'
    arms.append({
        'label': label,
        'exit_code': int((folder / 'exit-code.txt').read_text().strip()),
        'container_wall_ms': int((folder / 'container-wall-ms.txt').read_text().strip()),
        'summary_status': json.loads(summary.read_text())['status'] if summary.is_file() else None,
        'cap_status': json.loads(cap.read_text())['status'] if cap.is_file() else None,
    })
result = {
    'schema': 'n83.native-paired-abba/v1',
    'source_commit': commit,
    'arms': arms,
    'selected_best_total_runtime': None,
}
(root / 'result.json').write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
PY
