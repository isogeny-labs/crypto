#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
k=$1
case "$k" in 2048|4096) ;; *) exit 2;; esac
work=/home/ec2-user/scratch/n83-allocation-profile-20261021
old=/home/ec2-user/scratch/n83-natural-1m-20261020
proof=research/koblitz_n83_factor_base_sweep_20261008/verification/native-allocation-profile-20261021
runner="$work/bin/runner-v22"
worker="$work/bin/native-storage-baseline"
test -z "$(git status --porcelain)"
head=$(git rev-parse HEAD)
protocol_sha=$(sha256sum "$proof/PROTOCOL.md"); protocol_sha=${protocol_sha%% *}
oldspec="$old/specs-v20/k$k.json"
panel=$(jq -r .panel "$oldspec"); fixtures=$(jq -r .fixtures "$oldspec"); image=$(jq -r .image "$oldspec")
"$runner" freeze "$PWD" "$panel" "$fixtures" "$worker" "$image" "$k" 450000 2400 "$protocol_sha" "$work/specs/k$k-p22.freeze.json"
jq '.isolate=true | .cpu_reservation=[14,30] | .memory_limit_bytes=8589934592 | .systemd_swap_sync=true | .lock_wait_seconds=600' "$work/specs/k$k-p22.freeze.json" > "$work/specs/k$k-p22.json"
binds='[]'
oldworker=$(jq -r .worker "$oldspec")
while IFS= read -r path; do
 path=${path/"$old/bin/runner-v20"/"$runner"}
 path=${path/"$old/specs-v20/k$k.json"/"$work/specs/k$k-p22.json"}
 path=${path/"$oldworker"/"$worker"}
 digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
 binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done < <(jq -r '.binds[].path' "$old/cards/k$k-v20.json")
for path in "$PWD/$proof/PROTOCOL.md" "$PWD/$proof/QUEUED-V22.md" "$PWD/tools/n83-sweep-runner/src/reservation.rs" "$work/compile-queued-card.sh" "$PWD/tools/n83-sweep-runner/src/storage_summary.rs" "$PWD/research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-1m-20261020/exec/k$k-v20/stderr.log"; do
 digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
 binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done
card="$work/cards/k$k-p22.json"
jq --arg h "$head" --argjson b "$binds" --arg runner "$runner" --arg spec "$work/specs/k$k-p22.json" '
 walk(if type=="string" then gsub("native-natural-1m-20261020";"native-allocation-profile-20261021") | gsub("native-natural-1m";"native-allocation-profile") | gsub("-v20";"-p22") else . end)
 | .commit=$h | .binds=$b | .commands[0].argv[0]=$runner | .commands[0].argv[2]=$spec
 | .commands[0].timeout_seconds=3600
 | .commands[0].outputs += [(.commands[0].argv[-1]+"/lock-wait.json")]
 | .must_not_exist=([.commands[]|.argv[-1],.stdout,.stderr]+[.receipt])
' "$old/cards/k$k-v20.json" > "$card"
native_path=$(jq -r .native_path "$card")
git check-ignore "$proof/exec/k$k-p22.relay.json" "$proof/exec/k$k-p22.receipt.json"
sudo -n env "PATH=$native_path" "$runner" card-prompt "$card" > "$work/cards/k$k-p22.prompt.txt"
test -z "$(git status --porcelain)"
jq -n --arg h "$head" --argjson k "$k" '{role:"experiment-director",mode:"compile-only",commit:$h,columns:$k,conflicts:450000,worker_seconds:2400,command_seconds:3600,memory_bytes:8589934592,experiments_executed:false}' > "$work/cards/k$k-p22.compile-provenance.json"
sha256sum "$card" "$work/cards/k$k-p22.prompt.txt"
