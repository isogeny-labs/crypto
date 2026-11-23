#!/bin/bash
set -euo pipefail
repo=/home/ec2-user/crypto-pencil-signed-graph-20261029
packet=research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/quintic-reduction-execution
rev=55b56b1e04
D=$(mktemp -d /var/tmp/quintic-independent-review-XXXXXXXX)
printf '%s\n' "$D" > /var/tmp/quintic-independent-review-path
mkdir "$D/input" "$D/deps" "$D/logs"
git -C "$repo" rev-parse "$rev" > "$D/commit.txt"
while IFS= read -r f; do
 name=${f##*/}
 [[ "$f" = "$packet/$name" ]] || continue
 case "$name" in
 *.lean.txt) git -C "$repo" show "$rev:$f" > "$D/input/${name%.txt}" ;;
 dependencies.txt|check.rs|inventory.txt|REPORT.md) git -C "$repo" show "$rev:$f" > "$D/input/$name" ;;
 esac
done < <(git -C "$repo" ls-tree -r --name-only "$rev" "$packet")
# Reuse helper caches only, after checking their neighboring source matches committed source.
for source in "$D/input/"*.lean; do
 name=${source##*/}
 case "$name" in ReductionStep.lean|ReductionRun.lean|ReductionEntry.lean|FallbackReduced.lean|ReductionExecutionControls.lean|ReductionExecutionReadback.lean|ExpectedFailure.lean) continue;; esac
 old=/var/tmp/pencil-reduction-step-20261101-v1/$name
 cmp "$source" "$old"
 cp "${old%.lean}.olean" "$D/input/"
done
roots="/var/tmp/pencil-quotient-storage-20261101-v1:/var/tmp/pencil-division-quotient-20261101-v1:$(cat "$D/input/dependencies.txt")"
IFS=: read -ra paths <<< "$roots"
newpath="$D/input"
i=0
for p in "${paths[@]}"; do
 dest="$D/deps/$i"; mkdir "$dest"
 find "$p" -maxdepth 1 -type f \( -name '*.lean' -o -name '*.olean' \) -exec cp -p '{}' "$dest/" \;
 printf '%s\t%s\n' "$p" "$dest" >> "$D/dependency-path-map.tsv"
 newpath="$newpath:$dest"; i=$((i+1))
done
printf '%s\n' "$newpath" > "$D/lean-path.txt"
git -C "$repo" show 2bf73d8b6b:src/cryptanalysis/three_leads.rs > "$D/native-three_leads.rs"
find "$D/input" "$D/deps" -type f -print0 | sort -z | xargs -0 sha256sum > "$D/before.sha256"
sha256sum /var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean > "$D/toolchain.sha256"
printf '%s\n' "$D"
