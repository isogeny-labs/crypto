set -euo pipefail
repo=/home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-natural-800k-20261020
runner="$work/bin/runner-v13"
study=research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-800k-20261020
root="$study/exec"
head=$(git rev-parse HEAD)
test -z "$(git status --porcelain)"
protocol_sha=$(sha256sum "$study/MEMORY-V13.md"); protocol_sha=${protocol_sha%% *}
source_spec="$work/specs/k2048.json"
panel=$(jq -r '.panel' "$source_spec"); fixtures=$(jq -r '.fixtures' "$source_spec")
worker=$(jq -r '.worker' "$source_spec"); image=$(jq -r '.image' "$source_spec")
"$runner" freeze "$repo" "$panel" "$fixtures" "$worker" "$image" 2048 800000 4800 "$protocol_sha" "$work/specs-v13/k2048.freeze.json"
jq '.isolate=true | .cpu_reservation=[14,30] | .memory_limit_bytes=8589934592' "$work/specs-v13/k2048.freeze.json" > "$work/specs-v13/k2048.json"
binds='[]'
while IFS= read -r path; do
 path=${path/"$work/bin/runner-v12"/"$runner"}
 path=${path/"$work/specs/k2048.json"/"$work/specs-v13/k2048.json"}
 digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
 binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done < <(jq -r '.binds[].path' "$work/cards/k2048-v12.json")
for path in "$repo/$study/MEMORY-V13.md" "$work/cards/k2048-v12.assessment.json" "$work/cards/k4096-v12.assessment.json"; do
 digest=$(sha256sum "$path"); digest=${digest%% *}
 binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done
card="$work/cards/k2048-v13.json"
jq --arg h "$head" --argjson b "$binds" --arg runner "$runner" --arg spec "$work/specs-v13/k2048.json" '
 walk(if type=="string" then gsub("k2048-v12";"k2048-v13") else . end)
 | .commit=$h | .binds=$b | .commands[0].argv[0]=$runner | .commands[0].argv[2]=$spec
 | .commands[0].timeout_seconds=5400 | .must_not_exist=([.commands[]|.argv[-1],.stdout,.stderr]+[.receipt])
' "$work/cards/k2048-v12.json" > "$card"
native_path=$(jq -r '.native_path' "$card")
git check-ignore "$root/k2048-v13.relay.json" "$root/k2048-v13.receipt.json"
sudo -n env "PATH=$native_path" "$runner" card-prompt "$card" > "$work/cards/k2048-v13.prompt.txt"
test -z "$(git status --porcelain)"
jq -n --arg h "$head" '{role:"experiment-director",mode:"compile-only",commit:$h,separately_frozen_memory_repeat:true,memory_limit_bytes:8589934592,prior_attempts_preserved:true,cumulative_worker_allocation_seconds:19200,worker_caps:{conflicts:800000,wall_seconds:4800},command_seconds:5400,experiments_executed:false}' > "$work/cards/k2048-v13.compile-provenance.json"
sha256sum "$card" "$work/cards/k2048-v13.prompt.txt"
