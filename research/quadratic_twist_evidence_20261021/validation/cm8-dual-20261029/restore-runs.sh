#!/usr/bin/env bash
# Exact file restoration and deterministic archive check; no research arithmetic.
set -euo pipefail
root=${TWIST_REPO_ROOT:-$(git rev-parse --show-toplevel)}
root=$(cd "$root" && pwd)
validation=$root/research/quadratic_twist_evidence_20261021/validation/cm8-dual-20261029
out=${1:?usage: restore-runs.sh FRESH_DIRECTORY}
test ! -e "$out"
mkdir -p "$out"
out=$(cd "$out" && pwd)
tar -xJf "$validation/runs.tar.xz" -C "$out"
python3 "$validation/restore-files.py" "$root" "$validation" "$out"
(cd "$out" && sha256sum -c "$validation/RUNS.sha256")
python3 - "$validation/DEHYDRATED.json" "$out/excluded.txt" <<'PY'
import json, pathlib, sys
paths = json.loads(pathlib.Path(sys.argv[1]).read_text())
pathlib.Path(sys.argv[2]).write_text(''.join(p + '\n' for p in sorted(paths)))
PY
(cd "$out" && tar --sort=name --mtime='2026-10-29 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu --exclude-from=excluded.txt \
  -cf - runs | xz -6 -T1 > repacked.tar.xz)
cmp "$validation/runs.tar.xz" "$out/repacked.tar.xz"
printf '%s\n' 'PASS: every original raw hash and deterministic archive bytes restored'
