set -euo pipefail
repo=/home/ec2-user/crypto-n83-resume
opt=/home/ec2-user/scratch/n83-xor-opt-20261019
next=/home/ec2-user/scratch/n83-natural-100k-20261019
runner="$opt/bin/runner-preflight-v4"
root=research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-100k-20261019/exec
protocol="$repo/research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-100k-20261019/PROTOCOL.md"
head=$(git -C "$repo" rev-parse HEAD)
protocol_sha=$(sha256sum "$protocol"); protocol_sha=${protocol_sha%% *}
for k in k2048 k4096; do
  jq --arg h "$head" --arg p "$protocol_sha" '.conflicts=100000 | .wall_seconds=900 | .source_commit=$h | .protocol_sha256=$p' "$opt/specs-v5/$k.json" > "$next/specs/$k.json"
done
binds='[]'
while IFS= read -r path; do
  path=${path/"$opt/specs-v5/"/"$next/specs/"}
  digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
  binds=$(jq --arg p "$path" --arg h "$digest" '. + [{path:$p,sha256:$h}]' <<< "$binds")
done < <(jq -r '.binds[].path' "$opt/cards-v5/natural-10k-v5.json")
binds=$(jq --arg p "$protocol" --arg h "$protocol_sha" '. + [{path:$p,sha256:$h}]' <<< "$binds")
card="$next/cards/native-natural-100k-v6.json"
jq --arg head "$head" --argjson binds "$binds" --arg root "$root" --arg oldspec "$opt/specs-v5/" --arg newspec "$next/specs/" '
 .commit=$head | .binds=$binds | .card_id="CARD-native-natural-100k-v6" | .experiment_id="native-natural-100k-v6"
 | .receipt=($root+"/native-natural-100k-v6.receipt.json")
 | .relay_stdout=($root+"/native-natural-100k-v6.relay.json")
 | .write_scope=[$root+"/"] | .bus_address="executor-mechanical-native-natural-100k-v6"
 | .commands |= map(.id |= sub("-10k-v5$";"") | .id as $id | .timeout_seconds=990
   | .argv[2] |= sub($oldspec;$newspec) | .argv[-1]=($root+"/"+$id)
   | .outputs |= map(split("/")[-1] | $root+"/"+$id+"/"+.)
   | .stdout=($root+"/"+$id+".stdout") | .stderr=($root+"/"+$id+".stderr"))
 | .must_not_exist = ([.commands[]|.argv[-1],.stdout,.stderr] + [.receipt])
' "$opt/cards-v5/natural-10k-v5.json" > "$card"
native_path=$(jq -r '.native_path' "$card")
sudo -n env "PATH=$native_path" "$runner" card-prompt "$card" > "$next/cards/native-natural-100k-v6.prompt.txt"
card_sha=$(sha256sum "$card"); card_sha=${card_sha%% *}
prompt_sha=$(sha256sum "$next/cards/native-natural-100k-v6.prompt.txt"); prompt_sha=${prompt_sha%% *}
jq -n --arg h "$head" --arg card "$card" --arg cs "$card_sha" --arg ps "$prompt_sha" --arg proto "$protocol_sha" '{commit:$h,role:"experiment-director",mode:"compile-only",card:$card,card_sha256:$cs,prompt_sha256:$ps,protocol_sha256:$proto,native_card_prompt_validated:true,experiments_executed:false}' > "$next/cards/compile-provenance.json"
sha256sum "$card"
