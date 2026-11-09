#!/usr/bin/env bash
# Rebuild the pod part of a command-Job status from the EKS API-server audit log, for a Job that
# FAILED and whose Pod object was deleted (node removed) before the status query, so that the
# isolab.eks.command-status/v1 answer carries "pods": [].
#   status_with_audit_pod.sh FINAL_STATUS_JSON START_ISO END_ISO OUT_JSON
# FINAL_STATUS_JSON: final-status.json written by wait_command_status.sh; it must say state failed,
#   a non-empty job_uid and pods [] (any other answer is refused: use the ordinary verify instead).
# START_ISO, END_ISO: the audit-log window to search (UTC, e.g. 2026-11-08T00:00:00Z).
# OUT_JSON: FINAL_STATUS_JSON with "pods" replaced by the one pod found in the audit log and a
#   "pods_source" object naming the audit events used. Feed it to the study's verify script.
# The pod is accepted only when:
#   - exactly one pod create (HTTP 201, user system:serviceaccount:kube-system:job-controller) has an
#     ownerReference of kind Job whose uid equals the status job_uid;
#   - at least one status update of that pod by its node (user system:node:*) reports container
#     state terminated, and every such report gives the same exit code.
# Exit codes: 60 status not (failed, job_uid, pods []); 61 not exactly one owned pod create;
#   62 no terminated report by a node; 63 conflicting exit codes; 64 audit-log query failed.
set -euo pipefail
[[ $# == 4 ]] || { echo 'usage: status_with_audit_pod.sh FINAL_STATUS_JSON START_ISO END_ISO OUT_JSON' >&2; exit 2; }
status=$1 start=$2 end=$3 out=$4
LG=/aws/eks/isolab/cluster
jq -e '.state=="failed" and (.job_uid|length)>0 and .pods==[]' "$status" >/dev/null \
  || { echo 'FAIL: status is not state failed with a job_uid and pods []' >&2; exit 60; }
job=$(jq -er .job_name "$status"); juid=$(jq -er .job_uid "$status")
s=$(date -u -d "$start" +%s)000; e=$(date -u -d "$end" +%s)000
q() { aws logs filter-log-events --region us-east-2 --log-group-name "$LG" --log-stream-name-prefix kube-apiserver-audit \
        --start-time "$s" --end-time "$e" --filter-pattern "$1" --query 'events[].message' --output json; }
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
q "\"$job\" \"create\" \"pods\"" > "$tmp/create.json" || { echo 'FAIL: audit-log query (create) failed' >&2; exit 64; }
jq -c --arg u "$juid" '[.[] | fromjson | select(.verb=="create" and .objectRef.resource=="pods" and (.objectRef.subresource//"")==""
      and .responseStatus.code==201 and .user.username=="system:serviceaccount:kube-system:job-controller")
      | select([(.responseObject.metadata.ownerReferences // .requestObject.metadata.ownerReferences // [])[] | select(.kind=="Job" and .uid==$u)] | length == 1)
      | {name:.responseObject.metadata.name, uid:.responseObject.metadata.uid, audit_id:.auditID, t:.requestReceivedTimestamp}] | unique_by(.uid)' \
  "$tmp/create.json" > "$tmp/pods.json"
[[ $(jq length "$tmp/pods.json") == 1 ]] || { echo "FAIL: $(jq length "$tmp/pods.json") pod creates owned by job uid $juid (need exactly 1)" >&2; exit 61; }
pname=$(jq -er '.[0].name' "$tmp/pods.json"); puid=$(jq -er '.[0].uid' "$tmp/pods.json")
q "\"$pname\" \"terminated\"" > "$tmp/term.json" || { echo 'FAIL: audit-log query (status) failed' >&2; exit 64; }
jq -c --arg n "$pname" --arg u "$puid" '[.[] | fromjson | select(.objectRef.resource=="pods" and .objectRef.name==$n and .objectRef.subresource=="status"
      and (.user.username|startswith("system:node:")) and ((.objectRef.uid // .requestObject.metadata.uid // $u)==$u) and (.responseStatus.code//200) < 300)
      | . as $ev | (.requestObject.status.containerStatuses // [])[] | select(.state.terminated != null)
      | {container:.name, exit_code:.state.terminated.exitCode, reason:.state.terminated.reason, started_at:.state.terminated.startedAt,
         finished_at:.state.terminated.finishedAt, node:$ev.user.username, audit_id:$ev.auditID, t:$ev.requestReceivedTimestamp}]' \
  "$tmp/term.json" > "$tmp/terms.json"
[[ $(jq length "$tmp/terms.json") -ge 1 ]] || { echo "FAIL: no terminated status report by a node for pod $pname" >&2; exit 62; }
[[ $(jq '[.[].exit_code] | unique | length' "$tmp/terms.json") == 1 ]] || { echo "FAIL: conflicting exit codes for pod $pname: $(jq -c '[.[].exit_code]|unique' "$tmp/terms.json")" >&2; exit 63; }
jq -n --slurpfile st "$status" --slurpfile p "$tmp/pods.json" --slurpfile t "$tmp/terms.json" --arg lg "$LG" --arg a "$start" --arg b "$end" '
  $st[0] + {pods: [{name: $p[0][0].name, uid: $p[0][0].uid, exit_code: $t[0][-1].exit_code, reason: $t[0][-1].reason,
                    started_at: $t[0][-1].started_at, finished_at: $t[0][-1].finished_at, node: $t[0][-1].node}],
            pods_source: {schema: "isolab.eks.pod-from-audit/v1", log_group: $lg, window: [$a, $b],
                          create: $p[0][0], terminated_reports: $t[0]}}' > "$out"
echo "pod $pname uid $puid exit $(jq -r '.pods[0].exit_code' "$out") (from $(jq length "$tmp/terms.json") node report(s)); job $job uid $juid"
