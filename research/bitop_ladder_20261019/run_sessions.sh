#!/usr/bin/env bash
# Run and verify this study's sessions: one per spec named on the command
# line, four at a time, largest curve first. Counts only (PROTOCOL.md): L0,
# `--cpus none`, one lock per session so they do not queue on each other.
#
#   research/bitop_ladder_20261019/run_sessions.sh specs/train-P*.json
#
# Each spec `specs/<name>.json` runs into `sessions/<name>/` (which must not
# exist) and is then audited with `ecbench verify --replay 2` into
# `sessions/<name>/verify.json`. Exit status is nonzero if any run or audit
# failed; the logs say which.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
bin="$root/target/release/ecbench"
[ -x "$bin" ] || { echo "missing $bin; cargo build --release --bin ecbench" >&2; exit 2; }
mkdir -p "$here/sessions" "$here/logs"

one() {
  local spec=$1 name out
  name=$(basename "$spec" .json)
  out="$here/sessions/$name"
  "$bin" run --spec "$here/$spec" --out "$out" --cpus none \
    --lock "$here/logs/$name.lock" --quiet >"$here/logs/$name.run.log" 2>&1 || {
    echo "RUN FAILED $name"; return 1; }
  "$bin" verify --dir "$out" --replay 2 --exit-code --out "$out/verify.json" \
    >"$here/logs/$name.verify.log" 2>&1 || {
    echo "VERIFY FAILED $name"; return 1; }
  echo "DONE $name"
}
export -f one
export here bin

cd "$here"
printf '%s\n' "$@" | sort -t P -k2 -n -r | xargs -P 4 -I{} bash -c 'one "$@"' _ {}
