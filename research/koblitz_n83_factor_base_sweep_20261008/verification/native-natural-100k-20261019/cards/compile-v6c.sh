set -euo pipefail
repo=/home/ec2-user/crypto-n83-resume
next=/home/ec2-user/scratch/n83-natural-100k-20261019
runner=/home/ec2-user/scratch/n83-xor-opt-20261019/bin/runner-preflight-v4
study=research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-100k-20261019
root="$study/exec"
head=$(git rev-parse HEAD)
test -z "$(git status --porcelain)"
mkdir -p "$root" "$next/specs-v6c"
for k in k2048 k4096; do
  jq --arg h "$head" '.source_commit=$h' "$next/specs/$k.json" > "$next/specs-v6c/$k.json"
  card="$next/cards/$k-v6c.json"
  binds='[]'
  while IFS= read -r path; do
    path=${path/"$next/specs/"/"$next/specs-v6c/"}
    digest=$(sudo -n sha256sum "$path"); digest=${digest%% *}
    binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
  done < <(jq -r '.binds[].path' "$next/cards/native-natural-100k-v6b.json")
  for suffix in LAUNCH-SETUP-V6C.md .gitignore; do
    path="$repo/$study/$suffix"
    digest=$(sha256sum "$path"); digest=${digest%% *}
    binds=$(jq --arg p "$path" --arg h "$digest" '.+[{path:$p,sha256:$h}]' <<< "$binds")
  done
  jq --arg h "$head" --arg k "$k" --arg r "$root" --arg spec "$next/specs-v6c/$k.json" --argjson b "$binds" '
    .commit=$h | .binds=$b | .card_id=("CARD-native-natural-100k-"+$k+"-v6c")
    | .experiment_id=("native-natural-100k-"+$k+"-v6c")
    | .bus_address=("executor-mechanical-native-natural-100k-"+$k+"-v6c")
    | .receipt=($r+"/"+$k+"-v6c.receipt.json") | .relay_stdout=($r+"/"+$k+"-v6c.relay.json")
    | .commands |= map(select(.id==$k) | .id=($k+"-v6c") | .argv[2]=$spec | .argv[-1]=($r+"/"+$k+"-v6c")
       | .stdout=($r+"/"+$k+"-v6c.stdout") | .stderr=($r+"/"+$k+"-v6c.stderr")
       | .outputs |= map(split("/")[-1] | $r+"/"+$k+"-v6c/"+.))
    | .must_not_exist=([.commands[]|.argv[-1],.stdout,.stderr]+[.receipt])
  ' "$next/cards/native-natural-100k-v6b.json" > "$card"
  git check-ignore "$root/$k-v6c.relay.json" "$root/$k-v6c.receipt.json"
  native_path=$(jq -r '.native_path' "$card")
  sudo -n env "PATH=$native_path" "$runner" card-prompt "$card" > "$next/cards/$k-v6c.prompt.txt"
  sha256sum "$card"
done
test -z "$(git status --porcelain)"
