#!/usr/bin/env bash
set -euo pipefail

study=research/koblitz_n83_factor_base_sweep_20261008
evidence="$study/verification/k4096-orbit-model-20261018/evidence"
audit="$study/verification/k4096-native-phases-20261018/audit_phase_trace.py"
out="$EPHEMERAL_OUT/k4096-native-phases"
panel="$out/panel"
fixture="$out/fixtures"
snapshot="$out/source-snapshot"
mkdir -p "$panel/objects" "$fixture" "$snapshot" "$out/run"

test -z "$(git status --porcelain)"
source_commit=$(git rev-parse HEAD)
test "$(uname -m)" = x86_64

cp "$evidence/panel/manifest.json" "$evidence/panel/replay.json" \
   "$evidence/panel/upload-receipt.json" \
   "$evidence/panel/probe-corpus.json" "$panel/"
cp "$evidence/fixtures/probe-corpus.json" \
   "$evidence/fixtures/probe-validation.json" "$fixture/"
object="$panel/objects/a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779.jsonl.gz"
aws s3api get-object --bucket ephemeral-runs-104186811093-us-east-1 \
  --key jobs/20261019-015400-n83-k4096-phases-5438e8/input/ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad.jsonl.gz \
  --region us-east-1 "$object" > "$out/s3-get.json"

python3 - "$panel" "$fixture" "$source_commit" "$snapshot" <<'PY'
import hashlib
import json
from pathlib import Path
import shutil
import sys

panel, fixture, commit, snapshot = map(Path, sys.argv[1:])
want = json.loads(Path('research/koblitz_n83_factor_base_sweep_20261008/verification/cloud-matched-k2048-k4096-20261018/input/fresh-inputs.json').read_text())
base = next(row for row in want['bases'] if row['columns'] == 4096)
def check(path, digest):
    observed = hashlib.sha256(path.read_bytes()).hexdigest()
    if observed != digest:
        raise SystemExit(f'input SHA-256 mismatch: {path}')
    return observed
check(next((panel / 'objects').iterdir()), base['sha256'])
for name, digest in base['panel_sha256'].items():
    check(panel / name, digest)
for name, digest in want['fixture_sha256'].items():
    check(fixture / name, digest)
paths = [
    'examples/koblitz_n83_factor_base_export.rs',
    'research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs',
    'research/koblitz_n83_factor_base_sweep_20261008/size-frontier-v2.json',
    'src/cryptanalysis/binary_semaev_chain_sat.rs',
    'src/cryptanalysis/koblitz_index_calculus.rs',
]
for name in paths:
    target = snapshot / name
    target.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(name, target)
(snapshot / 'attestation.json').write_text(json.dumps({
    'schema': 'n83.chain-s3-source-attestation/v1',
    'status_clean': True,
    'source_commit': str(commit),
}, sort_keys=True) + '\n')
print(json.dumps({'status': 'PASS_FROZEN_INPUTS', 'source_commit': str(commit),
                  'base_sha256': base['sha256']}, sort_keys=True))
PY

cargo build --release --locked --example koblitz_n83_factor_base_export
binary="$PWD/target/release/examples/koblitz_n83_factor_base_export"
sha256sum "$binary" > "$out/binary.sha256"
if ! command -v docker >/dev/null 2>&1; then
  sudo dnf -y install docker | tee "$out/docker-install.log"
fi
sudo systemctl start docker
sudo docker pull python:3.11-slim | tee "$out/docker-pull.log"
sudo docker image inspect python:3.11-slim --format '{{.Id}}' \
  | tee "$out/container-image-id.txt"
set +e
timeout --signal=TERM --kill-after=10s 620s sudo docker run --name n83-k4096-phase --rm \
  --network none --cpus 1 --memory 4g --memory-swap 4g \
  -e N83_CHAIN_PHASE_TRACE=1 -e ICV1_FROZEN_SOURCE_DIR=/source \
  -v "$binary:/worker:ro" -v "$snapshot:/source:ro" \
  -v "$panel:/panel:ro" -v "$fixture:/fixture:ro" -v "$out/run:/out" \
  python:3.11-slim /worker primary-orbit-chain-cold \
  /panel /fixture 4096 public_x_hash 2026100801 5 1 \
  150000 2000000 32 1000 600 4096 /out/worker \
  > "$out/worker.stdout" 2> "$out/worker.stderr"
exit_code=$?
set -e
sudo docker rm -f n83-k4096-phase >/dev/null 2>&1 || true
printf '%s\n' "$exit_code" > "$out/worker-exit-code.txt"

if test -f "$out/run/worker/events.jsonl"; then
  python3 "$audit" --stderr "$out/worker.stderr" \
    --events "$out/run/worker/events.jsonl" \
    --fixture "$fixture/probe-corpus.json" \
    --output "$out/independent-phase-audit.json" \
    > "$out/audit.stdout" 2> "$out/audit.stderr" || true
fi
python3 - "$out" "$source_commit" "$exit_code" <<'PY'
import json
from pathlib import Path
import sys
root, commit, code = Path(sys.argv[1]), sys.argv[2], int(sys.argv[3])
audit = root / 'independent-phase-audit.json'
result = {
    'schema': 'n83.k4096-native-phase-result/v1',
    'source_commit': commit,
    'worker_exit_code': code,
    'audit_status': json.loads(audit.read_text())['status'] if audit.exists() else 'AUDIT_INCOMPLETE',
    'selected_best_total_runtime': None,
}
(root / 'result.json').write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
PY
