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
  --argjson files "$files" --argjson candidates "$candidates" \
  --argjson seqs "$(printf '%s\n' "${seqs[@]}"|jq -s .)" '
  {schema:"ecbench.audit/v1",session_status:"complete",session_id:"metadata-control",
   records_bound:true,auditor_binary_sha256:$binary,files:$files,ok:true,
   checks:{records_hash:true,executions_planned:true,replays:true},problems:[],
   replay_selection:$selection,replay_candidates:$candidates,
   replays:[$seqs[]|{seq:.,record_id:("metadata-"+tostring),reproduced:true,detail:"identical"}]}' > "$out"
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
head -1 research/study/session/records.jsonl | jq '.seq=3|.record_id="metadata-3"' >> research/study/session/records.jsonl
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
echo "partition metadata controls: $controls passed"
