#!/usr/bin/env bash
# Offline tests for the Delphi tool wrappers. No network, no credentials.
# shellcheck disable=SC2016 # bash -c bodies are intentionally single-quoted
set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
real_root=$(cd "$here/../.." && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
fails=0

t() {
  local name=$1
  shift
  if "$@" >/dev/null 2>&1; then echo "PASS: $name"; else echo "FAIL: $name"; fails=$((fails + 1)); fi
}
skip() { echo "SKIP: $1"; }
one_verdict() { printf '%s' "$1" | jq -e -s 'length == 1 and (.[0].schema == "pcc.verdict/v1")'; }
verdict_is() { printf '%s' "$1" | jq -e --arg v "$2" '.verdict == $v'; }

export DELPHI_TOOLS_SKIP_GATE=1

# ---- card-lint -------------------------------------------------------------
card=""
for c in "$real_root"/research/*/cards/*.yaml; do
  [ -f "$c" ] || continue
  rel=${c#"$real_root"/}
  if (cd "$real_root" && timeout 60 python3 tools/task_card.py lint "$rel") >/dev/null 2>&1; then
    card=$rel
    break
  fi
done
if [ -n "$card" ]; then
  rc=0; out=$(DELPHI_TOOLS_ROOT=$real_root DELPHI_CLAIM_ID=C7 "$here/card-lint" "$card" 2>/dev/null) || rc=$?
  t "card-lint pass: exit 0" test "$rc" -eq 0
  t "card-lint pass: one verdict object" one_verdict "$out"
  t "card-lint pass: verdict pass, claim_id" bash -c 'printf "%s" "$1" | jq -e ".verdict == \"pass\" and .claim_id == \"C7\""' _ "$out"
else
  skip "card-lint pass (no committed card lints here)"
fi
rc=0; out=$(DELPHI_TOOLS_ROOT=$real_root "$here/card-lint" research/does/not/exist.yaml 2>/dev/null) || rc=$?
t "card-lint fail: exit 0" test "$rc" -eq 0
t "card-lint fail: one verdict object" one_verdict "$out"
t "card-lint fail: verdict fail" verdict_is "$out" fail
rc=0; out=$("$here/card-lint" '../etc/passwd' 2>/dev/null) || rc=$?
t "card-lint bad arg: fail verdict, exit 0" bash -c '[ "$1" -eq 0 ] && printf "%s" "$2" | jq -e ".verdict == \"fail\""' _ "$rc" "$out"

# ---- lean-verify with a fake gate -----------------------------------------
fake=$tmp/fake
mkdir -p "$fake/tools/lean"
cat >"$fake/tools/lean/proof-gate.sh" <<'GATE'
#!/usr/bin/env bash
out=""
while [ $# -gt 0 ]; do
  case "$1" in --out) out=$2; shift ;; esac
  shift
done
mkdir -p "$out/t"
echo "build log for receipt" >"$out/t/build.log"
echo PASS >"$out/t/status"
echo "gate says line1"; echo "gate says line2"
exit "${FAKE_GATE_RC:-0}"
GATE
chmod 755 "$fake/tools/lean/proof-gate.sh"
for rc_in in 0 1; do
  rc=0; out=$(FAKE_GATE_RC=$rc_in DELPHI_TOOLS_ROOT=$fake "$here/lean-verify" some/Target 2>/dev/null) || rc=$?
  want=pass; [ "$rc_in" -eq 1 ] && want=fail
  t "lean-verify gate $rc_in: exit 0" test "$rc" -eq 0
  t "lean-verify gate $rc_in: one verdict object" one_verdict "$out"
  t "lean-verify gate $rc_in: verdict $want" verdict_is "$out" "$want"
  t "lean-verify gate $rc_in: artifact_sha256 is 64 hex" bash -c 'printf "%s" "$1" | jq -e ".artifact_sha256 | test(\"^[0-9a-f]{64}$\")"' _ "$out"
  t "lean-verify gate $rc_in: reason has log tail" bash -c 'printf "%s" "$1" | jq -e ".reason | contains(\"gate says line2\")"' _ "$out"
done
rc=0; out=$(FAKE_GATE_RC=3 DELPHI_TOOLS_ROOT=$fake "$here/lean-verify" some/Target 2>/dev/null) || rc=$?
t "lean-verify gate 3: exit 3" test "$rc" -eq 3
t "lean-verify gate 3: empty stdout" test -z "$out"
rc=0; out=$(DELPHI_TOOLS_ROOT=$fake "$here/lean-verify" --list 2>/dev/null) || rc=$?
t "lean-verify rejects option-like arg" bash -c '[ "$1" -eq 3 ] && [ -z "$2" ]' _ "$rc" "$out"

# ---- isolab-dispatch / isolab-status with a stub gh -----------------------
stub=$tmp/stub
mkdir -p "$stub"
cat >"$stub/gh" <<'GH'
#!/usr/bin/env bash
echo "$*" >>"$GH_LOG"
case "$1 $2" in
  "workflow run") exit 0 ;;
  "run list") cat "$GH_LIST" ;;
  "run view") [ -z "${GH_FAIL:-}" ] || exit 1; cat "$GH_VIEW" ;;
  *) exit 1 ;;
esac
GH
chmod 755 "$stub/gh"
export GH_LOG=$tmp/gh.log GH_LIST=$tmp/list.json GH_VIEW=$tmp/view.json
export DELPHI_TOOLS_POLL_SLEEP=0
sha=0123456789abcdef0123456789abcdef01234567
echo '[{"databaseId":111,"headSha":"aa","createdAt":"2000-01-01T00:00:00Z","url":"https://x/old"},{"databaseId":222,"headSha":"bb","createdAt":"2099-01-01T00:00:00Z","url":"https://x/new"}]' >"$GH_LIST"
rc=0; out=$(PATH="$stub:$PATH" DELPHI_TOOLS_ROOT=$fake "$here/isolab-dispatch" "$sha,spec_path=taskq/a.json,seconds=1800" 2>/dev/null) || rc=$?
t "isolab-dispatch: exit 0" test "$rc" -eq 0
t "isolab-dispatch: run_id 222" bash -c 'printf "%s" "$1" | jq -e ".run_id == 222 and .inputs.seconds == \"1800\""' _ "$out"
t "isolab-dispatch: record file written" jq -e '.run_id == 222' "$fake/.delphi-tools/isolab/222.json"
t "isolab-dispatch: gh argv" grep -q -- "workflow run isolab-experiment.yml --ref main -f source_revision=$sha -f spec_path=taskq/a.json -f seconds=1800" "$GH_LOG"
rc=0; PATH="$stub:$PATH" DELPHI_TOOLS_ROOT=$fake "$here/isolab-dispatch" "$sha,bogus=1" >/dev/null 2>&1 || rc=$?
t "isolab-dispatch: bad key exits 2" test "$rc" -eq 2
echo '[]' >"$GH_LIST"
rc=0; out=$(PATH="$stub:$PATH" DELPHI_TOOLS_ROOT=$fake "$here/isolab-dispatch" "$sha" 2>/dev/null) || rc=$?
t "isolab-dispatch: run not found -> run_id null, exit 0" bash -c '[ "$1" -eq 0 ] && printf "%s" "$2" | jq -e ".dispatched == true and .run_id == null"' _ "$rc" "$out"

status_case() { # name status conclusion want
  echo "{\"status\":\"$2\",\"conclusion\":\"$3\",\"url\":\"https://x/r/222\",\"headSha\":\"bb\",\"displayTitle\":\"t\"}" >"$GH_VIEW"
  local rc=0 out
  out=$(PATH="$stub:$PATH" DELPHI_TOOLS_ROOT=$fake "$here/isolab-status" 222 2>/dev/null) || rc=$?
  t "isolab-status $1: exit 0, one verdict" bash -c '[ "$1" -eq 0 ]' _ "$rc"
  t "isolab-status $1: one object" one_verdict "$out"
  t "isolab-status $1: verdict $4" verdict_is "$out" "$4"
  t "isolab-status $1: reason has url and sha" bash -c 'printf "%s" "$1" | jq -e ".reason | contains(\"https://x/r/222\") and contains(\"bb\")"' _ "$out"
}
status_case running in_progress "" not_covered
status_case success completed success pass
status_case failure completed failure fail
t "isolab-status: reason has source_revision" bash -c 'PATH="$1:$PATH" DELPHI_TOOLS_ROOT="$2" "$3" 222 2>/dev/null | jq -e ".reason | contains(\"source_revision '"$sha"'\")"' _ "$stub" "$fake" "$here/isolab-status"
rc=0; GH_FAIL=1 PATH="$stub:$PATH" DELPHI_TOOLS_ROOT=$fake "$here/isolab-status" 222 >/dev/null 2>&1 || rc=$?
t "isolab-status: failed view exits 3" test "$rc" -eq 3

# ---- writeback -------------------------------------------------------------
wb=$tmp/wb
mkdir -p "$wb"
cat >"$tmp/r1.json" <<'JSON'
{"schema":"delphi.program-round/v1","program_id":"abc123def456","program_dir":"/x","goal":"Find a faster relation filter | test","round":1,"max_rounds":5,"status":"running",
 "proposal":{"claim":"Filter F halves the sieve cost","prediction":"p","test":"t","falsification":"f","goal":"g"},
 "goal_dir":"/x/goals/r1","goal_status":"proof_verified","goal_rounds":2,
 "claims":[{"id":"C1","text":"a","action":1,"status":"verified","reason":"ok"},{"id":"C2","text":"b","action":1,"status":"unverified","reason":"no"}],
 "actions":[{"index":1,"tool":"lean-verify","arg":"x","status":"ok","exit":0,"verdict":"pass"}],
 "review":{"outcome":"supported","learned":"Filter works on the toy case.","next":"continue","feedback":"Try the larger case."},
 "cost_usd":0.12,"spent_usd":0.4,"budget_usd":50,"finished_at":"2026-11-04T06:00:00Z"}
JSON
cat >"$tmp/r2.json" <<'JSON'
{"schema":"delphi.program-round/v1","program_id":"abc123def456","goal":"Find a faster relation filter | test","round":2,"status":"needs_review",
 "proposal":{"claim":"Run on EKS"},"goal_status":"needs_review",
 "actions":[{"index":1,"tool":"isolab-dispatch","status":"refused","exit":2,"reason":"gate closed"},{"index":2,"tool":"card-run","status":"timeout","exit":-1}],
 "review":{"outcome":"blocked","learned":"No access.","next":"stop","feedback":"Checkpoint gate refuses launchers."},
 "cost_usd":0.05,"finished_at":"2026-11-04T07:00:00Z"}
JSON
echo '{"schema":"delphi.program-round/v1","program_id":"0000000000aa","round":3}' >"$tmp/r3.json"
wbrun() { DELPHI_TOOLS_NO_GIT=1 DELPHI_TOOLS_ROOT=$wb "$here/writeback" "$1"; }
today=$(date -u +%F)
rc=0; out=$(wbrun "$tmp/r1.json" 2>/dev/null) || rc=$?
t "writeback r1: exit 0" test "$rc" -eq 0
t "writeback r1: stdout is one JSON line" bash -c 'printf "%s" "$1" | jq -e -s "length == 1 and .[0].round == 1 and .[0].pr == null and (.[0].files | length) == 3"' _ "$out"
t "writeback r1: round copy identical" cmp "$tmp/r1.json" "$wb/knowledge/programs/abc123def456/round-1.json"
rc=0; wbrun "$tmp/r2.json" >/dev/null 2>&1 || rc=$?
t "writeback r2: exit 0" test "$rc" -eq 0
led=$wb/knowledge/programs/abc123def456/LEDGER.md
t "ledger: header table" grep -qF '| round | claim | goal status | claims verified/total | outcome | next | cost USD |' "$led"
t "ledger: round 1 row" grep -qF '| 1 | Filter F halves the sieve cost | proof_verified | 1/2 | supported | continue | 0.12 |' "$led"
t "ledger: round 2 row" grep -qF '| 2 | Run on EKS | needs_review | 0/0 | blocked | stop | 0.05 |' "$led"
t "ledger: rounds in order" bash -c '[ "$(grep -n "^| 1 |" "$1" | cut -d: -f1)" -lt "$(grep -n "^| 2 |" "$1" | cut -d: -f1)" ]' _ "$led"
t "ledger: goal from newest, pipe escaped" grep -qF 'Goal: Find a faster relation filter \| test' "$led"
t "ledger: learned paragraph" grep -qF 'Filter works on the toy case.' "$led"
log=$wb/docs/session-logs/$today.md
t "session log: file header" grep -qxF "# Session log — $today" "$log"
t "session log: entry heading r1" grep -qxF '## 2026-11-04T06:00:00Z — Delphi program abc123def456 round 1: supported' "$log"
t "session log: entry heading r2" grep -qxF '## 2026-11-04T07:00:00Z — Delphi program abc123def456 round 2: blocked' "$log"
for h in 'What was done (commands, files, features)' 'Decisions made (and why)' 'Open threads (unfinished, pick up next time)' 'Errors (what went wrong, how it was handled)'; do
  t "session log: heading '$h' twice" bash -c '[ "$(grep -cxF "### $1" "$2")" -eq 2 ]' _ "$h" "$log"
done
t "session log: proposal disclaimer" grep -qF "unverified assertions." "$log"
t "session log: failed action listed" grep -qF 'isolab-dispatch: refused' "$log"
t "session log: next proposal pending" grep -qF 'Next proposal pending' "$log"
prob=$wb/problems/delphi-program-abc123def456-r2.md
t "problem file: created for blocked" test -f "$prob"
t "problem file: status/kind/owner" bash -c 'grep -qF "**Status:** \`open\`" "$1" && grep -qF "access/tool limit (reported by a Delphi program round)" "$1" && grep -qF "explicitly unassigned" "$1" && grep -qF "Checkpoint gate refuses launchers." "$1"' _ "$prob"
t "problem file: not created for supported" test ! -e "$wb/problems/delphi-program-abc123def456-r1.md"
wbrun "$tmp/r2.json" >/dev/null 2>&1 || true
t "rerun: session-log entry not duplicated" bash -c '[ "$(grep -cF "round 2: blocked" "$1")" -eq 1 ]' _ "$log"
t "rerun: problem history appended" bash -c '[ "$(grep -c "write-back ran again" "$1")" -eq 1 ]' _ "$prob"
rc=0; wbrun "$tmp/r3.json" >/dev/null 2>&1 || rc=$?
t "writeback: minimal record does not break" test "$rc" -eq 0

# ---- make-tools-json -------------------------------------------------------
rc=0; tj=$(DELPHI_TOOLS_ROOT=$real_root "$here/make-tools-json" 2>/dev/null) || rc=$?
t "make-tools-json: exit 0 and valid JSON" bash -c '[ "$1" -eq 0 ] && printf "%s" "$2" | jq . ' _ "$rc" "$tj"
t "make-tools-json: six tools, max_actions 4" bash -c 'printf "%s" "$1" | jq -e ".max_actions == 4 and (.tools | keys | length) == 6"' _ "$tj"
t "make-tools-json: verifiers" bash -c 'printf "%s" "$1" | jq -e "[.tools | to_entries[] | select(.value.verifier == true) | .key] | sort == [\"card-lint\",\"isolab-status\",\"lean-verify\"]"' _ "$tj"
t "make-tools-json: argv files are executable" bash -c 'printf "%s" "$1" | jq -r ".tools[].argv[0]" | while read -r f; do test -x "$f"; done' _ "$tj"
while IFS= read -r pat; do
  t "pattern compiles: $pat" bash -c 'printf "x" | grep -E -- "$1"; [ $? -le 1 ]' _ "$pat"
done < <(printf '%s' "$tj" | jq -r '.tools[] | .pattern // empty')
t "pattern accepts a valid dispatch arg" bash -c "printf '%s' '$sha,seconds=1800' | grep -Eq \"\$(printf '%s' \"\$1\" | jq -r '.tools[\"isolab-dispatch\"].pattern')\"" _ "$tj"

if [ "$fails" -eq 0 ]; then echo "ALL TESTS PASSED"; else echo "$fails TEST(S) FAILED"; exit 1; fi
