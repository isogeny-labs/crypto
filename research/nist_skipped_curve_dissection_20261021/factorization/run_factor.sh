#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
task=$1
out=$2
mkdir -p "$out/certs-initial" "$out/certs-pari"
[[ ! -e $out/started.txt ]] || { echo 'Choose a fresh attempt directory' >&2; exit 1; }
date -u +%FT%TZ > "$out/started.txt"
cp "$task" "$out/task.gp"
git -C "$here" rev-parse HEAD > "$out/code-commit.txt"
(cd "$here" && sha256sum factor_one.gp run_factor.sh) > "$out/code-sha256.txt"
"${GP_BIN:-gp}" --version > "$out/version.txt" 2>&1
set +e
TASK_FILE="$out/task.gp" timeout --kill-after=10s 650 "${GP_BIN:-gp}" -q -f -s 128000000 --default parisizemax=2000000000 "$here/factor_one.gp" > "$out/stdout.txt" 2> "$out/stderr.txt"
status=$?
set -e
date -u +%FT%TZ > "$out/finished.txt"
printf '%s\n' "$status" > "$out/exit-status.txt"
[[ $status == 0 && -s $out/pari.json ]] || exit 1
if rg -q 'FATAL:' "$out/stdout.txt" || (rg -v 'Warning:' "$out/stderr.txt" | rg -q '\*\*\*'); then exit 1; fi
echo "Finished $(jq -r '.id + ": " + .status' "$out/pari.json")"
