#!/bin/bash
# Sourced by the native controller; its caller provides out, started and upload().
lefschetz_finish() {
 local code=$?
 trap - EXIT TERM INT
 local phases='{}' name status
 for name in producer replay; do
  status='{"started":false,"exit_code":125}'
  [[ ! -f "$out/$name-status.json" ]] || status=$(cat "$out/$name-status.json")
  phases=$(jq -c --arg name "$name" --argjson status "$status" '. + {($name):$status}' <<< "$phases")
 done
 jq -n --argjson code "$code" --arg start "$started" --arg finish "$(date -u +%FT%TZ)" --argjson phases "$phases" --arg backend "${N83_RUN_BACKEND:-development}" \
  '{exit_code:$code,started_at:$start,finished_at:$finish,backend:$backend,wall_valid:false,phases:$phases}' > "$out/terminal.json"
 (cd "$out" && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
 upload || exit 91
 exit "$code"
}
lefschetz_signal() {
 local code=$1
 if [[ -n "${active_phase:-}" ]]; then
  [[ -z "${active_pid:-}" ]] || kill -TERM "$active_pid" 2>/dev/null || true
  [[ -z "${active_pid:-}" ]] || wait "$active_pid" 2>/dev/null || true
  jq -n --argjson code "$code" '{started:true,exit_code:$code,interrupted:true}' > "$out/$active_phase-status.json"
 fi
 exit "$code"
}
lefschetz_phase() {
 local name=$1 code; shift
 active_phase=$name
 (ulimit -v 2097152; exec timeout --signal=TERM --kill-after=5 600 "$@") > "$out/$name.log" 2>&1 &
 active_pid=$!
 if wait "$active_pid"; then code=0; else code=$?; fi
 active_pid='';active_phase=''
 jq -n --argjson code "$code" '{started:true,exit_code:$code}' > "$out/$name-status.json"
 upload
 return "$code"
}
active_phase='';active_pid=''
trap lefschetz_finish EXIT
trap 'lefschetz_signal 143' TERM
trap 'lefschetz_signal 130' INT
