#!/usr/bin/env bash
# Thin orchestration only: every spec under specs/ into sessions/<vid>__<spec>, four lanes,
# each session audited with 8 exact replays.  Skips sessions that exist (never re-runs one).
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
ECB=${ECBENCH:-$HERE/../../target/release/ecbench}
mkdir -p "$HERE/sessions" "$HERE/.locks"
one() {
  spec=$1; vid=$(basename "$(dirname "$spec")"); name=$(basename "$spec" .json)
  out="$HERE/sessions/${vid}__${name}"
  [ -e "$out" ] && { echo "skip $out"; return 0; }
  echo "start ${vid}__${name} $(date -u +%H:%M:%SZ)"
  "$ECB" run --allow-busy --cpus none --lock "$HERE/.locks/${vid}__${name}" --spec "$spec" --out "$out" > "$out.run.log" 2>&1
  r=$?
  "$ECB" verify --dir "$out" --replay 8 > "$out.verify.log" 2>&1
  echo "done  ${vid}__${name} $(date -u +%H:%M:%SZ) run=$r verify=$? $(tail -1 "$out.verify.log")"
}
export -f one
export HERE ECB
# longest first: tier b, then the tier-a crater/floor specs, then the rest
ls "$HERE"/specs/b*/*.json "$HERE"/specs/a*/spec_t150.json "$HERE"/specs/a*/spec_t38.json "$HERE"/specs/a*/*.json | grep -v vertices.json | awk '!seen[$0]++' \
  | xargs -P 4 -I{} bash -c 'one "$@"' _ {}
echo "ALL SESSIONS FINISHED $(date -u +%H:%M:%SZ)"
