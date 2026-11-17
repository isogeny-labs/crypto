#!/bin/bash
# save_p3.sh <tag>...: copy finished P3 cards' receipts/logs, mirror full/ to completed-card outputs (sync_p3.py),
# push data to the artifact store, commit p3 paths, push. Orchestration only.
set -e
SP=/tmp/claude-0/-home-user/fafe457a-a14a-53c5-a937-e53817fc07eb/scratchpad
W=/home/user/crypto-p3run/research/isogeny_class_ic_yield_20261102; S=research/isogeny_class_ic_yield_20261102; F=$S/p3_ecc2k130/full
cd /home/user/crypto
export AWS_SECRET_ACCESS_KEY=$AWS_SECRET_KEY PATH=$SP/venv/bin:$PATH
for tag in "$@"; do for f in $W/cards/CARD-isogeny-ic-yield-p3-$tag-20261102-*.{receipt.json,launch.log,*stdout,*stderr,assessment.yaml}; do [ -f "$f" ] && cp -p "$f" $S/cards/; done; done
python3 $SP/sync_p3.py
[ -s $SP/sync_p3.big ] && python3 tools/artifacts/artifacts.py push $(cat $SP/sync_p3.big) >/dev/null 2>&1 || true
git add $(cat $SP/sync_p3.small) $F/.artifacts.json $F/.gitignore; git add -A $F
for tag in "$@"; do git add $S/cards/CARD-isogeny-ic-yield-p3-$tag-20261102-* 2>/dev/null || true; done
git commit -q -m "research: P3 run outputs for cards: $*

Copied from the pinned run worktree (d97f5953b2); full/ mirrors completed-card receipt outputs; data in the artifact store.

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_01A52ATzyLRNCyniLcNxXnF5" || true
git push -q origin claude/isogeny-class-ic-yield-20261102 2>&1 | grep -v remote || true
python3 tools/artifacts/artifacts.py check origin/main HEAD 2>&1 | tail -1
git log --oneline -1
