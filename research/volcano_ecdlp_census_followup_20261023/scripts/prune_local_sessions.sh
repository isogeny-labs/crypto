#!/bin/bash
# usage: prune_local_sessions.sh --session NAME   (prune that one archived session)
#        prune_local_sessions.sh TARGET_FREE_KB   (prune oldest done sessions until that much is free)
# A1.6 conditions: archive committed and pushed, xz -t, sha256 equals progress line,
# extract + verify --replay 3 OK. Overrides: STUDY ECB SC.
STUDY=${STUDY:-/home/user/crypto/research/volcano_ecdlp_census_followup_20261023}
cd "$STUDY" || exit 1
ECB=${ECB:-/home/user/crypto/target/release/ecbench}
SC=${SC:-/tmp/claude-0/-home-user/a7557d77-b66e-59ae-b32e-04991c48472e/scratchpad/ex}
REPO=$(git rev-parse --show-toplevel); REL=$(realpath --relative-to="$REPO" "$STUDY")
PROG=runs/progress.jsonl
prune_one() {
  local name=$1 a=sessions/$1.tar.xz want ok c
  [ -d sessions/$name ] || return 0
  git -C "$REPO" ls-files --error-unmatch $REL/$a >/dev/null 2>&1 || { echo "skip $name: archive not in git"; return 1; }
  [ -z "$(git -C "$REPO" diff HEAD --stat -- $REL/$a)" ] || { echo "skip $name: archive differs"; return 1; }
  c=$(git -C "$REPO" log -1 --format=%H -- $REL/$a)
  git -C "$REPO" branch -r --contains "$c" 2>/dev/null | grep -q origin || { echo "skip $name: not pushed"; return 1; }
  xz -t $a || { echo "skip $name: xz -t"; return 1; }
  want=$(jq -r --arg s "$name" 'select(.out==("sessions/"+$s) and .status=="done")|.sha256_tar_xz' $PROG | head -1)
  [ -n "$want" ] && [ "$(sha256sum $a | cut -d' ' -f1)" = "$want" ] || { echo "skip $name: sha mismatch"; return 1; }
  rm -rf $SC; mkdir -p $SC; tar -C $SC -xf $a || { echo "skip $name: extract"; rm -rf $SC; return 1; }
  $ECB verify --dir $SC/$name --replay 3 --out $SC/a.json > $SC/v.log 2>&1; ok=$(jq -r .ok $SC/a.json 2>/dev/null)
  rm -rf $SC
  if [ "$ok" = true ]; then
    rm -rf sessions/$name; echo "removed $name"
    jq -nc --arg t "$(date -u +%FT%TZ)" --arg s "$name" '{type:"cleanup",time:$t,session:$s,condition:"A1.6: archive committed+pushed, xz -t, sha matches progress, extract + verify --replay 3 OK"}' >> $PROG
  else echo "skip $name: verify failed"; return 1; fi
}
if [ "${1:-}" = "--session" ]; then prune_one "$2"; rc=$?; df -h . | tail -1; exit $rc; fi
target=$1
for name in $(jq -r 'select(.spec)|select(.status=="done")|.out|sub("^sessions/";"")' $PROG); do
  [ "$(df -Pk . | awk 'NR==2{print $4}')" -ge "$target" ] && break
  prune_one "$name"
done
df -h . | tail -1
