#!/usr/bin/env bash
# Validator full replay of the two L2 sessions on COPIES (the committed session directories are not touched).
# Usage: l2_replay_all.sh <scratch_dir>
set -eu
S=${1:?scratch dir}; EB=/home/user/ctarget-f1/release/ecbench   # expected sha256 fc6bd9b7...e4e8
R=/home/user/crypto/research/curve25519_wave6_runs_20261115/L2_ladder_check/sessions
sha256sum "$EB"
for k in 20 24; do
  rm -rf "$S/k$k"; mkdir -p "$S/k$k/exec"
  cp "$R/k$k"/{host.json,plan.json,records.jsonl,session.json,spec.json} "$S/k$k/"
  "$EB" verify --dir "$S/k$k" --replay-all --out "$S/verify-all-k$k.json" > "$S/verify-all-k$k.log" 2>&1
  tail -1 "$S/verify-all-k$k.log"
done
