#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
k=$1
case "$k" in 2048|4096) ;; *) exit 2;; esac
work=/home/ec2-user/scratch/n83-arena-reclamation-20261022
old=/home/ec2-user/scratch/n83-natural-1m-20261020
proof=research/koblitz_n83_factor_base_sweep_20261008/verification/native-arena-reclamation-20261023
runner="$work/bin/runner-arena-continuation-v3"
worker="$work/bin/native-arena-final"
test -z "$(git status --porcelain)"
rg -qx PASS_EXACT_1P5M_9000_NATURAL_PAIR_AND_ALL_NATIVE_GUARD_CARD_TESTS "$work/guard-continuation-validation-v3/status.txt"
sha256sum -c "$work/guard-continuation-validation-v3/guard.sha256"
for base in 2048 4096; do
 jq -e '.status=="PASS_OWNERSHIP_COUNTERS_AND_RETAINED_SNAPSHOT_SIGNATURES" and .matched_conflict_snapshots==1001 and .reclamation_required==true' "$work/assessment/k$base-r1/reclamation.json" > /dev/null
 jq -e '.status=="PASS_CONTAINER_MEMORY_PROFILE_CUSTODY_AND_COUNTERS"' "$work/assessment/k$base-r1/memory.json" > /dev/null
 jq -e '.outcome=="executed_as_specified" and .relay_verbatim==true and .specification_errors==[]' "$work/director-k$base-r1-assessment-20261023/assessment.json" > /dev/null
done
rg -qx PASS_K2048_PUT_HEAD_FRESH_GET_PAYLOAD_NATIVE_AUDIT_MEMORY_RECLAMATION_AND_REPACK "$work/k2048-r1-publication-v3/status.txt"
rg -qx PASS_K4096_PUT_HEAD_FRESH_GET_PAYLOAD_NATIVE_AUDIT_MEMORY_RECLAMATION_AND_REPACK "$work/k4096-r1-publication/status.txt"
head=$(git rev-parse HEAD)
protocol_sha=$(sha256sum "$proof/PROTOCOL.md"); protocol_sha=${protocol_sha%% *}
oldspec="$old/specs-v20/k$k.json"
panel=$(jq -r .panel "$oldspec"); fixtures=$(jq -r .fixtures "$oldspec"); image=$(jq -r .image "$oldspec")
mkdir -p "$work/specs" "$work/cards" "$proof/exec"
"$runner" freeze "$PWD" "$panel" "$fixtures" "$worker" "$image" "$k" 1500000 9000 "$protocol_sha" "$work/specs/k$k-c1.freeze.json"
jq '.isolate=true | .cpu_reservation=[14,30] | .memory_limit_bytes=8589934592 | .systemd_swap_sync=true | .lock_wait_seconds=600' "$work/specs/k$k-c1.freeze.json" > "$work/specs/k$k-c1.json"
binds='[]'
oldworker=$(jq -r .worker "$oldspec")
while IFS= read -r path; do
 path=${path/"$old/bin/runner-v20"/"$runner"}
 path=${path/"$old/specs-v20/k$k.json"/"$work/specs/k$k-c1.json"}
 path=${path/"$oldworker"/"$worker"}
 digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
 binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done < <(jq -r '.binds[].path' "$old/cards/k$k-v20.json")
for path in "$PWD/$proof/PROTOCOL.md" "$PWD/$proof/CONTINUATION.md" "$work/compile-continuation-card.sh" "$PWD/tools/n83-sweep-runner/src/storage_summary.rs" "$PWD/research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-1m-20261020/exec/k$k-v20/stderr.log" "$work/assessment/k2048-r1/reclamation.json" "$work/assessment/k4096-r1/reclamation.json"; do
 digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
 binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done
card="$work/cards/k$k-c1.json"
jq --arg h "$head" --argjson b "$binds" --arg runner "$runner" --arg spec "$work/specs/k$k-c1.json" '
 walk(if type=="string" then gsub("native-natural-1m-20261020";"native-arena-reclamation-20261023") | gsub("native-natural-1m";"native-arena-reclamation-continuation") | gsub("-v20";"-c1") else . end)
 | .commit=$h | .binds=$b | .commands[0].argv[0]=$runner | .commands[0].argv[2]=$spec
 | .commands[0].timeout_seconds=10200
 | .commands[0].outputs += [(.commands[0].argv[-1]+"/lock-wait.json")]
 | .must_not_exist=([.commands[]|.argv[-1],.stdout,.stderr]+[.receipt])
' "$old/cards/k$k-v20.json" > "$card"
native_path=$(jq -r .native_path "$card")
git check-ignore "$proof/exec/k$k-c1.relay.json" "$proof/exec/k$k-c1.receipt.json"
sudo -n env "PATH=$native_path" "$runner" card-prompt "$card" > "$work/cards/k$k-c1.prompt.txt"
test -z "$(git status --porcelain)"
sha256sum "$card" "$work/cards/k$k-c1.prompt.txt"
