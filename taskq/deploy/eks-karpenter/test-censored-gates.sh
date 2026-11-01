#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

# A clean audit may contain valid right-censored outcomes; verified_records is
# a status count, not an integrity count.
jq -n '{ok:true,regraded:true,records_bound:true,records:364,verified_records:308,
  replays:[range(0;312)|{reproduced:true}]}' > "$tmp/audit.json"
jq -e '.ok and .regraded and .records_bound and .records>0
  and (.replays|length)>0 and all(.replays[];.reproduced)' "$tmp/audit.json" >/dev/null
! rg -q 'verified_records==\.records|verified_records.*!=.*records|records.*!=.*verified_records' \
  "$here/run-ecbench" "$here/remote/run-request" \
  "$here/resume/controller/src/cloud.rs" "$here/resume/controller/src/supervisor.rs"

record() {
  jq -nc --argjson seq "$1" --arg status "$2" --arg level "$3" \
    --argjson matches "$4" \
    '{seq:$seq,arm:"fixture",outcome:{status:$status,matches_target:$matches},
      isolation:{level:$level}}'
}
{
  record 0 verified L2 true
  record 1 exhausted L2 null
  record 2 timeout L3 null
} > "$tmp/records.jsonl"

l2_gate() {
  jq -s '{required:"L2",executions:length,
    invalid_outcomes:[.[] | select(
      (.outcome.status=="verified" and .outcome.matches_target!=true)
      or (.outcome.status!="verified" and .outcome.status!="exhausted"
        and .outcome.status!="timeout")) | {seq,arm,outcome}],
    censored_outcomes:[.[] | select(.outcome.status=="exhausted"
      or .outcome.status=="timeout") | {seq,arm,status:.outcome.status}],
    below_l2:[.[] | select(.isolation.level != "L2" and .isolation.level != "L3") | {seq,arm,isolation}],
    grades:(group_by(.isolation.level)|map({level:.[0].isolation.level,count:length}))}
    | .accepted=(.executions > 0 and (.below_l2|length)==0
      and (.invalid_outcomes|length)==0)'
}
l2_gate < "$tmp/records.jsonl" > "$tmp/l2.json"
jq -e '.accepted and .invalid_outcomes==[] and (.censored_outcomes|length)==2' \
  "$tmp/l2.json" >/dev/null

record 3 wrong_answer L2 false >> "$tmp/records.jsonl"
l2_gate < "$tmp/records.jsonl" > "$tmp/wrong.json"
jq -e '.accepted==false and [.invalid_outcomes[].seq]==[3]' "$tmp/wrong.json" >/dev/null

record 4 exhausted L1 null >> "$tmp/records.jsonl"
l2_gate < "$tmp/records.jsonl" > "$tmp/l1.json"
jq -e '.accepted==false and [.below_l2[].seq]==[4]' "$tmp/l1.json" >/dev/null
printf 'Audited censored outcomes are retained; wrong answers and sub-L2 records are rejected\n'
