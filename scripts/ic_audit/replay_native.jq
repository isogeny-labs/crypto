# Receipt metadata only. The native auditor performs all cryptographic checks.
def valid_native($binary):
  .schema == "ecbench.audit/v1" and .session_status == "complete" and
  .records_bound == true and .auditor_binary_sha256 == $binary and
  (.session_id | type == "string" and length > 0) and
  (.files | type == "object" and
    has("session.json") and has("spec.json") and has("plan.json") and
    has("host.json") and has("records.jsonl")) and
  all(.files | to_entries[];
    (.key | test("^[A-Za-z0-9_./-]+$") and (startswith("/") | not) and
      (split("/") | index("..") | not)) and
    (.value | type == "string" and test("^[0-9a-f]{64}$"))) and
  (.checks | type == "object" and has("records_hash") and has("executions_planned") and
    has("replays")) and
  all(.checks | to_entries[]; .key == "replays" or .value == true) and
  (.replays | type == "array" and length > 0) and
  all(.replays[];
    (.seq | type == "number" and . >= 0 and . == floor) and
    (.record_id | type == "string" and length > 0) and
    (.reproduced | type == "boolean") and (.detail | type == "string")) and
  ([.replays[].seq] | length == (unique | length)) and
  .checks.replays == all(.replays[]; .reproduced) and
  .ok == .checks.replays and
  .problems == [.replays[] | select(.reproduced | not) |
    "replay of seq \(.seq): \(.detail)"];

def replay_keys: map({seq, record_id}) | sort_by(.seq);

# Historical counter debt cannot excuse a different failed record or scalar error.
def valid_drift_expectations:
  . as $plan | all(.sessions[];
    if has("expected_drift") then
      (.expected_drift | type == "array") and
      (if .expect == "identical" then .expected_drift == [] else
        (.expected_drift | length > 0 and
          all(.[]; (.seq | type == "number" and . >= 0 and . == floor) and
            (.record_id | type == "string" and length > 0) and
            (.detail | type == "string" and length > 0)) and
          ([.[].seq] | length == (unique | length))) end)
    else ($plan.require_pinned_drift != true or .expect != "drift") end);

def same_pinned_drifts($entry):
  if $entry | has("expected_drift") then
    ([.replays[] | select(.reproduced | not) | {seq,record_id,detail}] | sort_by(.seq)) ==
      ($entry.expected_drift | sort_by(.seq))
  else true end;
