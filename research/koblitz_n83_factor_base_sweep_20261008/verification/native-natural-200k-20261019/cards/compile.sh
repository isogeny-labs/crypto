set -euo pipefail
repo=/home/ec2-user/crypto-n83-resume
prior=/home/ec2-user/scratch/n83-natural-100k-20261019
next=/home/ec2-user/scratch/n83-natural-200k-20261019
runner="$next/bin/runner-v7"
study=research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-200k-20261019
root="$study/exec"
head=$(git rev-parse HEAD)
test -z "$(git status --porcelain)"
mkdir -p "$root"
protocol_sha=$(sha256sum "$study/PROTOCOL.md"); protocol_sha=${protocol_sha%% *}
for k in k2048 k4096; do
  source_spec="$prior/specs-v6c/$k.json"
  panel=$(jq -r '.panel' "$source_spec"); fixtures=$(jq -r '.fixtures' "$source_spec")
  worker=$(jq -r '.worker' "$source_spec"); image=$(jq -r '.image' "$source_spec")
  "$runner" freeze "$repo" "$panel" "$fixtures" "$worker" "$image" "${k#k}" 200000 1200 "$protocol_sha" "$next/specs/$k.freeze.json"
  jq '.isolate=true' "$next/specs/$k.freeze.json" > "$next/specs/$k.json"
done
for k in k2048 k4096; do
  old="$prior/cards/$k-v6c.json"
  card="$next/cards/$k-v7.json"
  binds='[]'
  while IFS= read -r path; do
    path=${path/"$prior/specs-v6c/"/"$next/specs/"}
    path=${path/\/home\/ec2-user\/scratch\/n83-xor-opt-20261019\/bin\/runner-preflight-v4/"$runner"}
    digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
    binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
  done < <(jq -r '.binds[].path' "$old")
  for suffix in PROTOCOL.md .gitignore; do
    path="$repo/$study/$suffix"; digest=$(sha256sum "$path"); digest=${digest%% *}
    binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
  done
  jq --arg h "$head" --arg k "$k" --arg r "$root" --arg runner "$runner" --arg spec "$next/specs/$k.json" --argjson b "$binds" '
    .commit=$h | .binds=$b | .card_id=("CARD-native-natural-200k-"+$k+"-v7")
    | .experiment_id=("native-natural-200k-"+$k+"-v7") | .bus_address=("executor-mechanical-native-natural-200k-"+$k+"-v7")
    | .write_scope=[$r+"/"] | .receipt=($r+"/"+$k+"-v7.receipt.json") | .relay_stdout=($r+"/"+$k+"-v7.relay.json")
    | .commands |= map(.id=($k+"-v7") | .timeout_seconds=1290 | .argv[0]=$runner | .argv[2]=$spec | .argv[-1]=($r+"/"+$k)
       | .stdout=($r+"/"+$k+"-v7.stdout") | .stderr=($r+"/"+$k+"-v7.stderr")
       | .outputs |= map(split("/")[-1] | $r+"/"+$k+"/"+.))
    | .must_not_exist=([.commands[]|.argv[-1],.stdout,.stderr]+[.receipt])
  ' "$old" > "$card"
  git check-ignore "$root/$k-v7.relay.json" "$root/$k-v7.receipt.json"
  native_path=$(jq -r '.native_path' "$card")
  sudo -n env "PATH=$native_path" "$runner" card-prompt "$card" > "$next/cards/$k-v7.prompt.txt"
  sha256sum "$card"
done
test -z "$(git status --porcelain)"
jq -n --arg h "$head" --arg proto "$protocol_sha" '{role:"experiment-director",mode:"compile-only",commit:$h,protocol_sha256:$proto,specs_generated_with_native_freeze:true,isolate:true,json_paths_ignored:true,clean_source:true,native_root_prompt_validated:true,experiments_executed:false}' > "$next/cards/compile-provenance.json"
