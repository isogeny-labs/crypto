#!/usr/bin/env bash
# Thin run orchestration for the validator's round-2 replays (no logic):
#   run.sh <name> <binary> [VAR=value ...] -- <args...>
# Hashes the binary, refuses a binary that is not one of the two pins, runs it
# from /home/user/crypto with the given environment (and ZERO_DIM_LA_* unset),
# then appends one JSON line to round2/journal.jsonl: name, argv, env, binary
# sha256, exit code, start/end UTC, and the sha256 of every file the run wrote
# under round2/out/<name>.*.  Comparison is done by vtool, not here.
set -u
REPO=/home/user/crypto
HERE=$REPO/research/postgb_linear_stage_20261026/validation/round2
name=$1; bin=$2; shift 2
envs=()
while [ "$1" != "--" ]; do envs+=("$1"); shift; done
shift
digest=$(sha256sum "$bin" | cut -c1-64)
case "$digest" in
  06351d89429a1d50d5026e9bfe99c7299e25e3d15b0024b1c0b7cda4d686444b|a49184b0c25ae27538f083999459ca9c4cc88abf4f9c99cc06b007253b5feb85) ;;
  *) echo "binary $bin sha256 $digest is not a round-2 pin" >&2; exit 2 ;;
esac
for f in "$HERE"/out/"$name".*; do [ -e "$f" ] && { echo "outputs for $name exist" >&2; exit 2; }; done
start=$(date -u +%Y-%m-%dT%H:%M:%SZ)
cd "$REPO"
env -u ZERO_DIM_LA_SIMD -u ZERO_DIM_LA_THREADS -u JV_LIN -u JV_RHO_NEG -u JV_DUMP_INSTANCE "${envs[@]}" \
  "$bin" "$@" >"$HERE/out/$name.stdout" 2>"$HERE/out/$name.stderr"
code=$?
end=$(date -u +%Y-%m-%dT%H:%M:%SZ)
outs=$(cd "$REPO" && for f in research/postgb_linear_stage_20261026/validation/round2/out/"$name".*; do
  printf '"%s":"%s",' "$f" "$(sha256sum "$f" | cut -c1-64)"; done)
argv=$(printf '"%s",' "$bin" "$@")
envj=$(printf '"%s",' "${envs[@]}")
printf '{"name":"%s","argv":[%s],"env":[%s],"cwd":"%s","binary_sha256":"%s","exit_code":%d,"start_utc":"%s","end_utc":"%s","outputs":{%s}}\n' \
  "$name" "${argv%,}" "${envj%,}" "$REPO" "$digest" "$code" "$start" "$end" "${outs%,}" >>"$HERE/journal.jsonl"
echo "$name exit $code"
