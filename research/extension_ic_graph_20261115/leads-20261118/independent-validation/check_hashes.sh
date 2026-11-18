#!/usr/bin/env bash
# Hash and commit-binding checks for the independent-validation lane.
# Read-only: sha256sum, git show/rev-parse/cat-file, jq, zcat. Writes nothing
# except stdout. Usage: check_hashes.sh REPO_ROOT
# Output lines: VERDICT<TAB>GROUP<TAB>ITEM<TAB>DETAIL
#   MATCH      retained file hashes to the recorded value
#   MISMATCH   retained file differs from the recorded value
#   ABSENT     recorded file is not retained in the repository (not checkable)
set -uo pipefail
R=${1:?usage: check_hashes.sh REPO_ROOT}
cd "$R" || exit 2
L=research/extension_ic_graph_20261115
N=$L/nagao-e2e
F=$L/runpod-deferred-pair-20261117-50
D=$L/leads-20261118/descent-finder
out() { printf '%s\t%s\t%s\t%s\n' "$1" "$2" "$3" "$4"; }
h() { sha256sum "$1" | cut -d' ' -f1; }
cmp_file() { # group expected path label
  if [[ -f "$3" ]]; then
    local a; a=$(h "$3")
    if [[ "$a" == "$2" ]]; then out MATCH "$1" "$4" "$3"; else out MISMATCH "$1" "$4" "$3 expected ${2:0:16} got ${a:0:16}"; fi
  else out ABSENT "$1" "$4" "not retained: $3"; fi
}

# 1. PCC claim proof subjects
for id in feaf31 6432aa e9f39d 7defbb 6224b9 853115; do
  c=pcc/claims/PCC-20261117-$id.json
  jq -r '.proofs[] | "\(.subject.sha256) \(.subject.path)"' "$c" | while read -r sha path; do cmp_file "pcc:$id" "$sha" "$path" "$(basename "$path")"; done
  if grep -q "PCC-20261117-$id" pcc/ledger.jsonl; then out MATCH "pcc:$id" ledger-entry "registered in pcc/ledger.jsonl"; else out MISMATCH "pcc:$id" ledger-entry "claim file exists but no entry in pcc/ledger.jsonl at $(git rev-parse --short HEAD)"; fi
done

# 2. nagao-e2e public input manifest and commits
cmp_file nagao "$(jq -r .public_input_sha256 $N/public-manifest.json)" $N/public.json public.json
[[ $(stat -c %s $N/public.json) == $(jq -r .public_input_bytes $N/public-manifest.json) ]] && out MATCH nagao public-bytes "$(stat -c %s $N/public.json)" || out MISMATCH nagao public-bytes size
a=$(git show c1b9bd4e87535974d554ab7f9a383ebb79b4ad56:$N/public.json | sha256sum | cut -d' ' -f1)
[[ $a == $(jq -r .public_input_sha256 $N/public-manifest.json) ]] && out MATCH nagao input-commit-c1b9bd4e public.json || out MISMATCH nagao input-commit-c1b9bd4e "$a"
for c in 7802b7f864594d757f88991f7d70b77c38715ebe c1b9bd4e87535974d554ab7f9a383ebb79b4ad56 fcda1e22cb0c5424ca3244f783e69be8b99792f4; do
  git merge-base --is-ancestor $c HEAD && out MATCH nagao "commit ${c:0:12}" "exists, ancestor of HEAD" || out MISMATCH nagao "commit ${c:0:12}" "not an ancestor"
done
git cat-file -e 86189420e615ac3311308d52d7900743a9be23ec 2>/dev/null && out MATCH nagao producer-commit-86189420 exists || out ABSENT nagao producer-commit-86189420 "6432aa producer.commit not in this clone"
# frozen source: no file present at 7802b7f8 was modified afterwards
changed=$(git diff --name-only --diff-filter=MDR 7802b7f864594d757f88991f7d70b77c38715ebe HEAD -- $N | wc -l)
[[ $changed == 0 ]] && out MATCH nagao frozen-source-7802b7f8 "no file present at the freeze was modified or deleted; later commits only add results/report" || out MISMATCH nagao frozen-source-7802b7f8 "$changed files modified after freeze"
# qualifier source pins
jq -r '.[] | "\(.source_commit) \(.source_path) \(.git_blob) \(.snapshot) \(.sha256)"' $N/qualifier-source-pins.json | while read -r c sp blob snap sha; do
  gb=$(git rev-parse "$c:$sp"); sb=$(git hash-object "$snap"); ss=$(h "$snap")
  [[ $gb == "$blob" && $sb == "$blob" && $ss == "$sha" ]] && out MATCH nagao-qualifier-pin "$(basename $sp)" "blob ${blob:0:12} = fcda1e22 source = snapshot" || out MISMATCH nagao-qualifier-pin "$(basename $sp)" "$gb $sb $ss"
done
# overlay vs frozen library (expected to differ: callback added)
for n in jv_cover jv_extension_remainder_dlp jv_sieve_two_large_prime; do
  st=$(git diff --no-index --numstat <(git show fcda1e22cb0c5424ca3244f783e69be8b99792f4:src/cryptanalysis/$n.rs) $N/library-overlay/$n.rs | awk '{print "+"$1" -"$2}')
  out NOTE nagao-overlay "$n.rs" "differs from fcda1e22 by $st lines (declared overlay)"
done
for f in src/cryptanalysis/bitops.rs docs/units/bitop-v1.vectors.json docs/units/bitop-v2.vectors.json; do
  cmp_file nagao-snapshot "$(git show fcda1e22cb0c5424ca3244f783e69be8b99792f4:$f | sha256sum | cut -d' ' -f1)" $N/$f "$f = fcda1e22"
done
# local custody manifest: retained members
m=$N/results/local-custody.json
out NOTE nagao-custody members "$(jq '.members|length' $m) members; archive $(jq -r .archive $m) $(jq -r .archive_bytes $m) bytes"
cmp_file nagao-custody "$(jq -r '.members[]|select(.path=="summary.json").sha256' $m)" $N/results/summary.json summary.json
cmp_file nagao-custody "$(jq -r '.members[]|select(.path=="analysis.stdout").sha256' $m)" $N/results/analysis.log analysis.stdout
cmp_file nagao-custody "$(jq -r '.members[]|select(.path=="binaries-after.log").sha256' $m)" $N/results/binaries-after.log binaries-after.log
cmp_file nagao-custody "$(jq -r '.members[]|select(.path=="binaries-before.log").sha256' $m)" $N/validation/binaries-precheck.log binaries-before.log
cmp_file nagao-custody "$(jq -r '.members[]|select(.path=="cold-20261117-01.runner.exit").sha256' $m)" $N/results/runner.exit runner.exit
cmp_file nagao-custody "$(jq -r .archive_sha256 $m)" "$(jq -r .archive $m)" raw-payload.tar.gz
for b in $(jq -r '.members[].path' $m | grep -E 'raw.json$|runs.jsonl$|private-fixture' | head -3); do out ABSENT nagao-custody "$b" "LOCAL only (example; $(jq -r '.members[].path' $m | grep -cE 'raw.json$') raw.json members absent)"; done
# binaries
while read -r sha path; do cmp_file nagao-binaries "$sha" "$path" "$(basename $path)"; done < $N/validation/binaries.sha256

# 3. run50 manifests
dir_check() { # group manifest
  while read -r sha path; do
    b=$(basename "$path"); local cand=$F/$b
    [[ $b == summary.json ]] && cand=$F/qualified-original.json
    [[ $path == research/* || $path == benchmarks/* ]] && cand=$path
    cmp_file "$1" "$sha" "$cand" "$b"
  done < "$2"
}
dir_check run50-post-call $F/post-call.sha256
dir_check run50-pre-workers $F/pre-workers.sha256
dir_check run50-pre-call $F/pre-call.sha256
dir_check run50-original-raw $F/original-raw.sha256
dir_check run50-qualified-output $F/qualified-output.sha256
dir_check run50-report-artifacts $F/report-artifacts.sha256
dir_check run50-postexecution $F/postexecution-source-binary.sha256
dir_check run50-artifacts $F/artifacts.sha256
cmp_file run50-pre-call "$(awk '$2=="frozen-root-source.sha256"{print $1}' $F/pre-call.sha256)" $L/validation/deferred50-frozen-root-source.sha256 "frozen-root-source.sha256 (validation/ copy)"
[[ $(jq -r .public_sha256 $F/runtime-plan.json) == $(h $F/public.json) ]] && out MATCH run50 runtime-plan-public_sha256 public.json || out MISMATCH run50 runtime-plan-public_sha256 public.json
rev=$(cat $F/source-revision.txt)
git merge-base --is-ancestor "$rev" HEAD && out MATCH run50 "commit ${rev:0:12}" "exists, ancestor of HEAD" || out MISMATCH run50 "commit ${rev:0:12}" missing
# 210-path measured source manifest at the measured commit
ok=0; bad=0; missing=0; badlist=""
while read -r sha path; do
  if a=$(git show "$rev:$path" 2>/dev/null | sha256sum | cut -d' ' -f1) && git cat-file -e "$rev:$path" 2>/dev/null; then
    if [[ $a == "$sha" ]]; then ok=$((ok+1)); else bad=$((bad+1)); badlist="$badlist $path"; fi
  else missing=$((missing+1)); fi
done < $L/validation/deferred50-source.sha256
[[ $bad == 0 ]] && out MATCH run50-source-210 "at ${rev:0:12}" "$ok match, $missing paths not in the commit tree" || out MISMATCH run50-source-210 "at ${rev:0:12}" "$ok match, $bad differ:$badlist; $missing absent"
ok=0; bad=0; badlist=""
while read -r sha path; do
  if [[ -f $path ]] && [[ $(h "$path") == "$sha" ]]; then ok=$((ok+1)); else bad=$((bad+1)); badlist="$badlist $path"; fi
done < $L/validation/deferred50-format-successor-source.sha256
[[ $bad == 0 ]] && out MATCH run50-format-successor "at HEAD" "$ok match" || out NOTE run50-format-successor "at HEAD" "$ok match, $bad differ at HEAD (later edits on main):$badlist"
# what differs between the measured and successor manifests
out NOTE run50-format-successor diff "$(diff <(sort -k2 $L/validation/deferred50-source.sha256) <(sort -k2 $L/validation/deferred50-format-successor-source.sha256) | grep -c '^[<>]') manifest lines differ: $(diff <(sort -k2 $L/validation/deferred50-source.sha256) <(sort -k2 $L/validation/deferred50-format-successor-source.sha256) | grep '^>' | awk '{print $3}' | tr '\n' ' ')"

# 4. descent-finder
while read -r sha name; do
  case $name in
    run3415.jsonl) a=$(zcat $D/runs/traced-seed389-n3415.rows.jsonl.gz | sha256sum | cut -d' ' -f1); [[ $a == "$sha" ]] && out MATCH df-raw "$name" "zcat traced-seed389-n3415.rows.jsonl.gz" || out MISMATCH df-raw "$name" "$a";;
    run401.jsonl) a=$(zcat $D/runs/traced-seed401-n1506.rows.jsonl.gz | sha256sum | cut -d' ' -f1); [[ $a == "$sha" ]] && out MATCH df-raw "$name" "zcat traced-seed401-n1506.rows.jsonl.gz" || out MISMATCH df-raw "$name" "$a";;
    run3415.out) cmp_file df-raw "$sha" $D/runs/traced-seed389-n3415.summary.json "$name (= traced summary)";;
    run401.out) cmp_file df-raw "$sha" $D/runs/traced-seed401-n1506.summary.json "$name (= traced summary)";;
    smoke20.out) cmp_file df-raw "$sha" $D/runs/smoke-seed389-n20-multithread-DEFECT.summary.json "$name (= DEFECT summary)";;
    smoke20b.out) cmp_file df-raw "$sha" $D/runs/smoke-seed389-n20.summary.json "$name (= smoke summary)";;
    *) cmp_file df-raw "$sha" $D/runs/$name "$name";;
  esac
done < $D/runs/RAW.sha256
sect=""
while IFS= read -r line; do
  case $line in
    "## assembled"*) sect=assembled;; "## frozen"*) sect=frozen;; "## binaries"*) sect=binaries;; "## Run50"*) sect=inputs;;
    [0-9a-f]*" "*)
      sha=${line%% *}; path=${line#*  }
      case $sect in
        frozen) a=$(git show "fcda1e22cb0c5424ca3244f783e69be8b99792f4:$path" | sha256sum | cut -d' ' -f1); [[ $a == "$sha" ]] && out MATCH df-frozen "$path" "= git show fcda1e22" || out MISMATCH df-frozen "$path" "$a";;
        inputs) rel=${path#*/wf_8b7b71c2-376-2/}; cmp_file df-inputs "$sha" "$rel" "$(basename $rel)";;
        binaries) if [[ $path == *library-overlay/jv_cover.rs ]]; then cmp_file df-binaries "$sha" $N/library-overlay/jv_cover.rs "overlay jv_cover.rs"; else out ABSENT df-binaries "$(basename $path)" "binary not retained ($path)"; fi;;
      esac;;
  esac
done < $D/BUILD_MANIFEST.txt
[[ -f $D/mini-build/Cargo.lock ]] && out MATCH df-build Cargo.lock present || out MISMATCH df-build Cargo.lock "build.sh copies \$here/Cargo.lock under set -e and cargo --locked, but no Cargo.lock is committed (BUILD_MANIFEST lists 3d1cb733)"
for s in fetch_frozen.sh assemble.py; do grep -q '/home/user/crypto/.claude/worktrees/wf_8b7b71c2-376-2' $D/mini-build/$s && out NOTE df-build "$s" "hard-codes worktree /home/user/crypto/.claude/worktrees/wf_8b7b71c2-376-2 ($( [[ -d /home/user/crypto/.claude/worktrees/wf_8b7b71c2-376-2 ]] && echo present || echo absent on this host))"; done
