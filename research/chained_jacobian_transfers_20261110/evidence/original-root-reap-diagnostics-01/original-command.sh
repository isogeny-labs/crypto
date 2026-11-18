#!/usr/bin/env bash
set -u
export RUST_BACKTRACE=1
export CHECKPOINT_SKIP=ordinary_original_unit_diagnostics
out=/workspace/scratch/ct69-original-root-reap-diagnostics-01
binary=/workspace/scratch/ct55-g3-root-regression-01-build-cache/release/deps/crypto_lib-0f5740d1cfe22c09
cd /workspace/crypto
date -u +%Y-%m-%dT%H:%M:%SZ > "$out/start.txt"
for label in ok_without_observed_reap_cannot_mark_or_read_sidecars publication_collision_preserves_previous_bytes_and_pending_failure; do
  timeout 120 "$binary" --exact "cryptanalysis::ecbench::runner::diagnostic_reap_publication_regression_tests::$label" --nocapture --test-threads=1 > "$out/$label.log" 2>&1
  code=$?
  printf '%s\n' "$code" > "$out/$label.exit"
done
date -u +%Y-%m-%dT%H:%M:%SZ > "$out/finish.txt"
python3 - "$out" <<'GUARDS'
from pathlib import Path
import hashlib,json,sys
o=Path(sys.argv[1]);before=json.loads((o/'before.json').read_text());after={p:hashlib.sha256(Path(p).read_bytes()).hexdigest()for p in before};changed=[p for p,h in before.items()if after[p]!=h]
(o/'after.json').write_text(json.dumps(after,indent=2)+'\n');(o/'guards.json').write_text(json.dumps(dict(count=len(before),changed=changed),indent=2)+'\n');raise SystemExit(bool(changed))
GUARDS
guard=$?
printf '%s\n' "$guard" > "$out/guard.exit"
exit "$guard"
