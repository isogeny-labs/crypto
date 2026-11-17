set -euo pipefail
repo=/home/ec2-user/crypto-n83-resume
next=/home/ec2-user/scratch/n83-natural-400k-20261019
runner="$next/bin/runner-v11"
study=research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-400k-20261019
root="$study/exec"
head=$(git rev-parse HEAD)
test -z "$(git status --porcelain)"
protocol_sha=$(sha256sum "$study/PROTOCOL.md"); protocol_sha=${protocol_sha%% *}
source_spec="$next/specs/k4096.json"
panel=$(jq -r '.panel' "$source_spec"); fixtures=$(jq -r '.fixtures' "$source_spec")
worker=$(jq -r '.worker' "$source_spec"); image=$(jq -r '.image' "$source_spec")
"$runner" freeze "$repo" "$panel" "$fixtures" "$worker" "$image" 4096 400000 2400 "$protocol_sha" "$next/specs-v11/k4096.freeze.json"
jq '.isolate=true | .cpu_reservation=[14,30]' "$next/specs-v11/k4096.freeze.json" > "$next/specs-v11/k4096.json"
binds='[]'
while IFS= read -r path; do
  path=${path/"$next/bin/runner-v10"/"$runner"}
  path=${path/"$next/specs/k4096.json"/"$next/specs-v11/k4096.json"}
  digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
  binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done < <(jq -r '.binds[].path' "$next/cards/k4096-v10.json")
for path in "$repo/$study/INSPECTION-V11.md" "$next/k4096-v10-later-absence.stdout" "$next/k4096-v10-later-absence.exit"; do
  digest=$(sha256sum "$path"); digest=${digest%% *}
  binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done
card="$next/cards/k4096-v11.json"
jq --arg h "$head" --argjson b "$binds" --arg runner "$runner" --arg spec "$next/specs-v11/k4096.json" '
 walk(if type=="string" then gsub("k4096-v10";"k4096-v11") else . end)
 | .commit=$h | .binds=$b | .commands[0].argv[0]=$runner | .commands[0].argv[2]=$spec
 | .commands[0].timeout_seconds=3000 | .must_not_exist=([.commands[]|.argv[-1],.stdout,.stderr]+[.receipt])
' "$next/cards/k4096-v10.json" > "$card"
native_path=$(jq -r '.native_path' "$card")
git check-ignore "$root/k4096-v11.relay.json" "$root/k4096-v11.receipt.json"
sudo -n env "PATH=$native_path" "$runner" card-prompt "$card" > "$next/cards/k4096-v11.prompt.txt"
test -z "$(git status --porcelain)"
jq -n --arg h "$head" '{role:"experiment-director",mode:"compile-only",commit:$h,separately_frozen_corrective_repeat:true,prior_attempt_preserved:true,cumulative_worker_allocation_seconds:7200,worker_caps:{conflicts:400000,wall_seconds:2400},command_seconds:3000,experiments_executed:false}' > "$next/cards/k4096-v11.compile-provenance.json"
sha256sum "$card" "$next/cards/k4096-v11.prompt.txt"
