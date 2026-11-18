set -euo pipefail
repo=/home/ec2-user/crypto-n83-resume
next=/home/ec2-user/scratch/n83-natural-200k-20261019
runner="$next/bin/runner-v8"
study=research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-200k-20261019
root="$study/exec"
head=$(git rev-parse HEAD)
test -z "$(git status --porcelain)"
protocol_sha=$(sha256sum "$study/PROTOCOL.md"); protocol_sha=${protocol_sha%% *}
for k in k2048 k4096; do
  source_spec="$next/specs/$k.json"
  panel=$(jq -r '.panel' "$source_spec"); fixtures=$(jq -r '.fixtures' "$source_spec")
  worker=$(jq -r '.worker' "$source_spec"); image=$(jq -r '.image' "$source_spec")
  "$runner" freeze "$repo" "$panel" "$fixtures" "$worker" "$image" "${k#k}" 200000 1200 "$protocol_sha" "$next/specs-v8/$k.freeze.json"
  jq '.isolate=true | .cpu_reservation=[14,30]' "$next/specs-v8/$k.freeze.json" > "$next/specs-v8/$k.json"
done
old="$next/cards/k2048-v7.json"
card="$next/cards/k2048-v8.json"
binds='[]'
while IFS= read -r path; do
  path=${path/"$next/specs/"/"$next/specs-v8/"}
  path=${path/"$next/bin/runner-v7"/"$runner"}
  digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
  binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done < <(jq -r '.binds[].path' "$old")
for path in "$repo/$study/RESERVATION-V8.md" "$repo/tools/n83-sweep-runner/src/reservation.rs" "$repo/tools/n83-sweep-runner/src/main.rs"; do
  digest=$(sha256sum "$path"); digest=${digest%% *}
  binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
done
jq --arg h "$head" --arg r "$root" --arg runner "$runner" --arg spec "$next/specs-v8/k2048.json" --argjson b "$binds" '
 .commit=$h | .binds=$b | .card_id="CARD-native-natural-200k-k2048-v8" | .experiment_id="native-natural-200k-k2048-v8"
 | .bus_address="executor-mechanical-native-natural-200k-k2048-v8"
 | .receipt=($r+"/k2048-v8.receipt.json") | .relay_stdout=($r+"/k2048-v8.relay.json")
 | .commands |= map(.id="k2048-v8" | .argv[0]=$runner | .argv[2]=$spec | .argv[-1]=($r+"/k2048-v8")
    | .stdout=($r+"/k2048-v8.stdout") | .stderr=($r+"/k2048-v8.stderr")
    | .outputs |= map(split("/")[-1] | $r+"/k2048-v8/"+.))
 | .must_not_exist=([.commands[]|.argv[-1],.stdout,.stderr]+[.receipt])
' "$old" > "$card"
git check-ignore "$root/k2048-v8.relay.json" "$root/k2048-v8.receipt.json"
native_path=$(jq -r '.native_path' "$card")
sudo -n env "PATH=$native_path" "$runner" card-prompt "$card" > "$next/cards/k2048-v8.prompt.txt"
test -z "$(git status --porcelain)"
jq -n --arg h "$head" '{role:"experiment-director",mode:"compile-only",commit:$h,cpu_reservation:[14,30],specs_native_freeze:true,root_native_prompt_validated:true,experiments_executed:false}' > "$next/cards/k2048-v8.compile-provenance.json"
sha256sum "$card" "$next/cards/k2048-v8.prompt.txt"
