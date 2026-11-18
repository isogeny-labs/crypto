#!/usr/bin/env bash
# Freeze integrity and bindings for both genus-4 studies (validator script).
set -u
cd /home/user/crypto
V=62307fead312100cc2e1b133a74bf20ecf70c968
echo "validated commit (HEAD at dispatch): $V; HEAD now: $(git rev-parse HEAD)"
echo "study paths changed between $V and HEAD now: $(git diff --stat $V HEAD -- research/genus4_positive_control_20261116 research/genus4_rho_refs_20261116 research/genus_ladder_bound_r_20261114 benchmarks/records/20261116T1540Z-genus4-positive-control.json benchmarks/records/20261116T1650Z-genus4-rho-refs.json | tail -1)(empty = none)"
echo "working-tree files of the study dirs vs blobs at $V:"; for f in $(git ls-tree -r --name-only $V research/genus4_positive_control_20261116 research/genus4_rho_refs_20261116); do a=$(sha256sum < $f | cut -d" " -f1); b=$(git show $V:$f | sha256sum | cut -d" " -f1); [ "$a" = "$b" ] || echo "  DIFF $f"; done; echo "  (end: $(git ls-tree -r --name-only $V research/genus4_positive_control_20261116 research/genus4_rho_refs_20261116 | wc -l) tracked files compared)"
echo "uncommitted changes (other sessions) in study dirs, src, Cargo.*:"; git status --porcelain -- research/genus4_positive_control_20261116 research/genus4_rho_refs_20261116 src/ Cargo.toml Cargo.lock | sed "s/^/  /"; echo "  (end; builds use git archive of the freeze commits, not the worktree)"
for spec in "bce0eb2148 research/genus4_positive_control_20261116" "a99e7267f1 research/genus4_rho_refs_20261116"; do
  set -- $spec; c=$1; d=$2
  echo "== $d  freeze $c ($(git log -1 --format='%H %ad' --date=iso $c))"
  git show $c:$d/frozen.sha256 | while read h p; do
    [[ $p == /* ]] && continue
    a=$(git show $c:$p | sha256sum | cut -d' ' -f1)
    echo "  frozen $p: $([ $h = $a ] && echo OK || echo MISMATCH $a)"
  done
  for f in PROTOCOL.md frozen.sha256 binary.sha256 test.sh; do
    git diff --quiet $c $V -- $d/$f && echo "  $f unchanged freeze..$V" || echo "  $f CHANGED after freeze"
  done
  echo "  binary.sha256: $(git show $c:$d/binary.sha256)"
  echo "  commits touching $d, analyze.py and data, oldest first:"
  git log --reverse --format="    %h %ad %s" --date=iso $V -- $d | head -20
  echo "  analyze.py changes after first commit: $(git log --format=%h -- $d/analyze.py | wc -l) commit(s)"
done
echo "== study 1 run.jsonl (store-managed, not in git)"
sha256sum research/genus4_positive_control_20261116/run.jsonl
python3 -I -c "import json;print('  manifest:', json.load(open('research/genus4_positive_control_20261116/.artifacts.json'))['files']['run.jsonl'])"
F=research/genus4_positive_control_20261116/run.jsonl
for c in 2838fc9fd5 1c6f9c05ad 543cf4571b 3fad370c98 1723115bda; do n=$(git show $c:$F | wc -c); cmp -s <(git show $c:$F) <(head -c $n $F) && echo "  partial commit $c ($n bytes) is a prefix of the final file" || echo "  partial commit $c NOT a prefix"; done
echo "== study 2 run.jsonl"
sha256sum research/genus4_rho_refs_20261116/run.jsonl
F2=research/genus4_rho_refs_20261116/run.jsonl; n=$(git show dc3c684916:$F2 | wc -c); cmp -s <(git show dc3c684916:$F2) <(head -c $n $F2) && echo "  partial commit dc3c684916 ($n bytes) is a prefix of the final file"
echo "== bench records"
for r in benchmarks/records/20261116T1540Z-genus4-positive-control.json benchmarks/records/20261116T1650Z-genus4-rho-refs.json; do
  python3 -I -c "import json,sys;d=json.load(open('$r'));print('  $r commit',d['commit'],'dirty',d['commit_dirty'],'started',d['commands'][0]['started_utc'],'wall_s',d['commands'][0]['wall_s'],'isolation',d['host']['isolation']);[print('   raw',x['path'],x['sha256'],'uploaded',x['uploaded']) for x in d['raw']]"
done
for pair in "2ca7ece552 bce0eb2148" "1a3f5a3d46 a99e7267f1"; do set -- $pair; echo "  build inputs $1 vs $2 (src, Cargo.*): $(git diff --stat $1 $2 -- src Cargo.toml Cargo.lock | tail -1)"; done
echo "== producer binaries in the shared scratchpad (hashed before any validator build)"
sha256sum /tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.g4 /tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.rhorefs
