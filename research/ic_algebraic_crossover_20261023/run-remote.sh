#!/usr/bin/env bash
# One frozen shard. Provision only through taskq/deploy/ephemeral.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
kind=${1:?guess, hky, or quartic}
shard=${2:?zero-based shard}
out=${3:?fresh output directory}
case "$kind" in
 guess|hky) memory=68719476736; seconds=7200 ;;
 quartic) memory=25769803776; seconds=169200 ;;
 *) exit 2 ;;
esac
test ! -e "$out"
mkdir -p "$out"
jq --argjson shard "$shard" --arg session "$out/session" \
  '.[$shard] | if .==null then error("unknown shard") else . end | .argv|=map(if .=="{session}" then $session else . end)' \
  "$study/remote-$kind-plan.json" > "$out/job.json"
mapfile -d '' -t argv < <(jq -jr '.argv[]+"\u0000"' "$out/job.json")
mapfile -d '' -t settings < <(jq -jr '.env|to_entries[]|(.key+"="+.value+"\u0000")' "$out/job.json")
git rev-parse HEAD > "$out/source-commit.txt"
sha256sum "${argv[0]}" "$study/PROTOCOL.md" "$study/remote-$kind-plan.json" > "$out/hashes.txt"
lscpu > "$out/host.txt"
cat /sys/fs/cgroup/memory.max > "$out/cgroup-memory-max.txt"
date -u +%FT%TZ > "$out/started.txt"
set +e
/usr/bin/time -v -o "$out/rusage.txt" timeout --kill-after=30s "${seconds}s" \
  prlimit --as="$memory" -- env "${settings[@]}" "${argv[@]}" > "$out/rows.jsonl" 2> "$out/stderr.txt"
rc=$?
set -e
jq -n --argjson rc "$rc" --argjson memory "$memory" --argjson seconds "$seconds" \
  '{exit_code:$rc,address_space_bytes:$memory,timeout_seconds:$seconds,status:(if $rc==0 then "finished" elif $rc==124 then "timeout" else "failed" end)}' > "$out/status.json"
date -u +%FT%TZ > "$out/ended.txt"
exit "$rc"
