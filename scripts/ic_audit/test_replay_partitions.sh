#!/bin/bash
# Metadata-only controls: the fake auditor never solves or produces research evidence.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
cd "$tmp"
mkdir -p research/study/session
for name in session spec plan host; do printf '{}\n' > "research/study/session/$name.json"; done
for seq in 0 1 2; do
  jq -nc --argjson seq "$seq" '{seq:$seq,record_id:("metadata-"+($seq|tostring)),warmup:false,
    outcome:{status:"verified"},method:{id:"metadata-control",params:{}},
    cost:{deterministic:true,total_gae:1}}'
done > research/study/session/records.jsonl
export REPLAY_PLAN="$tmp/plan.json"
printf '%s\n' '{"schema":"ic-replay-plan/v1","sessions":[{"dir":"research/study/session","replay":"all","expect":"identical"}]}' > "$REPLAY_PLAN"
cat > fake-native <<'STUB'
#!/bin/bash
set -euo pipefail
seqs=(); selection=explicit; candidates=null
while (($#)); do
  case $1 in
    --dir) dir=$2; shift;;
    --out) out=$2; shift;;
    --replay-seq) seqs+=("$2"); shift;;
    --replay-seed) shift;;
    --replay) if [[ $2 != 0 ]]; then selection=sample; candidates=3; seqs=(0); fi; shift;;
  esac
  shift
done
files='{}'
for name in session.json spec.json plan.json host.json records.jsonl; do
  digest=$(sha256sum "$dir/$name"|cut -d' ' -f1)
  files=$(jq -c --arg name "$name" --arg digest "$digest" '.+{($name):$digest}' <<< "$files")
done
jq -n --arg binary "$(sha256sum "$0"|cut -d' ' -f1)" --arg selection "$selection" \
  --arg zero "${METADATA_ZERO:-0}" \
  --arg zero_sid "${METADATA_ZERO_SESSION_ID:-ECBS1h302934ff99ed}" \
  --argjson files "$files" --argjson candidates "$candidates" \
  --argjson seqs "$(printf '%s\n' "${seqs[@]}"|jq -s .)" '
  {schema:"ecbench.audit/v1",session_status:"complete",session_id:"metadata-control",
   records_bound:true,auditor_binary_sha256:$binary,files:$files,ok:true,
   checks:{records_hash:true,executions_planned:true,replays:true},problems:[],
   replay_selection:$selection,replay_candidates:$candidates,
   replays:[$seqs[]|{seq:.,record_id:("metadata-"+tostring),reproduced:true,detail:"identical"}]} |
  if $zero == "1" then
    .session_id=$zero_sid | .records=3 | .planned_executions=3 |
    .verified_records=0 | .regraded=true | .replays=[] | .replay_candidates=0 |
    del(.replay_selection) |
    .checks=(["arms","env_class","executions","executions_planned","host_hash","plan_hash",
      "record_count","records","records_hash","regraded","session_build",
      "session_complete","session_id","spec_id","status_counts","workloads"] |
      map({key:.,value:true})|from_entries)
  else . end' > "$out"
exit "${METADATA_STUB_EXIT:-0}"
STUB
chmod +x fake-native
controls=0
pass() { "$@" > "$tmp/control.log" 2>&1 || { cat "$tmp/control.log"; exit 1; }; controls=$((controls+1)); }
fail() { if "$@" > "$tmp/control.log" 2>&1; then echo "Unexpected acceptance: $*"; exit 1; fi; controls=$((controls+1)); }
for p in 0 1 2 3; do pass bash "$here/replay_partition.sh" ./fake-native "$tmp/out/$p" "$p" 4; done
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/out" 4
cp -a "$tmp/out" "$tmp/good"
for mutation in '.replays=[]' '.replays += .replays' '.replays[0].seq=99' \
  '.replays[0].record_id="wrong"' '.checks.records_hash=false' \
  '.files["records.jsonl"]=("0"*64)' '.auditor_binary_sha256=("0"*64)' \
  '.records_bound=false' '.problems=["replay of seq 0: unrelated failure"]'; do
  jq "$mutation" "$tmp/good/0/0000.receipt.json" > "$tmp/out/0/0000.receipt.json"
  fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/out" 4
done
cp "$tmp/good/0/0000.receipt.json" "$tmp/out/0/0000.receipt.json"
rm "$tmp/out/1/0000.partition.json"
fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/out" 4
cp "$tmp/good/1/0000.partition.json" "$tmp/out/1/0000.partition.json"
printf 'changed\n' >> research/study/session/spec.json
fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/out" 4
printf '{}\n' > research/study/session/spec.json
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/out" 4
jq '.sessions[0].expect="drift"' "$REPLAY_PLAN" > changed-plan.json; mv changed-plan.json "$REPLAY_PLAN"
fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/out" 4
jq '.sessions[0].expect="identical"' "$REPLAY_PLAN" > changed-plan.json; mv changed-plan.json "$REPLAY_PLAN"
for status in 1 7; do
  export METADATA_STUB_EXIT=$status
  fail bash "$here/replay_partition.sh" ./fake-native "$tmp/out/0" 0 4
  test ! -e "$tmp/out/0/0000.partition.json"
done
unset METADATA_STUB_EXIT
# A native-count mismatch must reject the proposed candidate vector.
jq -cs '.[0]|.seq=3|.record_id="metadata-3"' research/study/session/records.jsonl > extra-record.json
cat extra-record.json >> research/study/session/records.jsonl
fail bash "$here/replay_partition.sh" ./fake-native "$tmp/out/0" 0 4
head -3 research/study/session/records.jsonl > records.tmp; mv records.tmp research/study/session/records.jsonl
# Reusing a full receipt is permitted only for the same binary and every input.
mkdir cache
./fake-native verify --dir research/study/session --replay 0 --replay-seq 0 --replay-seq 1 --replay-seq 2 --out cache/all.json
jq '.replay_selection="all"|.replay_candidates=3' cache/all.json > cache/0000.receipt.json
export REPLAY_RECEIPT_CACHE="$tmp/cache"
for p in 0 1 2 3; do pass bash "$here/replay_partition.sh" ./fake-native "$tmp/cached/$p" "$p" 4; done
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/cached" 4
cp cache/0000.receipt.json cache/original.json
for mutation in '.replay_candidates=2' '.auditor_binary_sha256=("0"*64)' '.replays=[]'; do
  jq "$mutation" cache/original.json > cache/0000.receipt.json
  fail bash "$here/replay_partition.sh" ./fake-native "$tmp/cached/0" 0 4
done
jq '.sessions=[]' "$REPLAY_PLAN" > empty-plan.json
REPLAY_PLAN="$tmp/empty-plan.json" fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/cached" 4
unset REPLAY_RECEIPT_CACHE
for p in 0 1 2; do
  REPLAY_PARTITION_CACHE="$tmp/good/$p" METADATA_STUB_EXIT=7 \
    pass bash "$here/replay_partition.sh" ./fake-native "$tmp/resumed/$p" "$p" 4
done
pass bash "$here/replay_partition.sh" ./fake-native "$tmp/resumed/3" 3 4
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/resumed" 4
cp "$tmp/good/0/0000.receipt.json" original-part.json
for mutation in '.auditor_binary_sha256=("0"*64)' '.replays[0].seq=99' '.checks.records_hash=false'; do
  jq "$mutation" original-part.json > "$tmp/good/0/0000.receipt.json"
  REPLAY_PARTITION_CACHE="$tmp/good/0" fail bash "$here/replay_partition.sh" ./fake-native "$tmp/resumed/0" 0 4
done
cp original-part.json "$tmp/good/0/0000.receipt.json"
cp "$tmp/good/0/0000.probe.json" original-probe.json
jq '.replay_candidates=2' original-probe.json > "$tmp/good/0/0000.probe.json"
REPLAY_PARTITION_CACHE="$tmp/good/0" fail bash "$here/replay_partition.sh" ./fake-native "$tmp/resumed/0" 0 4
cp original-probe.json "$tmp/good/0/0000.probe.json"
printf '1\n' > "$tmp/good/0/0000.receipt.json.exit"
REPLAY_PARTITION_CACHE="$tmp/good/0" fail bash "$here/replay_partition.sh" ./fake-native "$tmp/resumed/0" 0 4
printf '0\n' > "$tmp/good/0/0000.receipt.json.exit"
cp "$tmp/good/0/0000.partition.json" original-partition.json
jq '.partition=1' original-partition.json > "$tmp/good/0/0000.partition.json"
REPLAY_PARTITION_CACHE="$tmp/good/0" fail bash "$here/replay_partition.sh" ./fake-native "$tmp/resumed/0" 0 4
cp original-partition.json "$tmp/good/0/0000.partition.json"
printf 'changed\n' >> research/study/session/spec.json
REPLAY_PARTITION_CACHE="$tmp/good/0" fail bash "$here/replay_partition.sh" ./fake-native "$tmp/resumed/0" 0 4
printf '{}\n' > research/study/session/spec.json
REPLAY_PARTITION_CACHE="$tmp/good/0" pass bash "$here/replay_partition.sh" ./fake-native "$tmp/resumed/0" 0 4
printf '\n' >> "$tmp/resumed/0/0000.cache-source.json"
fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/resumed" 4
REPLAY_PARTITION_CACHE="$tmp/good/0" pass bash "$here/replay_partition.sh" ./fake-native "$tmp/resumed/0" 0 4
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/resumed" 4
cp -a research/study/session research/study/session-b
jq '.sessions += .sessions | .sessions[1].dir="research/study/session-b"' "$REPLAY_PLAN" > grouped-plan.json
export REPLAY_PLAN="$tmp/grouped-plan.json" REPLAY_GROUP_COUNT=2
for p in 0 1 2 3; do
  REPLAY_GROUP_INDEX=0 pass bash "$here/replay_partition.sh" ./fake-native "$tmp/grouped/$p" "$p" 4
done
fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/grouped" 4
for p in 0 1 2 3; do
  REPLAY_GROUP_INDEX=1 pass bash "$here/replay_partition.sh" ./fake-native "$tmp/grouped/$p" "$p" 4
done
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/grouped" 4
# A session can checkpoint native receipts without inventing a combined native receipt.
unset REPLAY_GROUP_COUNT
export REPLAY_PLAN="$tmp/plan.json" REPLAY_CHUNK_SIZE=1
pass bash "$here/replay_partition.sh" ./fake-native "$tmp/chunked/0" 0 1
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/chunked" 1
jq -e 'length == 3' "$tmp/chunked/0/0000.chunks.json" >/dev/null
REPLAY_CHUNK_CACHE="$tmp/chunked/0" METADATA_STUB_EXIT=7 \
  pass bash "$here/replay_partition.sh" ./fake-native "$tmp/chunk-resumed/0" 0 1
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/chunk-resumed" 1
cp -a "$tmp/chunked" "$tmp/chunk-good"
for mutation in '.auditor_binary_sha256=("0"*64)' '.replays[0].record_id="wrong"' \
  '.checks.records_hash=false' '.files["records.jsonl"]=("0"*64)'; do
  jq "$mutation" "$tmp/chunk-good/0/0000.chunk0000.json" > "$tmp/chunked/0/0000.chunk0000.json"
  REPLAY_CHUNK_CACHE="$tmp/chunked/0" fail bash "$here/replay_partition.sh" ./fake-native "$tmp/chunk-resumed/0" 0 1
  fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/chunked" 1
done
cp "$tmp/chunk-good/0/0000.chunk0000.json" "$tmp/chunked/0/0000.chunk0000.json"
printf '1\n' > "$tmp/chunked/0/0000.chunk0000.json.exit"
REPLAY_CHUNK_CACHE="$tmp/chunked/0" fail bash "$here/replay_partition.sh" ./fake-native "$tmp/chunk-resumed/0" 0 1
fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/chunked" 1
printf '0\n' > "$tmp/chunked/0/0000.chunk0000.json.exit"
# Rebind intentionally malformed manifests, so these exercise coverage/name checks.
for mutation in '.[0].file="../borrowed.json"' '. += [.[0]]' '.[1:]'; do
  jq "$mutation" "$tmp/chunk-good/0/0000.chunks.json" > "$tmp/chunked/0/0000.chunks.json"
  jq --arg digest "$(sha256sum "$tmp/chunked/0/0000.chunks.json" | cut -d' ' -f1)" \
    '.chunks_sha256=$digest' "$tmp/chunk-good/0/0000.partition.json" > "$tmp/chunked/0/0000.partition.json"
  fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/chunked" 1
done
cp "$tmp/chunk-good/0/0000.chunks.json" "$tmp/chunked/0/0000.chunks.json"
cp "$tmp/chunk-good/0/0000.partition.json" "$tmp/chunked/0/0000.partition.json"
rm "$tmp/chunked/0/0000.chunk0002.json"
fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/chunked" 1
# An interrupted session has no final binding; its completed native chunks survive.
rm "$tmp/chunked/0/0000.partition.json" "$tmp/chunked/0/0000.chunks.json"
rm "$tmp/chunked/0/0000.chunk0002.json.exit"
REPLAY_CHUNK_CACHE="$tmp/chunked/0" pass bash "$here/replay_partition.sh" ./fake-native "$tmp/chunk-partial/0" 0 1
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/chunk-partial" 1
printf '137\n' > "$tmp/chunked/0/0000.chunk0002.json.exit"
REPLAY_CHUNK_CACHE="$tmp/chunked/0" pass bash "$here/replay_partition.sh" ./fake-native "$tmp/chunk-after-kill/0" 0 1
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/chunk-after-kill" 1
jq '.replay_candidates=2' "$tmp/chunk-good/0/0000.probe.json" > "$tmp/chunked/0/0000.probe.json"
REPLAY_CHUNK_CACHE="$tmp/chunked/0" fail bash "$here/replay_partition.sh" ./fake-native "$tmp/chunk-resumed/0" 0 1
cp "$tmp/chunk-good/0/0000.probe.json" "$tmp/chunked/0/0000.probe.json"
printf 'changed\n' >> research/study/session/spec.json
REPLAY_CHUNK_CACHE="$tmp/chunked/0" fail bash "$here/replay_partition.sh" ./fake-native "$tmp/chunk-resumed/0" 0 1
printf '{}\n' > research/study/session/spec.json
REPLAY_CHUNK_SIZE=invalid fail bash "$here/replay_partition.sh" ./fake-native "$tmp/chunk-resumed/0" 0 1
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/good" 4
for kind in probe receipt; do
  printf '7\n' > "$tmp/good/0/0000.$kind.json.exit"
  fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/good" 4
  printf '0\n' > "$tmp/good/0/0000.$kind.json.exit"
  pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/good" 4
done
# A precise historical counter drift cannot waive new scalar or coverage failures.
cp -a "$tmp/good" "$tmp/pinned"
jq '.require_pinned_drift=true | .sessions[0].expect="drift" |
  .sessions[0].expected_drift=[{seq:0,record_id:"metadata-0",detail:"differs in phase counts"}]' \
  "$REPLAY_PLAN" > "$tmp/pinned-plan.json"
export REPLAY_PLAN="$tmp/pinned-plan.json"
drift_receipt() {
  jq --arg detail "$2" '.ok=false | .checks.replays=false |
    .replays[0].reproduced=false | .replays[0].detail=$detail |
    .problems=["replay of seq \(.replays[0].seq): \($detail)"]' \
    "$tmp/good/$1/0000.receipt.json" > "$tmp/pinned/$1/0000.receipt.json"
  printf '1\n' > "$tmp/pinned/$1/0000.receipt.json.exit"
}
drift_receipt 0 'differs in phase counts'
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/pinned" 4
drift_receipt 0 'wrong recovered scalar'
fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/pinned" 4
drift_receipt 0 'differs in phase counts'
drift_receipt 1 'differs in phase counts'
fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/pinned" 4
cp "$tmp/good/0/0000.receipt.json" "$tmp/pinned/0/0000.receipt.json"
cp "$tmp/good/0/0000.receipt.json.exit" "$tmp/pinned/0/0000.receipt.json.exit"
fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/pinned" 4
cp "$tmp/good/1/0000.receipt.json" "$tmp/pinned/1/0000.receipt.json"
cp "$tmp/good/1/0000.receipt.json.exit" "$tmp/pinned/1/0000.receipt.json.exit"
fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/pinned" 4
drift_receipt 0 'differs in phase counts'
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/pinned" 4
cp "$REPLAY_PLAN" "$tmp/good-pinned-plan.json"
for mutation in 'del(.sessions[0].expected_drift)' '.sessions[0].expected_drift=[]' \
  '.sessions[0].expected_drift += .sessions[0].expected_drift' \
  '.sessions[0].expected_drift[0].detail=""'; do
  jq "$mutation" "$tmp/good-pinned-plan.json" > "$REPLAY_PLAN"
  fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/pinned" 4
done
# Explicitly synthetic zero-candidate controls; never research evidence.
unset REPLAY_CHUNK_SIZE
export REPLAY_PLAN="$tmp/zero-plan.json" METADATA_ZERO=1
printf '%s\n' '{"schema":"ic-replay-plan/v1","sessions":[{"dir":"research/study/session","replay":"all","expect":"identical"}]}' > "$REPLAY_PLAN"
cp research/study/session/records.jsonl positive-records.jsonl
jq -c '.session_id="ECBS1h302934ff99ed" | .outcome.status="exhausted"' \
  positive-records.jsonl > research/study/session/records.jsonl
fail bash "$here/replay_partition.sh" ./fake-native "$tmp/unmarked-zero/0" 0 4
./fake-native verify --dir research/study/session --replay 1 --out "$tmp/zero-probe.json"
jq --slurpfile probe "$tmp/zero-probe.json" '.sessions[0].zero_replay_probe =
  ($probe[0] | {schema:"ic-static-zero-replay/v1",session_id,records,planned_executions,
    verified_records,files,reason:"Synthetic metadata control; not a native research receipt"})' \
  "$REPLAY_PLAN" > zero-plan.tmp
mv zero-plan.tmp "$REPLAY_PLAN"
for p in 0 1 2 3; do pass bash "$here/replay_partition.sh" ./fake-native "$tmp/zero/$p" "$p" 4; done
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/zero" 4
pass bash "$here/replay_plan.sh" ./fake-native "$tmp/zero-serial"
METADATA_STUB_EXIT=1 fail bash "$here/replay_plan.sh" ./fake-native "$tmp/zero-serial"
cp "$REPLAY_PLAN" zero-good-plan.json
cp "$tmp/zero/0/0000.probe.json" zero-good-probe.json
for mutation in '.replay_candidates=1' '.replays=[{seq:0,record_id:"metadata-0",reproduced:true,detail:"identical"}]' \
  '.session_id="unknown"' '.records=4' '.planned_executions=4' '.verified_records=1' \
  '.records_bound=false' '.regraded=false' '.session_status="interrupted"' \
  '.auditor_binary_sha256=("0"*64)' '.files["records.jsonl"]=("0"*64)' \
  '.ok=false' '.problems=["failure"]' '.checks.records=false' '.checks.replays=false' \
  'del(.checks.status_counts)' 'del(.checks.records_hash)' '.replay_selection="all"'; do
  jq "$mutation" zero-good-probe.json > "$tmp/zero/0/0000.probe.json"
  fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/zero" 4
done
cp zero-good-probe.json "$tmp/zero/0/0000.probe.json"
printf '1\n' > "$tmp/zero/0/0000.probe.json.exit"
fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/zero" 4
printf '0\n' > "$tmp/zero/0/0000.probe.json.exit"
for mutation in 'del(.sessions[0].zero_replay_probe)' '.sessions[0].expect="drift"' \
  '.sessions[0].zero_replay_probe.session_id="unknown"' \
  '.sessions[0].zero_replay_probe.files["spec.json"]=("0"*64)' \
  '.sessions[0].zero_replay_probe.verified_records=1'; do
  jq "$mutation" zero-good-plan.json > "$REPLAY_PLAN"
  fail bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/zero" 4
  fail bash "$here/replay_plan.sh" ./fake-native "$tmp/zero-serial"
done
cp zero-good-plan.json "$REPLAY_PLAN"
METADATA_ZERO=0 fail bash "$here/replay_partition.sh" ./fake-native "$tmp/zero-positive-native/0" 0 4
METADATA_ZERO=0 fail bash "$here/replay_plan.sh" ./fake-native "$tmp/zero-serial"
printf 'changed\n' >> research/study/session/spec.json
fail bash "$here/replay_plan.sh" ./fake-native "$tmp/zero-serial"
printf '{}\n' > research/study/session/spec.json
cp positive-records.jsonl research/study/session/records.jsonl
fail bash "$here/replay_partition.sh" ./fake-native "$tmp/zero-positive-proposal/0" 0 4
jq 'del(.sessions[0].zero_replay_probe)' zero-good-plan.json > "$REPLAY_PLAN"
fail bash "$here/replay_partition.sh" ./fake-native "$tmp/positive-proposal-zero-native/0" 0 4
# X19 is also static-only under the existing registry: host timeouts, not
# newly exempted exhausted outcomes. Its exact ID still needs an explicit pin.
export METADATA_ZERO_SESSION_ID=ECBS1h9453f8e9b41e
jq -c --arg sid "$METADATA_ZERO_SESSION_ID" '.session_id=$sid |
  .outcome.status="timeout" | .cost.deterministic=false' positive-records.jsonl \
  > research/study/session/records.jsonl
fail bash "$here/replay_partition.sh" ./fake-native "$tmp/unmarked-timeout/0" 0 4
./fake-native verify --dir research/study/session --replay 1 --out timeout-probe.json
jq --slurpfile probe timeout-probe.json '.sessions[0].zero_replay_probe =
  ($probe[0] | {schema:"ic-static-zero-replay/v1",session_id,records,planned_executions,
    verified_records,files,reason:"Synthetic timeout metadata control; not research evidence"})' \
  "$REPLAY_PLAN" > timeout-plan.json
export REPLAY_PLAN="$tmp/timeout-plan.json"
for p in 0 1 2 3; do pass bash "$here/replay_partition.sh" ./fake-native "$tmp/timeout/$p" "$p" 4; done
pass bash "$here/check_replay_partitions.sh" ./fake-native "$tmp/timeout" 4
pass bash "$here/replay_plan.sh" ./fake-native "$tmp/timeout-serial"
echo "partition metadata controls: $controls passed"
