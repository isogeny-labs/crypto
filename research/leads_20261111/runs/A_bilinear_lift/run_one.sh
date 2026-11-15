#!/bin/bash
# Usage: run_one.sh TAG D FULL [ROWCAP] -- emit_lift.py args...
# Emits the system, runs polyclose, appends {"tag","hdr","res"} to out/main.jsonl; progress log in out/log/TAG.log.
cd "$(dirname "$0")"
tag=$1; D=$2; FULL=$3; RC=${4:-0}; shift 4; [ "$1" = "--" ] && shift
mkdir -p out/log
sys=$(mktemp); hdr=$(python3 emit_lift.py "$@" 2>&1 >"$sys")
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
res=$(./polyclose "$D" 8192 "$FULL" "$RC" <"$sys" 2>out/log/$tag.log)
rc=$?; rm -f "$sys"
echo "{\"tag\": \"$tag\", \"args\": \"$*\", \"rc\": $rc, \"hdr\": $hdr, \"res\": ${res:-null}}" >> out/main.jsonl
echo "$tag rc=$rc $res"
