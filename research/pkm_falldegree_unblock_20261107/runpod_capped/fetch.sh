#!/usr/bin/env bash
# fetch.sh -- check in on, collect, classify and delete one run started by run.sh (Amendment 33 draft,
# AMENDMENT_R2_H_runpod_capped.md, sections 3, 4 and 6). Shell only. RunPod REST v2. It holds the route's single
# outcome classifier.
#   fetch.sh NAME                    check in; collect, classify and delete when the outcome table says so
#   fetch.sh NAME --abort            session abort: collect what exists and delete now (ABORTED unless the run is done,
#                                    has positive evidence of loss or of the limit; aborting never buys a relaunch)
#   fetch.sh NAME --delete REF       after a completed collection: the git gate (owner decision 22, no S3). REF is a git
#                                    commit or ref; the pod is deleted only when (a) every file of the kept tree, and the
#                                    receipts' outcome.json and local-sha256-<k>.txt, are in REF with the blob hash of
#                                    the local file, (b) each local file still has its manifest SHA-256, and (c) REF is
#                                    an ancestor of a branch head that origin lists now (git ls-remote --heads origin)
#   fetch.sh --classify-dir REC RES  offline: print the outcome table's verdict for a receipts dir and the run's
#                                    results dir (which holds collect-<k>/ trees)
# Deadlines: the run deadline is the runner's recorded start + 90,000 s; the hard deadline is creation + 26 h. "No
# information" (runner unreachable, REST unreadable, transfer check failed) is never a loss: it is retried at every
# check-in until the hard deadline, when the pod is deleted for cost (class NO_EVIDENCE unless the receipts say more).
# Every collection attempt k writes new files (collect-started-<k>.json, transfer-<k>.json, local-sha256-<k>.txt,
# tarball-<k>.sha256, admission-lines-<k>.txt, collect-<k>.json) and a new tree results/<name>/collect-<k>/; nothing is
# overwritten or removed. Every delete the session makes is logged, with its reason, before the API call.
# Exit: 0 collected (or nothing to collect) and pod deleted, confirmed gone; 41 nothing to do yet (pod untouched);
#       2 usage; 6 transfer check failed before the hard deadline (pod kept; run again); 7 pod still listed after delete
#       (run again, or reap.sh); 8 collected, pod kept until the git gate passes (at most 2 h, never past the hard
#       deadline); 9 refused: this run is finished (cost.json exists); 12 a receipt is unparsable or two collections
#       disagree: owner (pod kept until the hard deadline).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); . "$here/common.sh"

# ---------------------------------------------------------------------------------------------------------------------
# The outcome classifier (version $CLASSIFIER_VERSION, common.sh). Input: receipts only and the kept results trees.
# Output: one JSON object with class, rule (the precedence step that fired), the relaunch route for LOST, the memory
# figures, the classifier version and the SHA-256 of fetch.sh and common.sh. Precedence, first match wins:
#   P0  a receipt is unparsable (a zero-byte start.json counts as "no response", not as unparsable) -> NO_EVIDENCE
#   P0b two complete collections of the pod, both made with the runner done, have different trees -> NO_EVIDENCE
#   P1  no receipt shows a runner start                                                   -> NOT_AN_ATTEMPT
#   P2  completion secured (collection reason done, two identical fetches, a pre-fetch check-in with the runner done
#       under the recorded start, no loss evidence up to that check-in, the tree's runner_state agreeing, one start
#       line):
#       P2a ended after the run deadline                                                  -> LIMIT
#       P2b an end line exit 137 and oom_kill > 0                                         -> OOM (Amendment 14)
#       P2c an end line exit 137, no oom evidence                                         -> LOST
#       P2k cgroup oom_kill > 0, whatever the exit codes                                  -> OOM (censored at this size)
#       P2p the memory peak exceeds the pod's numeric cgroup memory.max                   -> OOM (censored at this size)
#       P2u no memory peak observable                                                     -> NOT_ADMITTED
#       P2d admission rules hold                                                          -> ADMITTED, with a reading
#       P2e otherwise                                                                     -> NOT_ADMITTED
#   Reading of an ADMITTED row at cap C (Amendment 7 B; owner decision 24): inconsistent true -> d_F = C; degree C
#   completed with inconsistent, oversize and timed_out all false -> d_F >= C+1; oversize true (no contrary stop line)
#   -> censored by size, d_F >= C stands; timed_out true without oversize (no contrary stop line: a deadline caught at
#   the top of the step loop prints none) -> censored by the
#   budget; anything else -> no reading (owner). OOM and LIMIT read as censored, d_F >= C stands.
#   P3  positive loss evidence dated before the run deadline                              -> LOST
#   P4  a check-in at or after the run deadline shows the runner running, or done after the deadline, under s0 -> LIMIT
#   P5  collected on --abort                                                              -> ABORTED
#   P6  positive loss evidence dated at or after the run deadline                         -> LOST
#   P7  collected (at the hard deadline) with none of the above                           -> NO_EVIDENCE
#   P8  not collected yet                                                                 -> PENDING
# Positive loss evidence: the runner, reachable, reports idle or a start other than s0; the REST v2 record (HTTP 200)
# shows status other than RUNNING; the pod is missing from a successful list AND its GET returns 404; the tree shows a
# second start. A changed startedAt alone is recorded but is not evidence. A LOST is relaunched automatically (relaunch
# "auto") only on host-side evidence tied to the loss (owner decision 13; see decide()): the pod gone from a successful
# list with GET 404, recorded between the last check-in that saw the runner alive and the first loss, before any action
# the session logged. v1's other form, a RunPod-side lastStatusChange, has no REST v2 field: it can no longer be
# observed, so it never counts (fail closed). Every other LOST has relaunch "owner".
# The memory peak is the pilot's own field (largest "memory R MB (peak P MB)" in runs/<cell>.log: VmHWM in MiB,
# x 1,048,576), else the cgroup figure (v2 memory.peak, else v1 memory.max_usage_in_bytes). Both are recorded.
# ---------------------------------------------------------------------------------------------------------------------
manifest() { (cd "$1" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -r -d '\n' sha256sum); }
jsonok() { [ -s "$1" ] && jq -e . "$1" >/dev/null 2>&1; }
jsonlok() { [ ! -s "$1" ] || jq -e -s . "$1" >/dev/null 2>&1; }

facts() {
  local R=$1 RES=$2 bad="" f stage cell s0 L0 ck acts cols K col O rs nstart pexit gexit oom cgp mmax man row floor
  local r1 r2 r3 r4 r5 r6 r7 cgb cgsrc logmib limit disagree lim stop
  # P0: every receipt the classifier reads must parse (fail closed).
  for f in "$R"/pod-create.json "$R"/pod-record.json "$R"/pod-state.json "$R"/pod-record-start.json "$R"/collect-[0-9]*.json "$R"/collect-started-*.json "$R"/transfer-*.json; do
    [ -e "$f" ] || continue; jsonok "$f" || bad="$bad ${f##*/}"
  done
  [ -e "$R/start.json" ] && [ -s "$R/start.json" ] && ! jsonok "$R/start.json" && bad="$bad start.json"
  for f in "$R"/checkins.jsonl "$R"/session-actions.jsonl; do [ -e "$f" ] && ! jsonlok "$f" && bad="$bad ${f##*/}"; done
  bad=${bad# }
  stage=$(jq -r '.stage // empty' "$R/pod-state.json" 2>/dev/null || true)
  [ -n "$stage" ] || stage=$(jq -r '.stages[0] // empty' "$R/start.json" 2>/dev/null || true)
  cell=""; [[ $stage =~ ^cap:([0-9]+):([01]):([0-9]+): ]] && cell="X-m2-t${BASH_REMATCH[1]}-k${BASH_REMATCH[2]}-c${BASH_REMATCH[3]}"
  ck='[]'; [ -s "$R/checkins.jsonl" ] && ck=$(jq -cs '.' "$R/checkins.jsonl" 2>/dev/null || echo '[]')
  acts='[]'; [ -s "$R/session-actions.jsonl" ] && acts=$(jq -cs '.' "$R/session-actions.jsonl" 2>/dev/null || echo '[]')
  # The collections: complete ones have collect-<k>.json. The one classified is the latest whose fetches agreed with the
  # runner done; otherwise the latest complete one.
  cols=$(for f in "$R"/collect-[0-9]*.json; do [ -e "$f" ] && jq -c --arg k "$(basename "$f" .json | sed 's/collect-//')" '. + {k:($k|tonumber)}' "$f" 2>/dev/null || true; done | jq -cs 'sort_by(.k)')
  [ -n "$cols" ] || cols='[]'
  K=$(jq -r '([.[] | select(.fetched == true and .transfer_ok == true and .prefetch.live.state == "done")] | last | .k) // (last | .k) // empty' <<<"$cols")
  col=null; O=/nonexistent
  if [ -n "$K" ]; then col=$(jq -c --argjson k "$K" '.[] | select(.k == $k)' <<<"$cols"); O="$RES/collect-$K"; fi
  # P0b: complete collections made with the runner done must have the same tree.
  disagree=false
  local mans; mans=$(for k in $(jq -r '.[] | select(.prefetch.live.state == "done") | .k' <<<"$cols"); do
    if [ -d "$RES/collect-$k" ]; then manifest "$RES/collect-$k" | sha256sum | cut -d' ' -f1; fi; done | sort -u | wc -l)
  [ "$mans" -gt 1 ] && disagree=true
  s0=$(jq -r 'select(.state == "running") | .started // empty' "$R/start.json" 2>/dev/null || true)
  [ -n "$s0" ] || s0=$(jq -r '[.[] | .live.started // empty] | first // empty' <<<"$ck")
  [ -n "$s0" ] || s0=$(jq -r '.started // empty' "$O/runner_state.json" 2>/dev/null || true)
  # REST v2 startedAt replaces v1 lastStartedAt (recorded, never evidence). v1 lastStatusChange has no v2 field.
  L0=$(jq -r '.startedAt // empty' "$R/pod-record-start.json" 2>/dev/null || true)
  [ -n "$L0" ] || L0=$(jq -r '.startedAt // empty' "$R/pod-create.json" 2>/dev/null || true)
  rs=$(jq -c '{state,exit,started,ended}' "$O/runner_state.json" 2>/dev/null || echo null)
  nstart=0 pexit=null gexit=null
  if [ -f "$O/progress.txt" ] && [ -n "$cell" ]; then
    nstart=$(grep -c " start $cell\$" "$O/progress.txt" || true)
    pexit=$(awk -v c="$cell" '$2 == "end" && $3 == c && $4 == "exit" && $5 ~ /^[0-9]+$/ { e = $5 } END { print (e == "" ? "null" : e) }' "$O/progress.txt")
    gexit=$(awk -v c="$cell.gridzeros" '$2 == "end" && $3 == c && $4 == "exit" && $5 ~ /^[0-9]+$/ { e = $5 } END { print (e == "" ? "null" : e) }' "$O/progress.txt")
  fi
  # cgroup evidence (decision D4): v2 memory.events, or v1 memory.oom_control; absent files mean "no evidence".
  oom=null cgp=false mmax=null
  if [ -f "$O/cgroup/memory.events" ]; then cgp=true; oom=$(awk '$1 == "oom_kill" { print $2 }' "$O/cgroup/memory.events"); fi
  if [ "$oom" = null ] || [ -z "$oom" ]; then
    if [ -f "$O/cgroup/memory_memory.oom_control" ]; then cgp=true; oom=$(awk '$1 == "oom_kill" { print $2 }' "$O/cgroup/memory_memory.oom_control"); fi
  fi
  [ -n "$oom" ] || oom=null
  [ -f "$O/cgroup/memory.max" ] && mmax=$(jq -n --rawfile f "$O/cgroup/memory.max" '$f | rtrimstr("\n")')
  [ "$mmax" = null ] && [ -f "$O/cgroup/memory_memory.limit_in_bytes" ] && mmax=$(jq -n --rawfile f "$O/cgroup/memory_memory.limit_in_bytes" '$f | rtrimstr("\n")')
  # The numeric memory limit the run had (P2p): the tree's cgroup memory.max, else the one read before start.
  lim=$(jq -rn --argjson m "$mmax" '$m | if type == "string" and test("^[0-9]+$") then tonumber else null end')
  if [ "$lim" = null ]; then lim=$(jq -r '(.cgroup_memory_limit // "") | if test("^[0-9]+$") then tonumber else null end' "$R/cgroup-prestart.json" 2>/dev/null || echo null); fi
  [ -n "$lim" ] || lim=null
  # The stop line of the cell's log ("budget" or "oversize"), read for the reading of section 2.
  stop=null
  if [ -n "$cell" ] && [ -f "$O/runs/$cell.log" ]; then
    stop=$(grep -oE 'tower stop in step [0-9]+ \((budget|oversize)' "$O/runs/$cell.log" | tail -1 | sed -E 's/.*\(//' || true)
    if [ -n "$stop" ]; then stop="\"$stop\""; else stop=null; fi
  fi
  # Memory peaks (P2p, P2u): the pilot's VmHWM field (MiB; primary) and the cgroup figure (bytes; fallback).
  cgb=null cgsrc=null logmib=null limit=null
  for f in memory.peak:cgroup_v2_memory.peak memory_memory.max_usage_in_bytes:cgroup_v1_memory.max_usage_in_bytes; do
    if [ "$cgb" = null ] && [ -f "$O/cgroup/${f%%:*}" ] && grep -qxE '[0-9]+' "$O/cgroup/${f%%:*}"; then cgb=$(cat "$O/cgroup/${f%%:*}"); cgsrc="\"${f#*:}\""; fi
  done
  if [ -n "$cell" ] && [ -f "$O/runs/$cell.log" ]; then
    logmib=$(grep -oE 'memory [0-9]+ MB \(peak [0-9]+ MB\)' "$O/runs/$cell.log" | sed -E 's/.*peak ([0-9]+) MB\)/\1/' | sort -n | tail -1 || true)
    [ -n "$logmib" ] || logmib=null
  fi
  case $stage in *:12800000000) limit='"56Gi"' ;; *:27900000000) limit='"112Gi"' ;; *:54000000000) limit='"238Gi"' ;; esac
  row=null
  [ -n "$cell" ] && [ -f "$O/runs/$cell.jsonl" ] && row=$(jq -cs '[.[] | select(has("oversize"))] | first // null | if . == null then null else {oversize, timed_out, inconsistent, degree_reached} end' "$O/runs/$cell.jsonl" 2>/dev/null || echo null)
  [ -n "$row" ] || row=null
  # Admission rules 1-7 (section 6), against the binary hashes pinned in common.sh.
  r1=false r2=false r3=false r4=false r5=false r6=false r7=false
  if [ -f "$O/binary.sha256" ] && cmp -s <(for b in pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check pfd_grid_zeros; do echo "$(bin_sha "$b")  /usr/local/bin/$b"; done) "$O/binary.sha256"; then r1=true; fi
  if [ -f "$O/host.txt" ] && grep -qxF "argv: $stage" "$O/host.txt" && grep -qxF "rayon: $RAYON" "$O/host.txt"; then r2=true; fi
  if [ "$nstart" = 1 ] && [ "$pexit" = 0 ] && [ "$gexit" = 0 ] && [ "$row" != null ]; then r3=true; fi
  if jq -e --arg s "$s0" '.state == "done" and .exit == 0 and .started == $s' <<<"$rs" >/dev/null 2>&1; then r4=true; fi
  if jq -e --argjson p "$(bin_sha_json)" '. as $d | $p | to_entries | all(.[]; $d.verified_sha256[.key] == .value)' "$O/runner_request.json" >/dev/null 2>&1; then r5=true; fi
  man=false
  if [ -n "$K" ] && [ -d "$O" ] && [ -f "$R/local-sha256-$K.txt" ] && cmp -s "$R/local-sha256-$K.txt" <(manifest "$O"); then man=true; fi
  if [ "$man" = true ] && jq -e '.transfer_ok == true' <<<"$col" >/dev/null 2>&1; then r6=true; fi
  floor=$(jq -r '.mem_floor_gb // empty' "$R/pod-state.json" 2>/dev/null || true); floor=${floor:-$MEM_FLOOR_GB}
  # Rule 7 on REST v2 fields: the create response (cloud, cpu.id, cpu.vcpuCount, cost = the catalog price read before
  # create), the GET record after create (cpu.memory >= the floor), the request record (cloud SECURE, cpu {id,
  # vcpuCount}, no interruptible field: v2 has none, so an interruptible pod cannot be requested).
  if jq -e --slurpfile p <(jq -s '.' "$R/price.jsonl" 2>/dev/null || echo '[]') --argjson v $VCPU --arg f "$FLAVOR" \
       '.cloud == "SECURE" and .cpu.id == $f and .cpu.vcpuCount == $v and ($p[0] | length) > 0 and ($p[0] | last | .price | type) == "number"
        and ((.cost * 10000 | round) == ($p[0] | last | .price * 10000 | round))' "$R/pod-create.json" >/dev/null 2>&1 \
     && jq -e --argjson m "$floor" '.cpu.memory >= $m' "$R/pod-record.json" >/dev/null 2>&1 \
     && jq -e --arg f "$FLAVOR" --argjson v $VCPU '.cloud == "SECURE" and .cpu == {id:$f, vcpuCount:$v} and (has("interruptible") | not)' "$R/pod-request.json" >/dev/null 2>&1; then r7=true; fi
  jq -n --arg bad "$bad" --argjson dis "$disagree" --arg stage "$stage" --arg cell "$cell" --arg s0 "$s0" --arg L0 "$L0" \
    --argjson ck "$ck" --argjson acts "$acts" --argjson col "$col" --argjson ncol "$(jq length <<<"$cols")" \
    --argjson rs "$rs" --argjson nstart "$nstart" --argjson pexit "$pexit" --argjson gexit "$gexit" --argjson oom "$oom" \
    --argjson cgp "$cgp" --argjson mmax "$mmax" --argjson row "$row" --argjson dls $DEADLINE_S --argjson lim "$lim" --argjson stop "$stop" \
    --argjson cgb "$cgb" --argjson cgsrc "$cgsrc" --argjson logmib "$logmib" --argjson size "$limit" \
    --argjson adm "{\"r1\":$r1,\"r2\":$r2,\"r3\":$r3,\"r4\":$r4,\"r5\":$r5,\"r6\":$r6,\"r7\":$r7}" \
    '{unparsable:$bad, collections_disagree:$dis, stage:$stage, cell:$cell, s0:(if $s0 == "" then null else $s0 end),
      L0:(if $L0 == "" then null else $L0 end),
      checkins:$ck, actions:$acts, collect:$col, collections:$ncol,
      tree:{runner_state:$rs, start_lines:$nstart, pilot_exit:$pexit, gridzeros_exit:$gexit,
      cgroup_present:$cgp, oom_kill:$oom, memory_max:$mmax, limit_bytes:$lim, stop:$stop}, row:$row, admission:$adm,
      mem:{size:$size, cgroup_peak_bytes:$cgb, cgroup_peak_source:$cgsrc, pilot_peak_mib:$logmib}, deadline_s:$dls}'
}

decide() { jq -c --arg ver "$CLASSIFIER_VERSION" --arg fsha "$(sha256sum "$here/fetch.sh" | cut -d' ' -f1)" \
  --arg csha "$(sha256sum "$here/common.sh" | cut -d' ' -f1)" '
  def ep: if . == null then null else fromdateiso8601 end;
  def lossev($c; $s0):
    [ (if $c.live != null and $c.live.state == "idle" then "runner reachable and idle after its start (container restarted)" else empty end),
      (if $c.live != null and $c.live.started != null and $c.live.started != $s0 then "runner reports a start other than start.json (a second start)" else empty end),
      (if $c.rest != null and $c.rest.http == 200 and ($c.rest.status // "RUNNING") != "RUNNING" then "REST record shows status \($c.rest.status)" else empty end),
      (if $c.listed == false and $c.rest != null and $c.rest.http == 404 then "pod missing from the list and GET returns 404" else empty end) ];
  . as $f | $f.s0 as $s0
  | {classifier:{version:$ver, fetch_sha256:$fsha, common_sha256:$csha}} as $cl
  | if $f.unparsable != "" then $cl + {class:"NO_EVIDENCE", rule:"P0", why:"unparsable receipt(s): \($f.unparsable); owner"}
    elif $f.collections_disagree then $cl + {class:"NO_EVIDENCE", rule:"P0b", why:"two collections made with the runner done have different trees; owner"}
    elif $s0 == null then $cl + {class:"NOT_AN_ATTEMPT", rule:"P1", why:"no receipt shows a runner start"}
    else
      (($s0 | ep) + $f.deadline_s) as $dl
    | ($f.checkins | sort_by(.at)) as $ck
    | ([ $ck[] | . as $c | lossev($c; $s0)[] | {at:$c.at, why:.} ]
       + (if $f.collect != null and $f.tree.start_lines > 1 then [{at:$f.collect.at, why:"\($f.tree.start_lines) start lines in progress.txt"}] else [] end)
       + (if $f.collect != null and $f.tree.runner_state != null and $f.tree.runner_state.started != null and $f.tree.runner_state.started != $s0
          then [{at:$f.collect.at, why:"runner_state.json start differs from start.json"}] else [] end)
      ) as $ev
    # host-side evidence (owner decision 13), tied to the loss (review round 3, R1, R2). The first loss is the earliest
    # loss evidence (or the collection, for a loss read from the tree). A host-side record counts only from a check-in
    # strictly after the last check-in that saw the runner alive under s0, or from the first-loss check-in itself; at or
    # before the first loss; and before any action the session logged for this pod (every session delete is logged
    # before its API call). The one form REST v2 can show: the pod gone from a successful list with GET 404. The v1 form
    # "a RunPod-side lastStatusChange" has no v2 field and never counts (fail closed: relaunch owner). Relaunch goes to
    # the owner also if any check-in before the first loss saw the runner done under s0 (R2).
    | ([ $f.actions[] | .at | ep ] | min) as $act0
    | (([ $ev[] | .at | ep ] | min) // ($f.collect.at | ep)) as $loss0
    | ([ $ck[] | select($loss0 != null and (.at | ep) <= $loss0 and .live != null and .live.started == $s0
           and (.live.state == "running" or .live.state == "done")) | .at | ep ] | max) as $alive
    | ([ $ck[] | select($loss0 != null and (.at | ep) < $loss0 and .live != null and .live.started == $s0 and .live.state == "done") ] | first) as $sawdone
    | ([ $ck[] | select($loss0 != null and (.at | ep) <= $loss0 and ($alive == null or (.at | ep) > $alive or (.at | ep) == $loss0)
           and ($act0 == null or (.at | ep) < $act0) and .rest != null and .listed == false and .rest.http == 404) ] | last) as $hs
    | (if $sawdone != null then {relaunch:"owner", host_side:null, relaunch_note:"a check-in at \($sawdone.at) saw the runner done before the loss"}
       elif $hs == null then {relaunch:"owner", host_side:null}
       else {relaunch:"auto", host_side:"pod gone from the list and GET 404 at \($hs.at), before any logged session action"} end) as $rl
    | ($f.collect.prefetch // null) as $pre
    | ($f.collect != null and $f.collect.reason == "done" and $f.collect.transfer_ok == true and $pre != null
       and $pre.live != null and $pre.live.state == "done" and $pre.live.started == $s0
       and ([ $ev[] | select((.at | ep) <= ($pre.at | ep)) ] | length) == 0
       and $f.tree.runner_state != null and $f.tree.runner_state.state == "done" and $f.tree.runner_state.started == $s0
       and $f.tree.runner_state.exit == $pre.live.exit and $f.tree.start_lines == 1) as $secured
    | ($f.tree.pilot_exit == 137 or $f.tree.gridzeros_exit == 137) as $k137
    | (if $f.mem.pilot_peak_mib != null then $f.mem.pilot_peak_mib * 1048576 else null end) as $pilotb
    | (if $pilotb != null then {peak_bytes:$pilotb, source:"pilot_log_VmHWM_MiB"}
       elif $f.mem.cgroup_peak_bytes != null then {peak_bytes:$f.mem.cgroup_peak_bytes, source:$f.mem.cgroup_peak_source}
       else {peak_bytes:null, source:null} end) as $pk
    | {size:$f.mem.size, peak_bytes_used:$pk.peak_bytes, source_used:$pk.source, pilot_peak_bytes:$pilotb,
       cgroup_peak_bytes:$f.mem.cgroup_peak_bytes, cgroup_peak_source:$f.mem.cgroup_peak_source} as $mem
    | (($f.tree.oom_kill // 0) > 0) as $oomk
    | ($cl + {deadline:($dl | todate), secured:$secured}) as $base
    | if $secured then
        if ($f.tree.runner_state.ended | ep) > $dl then $base + {class:"LIMIT", rule:"P2a", why:"the runner ended at \($f.tree.runner_state.ended), after the run deadline", memory:$mem}
        elif $k137 and $oomk then $base + {class:"OOM", rule:"P2b", memory:$mem, why:"exit 137 with cgroup oom_kill \($f.tree.oom_kill) (Amendment 14 OOMKilled)"}
        elif $k137 then $base + {class:"LOST", rule:"P2c", memory:$mem, why:"exit 137 without oom evidence (oom_kill \($f.tree.oom_kill // "absent"))"} + $rl
        elif $oomk then $base + {class:"OOM", rule:"P2k", memory:$mem, why:"cgroup oom_kill \($f.tree.oom_kill) (no exit 137 in the end lines); censored at this size"}
        elif $pk.peak_bytes != null and $f.tree.limit_bytes != null and $pk.peak_bytes > $f.tree.limit_bytes then $base + {class:"OOM", rule:"P2p", why:"memory peak \($pk.peak_bytes) bytes (\($pk.source)) above the pod'"'"'s cgroup memory.max \($f.tree.limit_bytes); censored at this size", memory:$mem}
        elif $pk.peak_bytes == null then $base + {class:"NOT_ADMITTED", rule:"P2u", why:"no memory peak observable (no pilot VmHWM field and no cgroup peak file)", memory:$mem}
        elif ($f.admission | all(.[]; . == true)) then $base + {class:"ADMITTED", rule:"P2d", why:"completed; admission rules 1-7 hold", row:$f.row, memory:$mem}
        else $base + {class:"NOT_ADMITTED", rule:"P2e", why:"completed, but admission rules failed: \([$f.admission | to_entries[] | select(.value != true) | .key] | join(","))", memory:$mem}
        end
      elif ([ $ev[] | select((.at | ep) < $dl) ] | length) > 0 then
        $base + {class:"LOST", rule:"P3", why:([ $ev[] | select((.at | ep) < $dl) ] | first | "\(.why) at \(.at)")} + $rl
      elif ([ $ck[] | select((.at | ep) >= $dl and .live != null and .live.started == $s0
                and (.live.state == "running" or (.live.state == "done" and (.live.ended | ep) > $dl))) ] | length) > 0 then
        $base + {class:"LIMIT", rule:"P4", why:"a check-in at or after the run deadline shows the runner not finished within it"}
      elif $f.collect != null and $f.collect.reason == "abort" then
        $base + {class:"ABORTED", rule:"P5", why:"session abort at \($f.collect.at): the attempt is used; no relaunch (owner)"}
      elif ($ev | length) > 0 then
        $base + {class:"LOST", rule:"P6", why:($ev | first | "\(.why) at \(.at), at or after the run deadline")} + $rl
      elif $f.collect != null then
        $base + {class:"NO_EVIDENCE", rule:"P7", why:"collected (reason \($f.collect.reason)) with no secured completion and no positive evidence of loss or of the limit; owner"}
      else $base + {class:"PENDING", rule:"P8", why:"not collected yet"}
      end
    end
  # An OOM with the peak well under the pod's limit points at a kill from outside this run's size (review 4 S8): noted.
  | if .class == "OOM" and (.memory.peak_bytes_used // null) != null and $f.tree.limit_bytes != null
         and .memory.peak_bytes_used < 0.9 * $f.tree.limit_bytes
    then . + {oom_note:"peak below 90 percent of memory.max: a host-level or group kill is likely, not a sign that this --max-dense is too small"} else . end
  # The reading (Amendment 7 B at cap C; owner decision 24), from the class and the row only.
  | ([ $f.stage | capture("^cap:[0-9]+:[01]:(?<c>[0-9]+):")? | .c | tonumber ] | first) as $cap
  | . + (if $cap == null then {} else {reading:(
      if .class == "ADMITTED" then (.row // {}) as $r
        | if $r.inconsistent == true then {d_F:"= \($cap)", why:"refutes at cap \($cap)"}
          elif $r.oversize == true and ($f.tree.stop // "oversize") == "oversize" then {d_F:">= \($cap)", censored:"size", why:"stopped oversize at this --max-dense; censored, the bound from the earlier caps stands"}
          elif $r.oversize == false and $r.timed_out == true and ($f.tree.stop // "budget") == "budget" then {d_F:">= \($cap)", censored:"budget", why:"stopped by the wall budget; censored, the bound from the earlier caps stands"}
          elif $r.inconsistent == false and $r.oversize == false and $r.timed_out == false and $r.degree_reached == $cap then {d_F:">= \($cap + 1)", why:"degree \($cap) completed without refuting"}
          else {d_F:null, why:"no reading: the row and the stop line do not match a case of the table; owner"} end
      elif .class == "OOM" then {d_F:">= \($cap)", censored:"memory", why:"memory kill or a peak above the limit; censored, the bound from the earlier caps stands"}
      elif .class == "LIMIT" then {d_F:">= \($cap)", censored:"time limit", why:"stopped at the job time limit; censored, the bound from the earlier caps stands"}
      else null end)} end)
  + {facts:{tree:$f.tree, admission:$f.admission, mem:$f.mem, L0:$f.L0, s0:$f.s0, cell:$f.cell, collections:$f.collections}}'; }
classify() { facts "$1" "$2" | decide; }

if [ "${1:-}" = --classify-dir ]; then
  [ $# -eq 3 ] || { echo "usage: fetch.sh --classify-dir REC RES" >&2; exit 2; }
  classify "$2" "$3"; exit 0
fi

# ---------------------------------------------------------------------------------------------------------------------
[ $# -ge 1 ] || { echo "usage: fetch.sh NAME [--abort | --delete REF]" >&2; exit 2; }
NAME=$1 MODE=${2:-} GREF=${3:-}
case $MODE in ""|--abort) ;; --delete) [ -n "$GREF" ] || { echo "usage: fetch.sh NAME --delete REF" >&2; exit 2; } ;;
  *) echo "usage: fetch.sh NAME [--abort | --delete REF]" >&2; exit 2 ;; esac
REC=$(rec_dir "$NAME") || { echo "unknown name $NAME" >&2; exit 2; }
[ -d "$REC" ] || { echo "no receipts $REC" >&2; exit 2; }
[ ! -f "$REC/cost.json" ] || { echo "REFUSED: $NAME is finished ($REC/cost.json exists); nothing is collected or deleted again" >&2; exit 9; }
if is_study "$NAME"; then frozen_ok fetch.sh common.sh || { echo "REFUSED: fetch.sh or common.sh is not the frozen version (S3)" >&2; exit 3; }; fi
RES="$OUT_ROOT/$NAME"
log() { echo "$(now) $*" | tee -a "$REC/fetch.log" >&2; }
rpc_init; trap 'rpc_cleanup; remind "${REC#"$here/"}"' EXIT
exec 9>"$RPC_STATE/$NAME.lock"; flock -w 900 9 || { echo "another run.sh, fetch.sh or reap.sh holds $NAME" >&2; exit 2; }
id=$(jq -r '.id // empty' "$REC/pod-create.json" 2>/dev/null || true)
[ -n "$id" ] || { echo "no pod id in $REC/pod-create.json" >&2; exit 2; }
nowe() { date +%s; }
created_e=$(epoch "$(jq -r '.createdAt' "$REC/pod-create.json")")
hard=$(jq -r '.hard_deadline // empty' "$REC/pod-state.json" 2>/dev/null || true)
if [ -n "$hard" ]; then hard=$(epoch "$hard"); else hard=$(( created_e + HARD_S )); fi

# --- a missing pod token is recovered from the REST v2 record's env (N6; GET /pods/{id} returns env, V2_FIELDS.md); it goes to RPC_STATE only, never to receipts
if [ ! -s "$RPC_STATE/$id.token" ]; then
  c=$(api_code GET "$API/pods/$id" "$RPC_STATE/$NAME.tok.tmp")
  t=""; [ "$c" = 200 ] && t=$(jq -r '.env.RUNNER_TOKEN // empty' "$RPC_STATE/$NAME.tok.tmp" 2>/dev/null || true)
  rm -f "$RPC_STATE/$NAME.tok.tmp" "$RPC_STATE/$NAME.tok.tmp.err"
  if [ -n "$t" ]; then ( umask 077; printf '%s' "$t" > "$RPC_STATE/$id.token" ); log "pod token recovered from the REST pod record (written to RPC_STATE only)"
  else log "no pod token here and none in the REST record (HTTP $c); the runner is unreachable from this session"; fi
fi

checkin() { # checkin PHASE: one live status, one REST record and one list membership; one atomic line in checkins.jsonl
  local stamp at live c rest listed l line
  stamp=$(date -u +%Y%m%dT%H%M%SZ)-$1; at=$(now)
  timeout 60 "$RP" status "$id" > "$REC/status-$stamp.json" 2>/dev/null || true
  live=$(jq -c 'if type == "object" and has("state") then {state,exit,started,ended} else null end' "$REC/status-$stamp.json" 2>/dev/null || echo null)
  [ -n "$live" ] || live=null
  c=$(api_code GET "$API/pods/$id" "$RPC_STATE/$NAME.rest.tmp")
  if [ "$c" = 200 ] && jsonok "$RPC_STATE/$NAME.rest.tmp"; then redact < "$RPC_STATE/$NAME.rest.tmp" > "$REC/rest-$stamp.json"
    rest=$(jq -c --argjson c "$c" '{http:$c, status, startedAt}' "$REC/rest-$stamp.json")
  else rest="{\"http\":${c:-0}}"; fi
  rm -f "$RPC_STATE/$NAME.rest.tmp" "$RPC_STATE/$NAME.rest.tmp.err"
  if l=$(list_pkm 2>/dev/null); then listed=$(jq -s --arg i "$id" 'any(.[]; .id == $i)' <<<"$l"); else listed=null; fi
  line=$(jq -nc --arg at "$at" --arg ph "$1" --argjson live "$live" --argjson rest "$rest" --argjson listed "$listed" \
    '{at:$at, phase:$ph, live:$live, rest:$rest, listed:$listed}')
  printf '%s\n' "$line" >> "$REC/checkins.jsonl"; printf '%s\n' "$line"
}

delete_phase() { # $1 reason. Log the action, delete, confirm by listing, write cost.json, close the ledger row.
  local stamp c i gone=0 deleted="" l ub=false
  stamp=$(date -u +%Y%m%dT%H%M%SZ); touch "$REC/delete-pending"
  session_action "$REC" DELETE "$id" "$1"
  c=$(api_code GET "$API/pods/$id" "$RPC_STATE/$NAME.final.tmp")
  [ "$c" = 200 ] && redact < "$RPC_STATE/$NAME.final.tmp" > "$REC/pod-record-final-$stamp.json"
  rm -f "$RPC_STATE/$NAME.final.tmp" "$RPC_STATE/$NAME.final.tmp.err"
  for i in 1 2 3; do
    c=$(api_code DELETE "$API/pods/$id" "$REC/delete-$stamp-$i.response"); echo "$(now) DELETE $id HTTP $c" >> "$REC/delete.http"
    case $c in 200|204) deleted=$(now); break ;; 404) ub=true; break ;; esac; sleep "$POLL_S"
  done
  log "delete pod $id ($1): HTTP $c"
  for i in $(seq 1 12); do
    if l=$(list_pkm 2>/dev/null); then
      if ! jq -se --arg i "$id" 'any(.[]; .id == $i)' <<<"$l" >/dev/null; then gone=1; echo "$l" > "$REC/pods-after-delete-$stamp.jsonl"; break; fi
    fi; sleep "$POLL_S"
  done
  [ "$gone" = 1 ] || { log "POD $id STILL LISTED (or the list is unreadable) after delete; run fetch.sh $NAME again or reap.sh"; exit 7; }
  [ -n "$deleted" ] || { deleted=$(now); ub=true; }
  local p s usd
  p=$(jq -r '.cost' "$REC/pod-create.json")
  s=$(( $(epoch "$deleted") - created_e )); usd=$(awk -v p="$p" -v s="$s" 'BEGIN{printf "%.4f", p * s / 3600}')
  jq -n --arg id "$id" --arg c "$(date -u -d "@$created_e" +%FT%TZ)" --arg d "$deleted" --argjson s "$s" --argjson p "$p" --argjson u "$usd" \
    --argjson ub "$ub" --arg r "$1" --arg cl "$(jq -r '.class // "unclassified"' "$REC/outcome.json" 2>/dev/null || echo unclassified)" \
    '{pod:$id, created:$c, deleted:$d, deleted_time_is_upper_bound:$ub, seconds:$s, cost_per_hr:$p, usd_derived:$u,
      delete_reason:$r, class:$cl, confirmed_gone:true}' > "$REC/cost.json"
  log "pod $id deleted and no longer listed; cost (derived: cost per hour x pod lifetime) US\$$usd over $s s at $p/h"
  rm -f "$REC/delete-pending"
  if [ "$(jq -r '.class' "$REC/outcome.json" 2>/dev/null || true)" = NOT_AN_ATTEMPT ]; then
    local k=1; while [ -e "$REC.prestart-$k" ]; do k=$((k + 1)); done
    ledger_close "$NAME/$id" "$usd" "${REC#"$here/"}.prestart-$k/cost.json"
    log "not an attempt: receipts moved to $REC.prestart-$k"; mv "$REC" "$REC.prestart-$k"; REC="$REC.prestart-$k"
  else ledger_close "$NAME/$id" "$usd" "${REC#"$here/"}/cost.json"; fi
  exit 0
}

git_gate() { # the git gate (owner decision 22: no S3). 0 when the kept tree of the latest collection, outcome.json and
  # local-sha256-<k>.txt are in commit GREF with the blob hash of each local file, each local file of the tree still has
  # its manifest SHA-256, and GREF is an ancestor of a branch head origin lists now. Every check is logged.
  local K top c rr rq h onorigin=0 n=0 bad=0 sha p f want got
  K=$(ls "$REC"/collect-[0-9]*.json 2>/dev/null | sed 's/.*collect-\([0-9]*\)\.json/\1/' | sort -n | tail -1 || true)
  [ -n "$K" ] && [ -f "$REC/local-sha256-$K.txt" ] && [ -d "$RES/collect-$K" ] && [ -f "$REC/outcome.json" ] || return 1
  top=$(git -C "$here" rev-parse --show-toplevel 2>/dev/null) || return 1; top=$(realpath "$top")
  c=$(git -C "$top" rev-parse --verify -q "$GREF^{commit}" 2>/dev/null) || { echo "$(now) ref $GREF: not a commit here" >> "$REC/git-gate.txt"; return 1; }
  rr=$(realpath --relative-to="$top" "$RES/collect-$K") rq=$(realpath --relative-to="$top" "$REC")
  declare -A blob=()
  while IFS=$'\t' read -r meta p; do blob[$p]=${meta##* }; done < <(git -C "$top" ls-tree -r --full-tree "$c" -- "$rr" "$rq/outcome.json" "$rq/local-sha256-$K.txt")
  while read -r sha p; do
    n=$((n + 1)); f="$RES/collect-$K/$p"
    [ -f "$f" ] && [ "$(sha256sum "$f" | cut -d' ' -f1)" = "$sha" ] || { bad=$((bad + 1)); echo "  manifest mismatch or missing: $p" >> "$REC/git-gate.txt"; continue; }
    want=$(git -C "$top" hash-object -- "$f"); got=${blob[$rr/$p]:-}
    [ "$got" = "$want" ] || { bad=$((bad + 1)); echo "  not in $c with this content: $rr/$p" >> "$REC/git-gate.txt"; }
  done < "$REC/local-sha256-$K.txt"
  for f in outcome.json "local-sha256-$K.txt"; do
    want=$(git -C "$top" hash-object -- "$REC/$f"); got=${blob[$rq/$f]:-}
    [ "$got" = "$want" ] || { bad=$((bad + 1)); echo "  not in $c with this content: $rq/$f" >> "$REC/git-gate.txt"; }
  done
  for h in $(git -C "$top" ls-remote --heads origin 2>/dev/null | awk '{ print $1 }'); do
    git -C "$top" cat-file -e "$h^{commit}" 2>/dev/null && git -C "$top" merge-base --is-ancestor "$c" "$h" 2>/dev/null && { onorigin=1; break; }
  done
  echo "$(now) ref $GREF commit $c collection $K: files in the kept tree $n, mismatches $bad, on an origin branch head now: $onorigin" >> "$REC/git-gate.txt"
  [ "$n" -gt 0 ] && [ "$bad" = 0 ] && [ "$onorigin" = 1 ]
}

# --- a delete that was not confirmed earlier resumes first
[ -f "$REC/delete-pending" ] && { log "resuming an unconfirmed delete"; delete_phase "resumed: $(jq -r 'select(.action == "DELETE") | .reason' "$REC/session-actions.jsonl" 2>/dev/null | tail -1)"; }

# --- orphan recovery (N3b): a collection whose tree was kept but whose collect-<k>.json was never written
for f in "$REC"/collect-started-*.json; do
  [ -e "$f" ] || continue; k=${f##*collect-started-}; k=${k%.json}
  [ -e "$REC/collect-$k.json" ] && continue
  if [ -d "$RES/collect-$k" ]; then
    ok=false
    [ -f "$REC/local-sha256-$k.txt" ] && cmp -s "$REC/local-sha256-$k.txt" <(manifest "$RES/collect-$k") \
      && jq -e '.transfer_ok == true' "$REC/transfer-$k.json" >/dev/null 2>&1 && ok=true
    jq --argjson ok "$ok" '. + {fetched:true, transfer_ok:$ok, recovered:true}' "$f" > "$REC/collect-$k.json"
    log "orphan collection $k recovered (tree kept in $RES/collect-$k; manifest and transfer check verified: $ok)"
  fi
done

s0=$(jq -r 'select(.state == "running") | .started // empty' "$REC/start.json" 2>/dev/null || true)
last=$(checkin regular)
[ -n "$s0" ] || s0=$(jq -r '.live.started // empty' <<<"$last")
dl=""; [ -n "$s0" ] && dl=$(( $(epoch "$s0") + DEADLINE_S ))

# --- collected with the runner done: wait for the git gate (at most PUSH_WAIT_S, never past the hard deadline)
lastK=$(ls "$REC"/collect-[0-9]*.json 2>/dev/null | sed 's/.*collect-\([0-9]*\)\.json/\1/' | sort -n | tail -1 || true)
if [ -n "$lastK" ] && jq -e '.reason == "done" and .transfer_ok == true' "$REC/collect-$lastK.json" >/dev/null 2>&1; then
  [ -f "$REC/outcome.json" ] || { classify "$REC" "$RES" > "$REC/outcome.json"; log "OUTCOME (late) $(jq -c '{class,rule,why}' "$REC/outcome.json")"; }
  if [ "$MODE" = --delete ]; then
    if git_gate; then log "git gate passed: the kept tree is committed in $GREF and on origin"; delete_phase "completed; git gate passed ($GREF)"; fi
    log "git gate not passed for $GREF (git-gate.txt)"
  fi
  until_e=$(( $(epoch "$(jq -r .at "$REC/collect-$lastK.json")") + PUSH_WAIT_S )); [ "$until_e" -le "$hard" ] || until_e=$hard
  [ "$(nowe)" -ge "$until_e" ] && { log "push wait over: deleting without the git gate (the tree is local)"; delete_phase "completed; push wait over"; }
  [ "$MODE" = --abort ] && { log "abort: deleting without the git gate (the collected tree is local)"; delete_phase "completed; session abort before the git gate"; }
  log "collected; pod kept until the git gate passes (fetch.sh $NAME --delete REF) or $(date -u -d "@$until_e" +%FT%TZ)"; exit 8
fi

# --- no runner start on record: a launch in progress, or a launcher that died before start
if [ -z "$s0" ]; then
  if [ "$MODE" = --abort ] || [ $(( $(nowe) - created_e )) -gt $(( START_WITHIN_S + 900 )) ]; then
    [ -f "$REC/outcome.json" ] || jq -n --arg v "$CLASSIFIER_VERSION" '{class:"NOT_AN_ATTEMPT", rule:"P1", why:"no receipt shows a runner start (launcher stopped before start)", classifier:{version:$v}}' > "$REC/outcome.json"
    log "no runner start on record; deleting the pod (not an attempt)"; delete_phase "no runner start (not an attempt)"
  fi
  log "no runner start on record yet (launch in progress?)"; exit 41
fi

# --- decide whether to collect: the classifier on the receipts so far, then the deadlines and the mode
pre=$(classify "$REC" "$RES" | jq -r '.rule')
lstate=$(jq -r '.live.state // "unreachable"' <<<"$last"); lstarted=$(jq -r '.live.started // empty' <<<"$last")
if [ "$pre" = P0 ] || [ "$pre" = P0b ]; then
  [ "$(nowe)" -ge "$hard" ] || { log "receipts unparsable or collections disagree ($pre): owner; pod kept until the hard deadline"; exit 12; }
  reason=hard
elif [ "$pre" = P3 ]; then reason=lost
elif [ "$lstate" = done ] && [ "$lstarted" = "$s0" ]; then reason=done
elif [ "$pre" = P4 ]; then reason=deadline
elif [ "$(nowe)" -ge "$hard" ]; then reason=hard
elif [ "$MODE" = --abort ]; then reason=abort
else log "state $lstate; run deadline $(date -u -d "@$dl" +%FT%TZ), hard deadline $(date -u -d "@$hard" +%FT%TZ); nothing to collect yet"; exit 41; fi
log "collecting: reason $reason (live state $lstate)"

# --- collection k: an intent record, then two fetches whose per-file manifests must agree; new files only
k=1; while [ -e "$REC/collect-started-$k.json" ]; do k=$((k + 1)); done
prefetch=null fetched=false tok=false
[ "$lstate" != unreachable ] && { [ "$reason" != done ] || {
  w=$(( $(epoch "$(jq -r '.live.ended' <<<"$last")") + 30 - $(nowe) )); [ "$w" -gt 0 ] && [ "$w" -le 60 ] && sleep "$w"; true; }; prefetch=$(checkin prefetch); }
jq -n --arg at "$(now)" --arg r "$reason" --arg m "$MODE" --argjson pre "$prefetch" --arg o "${RES#"$here/"}/collect-$k" \
  '{at:$at, reason:$r, mode:$m, prefetch:$pre, out:$o}' > "$REC/collect-started-$k.json"
if [ "$lstate" != unreachable ]; then
  wd="$RPC_STATE/$NAME.fetch-$k-$(date -u +%Y%m%dT%H%M%SZ)"; mkdir -p "$wd/a" "$wd/b"; okn=0
  for x in a b; do
    if timeout 900 "$RP" fetch "$id" "$wd/$x.tar.gz" >> "$REC/fetch.log" 2>&1 && gzip -t "$wd/$x.tar.gz" 2>/dev/null \
       && tar -xzf "$wd/$x.tar.gz" -C "$wd/$x" 2>/dev/null && [ -d "$wd/$x/results" ]; then
      manifest "$wd/$x/results" > "$wd/$x.sha256"; okn=$((okn + 1))
    else log "fetch $x failed"; fi
  done
  keep=""
  if [ "$okn" = 2 ] && cmp -s "$wd/a.sha256" "$wd/b.sha256"; then tok=true; keep=a
  elif [ "$okn" = 2 ]; then diff "$wd/a.sha256" "$wd/b.sha256" >> "$REC/fetch.log" 2>&1 || true; log "the two fetches differ"; keep=b
  elif [ "$okn" = 1 ]; then if [ -f "$wd/a.sha256" ]; then keep=a; else keep=b; fi; fi
  if [ "$reason" = done ] && [ "$tok" = false ] && [ "$(nowe)" -lt "$hard" ]; then
    log "TRANSFER CHECK FAILED (no information); pod $id kept, run fetch.sh $NAME again (collection $k abandoned)"; rm -rf "$wd"; exit 6
  fi
  if [ -n "$keep" ]; then
    fetched=true
    jq -n --argjson t "$tok" '{transfer_ok:$t}' > "$REC/transfer-$k.json"
    cp "$wd/$keep.sha256" "$REC/local-sha256-$k.txt"; (cd "$wd" && sha256sum "$keep.tar.gz") > "$REC/tarball-$k.sha256"
    mkdir -p "$RES"; mv "$wd/$keep/results" "$RES/collect-$k"
    cmp -s "$REC/local-sha256-$k.txt" <(manifest "$RES/collect-$k") || { log "kept tree differs from its fetched manifest"; tok=false; }
    o="$RES/collect-$k"; cl=$(sed -n 's/^.*start \(X-m2-[^ ]*\)$/\1/p' "$o/progress.txt" 2>/dev/null | head -1 || true)
    { echo "--- binary.sha256"; cat "$o/binary.sha256" 2>/dev/null || echo MISSING
      echo "--- host.txt (first 7 lines)"; head -7 "$o/host.txt" 2>/dev/null || echo MISSING
      echo "--- progress.txt"; cat "$o/progress.txt" 2>/dev/null || echo MISSING
      echo "--- runner_state.json"; cat "$o/runner_state.json" 2>/dev/null || echo MISSING
      echo "--- largest pilot memory peak line (VmHWM, MiB)"
      { [ -n "$cl" ] && grep -E 'memory [0-9]+ MB \(peak [0-9]+ MB\)' "$o/runs/$cl.log" 2>/dev/null \
          | awk '{ match($0, /peak [0-9]+ MB\)/); v = substr($0, RSTART + 5, RLENGTH - 9) + 0; if (v >= m) { m = v; l = $0 } } END { if (l != "") print l; else print "NONE" }'; } || echo NONE
      echo "--- cgroup"; for f in "$o"/cgroup/*; do [ -f "$f" ] && { echo "${f##*/}:"; cat "$f"; }; done 2>/dev/null || echo MISSING
    } > "$REC/admission-lines-$k.txt"
    log "results in $RES/collect-$k: $(wc -l < "$REC/local-sha256-$k.txt") files; transfer check $tok"
  fi
  rm -rf "$wd"
fi
jq --argjson f "$fetched" --argjson t "$tok" '. + {fetched:$f, transfer_ok:$t}' "$REC/collect-started-$k.json" > "$REC/collect-$k.json"
[ -f "$REC/outcome.json" ] || classify "$REC" "$RES" > "$REC/outcome.json"
log "OUTCOME $(jq -c '{class,rule,relaunch,why}' "$REC/outcome.json")"
if [ "$reason" = done ] && [ "$tok" = true ] && [ "$(nowe)" -lt "$hard" ] && [ "$MODE" != --abort ]; then
  log "commit and push the kept tree and the receipts (git add ${RES#"$here/"}/collect-$k ${REC#"$here/"}; git commit; git push), then fetch.sh $NAME --delete <pushed commit> (pod kept at most $((PUSH_WAIT_S / 3600)) h)"
  exit 8
fi
delete_phase "collected: reason $reason"
