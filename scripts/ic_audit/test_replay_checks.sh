#!/bin/bash
# Metadata controls only: the stub never executes a solver or fabricates research evidence.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
coverage="$root/scripts/ic_audit/check_plan_coverage.sh"
replay="$root/scripts/ic_audit/replay_plan.sh"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
git -C "$tmp" init -q
cd "$tmp"
mkdir -p research/study/sessions/a
export REPLAY_PLAN="$tmp/plan.json" IC_BOARD="$tmp/board.json"
printf '%s\n' '{"sources":{"study":{"path":"research/study/REPORT.md"}}}' > "$IC_BOARD"
printf '%s\n' '{"status":"complete"}' > research/study/sessions/a/session.json
printf '%s\n' '{"seq":2,"outcome":{"stderr_sha256":"metadata-control"}}' > research/study/sessions/a/records.jsonl
git add research/study/sessions/a/session.json
printf '%s\n' '{"schema":"ic-replay-plan/v1","sessions":[{"dir":"research/study/sessions/a","replay":"all","expect":"identical"}]}' > "$REPLAY_PLAN"
cp "$REPLAY_PLAN" "$tmp/original-plan.json"
controls=0
must_pass() {
  "$@" > "$tmp/control.log" 2>&1 || { cat "$tmp/control.log"; echo "FAIL: expected success: $*"; exit 1; }
  controls=$((controls + 1))
}
must_fail() {
  if "$@" > "$tmp/control.log" 2>&1; then echo "FAIL: expected rejection: $*"; exit 1; fi
  controls=$((controls + 1))
}
must_pass bash "$coverage"
jq '.sources.study.path="research/study/analysis/summary.json"' "$IC_BOARD" > "$tmp/nested-board.json"
mv "$tmp/nested-board.json" "$IC_BOARD"
must_pass bash "$coverage"
bash "$coverage" --list-inputs > "$tmp/inputs"
for name in session.json spec.json host.json plan.json records.jsonl exec/000002.stderr; do
  grep -Fx "research/study/sessions/a/$name" "$tmp/inputs" > /dev/null
done
controls=$((controls + 1))
mv research/study/sessions/a/records.jsonl "$tmp/records"
must_fail bash "$coverage"
mv "$tmp/records" research/study/sessions/a/records.jsonl
mv research/study/sessions/a/session.json "$tmp/session"
must_fail bash "$coverage"
mv "$tmp/session" research/study/sessions/a/session.json
jq '.sessions=[]' "$tmp/original-plan.json" > "$REPLAY_PLAN"
must_fail bash "$coverage"
cp "$tmp/original-plan.json" "$REPLAY_PLAN"
printf '%s\n' '{"status":"running"}' > research/study/sessions/a/session.json
must_pass bash "$coverage"
printf '%s\n' '{"status":"complete"}' > research/study/sessions/a/session.json
mkdir -p research/study/sessions/migrated
printf '%s\n' '{"schema":"artifacts.manifest/v1","files":{"session.json":{"sha256":"metadata-control","bytes":1}}}' > research/study/sessions/migrated/.artifacts.json
git add research/study/sessions/migrated/.artifacts.json
bash "$coverage" --list-inputs > "$tmp/inputs"
grep -Fx research/study/sessions/migrated/session.json "$tmp/inputs" > /dev/null
controls=$((controls + 1))
must_fail bash "$coverage"
printf 'invalid json\n' > research/study/sessions/migrated/.artifacts.json
must_fail bash "$coverage" --list-inputs

cat > "$tmp/ecbench-stub" <<'STUB'
#!/bin/bash
set -eu
while [ "$#" -gt 0 ]; do
  if [ "$1" = --out ]; then
    [ "${REPLAY_STUB_NO_RECEIPT:-0}" = 1 ] || cp "$REPLAY_STUB_RECEIPT" "$2"
    break
  fi
  shift
done
exit "${REPLAY_STUB_EXIT:-0}"
STUB
chmod +x "$tmp/ecbench-stub"
export REPLAY_STUB_RECEIPT="$tmp/receipt.json" REPLAY_STUB_EXIT=0
printf '%s\n' '{"schema":"ecbench.audit/v1","session_status":"complete","records_bound":true,"checks":{"records_hash":true,"replays":true},"problems":[],"replays":[{"reproduced":true}]}' > "$tmp/identical.json"
jq '.replays[0].reproduced=false | .checks.replays=false | .problems=["replay of seq 2: counters differ"]' "$tmp/identical.json" > "$tmp/drift.json"
run_replay() { bash "$replay" "$tmp/ecbench-stub" "$tmp/out"; }
cp "$tmp/identical.json" "$REPLAY_STUB_RECEIPT"
must_pass run_replay
cp "$tmp/drift.json" "$REPLAY_STUB_RECEIPT"
export REPLAY_STUB_EXIT=1
must_fail run_replay
jq '.sessions[0].expect="drift"' "$tmp/original-plan.json" > "$REPLAY_PLAN"
must_pass run_replay
cp "$tmp/identical.json" "$REPLAY_STUB_RECEIPT"
export REPLAY_STUB_EXIT=0
must_fail run_replay
for mutation in '.replays=[]' 'del(.replays)' '.replays[0].reproduced="false"' '.checks.records_hash=false' '.problems=["record hash differs"]' '.records_bound=false' '.session_status="interrupted"'; do
  jq "$mutation" "$tmp/drift.json" > "$REPLAY_STUB_RECEIPT"
  export REPLAY_STUB_EXIT=1
  must_fail run_replay
done
cp "$tmp/drift.json" "$REPLAY_STUB_RECEIPT"
export REPLAY_STUB_EXIT=7
must_fail run_replay
cp "$tmp/original-plan.json" "$REPLAY_PLAN"
cp "$tmp/identical.json" "$REPLAY_STUB_RECEIPT"
export REPLAY_STUB_EXIT=1
must_fail run_replay
export REPLAY_STUB_EXIT=0
must_pass run_replay
export REPLAY_STUB_NO_RECEIPT=1
# The previous successful receipt in the same output directory must not survive.
must_fail run_replay
unset REPLAY_STUB_NO_RECEIPT
printf 'invalid json\n' > "$REPLAY_STUB_RECEIPT"
must_fail run_replay
for mutation in '.sessions=[]' '.sessions += .sessions' '.sessions[0].replay=0' '.sessions[0].dir="../outside"'; do
  jq "$mutation" "$tmp/original-plan.json" > "$REPLAY_PLAN"
  must_fail run_replay
done
echo "replay metadata controls: $controls passed"
