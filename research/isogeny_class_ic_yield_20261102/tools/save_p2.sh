#!/bin/bash
# save_p2.sh <cell-tag e.g. c28A-m3>: copy a finished P2 card's receipt + run dir from the run worktree into main, gzip big files, commit p2 paths only, push.
set -e
tag=$1; dir=$(echo $tag | sed 's/-m/_m/')
W=/home/user/crypto-p2run/research/isogeny_class_ic_yield_20261102; S=research/isogeny_class_ic_yield_20261102
cd /home/user/crypto
cp -p $W/cards/CARD-isogeny-ic-yield-p2-$tag-20261102-3.{receipt.json,run.stdout,run.stderr} $S/cards/
rm -rf $S/p2_prime_analogues/runs/$dir; cp -rp $W/p2_prime_analogues/runs/$dir $S/p2_prime_analogues/runs/
find $S/p2_prime_analogues/runs/$dir -type f -size +1M ! -name '*.gz' -exec gzip -9 {} \;
export AWS_SECRET_ACCESS_KEY=$AWS_SECRET_KEY PATH=/tmp/claude-0/-home-user/fafe457a-a14a-53c5-a937-e53817fc07eb/scratchpad/venv/bin:$PATH
python3 tools/artifacts/artifacts.py push $S/p2_prime_analogues/runs/$dir $S/cards >/dev/null
git add -A $S/p2_prime_analogues/runs/$dir; git add $S/cards/CARD-isogeny-ic-yield-p2-$tag-20261102-3.* $S/p2_prime_analogues/runs/$dir
git commit -q --only $S/cards/CARD-isogeny-ic-yield-p2-$tag-20261102-3.receipt.json $S/cards/CARD-isogeny-ic-yield-p2-$tag-20261102-3.run.stdout $S/cards/CARD-isogeny-ic-yield-p2-$tag-20261102-3.run.stderr $S/p2_prime_analogues/runs/$dir -m "research: P2 cell $tag run outputs (card -3 receipt, session gzipped, audit, analysis)

Copied from the pinned run worktree (b2d892912).

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_01A52ATzyLRNCyniLcNxXnF5"
git push -q origin claude/isogeny-class-ic-yield-20261102 2>&1 | grep -v remote || true
git log --oneline -1
for f in $W/p2_prime_analogues/runs/$dir/session/records.jsonl $W/p2_prime_analogues/runs/$dir/session/plan.json; do [ -f $f ] && gzip -9 $f; done
