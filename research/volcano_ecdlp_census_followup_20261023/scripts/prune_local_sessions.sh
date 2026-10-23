#!/bin/bash
# usage: prune.sh TARGET_FREE_KB ; removes oldest verified session dirs
cd /home/user/crypto/research/volcano_ecdlp_census_followup_20261023 || exit 1
ECB=/home/user/crypto/target/release/ecbench
SC=/tmp/claude-0/-home-user/a7557d77-b66e-59ae-b32e-04991c48472e/scratchpad/ex
target=$1
for name in $(jq -r 'select(.spec)|select(.status=="done")|.spec' runs/progress.jsonl); do
  [ "$(df -Pk . | awk 'NR==2{print $4}')" -ge "$target" ] && break
  [ -d sessions/$name ] || continue
  a=sessions/$name.tar.xz
  git -C /home/user/crypto ls-files --error-unmatch research/volcano_ecdlp_census_followup_20261023/$a >/dev/null 2>&1 || { echo "skip $name: archive not in git"; continue; }
  [ -z "$(git -C /home/user/crypto diff HEAD --stat -- research/volcano_ecdlp_census_followup_20261023/$a)" ] || { echo "skip $name: archive differs"; continue; }
  git -C /home/user/crypto branch -r --contains $(git -C /home/user/crypto log -1 --format=%H -- research/volcano_ecdlp_census_followup_20261023/$a) 2>/dev/null | grep -q origin || { echo "skip $name: not pushed"; continue; }
  xz -t $a || { echo "skip $name: xz -t"; continue; }
  want=$(jq -r --arg s $name 'select(.spec==$s and .status=="done")|.sha256_tar_xz' runs/progress.jsonl | head -1)
  [ "$(sha256sum $a | cut -d' ' -f1)" = "$want" ] || { echo "skip $name: sha mismatch"; continue; }
  rm -rf $SC; mkdir -p $SC; tar -C $SC -xf $a || { echo "skip $name: extract"; continue; }
  $ECB verify --dir $SC/$name --replay 3 --out $SC/a.json > $SC/v.log 2>&1; ok=$(jq -r .ok $SC/a.json 2>/dev/null)
  rm -rf $SC
  if [ "$ok" = true ]; then rm -rf sessions/$name; echo "removed $name"; jq -nc --arg t "$(date -u +%FT%TZ)" --arg s $name '{type:"cleanup",time:$t,session:$s,condition:"A1.6: archive committed+pushed, xz -t, sha matches progress, extract + verify --replay 3 OK"}' >> runs/progress.jsonl; else echo "skip $name: verify failed"; fi
done
df -h . | tail -1
