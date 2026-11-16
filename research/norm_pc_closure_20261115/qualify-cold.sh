#!/usr/bin/env bash
set -euo pipefail
pc_build=$(realpath "${1:?usage: qualify-cold.sh BUILD NEW_OUTDIR}")
pc_root=$(realpath -m "${2:?new output directory required}")
pc_study=$(cd "$(dirname "$0")" && pwd)
mkdir "$pc_root"
cp "$pc_build/cold-source-binary.sha256" "$pc_root/build-source-binary.sha256"
cp "$pc_build/cold-openssl.txt" "$pc_root/openssl.txt"
sha256sum "$pc_build/"{produce,verify,cold,replay} "$pc_study/"*.cpp "$pc_study/"*.hpp > "$pc_root/pre-call-source-binary.sha256"
git -C "$pc_study" rev-parse HEAD > "$pc_root/source-commit.txt"
export PC_DEGREE_CAP=4 PC_VERIFIER="$pc_build/verify"
if (ulimit -v 2097152 || exit 125; exec timeout --signal=KILL 1200 "$pc_build/cold" "$pc_build/produce" "$pc_root/recovery") \
 > "$pc_root/stdout.log" 2> "$pc_root/stderr.log"; then pc_exit=0; else pc_exit=$?; fi
printf '%s\n' "$pc_exit" > "$pc_root/exit.txt"
test "$pc_exit" = 0
"$pc_build/replay" --cold-one "$pc_root/recovery" "$pc_root/recovery/summary.tsv" > "$pc_root/openssl-replay.log" 2>&1
cat "$pc_root/stdout.log" "$pc_root/openssl-replay.log"
