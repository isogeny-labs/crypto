#!/usr/bin/env bash
set -u
export PATH="$HOME/.cargo/bin:$HOME/.elan/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.99.1
export CARGO_TARGET_DIR=/workspace/scratch/ct55-g3-root-regression-01-build-cache
cd /workspace/crypto
out=/workspace/scratch/ct68-root-library-regression-03
date -u +%Y-%m-%dT%H:%M:%SZ > "$out/start.txt"
rustc -Vv > "$out/rustc.txt" 2>&1
cargo -V > "$out/cargo.txt" 2>&1
uname -a > "$out/host.txt" 2>&1
set +e
CHECKPOINT_SKIP=routine_original_unit_regression timeout 1800 cargo test --release --lib --offline --locked > "$out/native.log" 2>&1
status=$?
printf '%s\n' "$status" > "$out/native.exit"
date -u +%Y-%m-%dT%H:%M:%SZ > "$out/finish.txt"
python3 - "$out" <<'GUARD'
from pathlib import Path
import hashlib,json,sys
o=Path(sys.argv[1]);before=json.loads((o/'before.json').read_text());after={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in before};changed=[p for p in before if before[p]!=after[p]]
(o/'after.json').write_text(json.dumps(after,indent=2)+'\n');(o/'guards.json').write_text(json.dumps(dict(count=len(before),changed=changed),indent=2)+'\n');raise SystemExit(bool(changed))
GUARD
guard=$?
printf '%s\n' "$guard" > "$out/guard.exit"
if [ "$status" -ne 0 ]; then exit "$status"; fi
exit "$guard"
