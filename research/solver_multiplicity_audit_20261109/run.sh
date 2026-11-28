#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
: "${MSOLVE_BINARY:?set MSOLVE_BINARY to pinned msolve binary}"
: "${ANALYZE_BINARY:?set ANALYZE_BINARY to frozen analysis binary}"
: "${MSOLVE_LIBRARY_PATH:?set MSOLVE_LIBRARY_PATH to extracted dependency library directory}"
# This script is committed with the frozen protocol before any affected output.
sha256sum --check frozen-files.sha256
node - "$MSOLVE_BINARY" "$ANALYZE_BINARY" <<'JS'
const fs=require('fs'),crypto=require('crypto');
const p=JSON.parse(fs.readFileSync('protocol.json'));
if(p.freeze.status!=='frozen') throw Error('protocol is not frozen');
for(const [path,expected] of [[process.argv[2],p.arms[0].instrument.binary_sha256],
  [process.argv[3],p.extensions.analysis_binary_sha256]]) {
  const actual=crypto.createHash('sha256').update(fs.readFileSync(path)).digest('hex');
  if(actual!==expected) throw Error('binary hash mismatch: '+path);
}
JS
if [ -e evidence/runs ] || [ -e evidence/analysis.jsonl ]; then
  echo 'Refusing to overwrite an existing run.' >&2
  exit 2
fi
mkdir evidence/runs
for fixture in double split simple degree3; do
  for rep in 0 1; do
    for arm in gb param; do
      prefix="evidence/runs/$fixture-$rep-$arm"
      args=(-t 1 -l 2 -v 2 -f "inputs/$fixture.ms" -o "$prefix.out")
      if [ "$arm" = gb ]; then args+=(-g 2); fi
      date -u +%s > "$prefix.start-epoch"
      printf '%s\n' "$MSOLVE_BINARY" "${args[@]}" > "$prefix.argv"
      set +e
      LD_LIBRARY_PATH="$MSOLVE_LIBRARY_PATH" OMP_NUM_THREADS=1 \
        timeout 30s "$MSOLVE_BINARY" "${args[@]}" > "$prefix.stdout" 2> "$prefix.stderr"
      rc=$?
      set -e
      printf '%s\n' "$rc" > "$prefix.exit"
      date -u +%s > "$prefix.end-epoch"
    done
  done
done
set +e
"$ANALYZE_BINARY" "$PWD" > evidence/analysis.jsonl 2> evidence/analysis.stderr
rc=$?
set -e
printf '%s\n' "$rc" > evidence/analysis.exit
exit "$rc"
