set -euo pipefail
repo=/home/ec2-user/crypto-n83-resume
prior=/home/ec2-user/scratch/n83-natural-400k-20261019
next=/home/ec2-user/scratch/n83-natural-800k-20261020
runner="$next/bin/runner-v12"
study=research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-800k-20261020
root="$study/exec"
head=$(git rev-parse HEAD)
test -z "$(git status --porcelain)"
protocol_sha=$(sha256sum "$study/PROTOCOL.md"); protocol_sha=${protocol_sha%% *}
source_spec="$prior/specs/k4096.json"
panel=$(jq -r '.panel' "$source_spec"); fixtures=$(jq -r '.fixtures' "$source_spec")
worker=$(jq -r '.worker' "$source_spec"); image=$(jq -r '.image' "$source_spec")
"$runner" freeze "$repo" "$panel" "$fixtures" "$worker" "$image" 4096 800000 4800 "$protocol_sha" "$next/specs/k4096.freeze.json"
jq '.isolate=true | .cpu_reservation=[14,30]' "$next/specs/k4096.freeze.json" > "$next/specs/k4096.json"
old="$prior/cards/k4096-v10.json"
card="$next/cards/k4096-v12.json"
binds='[]'
while IFS= read -r path; do
  case "$path" in "$prior/specs/"*) continue ;; esac
  path=${path/"$prior/bin/runner-v10"/"$runner"}
  digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
  binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done < <(jq -r '.binds[].path' "$old")
for path in "$next/specs/k4096.json" "$repo/$study/PROTOCOL.md" "$repo/$study/.gitignore"; do
  digest=$(sha256sum "$path"); digest=${digest%% *}
  binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done
jq --arg h "$head" --arg r "$root" --arg runner "$runner" --arg spec "$next/specs/k4096.json" --argjson b "$binds" '
 .commit=$h | .binds=$b | .card_id="CARD-native-natural-800k-k4096-v12" | .experiment_id="native-natural-800k-k4096-v12"
 | .bus_address="executor-mechanical-native-natural-800k-k4096-v12" | .write_scope=[$r+"/"]
 | .receipt=($r+"/k4096-v12.receipt.json") | .relay_stdout=($r+"/k4096-v12.relay.json")
 | .commands |= map(.id="k4096-v12" | .timeout_seconds=5400 | .argv[0]=$runner | .argv[2]=$spec | .argv[-1]=($r+"/k4096-v12")
    | .stdout=($r+"/k4096-v12.stdout") | .stderr=($r+"/k4096-v12.stderr")
    | .outputs |= map(split("/")[-1] | $r+"/k4096-v12/"+.))
 | .must_not_exist=([.commands[]|.argv[-1],.stdout,.stderr]+[.receipt])
' "$old" > "$card"
test -d "$root"
git check-ignore "$root/k4096-v12.relay.json" "$root/k4096-v12.receipt.json"
native_path=$(jq -r '.native_path' "$card")
sudo -n env "PATH=$native_path" "$runner" card-prompt "$card" > "$next/cards/k4096-v12.prompt.txt"
test -z "$(git status --porcelain)"
jq -n --arg h "$head" '{role:"experiment-director",mode:"compile-only",commit:$h,cpu_reservation:[14,30],spec_native_freeze:true,root_native_prompt_validated:true,experiments_executed:false}' > "$next/cards/k4096-v12.compile-provenance.json"
sha256sum "$card" "$next/cards/k4096-v12.prompt.txt"
