#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
k=$1
case "$k" in 2048|4096) ;; *) exit 2;; esac
work=/home/ec2-user/scratch/n83-natural-1m-20261020
old=/home/ec2-user/scratch/n83-natural-800k-20261020
study=research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-1m-20261020
runner="$work/bin/runner-v15"
test -z "$(git status --porcelain)"
head=$(git rev-parse HEAD)
protocol_sha=$(sha256sum "$study/PROTOCOL.md"); protocol_sha=${protocol_sha%% *}
oldspec="$old/specs-v14/k$k.json"
panel=$(jq -r .panel "$oldspec"); fixtures=$(jq -r .fixtures "$oldspec")
worker=$(jq -r .worker "$oldspec"); image=$(jq -r .image "$oldspec")
"$runner" freeze "$PWD" "$panel" "$fixtures" "$worker" "$image" "$k" 1000000 6000 "$protocol_sha" "$work/specs/k$k.freeze.json"
jq '.isolate=true | .cpu_reservation=[14,30] | .memory_limit_bytes=8589934592' "$work/specs/k$k.freeze.json" > "$work/specs/k$k.json"
binds='[]'
while IFS= read -r path; do
 path=${path/"$old/bin/runner-v14"/"$runner"}
 path=${path/"$old/specs-v14/k$k.json"/"$work/specs/k$k.json"}
 digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
 binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done < <(jq -r '.binds[].path' "$old/cards/k$k-v14.json")
for path in "$PWD/$study/PROTOCOL.md" "$work/compile-card.sh"; do
 digest=$(sha256sum "$path"); digest=${digest%% *}
 binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done
card="$work/cards/k$k-v15.json"
jq --arg h "$head" --argjson b "$binds" --arg runner "$runner" --arg spec "$work/specs/k$k.json" '
 walk(if type=="string" then gsub("native-natural-800k-20261020";"native-natural-1m-20261020") | gsub("native-natural-800k";"native-natural-1m") | gsub("-v14";"-v15") else . end)
 | .commit=$h | .binds=$b | .commands[0].argv[0]=$runner | .commands[0].argv[2]=$spec
 | .commands[0].timeout_seconds=6600
 | .commands[0].outputs += [.commands[0].argv[-1]+"/memory-profile.jsonl"]
 | .must_not_exist=([.commands[]|.argv[-1],.stdout,.stderr]+[.receipt])
' "$old/cards/k$k-v14.json" > "$card"
native_path=$(jq -r .native_path "$card")
git check-ignore "$study/exec/k$k-v15.relay.json" "$study/exec/k$k-v15.receipt.json"
sudo -n env "PATH=$native_path" "$runner" card-prompt "$card" > "$work/cards/k$k-v15.prompt.txt"
test -z "$(git status --porcelain)"
jq -n --arg h "$head" --argjson k "$k" '{role:"experiment-director",mode:"compile-only",commit:$h,columns:$k,conflicts:1000000,worker_seconds:6000,command_seconds:6600,memory_bytes:8589934592,experiments_executed:false}' > "$work/cards/k$k-v15.compile-provenance.json"
sha256sum "$card" "$work/cards/k$k-v15.prompt.txt"
