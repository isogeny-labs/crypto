#!/bin/bash
# Execute only after the provider allocation receipt is copied to /workspace.
set -Eeuo pipefail

WORK=/workspace/n83_screen
EVIDENCE=/workspace/evidence
RECEIPT=/workspace/n83_screen_allocation.json
BRANCH=codex/n83-current-online-screen-runpod-20261019
mkdir -p "$EVIDENCE"
exec > >(tee -a "$EVIDENCE/bootstrap.log") 2>&1

archive_on_exit() {
  local status=$?
  trap - EXIT
  set +e
  printf '%s\n' "$status" > "$EVIDENCE/bootstrap_exit_status.txt"
  date -u +%FT%TZ > "$EVIDENCE/finished_utc.txt"
  if [ -d /workspace/results ]; then
    tar -C /workspace -czf /workspace/n83_screen_raw.tar.gz results evidence
  else
    tar -C /workspace -czf /workspace/n83_screen_raw.tar.gz evidence
  fi
  if [ -f /workspace/n83_screen_raw.tar.gz ]; then
    split -b 48m -d -a 2 /workspace/n83_screen_raw.tar.gz /workspace/raw.part.
    python3 - <<'PY'
import hashlib, json
from pathlib import Path
root=Path('/workspace')
archive=root/'n83_screen_raw.tar.gz'
parts=[]
for path in sorted(root.glob('raw.part.*')):
    parts.append({'name':path.name,'bytes':path.stat().st_size,
                  'sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
manifest={'schema':'n83.current-online-screen-runpod-raw-parts/v1',
          'archive_bytes':archive.stat().st_size,
          'archive_sha256':hashlib.sha256(archive.read_bytes()).hexdigest(),
          'parts':parts}
(root/'raw_parts_manifest.json').write_text(json.dumps(manifest,sort_keys=True,indent=2)+'\n')
print(json.dumps({'archive_bytes':manifest['archive_bytes'],
                  'archive_sha256':manifest['archive_sha256'],
                  'parts':len(parts)},sort_keys=True))
PY
  fi
}
trap archive_on_exit EXIT

test -f "$RECEIPT"
date -u +%FT%TZ > "$EVIDENCE/started_utc.txt"
cat /etc/os-release > "$EVIDENCE/os-release.txt"
uname -a > "$EVIDENCE/uname.txt"
cat /proc/cpuinfo > "$EVIDENCE/cpuinfo.txt"
cat /proc/meminfo > "$EVIDENCE/meminfo.txt"

apt-get update -qq
DEBIAN_FRONTEND=noninteractive apt-get install -y -qq \
  git curl ca-certificates build-essential pkg-config libssl-dev python3
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
  sh -s -- -y --profile minimal --default-toolchain 1.99.0
export PATH="/root/.cargo/bin:$PATH"
rustc --version > "$EVIDENCE/rustc-version.txt"
cargo --version > "$EVIDENCE/cargo-version.txt"

git clone --filter=blob:none --depth 1 --branch "$BRANCH" --sparse \
  https://github.com/aburan28/crypto.git "$WORK"
git -C "$WORK" sparse-checkout init --cone
git -C "$WORK" sparse-checkout set src examples docs \
  research/n83_current_online_screen_20261019 \
  research/n83_root4_20261014 \
  research/n83_indexed5_20261013 \
  research/n83_dense_root_20261013 \
  research/n83_root4_highk_confirm_results_20261016 \
  research/n83_online_finalists_20261018 \
  research/n83_root4_frontier_current_20261018
python3 - "$RECEIPT" "$WORK" <<'PY'
import hashlib,json,subprocess,sys
from pathlib import Path
receipt=json.loads(Path(sys.argv[1]).read_text())
root=Path(sys.argv[2]); head=subprocess.check_output(
    ['git','rev-parse','HEAD'],cwd=root,text=True).strip()
dispatch=json.loads((root/'research/n83_current_online_screen_20261019/runpod_dispatch.json').read_text())
if (head!=receipt['source_head_sha'] or dispatch['branch']!=
        'codex/n83-current-online-screen-runpod-20261019'):
    raise ValueError('Pod checkout differs from sealed allocation')
for relative,expected in dispatch['sha256'].items():
    if hashlib.sha256((root/relative).read_bytes()).hexdigest()!=expected:
        raise ValueError('sealed source/fixture differs: '+relative)
print('PASS_SEALED_RUNPOD_DISPATCH',head)
PY
git -C "$WORK" rev-parse HEAD > "$EVIDENCE/source-commit.txt"

export CARGO_TARGET_DIR=/workspace/cargo-target
export CARGO_BUILD_JOBS=8
export RUST_TEST_THREADS=1
cd "$WORK"
cargo test --release --locked --example koblitz_orbit_dlp_fast > \
  "$EVIDENCE/ic-example-tests.log" 2>&1
cargo build --release --locked --example koblitz_orbit_dlp_fast > \
  "$EVIDENCE/ic-build.log" 2>&1
sha256sum "$CARGO_TARGET_DIR/release/examples/koblitz_orbit_dlp_fast" > \
  "$EVIDENCE/ic-binary-sha256.txt"

export PYTHONDONTWRITEBYTECODE=1
python3 research/n83_current_online_screen_20261019/runpod_run.py --preflight > \
  "$EVIDENCE/fixture-preflight.json"
timeout 7200 python3 research/n83_current_online_screen_20261019/runpod_run.py \
  --binary "$CARGO_TARGET_DIR/release/examples/koblitz_orbit_dlp_fast" \
  --provider-receipt "$RECEIPT" --output /workspace/results > \
  "$EVIDENCE/run.log" 2>&1
timeout 1200 python3 research/n83_current_online_screen_20261019/runpod_verify.py \
  /workspace/results > "$EVIDENCE/verify.log" 2>&1
