#!/usr/bin/env bash
# fetch.sh -- check in on, collect, classify and delete one run started by run.sh (draft Amendment 18,
# AMENDMENT_R2_H_runpod_capped.md, sections 4 and 6). Shell only. It holds the route's single outcome classifier.
#   fetch.sh NAME                    check in; collect, classify and delete when the outcome table says so
#   fetch.sh NAME --abort            session abort: collect what exists and delete now (class ABORTED unless the run
#                                    is already done, lost or past its deadline; aborting cannot change a class)
#   fetch.sh NAME --delete RECORD    after a completed collection: check that RECORD (bench.record/v1) lists every file
#                                    of the kept tree as uploaded, then delete the pod
#   fetch.sh --classify-dir REC OUT  offline: print the outcome table's verdict for a receipts dir and a results tree
# The run deadline is the runner's recorded start (start.json) + 90,000 s. At or after it, or on --abort, the pod is
# deleted even if the transfer fails. A completed collection waits for the raw upload only until the run deadline.
# Exit: 0 collected (or nothing to collect) and pod deleted, confirmed gone; 41 nothing to do yet (pod untouched);
#       2 usage; 6 transfer check failed before the deadline (pod kept; run again); 7 pod still listed after delete
#       (run again, or reap.sh); 8 collected, pod kept until the raw upload is confirmed or the deadline passes;
#       9 refused: this run is finished (cost.json exists), or a results tree already exists.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); . "$here/common.sh"

# ---------------------------------------------------------------------------------------------------------------------
# The outcome classifier. Input: receipts only (start.json, pod-state.json, pod-create.json, pod-record-start.json,
# pod-request.json, price.jsonl, checkins.jsonl, collect.json, local-sha256.txt) and the kept results tree. Output: one
# JSON object with class, rule (the precedence step that fired) and the facts it used. Precedence, first match wins:
#   P1 NOT_AN_ATTEMPT  no receipt shows a runner start
#   P2 completion secured (collect reason done, two identical fetches, a pre-fetch check-in with the runner done under
#      the recorded start, no loss evidence up to that check-in, the tree's runner_state agreeing, one start line):
#      P2a ended after the run deadline           -> LIMIT
#      P2m a 56Gi attempt whose memory peak exceeds 56 GiB: the 56Gi attempt counts as OOM (Amendment 14) and this
#          run is classified as the cell's -m112 attempt (outcome .memory.as_m112 true):
#          P2m1 peak > 112 GiB, or exit 137 with oom_kill > 0     -> OOM (at -m112: censored)
#          P2m2 exit 137 without oom evidence                     -> LOST
#          P2m3 admission rules 1-7 hold                          -> ADMITTED (the cell's -m112 row)
#          P2m4 otherwise                                         -> NOT_ADMITTED
#      P2b an end line exit 137 and oom_kill > 0  -> OOM   (Amendment 14's OOMKilled)
#      P2c an end line exit 137, no oom evidence  -> LOST
#      P2p an -m112 attempt whose peak exceeds 112 GiB -> OOM (censored)
#      P2u no memory peak observable              -> NOT_ADMITTED
#      P2d admission rules 1-7 hold               -> ADMITTED
#      P2e otherwise                              -> NOT_ADMITTED
#   The memory peak is the pilot's own field: the largest "memory R MB (peak P MB)" value in runs/<cell>.log (VmHWM, MiB,
#   x 1,048,576 bytes), the source RESULTS_T13_BRACKET.md §2b used for the t = 13 run's 20,618 MB. Only when the log has
#   no such line is the cgroup figure used (v2 memory.peak, else v1 memory.max_usage_in_bytes, bytes). Both are
#   recorded. An exit 137 with oom_kill > 0 is OOM whatever the peaks say (P2m1 or P2b).
#   P3 loss evidence dated before the run deadline -> LOST
#   P4 a check-in at or after the deadline shows the runner running (or done after the deadline) under the recorded start
#                                                  -> LIMIT
#   P5 collected on --abort before the deadline    -> ABORTED
#   P6 collected (or deleted) otherwise            -> LOST (completion not secured; not observable at the deadline)
#   P7 not collected yet                           -> PENDING
# ---------------------------------------------------------------------------------------------------------------------
facts() {
  local R=$1 O=$2 stage cell s0 L0 ck col rs prog nstart pexit gexit oom cgp mmax mpeak man r1 r2 r3 r4 r5 r6 r7 row floor
  stage=$(jq -r '.stage // empty' "$R/pod-state.json" 2>/dev/null || true)
  [ -n "$stage" ] || stage=$(jq -r '.stages[0] // empty' "$R/start.json" 2>/dev/null || true)
  cell=""; [[ $stage =~ ^cap:([0-9]+):([01]):([0-9]+): ]] && cell="X-m2-t${BASH_REMATCH[1]}-k${BASH_REMATCH[2]}-c${BASH_REMATCH[3]}"
  s0=$(jq -r 'select(.state == "running") | .started // empty' "$R/start.json" 2>/dev/null || true)
  ck=$( { [ -f "$R/checkins.jsonl" ] && jq -cs '.' "$R/checkins.jsonl"; } 2>/dev/null || echo '[]')
  [ -n "$ck" ] || ck='[]'
  [ -n "$s0" ] || s0=$(jq -r '[.[] | .live.started // empty] | first // empty' <<<"$ck")
  [ -n "$s0" ] || s0=$(jq -r '.started // empty' "$O/runner_state.json" 2>/dev/null || true)
  L0=$(jq -r '.lastStartedAt // empty' "$R/pod-record-start.json" 2>/dev/null || true)
  [ -n "$L0" ] || L0=$(jq -r '.lastStartedAt // empty' "$R/pod-record.json" 2>/dev/null || true)
  [ -n "$L0" ] || L0=$(jq -r '.lastStartedAt // empty' "$R/pod-create.json" 2>/dev/null || true)
  col=$(jq -c '.' "$R/collect.json" 2>/dev/null || echo null)
  rs=$(jq -c '{state,exit,started,ended}' "$O/runner_state.json" 2>/dev/null || echo null)
  nstart=0 pexit=null gexit=null
  if [ -f "$O/progress.txt" ] && [ -n "$cell" ]; then
    nstart=$(grep -c " start $cell\$" "$O/progress.txt" || true)
    pexit=$(awk -v c="$cell" '$2 == "end" && $3 == c && $4 == "exit" && $5 ~ /^[0-9]+$/ { e = $5 } END { print (e == "" ? "null" : e) }' "$O/progress.txt")
    gexit=$(awk -v c="$cell.gridzeros" '$2 == "end" && $3 == c && $4 == "exit" && $5 ~ /^[0-9]+$/ { e = $5 } END { print (e == "" ? "null" : e) }' "$O/progress.txt")
  fi
  # cgroup evidence (decision D4): v2 memory.events, or v1 memory.oom_control; absent files mean "no evidence".
  oom=null cgp=false mmax=null mpeak=null
  if [ -f "$O/cgroup/memory.events" ]; then cgp=true; oom=$(awk '$1 == "oom_kill" { print $2 }' "$O/cgroup/memory.events"); fi
  if [ "$oom" = null ] || [ -z "$oom" ]; then
    if [ -f "$O/cgroup/memory_memory.oom_control" ]; then cgp=true; oom=$(awk '$1 == "oom_kill" { print $2 }' "$O/cgroup/memory_memory.oom_control"); fi
  fi
  [ -n "$oom" ] || oom=null
  [ -f "$O/cgroup/memory.max" ] && mmax=$(jq -Rn --rawfile f "$O/cgroup/memory.max" '$f | rtrimstr("\n")')
  [ "$mmax" = null ] && [ -f "$O/cgroup/memory_memory.limit_in_bytes" ] && mmax=$(jq -Rn --rawfile f "$O/cgroup/memory_memory.limit_in_bytes" '$f | rtrimstr("\n")')
  [ -f "$O/cgroup/memory.peak" ] && mpeak=$(jq -Rn --rawfile f "$O/cgroup/memory.peak" '$f | rtrimstr("\n")')
  # The memory peaks for the comparability rule (P2m, P2p, P2u): the pilot's VmHWM field in its F4 log (MiB; primary),
  # and the cgroup's v2 memory.peak, else v1 memory.max_usage_in_bytes (bytes; fallback).
  local cgb=null cgsrc=null logmib=null limit=null
  for f in memory.peak:cgroup_v2_memory.peak memory_memory.max_usage_in_bytes:cgroup_v1_memory.max_usage_in_bytes; do
    if [ "$cgb" = null ] && [ -f "$O/cgroup/${f%%:*}" ] && grep -qxE '[0-9]+' "$O/cgroup/${f%%:*}"; then cgb=$(cat "$O/cgroup/${f%%:*}"); cgsrc="\"${f#*:}\""; fi
  done
  if [ -n "$cell" ] && [ -f "$O/runs/$cell.log" ]; then
    logmib=$(grep -oE 'memory [0-9]+ MB \(peak [0-9]+ MB\)' "$O/runs/$cell.log" | sed -E 's/.*peak ([0-9]+) MB\)/\1/' | sort -n | tail -1 || true)
    [ -n "$logmib" ] || logmib=null
  fi
  case $stage in *:12800000000) limit='"56Gi"' ;; *:27900000000) limit='"112Gi"' ;; esac
  # Admission rules 1-7 (section 6).
  r1=false r2=false r3=false r4=false r5=false r6=false r7=false
  if [ -f "$O/binary.sha256" ] && cmp -s <(awk 'NR <= 4 { print $1 "  /usr/local/bin/" $2 }' "$study/image/binaries.sha256") "$O/binary.sha256"; then r1=true; fi
  if [ -f "$O/host.txt" ] && grep -qxF "argv: $stage" "$O/host.txt" && grep -qxF "rayon: $RAYON" "$O/host.txt"; then r2=true; fi
  if [ "$nstart" = 1 ] && [ "$pexit" = 0 ] && [ "$gexit" = 0 ]; then r3=true; fi
  if jq -e --arg s "$s0" '.state == "done" and .exit == 0 and .started == $s' <<<"$rs" >/dev/null 2>&1; then r4=true; fi
  if jq -e --slurpfile i "$study/image/image.json" '. as $d | [ "pkm_tower_pilot","prime_fall_degree_tools","hky_fp_tower_check","pfd_grid_zeros","entry_r2.sh","run.sh" ]
       | all(.[]; . as $k | ($i[0].study_binary_sha256[$k] != null) and ($d.verified_sha256[$k] == $i[0].study_binary_sha256[$k]))' \
       "$O/runner_request.json" >/dev/null 2>&1; then r5=true; fi
  man=false
  if [ -d "$O" ] && [ -f "$R/local-sha256.txt" ] && cmp -s "$R/local-sha256.txt" <(cd "$O" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -r -d '\n' sha256sum); then man=true; fi
  if [ "$man" = true ] && jq -e '.transfer_ok == true' <<<"$col" >/dev/null 2>&1; then r6=true; fi
  floor=$(jq -r '.mem_floor_gb // empty' "$R/pod-state.json" 2>/dev/null || true); floor=${floor:-$MEM_FLOOR_GB}
  if jq -e --slurpfile p <(jq -s '.' "$R/price.jsonl" 2>/dev/null || echo '[]') --argjson v $VCPU --argjson m "$floor" --arg f "$FLAVOR" \
       '.machine.secureCloud == true and .cpuFlavorId == $f and .vcpuCount == $v and .memoryInGb >= $m
        and ($p[0] | length) > 0 and .costPerHr == ($p[0] | last | .securePrice)' "$R/pod-create.json" >/dev/null 2>&1 \
     && jq -e --arg f "$FLAVOR" '.interruptible == false and .cloudType == "SECURE" and .cpuFlavorIds == [$f]' "$R/pod-request.json" >/dev/null 2>&1; then r7=true; fi
  row=null
  [ -n "$cell" ] && [ -f "$O/runs/$cell.jsonl" ] && row=$(jq -cs '[.[] | select(has("oversize"))] | first // null | if . == null then null else {oversize, timed_out} end' "$O/runs/$cell.jsonl" 2>/dev/null || echo null)
  [ -n "$row" ] || row=null
  jq -n --arg stage "$stage" --arg cell "$cell" --arg s0 "$s0" --arg L0 "$L0" --argjson ck "$ck" --argjson col "$col" \
    --argjson rs "$rs" --argjson nstart "$nstart" --argjson pexit "$pexit" --argjson gexit "$gexit" --argjson oom "$oom" \
    --argjson cgp "$cgp" --argjson mmax "$mmax" --argjson mpeak "$mpeak" --argjson row "$row" --argjson dls $DEADLINE_S \
    --argjson gap $UNREACH_GAP_S --argjson cgb "$cgb" --argjson cgsrc "$cgsrc" --argjson logmib "$logmib" --argjson size "$limit" \
    --argjson adm "{\"r1\":$r1,\"r2\":$r2,\"r3\":$r3,\"r4\":$r4,\"r5\":$r5,\"r6\":$r6,\"r7\":$r7}" \
    '{stage:$stage, cell:$cell, s0:(if $s0 == "" then null else $s0 end), L0:(if $L0 == "" then null else $L0 end),
      checkins:$ck, collect:$col, tree:{runner_state:$rs, start_lines:$nstart, pilot_exit:$pexit, gridzeros_exit:$gexit,
      cgroup_present:$cgp, oom_kill:$oom, memory_max:$mmax, memory_peak:$mpeak}, row:$row, admission:$adm,
      mem:{size:$size, cgroup_peak_bytes:$cgb, cgroup_peak_source:$cgsrc, pilot_peak_mib:$logmib},
      deadline_s:$dls, unreach_gap_s:$gap}'
}

decide() { jq -c '
  def ep: if . == null then null else fromdateiso8601 end;
  def healthy($c; $L0): ($c.rest != null and $c.rest.http == 200 and $c.rest.desiredStatus == "RUNNING"
                          and ($L0 == null or $c.rest.lastStartedAt == $L0));
  def lossev($c; $s0; $L0):
    [ (if $c.live != null and $c.live.state == "idle" then "runner idle after its start (container restarted)" else empty end),
      (if $c.live != null and $c.live.started != null and $c.live.started != $s0 then "runner start time differs from start.json (a second start)" else empty end),
      (if $c.rest != null and $c.rest.http == 200 and $L0 != null and $c.rest.lastStartedAt != null and $c.rest.lastStartedAt != $L0 then "pod lastStartedAt changed (pod restarted)" else empty end),
      (if $c.rest != null and $c.rest.http == 200 and ($c.rest.desiredStatus // "RUNNING") != "RUNNING" then "pod desiredStatus \($c.rest.desiredStatus)" else empty end),
      (if $c.listed == false then "pod no longer listed by the API" else empty end) ];
  . as $f | $f.s0 as $s0 | $f.L0 as $L0
  | if $s0 == null then {class:"NOT_AN_ATTEMPT", rule:"P1", why:"no receipt shows a runner start"}
    else
      (($s0 | ep) + $f.deadline_s) as $dl
    | ($f.checkins | sort_by(.at)) as $ck
    | ([ $ck[] | . as $c | lossev($c; $s0; $L0)[] | {at:$c.at, why:.} ]
       + (reduce $ck[] as $c ({start:null, ev:[]};
            if $c.live == null then
              (if .start == null then .start = ($c.at | ep) else . end)
              | if (($c.at | ep) - .start) >= $f.unreach_gap_s and (healthy($c; $L0) | not)
                then .ev += [{at:$c.at, why:"runner unreachable through the proxy for at least \($f.unreach_gap_s) s and the REST record not a healthy RUNNING pod with unchanged lastStartedAt"}]
                else . end
            else .start = null end) | .ev)
       + (if $f.collect != null and $f.tree.start_lines > 1 then [{at:$f.collect.at, why:"\($f.tree.start_lines) start lines in progress.txt"}] else [] end)
       + (if $f.collect != null and $f.tree.runner_state != null and $f.tree.runner_state.started != null and $f.tree.runner_state.started != $s0
          then [{at:$f.collect.at, why:"runner_state.json start differs from start.json"}] else [] end)
      ) as $ev
    | ($f.collect.prefetch // null) as $pre
    | ($f.collect != null and $f.collect.reason == "done" and $f.collect.transfer_ok == true and $pre != null
       and $pre.live != null and $pre.live.state == "done" and $pre.live.started == $s0
       and ([ $ev[] | select((.at | ep) <= ($pre.at | ep)) ] | length) == 0
       and $f.tree.runner_state != null and $f.tree.runner_state.state == "done" and $f.tree.runner_state.started == $s0
       and $f.tree.runner_state.exit == $pre.live.exit and $f.tree.start_lines == 1) as $secured
    | ($f.tree.pilot_exit == 137 or $f.tree.gridzeros_exit == 137) as $k137
    | 60129542144 as $G56 | 120259084288 as $G112
    | (if $f.mem.pilot_peak_mib != null then $f.mem.pilot_peak_mib * 1048576 else null end) as $pilotb
    | (if $pilotb != null then {peak_bytes:$pilotb, source:"pilot_log_VmHWM_MiB"}
       elif $f.mem.cgroup_peak_bytes != null then {peak_bytes:$f.mem.cgroup_peak_bytes, source:$f.mem.cgroup_peak_source}
       else {peak_bytes:null, source:null} end) as $pk
    | {size:$f.mem.size, peak_bytes_used:$pk.peak_bytes, source_used:$pk.source, pilot_peak_bytes:$pilotb,
       cgroup_peak_bytes:$f.mem.cgroup_peak_bytes, cgroup_peak_source:$f.mem.cgroup_peak_source, as_m112:false} as $mem
    | (($f.tree.oom_kill // 0) > 0) as $oomk
    | {deadline:($dl | todate), secured:$secured} as $base
    | if $secured then
        if ($f.tree.runner_state.ended | ep) > $dl then $base + {class:"LIMIT", rule:"P2a", why:"the runner ended at \($f.tree.runner_state.ended), after the run deadline", memory:$mem}
        elif $f.mem.size == "56Gi" and $pk.peak_bytes != null and $pk.peak_bytes > $G56 then
          ($base + {memory:($mem + {as_m112:true}), why56:"the 56Gi attempt counts as OOM (Amendment 14): memory peak \($pk.peak_bytes) bytes (\($pk.source)) > 56 GiB"})
          + (if $pk.peak_bytes > $G112 or ($k137 and $oomk) then {class:"OOM", rule:"P2m1", why:"classified as the cell'"'"'s -m112 attempt: OOM at -m112 (peak \($pk.peak_bytes) bytes, oom_kill \($f.tree.oom_kill // "absent")); censored"}
             elif $k137 then {class:"LOST", rule:"P2m2", why:"classified as the cell'"'"'s -m112 attempt: exit 137 without oom evidence"}
             elif ($f.admission | all(.[]; . == true)) then {class:"ADMITTED", rule:"P2m3", why:"classified as the cell'"'"'s -m112 attempt: peak within 112 GiB, admission rules 1-7 hold", row:$f.row}
             else {class:"NOT_ADMITTED", rule:"P2m4", why:"classified as the cell'"'"'s -m112 attempt: admission rules failed: \([$f.admission | to_entries[] | select(.value != true) | .key] | join(","))"} end)
        elif $k137 and $oomk then $base + {class:"OOM", rule:"P2b", memory:$mem, why:"exit 137 with cgroup oom_kill \($f.tree.oom_kill) (Amendment 14 OOMKilled)"}
        elif $k137 then $base + {class:"LOST", rule:"P2c", memory:$mem, why:"exit 137 without oom evidence (oom_kill \($f.tree.oom_kill // "absent"))"}
        elif $f.mem.size == "112Gi" and $pk.peak_bytes != null and $pk.peak_bytes > $G112 then $base + {class:"OOM", rule:"P2p", why:"-m112 attempt with memory peak \($pk.peak_bytes) bytes (\($pk.source)) > 112 GiB; censored", memory:$mem}
        elif $pk.peak_bytes == null then $base + {class:"NOT_ADMITTED", rule:"P2u", why:"no memory peak observable (no pilot VmHWM field and no cgroup peak file)", memory:$mem}
        elif ($f.admission | all(.[]; . == true)) then $base + {class:"ADMITTED", rule:"P2d", why:"completed; admission rules 1-7 hold", row:$f.row, memory:$mem}
        else $base + {class:"NOT_ADMITTED", rule:"P2e", why:"completed, but admission rules failed: \([$f.admission | to_entries[] | select(.value != true) | .key] | join(","))", memory:$mem}
        end
      elif ([ $ev[] | select((.at | ep) < $dl) ] | length) > 0 then
        $base + {class:"LOST", rule:"P3", why:([ $ev[] | select((.at | ep) < $dl) ] | first | "\(.why) at \(.at)")}
      elif ([ $ck[] | select((.at | ep) >= $dl and .live != null and .live.started == $s0
                and (.live.state == "running" or (.live.state == "done" and (.live.ended | ep) > $dl))) ] | length) > 0 then
        $base + {class:"LIMIT", rule:"P4", why:"a check-in at or after the run deadline shows the runner not finished within it"}
      elif $f.collect != null and $f.collect.reason == "abort" then
        $base + {class:"ABORTED", rule:"P5", why:"session abort at \($f.collect.at), before the run deadline"}
      elif $f.collect != null then
        $base + {class:"LOST", rule:"P6", why:"collected (reason \($f.collect.reason)) without a secured completion or positive deadline evidence"}
      else $base + {class:"PENDING", rule:"P7", why:"not collected yet"}
      end
    end
  + {facts:{tree:$f.tree, admission:$f.admission, mem:$f.mem, L0:$f.L0, s0:$f.s0, cell:$f.cell}}'; }
classify() { facts "$1" "$2" | decide; }

if [ "${1:-}" = --classify-dir ]; then
  [ $# -eq 3 ] || { echo "usage: fetch.sh --classify-dir REC OUT" >&2; exit 2; }
  classify "$2" "$3"; exit 0
fi

# ---------------------------------------------------------------------------------------------------------------------
[ $# -ge 1 ] || { echo "usage: fetch.sh NAME [--abort | --delete RECORD]" >&2; exit 2; }
NAME=$1 MODE=${2:-} RECORD=${3:-}
case $MODE in ""|--abort) ;; --delete) [ -n "$RECORD" ] || { echo "usage: fetch.sh NAME --delete RECORD" >&2; exit 2; } ;;
  *) echo "usage: fetch.sh NAME [--abort | --delete RECORD]" >&2; exit 2 ;; esac
REC=$(rec_dir "$NAME") || { echo "unknown name $NAME" >&2; exit 2; }
[ -d "$REC" ] || { echo "no receipts $REC" >&2; exit 2; }
[ ! -f "$REC/cost.json" ] || { echo "REFUSED: $NAME is finished ($REC/cost.json exists); nothing is collected or deleted again" >&2; exit 9; }
OUT="$OUT_ROOT/$NAME"
log() { echo "$(now) $*" | tee -a "$REC/fetch.log" >&2; }
rpc_init; trap rpc_cleanup EXIT
exec 9>"$RPC_STATE/$NAME.lock"; flock -w 900 9 || { echo "another fetch.sh holds $NAME" >&2; exit 2; }
id=$(jq -r '.id // empty' "$REC/pod-create.json" 2>/dev/null || true)
[ -n "$id" ] || { echo "no pod id in $REC/pod-create.json" >&2; exit 2; }
nowe() { date +%s; }

checkin() { # checkin PHASE: one live status, one REST record and one list membership; appended to checkins.jsonl
  local stamp at live c rest listed l
  stamp=$(date -u +%Y%m%dT%H%M%SZ)-$1; at=$(now)
  timeout 60 "$RP" status "$id" > "$REC/status-$stamp.json" 2>/dev/null || true
  live=$(jq -c 'if type == "object" and has("state") then {state,exit,started,ended} else null end' "$REC/status-$stamp.json" 2>/dev/null || echo null)
  [ -n "$live" ] || live=null
  c=$(api_code GET "$API/pods/$id" "$RPC_STATE/$NAME.rest.tmp")
  if [ "$c" = 200 ]; then redact < "$RPC_STATE/$NAME.rest.tmp" > "$REC/rest-$stamp.json"
    rest=$(jq -c --argjson c "$c" '{http:$c, desiredStatus, lastStartedAt}' "$REC/rest-$stamp.json")
  else rest="{\"http\":${c:-0}}"; fi
  rm -f "$RPC_STATE/$NAME.rest.tmp" "$RPC_STATE/$NAME.rest.tmp.err"
  if l=$(list_pkm 2>/dev/null); then listed=$(jq -s --arg i "$id" 'any(.[]; .id == $i)' <<<"$l"); else listed=null; fi
  jq -nc --arg at "$at" --arg ph "$1" --argjson live "$live" --argjson rest "$rest" --argjson listed "$listed" \
    '{at:$at, phase:$ph, live:$live, rest:$rest, listed:$listed}' | tee -a "$REC/checkins.jsonl"
}

delete_phase() { # delete the pod, confirm by listing, write cost.json, close the ledger row; exit 0, or 7 if still listed
  local stamp c i gone=0 deleted="" l ub=false
  stamp=$(date -u +%Y%m%dT%H%M%SZ); touch "$REC/delete-pending"
  c=$(api_code GET "$API/pods/$id" "$RPC_STATE/$NAME.final.tmp")
  [ "$c" = 200 ] && redact < "$RPC_STATE/$NAME.final.tmp" > "$REC/pod-record-final-$stamp.json"
  rm -f "$RPC_STATE/$NAME.final.tmp" "$RPC_STATE/$NAME.final.tmp.err"
  for i in 1 2 3; do
    c=$(api_code DELETE "$API/pods/$id" "$REC/delete-$stamp-$i.response"); echo "$(now) DELETE $id HTTP $c" >> "$REC/delete.http"
    case $c in 200|204) deleted=$(now); break ;; 404) ub=true; break ;; esac; sleep "$POLL_S"
  done
  log "delete pod $id: HTTP $c"
  for i in $(seq 1 12); do
    if l=$(list_pkm 2>/dev/null); then
      if ! jq -se --arg i "$id" 'any(.[]; .id == $i)' <<<"$l" >/dev/null; then gone=1; echo "$l" > "$REC/pods-after-delete-$stamp.jsonl"; break; fi
    fi; sleep "$POLL_S"
  done
  [ "$gone" = 1 ] || { log "POD $id STILL LISTED (or the list is unreadable) after delete; run fetch.sh $NAME again or reap.sh"; exit 7; }
  [ -n "$deleted" ] || { deleted=$(now); ub=true; }
  local cr p s usd
  cr=$(jq -r '.createdAt' "$REC/pod-create.json" | sed 's/ UTC$//'); p=$(jq -r '.costPerHr' "$REC/pod-create.json")
  s=$(( $(epoch "$deleted") - $(epoch "$cr") )); usd=$(awk -v p="$p" -v s="$s" 'BEGIN{printf "%.4f", p * s / 3600}')
  jq -n --arg id "$id" --arg c "$(date -u -d "$cr" +%FT%TZ)" --arg d "$deleted" --argjson s "$s" --argjson p "$p" --argjson u "$usd" \
    --argjson ub "$ub" --arg cl "$(jq -r '.class // "unclassified"' "$REC/outcome.json" 2>/dev/null || echo unclassified)" \
    '{pod:$id, created:$c, deleted:$d, deleted_time_is_upper_bound:$ub, seconds:$s, costPerHr:$p, usd_derived:$u,
      class:$cl, confirmed_gone:true}' > "$REC/cost.json"
  log "pod $id deleted and no longer listed; cost (derived: costPerHr x pod lifetime) US\$$usd over $s s at $p/h"
  rm -f "$REC/delete-pending"
  if [ "$(jq -r '.class' "$REC/outcome.json" 2>/dev/null || true)" = NOT_AN_ATTEMPT ]; then
    local k=1; while [ -e "$REC.prestart-$k" ]; do k=$((k + 1)); done
    ledger_close "$NAME/$id" "$usd" "${REC#"$here/"}.prestart-$k/cost.json"
    log "not an attempt: receipts moved to $REC.prestart-$k"; mv "$REC" "$REC.prestart-$k"
  else ledger_close "$NAME/$id" "$usd" "${REC#"$here/"}/cost.json"; fi
  exit 0
}

verify_upload() { # every file of the kept tree is in RECORD's raw[] with uploaded true (benchrec add-raw + upload)
  local miss
  [ -f "$RECORD" ] && [ -f "$REC/local-sha256.txt" ] || return 1
  miss=$(comm -23 <(awk '{ print $1 }' "$REC/local-sha256.txt" | sort -u) \
                  <(jq -r '.raw[]? | select(.uploaded == true) | .sha256' "$RECORD" | sort -u))
  { echo "$(now) record $RECORD sha256 $(sha256sum "$RECORD" | cut -d' ' -f1)"
    echo "files in the kept tree: $(wc -l < "$REC/local-sha256.txt"); not uploaded: $(printf '%s' "$miss" | grep -c . || true)"; } >> "$REC/upload-check.txt"
  [ -z "$miss" ]
}

# --- a delete that was not confirmed earlier resumes first
[ -f "$REC/delete-pending" ] && { log "resuming an unconfirmed delete"; delete_phase; }

s0=$(jq -r 'select(.state == "running") | .started // empty' "$REC/start.json" 2>/dev/null || true)
last=$(checkin regular)
[ -n "$s0" ] || s0=$(jq -r '.live.started // empty' <<<"$last")
if [ -n "$s0" ]; then dl=$(( $(epoch "$s0") + DEADLINE_S )); else dl=""; fi

# --- collected and waiting for the raw upload
if [ -f "$REC/collect.json" ]; then
  if [ "$MODE" = --delete ]; then
    if verify_upload; then log "raw upload confirmed for every file of the kept tree"; delete_phase; fi
    log "the record does not list every file of the kept tree as uploaded"
  fi
  if [ -n "$dl" ] && [ "$(nowe)" -ge "$dl" ]; then log "run deadline passed: deleting without a confirmed upload"; delete_phase; fi
  [ "$MODE" = --abort ] && { log "abort: deleting without a confirmed upload (the collected tree is local)"; delete_phase; }
  log "collected; pod kept until the raw upload is confirmed (fetch.sh $NAME --delete RECORD) or the run deadline"; exit 8
fi

# --- no runner start on record: a launch in progress, or a launcher that died before start
if [ -z "$s0" ]; then
  ce=$(epoch "$(jq -r '.createdAt' "$REC/pod-create.json" | sed 's/ UTC$//')")
  if [ "$MODE" = --abort ] || [ $(( $(nowe) - ce )) -gt $(( START_WITHIN_S + 900 )) ]; then
    jq -n '{class:"NOT_AN_ATTEMPT", rule:"P1", why:"no receipt shows a runner start (launcher stopped before start)"}' > "$REC/outcome.json"
    log "no runner start on record; deleting the pod (not an attempt)"; delete_phase
  fi
  log "no runner start on record yet (launch in progress?)"; exit 41
fi

# --- decide whether to collect (the classifier on the receipts so far, then the deadline and the mode)
pre=$(classify "$REC" "$OUT" | jq -r '.rule')
lstate=$(jq -r '.live.state // "unreachable"' <<<"$last"); lstarted=$(jq -r '.live.started // empty' <<<"$last")
if [ "$pre" = P3 ]; then reason=lost
elif [ "$lstate" = done ] && [ "$lstarted" = "$s0" ]; then reason=done
elif [ "$pre" = P4 ] || [ "$(nowe)" -ge "$dl" ]; then reason=deadline
elif [ "$MODE" = --abort ]; then reason=abort
else log "state $lstate; run deadline $(date -u -d "@$dl" +%FT%TZ); nothing to collect yet"; exit 41; fi
log "collecting: reason $reason (live state $lstate)"
[ ! -e "$OUT" ] || { log "REFUSED: results tree $OUT already exists (never overwritten)"; exit 9; }

# --- collect: a pre-fetch check-in, then two fetches whose per-file manifests must agree
prefetch=null fetched=false tok=false
if [ "$lstate" != unreachable ]; then
  if [ "$reason" = done ]; then   # let the boot loop write its final cgroup copy (it polls every 5 s)
    w=$(( $(epoch "$(jq -r '.live.ended' <<<"$last")") + 30 - $(nowe) )); [ "$w" -gt 0 ] && [ "$w" -le 60 ] && sleep "$w"
  fi
  prefetch=$(checkin prefetch)
  wd="$RPC_STATE/$NAME.fetch-$(date -u +%Y%m%dT%H%M%SZ)"; mkdir -p "$wd/a" "$wd/b"; okn=0
  for x in a b; do
    if timeout 900 "$RP" fetch "$id" "$wd/$x.tar.gz" >> "$REC/fetch.log" 2>&1 && gzip -t "$wd/$x.tar.gz" 2>/dev/null \
       && tar -xzf "$wd/$x.tar.gz" -C "$wd/$x" 2>/dev/null && [ -d "$wd/$x/results" ]; then
      (cd "$wd/$x/results" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -r -d '\n' sha256sum) > "$wd/$x.sha256"; okn=$((okn + 1))
    else log "fetch $x failed"; fi
  done
  keep=""
  if [ "$okn" = 2 ] && cmp -s "$wd/a.sha256" "$wd/b.sha256"; then tok=true; keep=a
  elif [ "$okn" = 2 ]; then diff "$wd/a.sha256" "$wd/b.sha256" >> "$REC/fetch.log" 2>&1 || true; log "the two fetches differ"; keep=b
  elif [ "$okn" = 1 ]; then [ -f "$wd/a.sha256" ] && keep=a || keep=b; fi
  if [ "$reason" = done ] && [ "$tok" = false ] && [ "$(nowe)" -lt "$dl" ]; then
    log "TRANSFER CHECK FAILED before the deadline; pod $id kept, run fetch.sh $NAME again"; exit 6
  fi
  if [ -n "$keep" ]; then
    fetched=true; mkdir -p "$OUT_ROOT"; mv "$wd/$keep/results" "$OUT"
    (cd "$OUT" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -r -d '\n' sha256sum) > "$REC/local-sha256.txt"
    cmp -s "$REC/local-sha256.txt" "$wd/$keep.sha256" || { log "kept tree differs from its fetched manifest"; tok=false; }
    (cd "$wd" && sha256sum "$keep.tar.gz") > "$REC/tarball.sha256"
    { echo "--- binary.sha256"; cat "$OUT/binary.sha256" 2>/dev/null || echo MISSING
      echo "--- host.txt (first 7 lines)"; head -7 "$OUT/host.txt" 2>/dev/null || echo MISSING
      echo "--- progress.txt"; cat "$OUT/progress.txt" 2>/dev/null || echo MISSING
      echo "--- runner_state.json"; cat "$OUT/runner_state.json" 2>/dev/null || echo MISSING
      echo "--- cgroup"; for f in "$OUT"/cgroup/*; do [ -f "$f" ] && { echo "${f##*/}:"; cat "$f"; }; done 2>/dev/null || echo MISSING
    } > "$REC/admission-lines.txt"
    log "results in $OUT: $(wc -l < "$REC/local-sha256.txt") files; transfer check $tok"
  fi
  rm -rf "$wd"
fi
jq -n --arg at "$(now)" --arg r "$reason" --arg m "$MODE" --argjson pre "$prefetch" --argjson f "$fetched" --argjson t "$tok" \
  --arg o "${OUT#"$here/"}" '{at:$at, reason:$r, mode:$m, prefetch:$pre, fetched:$f, transfer_ok:$t, out:$o}' > "$REC/collect.json"
classify "$REC" "$OUT" > "$REC/outcome.json"
log "OUTCOME $(jq -c '{class,rule,why}' "$REC/outcome.json")"
if [ "$reason" = done ] && [ "$tok" = true ] && [ "$(nowe)" -lt "$dl" ] && [ "$MODE" != --abort ]; then
  log "upload the kept tree (tools/benchrec.py add-raw RECORD $OUT/...; tools/benchrec.py upload RECORD), then fetch.sh $NAME --delete RECORD"
  exit 8
fi
delete_phase
