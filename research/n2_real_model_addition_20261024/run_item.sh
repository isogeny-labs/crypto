#!/bin/sh
# Record one run: run_item.sh <item> <mode> [VAR=value ...]
# Builds examples/n2_routes.rs, runs `n2_routes <dir> <mode>` with the given
# environment on one thread, and writes results/<item>-run-<UTC>/env.txt
# (git HEAD, tool and binary sha256, command, protocol sha256, rustc, host).
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../.." && pwd)
item=$1; mode=$2; shift 2
utc=$(date -u +%Y%m%dT%H%MZ)
dir="$HERE/results/$item-run-$utc"
mkdir -p "$dir"
BIN=${N2_BIN:-/tmp/claude-0/-home-user/829a5db7-e7b3-5b24-827a-595408da4b3a/scratchpad/n2_run}
( cd "$ROOT" && rustc --edition 2021 -O examples/n2_routes.rs -o "$BIN" )
{
  echo "git HEAD: $(git -C "$ROOT" rev-parse HEAD) (examples/n2_routes.rs modified in the working tree, uncommitted at run time)"
  echo "tool sha256: $(sha256sum "$ROOT/examples/n2_routes.rs" | cut -d' ' -f1)  examples/n2_routes.rs"
  echo "binary sha256: $(sha256sum "$BIN" | cut -d' ' -f1)"
  echo "protocol sha256: $(sha256sum "$HERE/PROTOCOL.md" | cut -d' ' -f1)  PROTOCOL.md"
  echo "command: env N2_THREADS=1 $* $BIN <dir> $mode   (built: rustc --edition 2021 -O examples/n2_routes.rs)"
  echo "rustc: $(rustc --version)"
  echo "host: $(uname -srm); $(grep -m1 'model name' /proc/cpuinfo | sed 's/.*: //'); $(nproc) cores shared with other agents; single thread"
  echo "started: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
} > "$dir/env.txt"
start=$(date +%s)
( env N2_THREADS=1 "$@" "$BIN" "$dir" "$mode" ) > "$dir/job.log" 2>&1
echo "exit-code: $?" >> "$dir/env.txt"
echo "wall seconds: $(( $(date +%s) - start ))" >> "$dir/env.txt"
echo "$dir"
