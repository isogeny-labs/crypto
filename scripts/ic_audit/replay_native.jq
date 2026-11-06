# Receipt metadata only. The native auditor performs all cryptographic checks.
def valid_audit_base($binary):
  .schema == "ecbench.audit/v1" and .session_status == "complete" and
  .records_bound == true and .auditor_binary_sha256 == $binary and
  (.session_id | type == "string" and length > 0) and
  (.files | type == "object" and
    has("session.json") and has("spec.json") and has("plan.json") and
    has("host.json") and has("records.jsonl")) and
  all(.files | to_entries[];
    (.key | test("^[A-Za-z0-9_./-]+$") and (startswith("/") | not) and
      (split("/") | index("..") | not)) and
    (.value | type == "string" and test("^[0-9a-f]{64}$")));

def valid_native($binary):
  valid_audit_base($binary) and
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

# Only an explicitly pinned historical negative may have no eligible replays.
# The first three IDs are audit.rs's FAILED_OUTCOME_REPLAY_EXEMPT. X19's
# four host timeouts are excluded by the ordinary native replay predicate.
# No new record or outcome is exempted by this metadata validator.
def valid_zero_pin:
  type == "object" and .schema == "ic-static-zero-replay/v1" and
  (.session_id as $sid |
    ["ECBS1hedc734e1870d","ECBS1h4c7b9db8a389","ECBS1h302934ff99ed",
     "ECBS1h9453f8e9b41e"] | index($sid) != null) and
  (.records | type == "number" and . > 0 and . == floor) and
  .planned_executions == .records and .verified_records == 0 and
  (.reason | type == "string" and length > 0) and
  (.files | type == "object" and length >= 5);

def valid_zero_probe($binary; $pin):
  ($pin | valid_zero_pin) and valid_audit_base($binary) and
  .session_id == $pin.session_id and .files == $pin.files and
  .records == $pin.records and .planned_executions == $pin.planned_executions and
  .verified_records == 0 and .regraded == true and
  .replay_candidates == 0 and .replays == [] and .replay_selection == null and
  .ok == true and .problems == [] and
  (.checks | type == "object" and
    (["arms","env_class","executions","executions_planned","host_hash","plan_hash",
      "record_count","records","records_hash","regraded","session_build",
      "session_complete","session_id","spec_id","status_counts","workloads"] - keys | length) == 0 and
    all(.[]; . == true));

def replay_keys: map({seq, record_id}) | sort_by(.seq);

# Historical counter debt cannot excuse a different failed record or scalar error.
def valid_drift_expectations:
  . as $plan | all(.sessions[];
    (if has("zero_replay_probe") then
      .expect == "identical" and (.zero_replay_probe | valid_zero_pin)
    else true end) and
    (if has("expected_drift") then
      (.expected_drift | type == "array") and
      (if .expect == "identical" then .expected_drift == [] else
        (.expected_drift | length > 0 and
          all(.[]; (.seq | type == "number" and . >= 0 and . == floor) and
            (.record_id | type == "string" and length > 0) and
            (.detail | type == "string" and length > 0)) and
          ([.[].seq] | length == (unique | length))) end)
    else ($plan.require_pinned_drift != true or .expect != "drift") end));

def same_pinned_drifts($entry):
  if $entry | has("expected_drift") then
    ([.replays[] | select(.reproduced | not) | {seq,record_id,detail}] | sort_by(.seq)) ==
      ($entry.expected_drift | sort_by(.seq))
  else true end;
