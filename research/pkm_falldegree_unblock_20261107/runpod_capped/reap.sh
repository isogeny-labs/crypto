#!/usr/bin/env bash
# reap.sh [--dry-run]  -- the backstop for the per-job ceiling (Amendment 33 draft, section 2.6). Shell only. Idempotent.
# Lists the account's pods (REST v2 GET /pods, every page). A pod whose name starts with pkm-x- is due only when it is past its hard deadline: the
# hard_deadline in its receipts' pod-state.json, or creation + 26 h when no deadline was recorded (owner decision 14,
# main-session decision N3c). It never deletes a pod before that, whatever fetch.sh says. For a due pod with receipts it
# runs fetch.sh NAME, which at the hard deadline collects what it can and deletes the pod (it recovers a missing pod
# token from the REST record); a due pod still listed afterwards is deleted directly, the action logged with its reason
# before the API call. A pod whose deadline cannot be read is not deleted: it is reported (exit 1). It takes the same
# per-name lock as run.sh and fetch.sh; a pod whose lock is held is skipped for this pass and the skip is recorded. Every pass appends
# one JSON line per pkm-x- pod to reaper/reap-YYYYMMDD.jsonl. It never touches a pod outside pkm-x-.
# The main session runs it as a scheduled check-in at most every 4 h (section 2.6).
# Exit: 0 no due pod remains listed; 1 the pod list, or a pod's deadline, could not be read; 7 a due pod is still listed.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); . "$here/common.sh"
DRY=0; [ "${1:-}" = --dry-run ] && DRY=1
rpc_init; trap rpc_cleanup EXIT
mkdir -p "$REAP_DIR"; rlog="$REAP_DIR/reap-$(date -u +%Y%m%d).jsonl"
note() { jq -nc --arg at "$(now)" "$@" >> "$rlog"; }
pods=$(list_pkm) || { note --arg e "pod list not readable" '{at:$at, error:$e}'; echo "pod list not readable" >&2; exit 1; }
[ -n "$pods" ] || { note '{at:$at, pkm_x_pods:0}'; echo "no pkm-x- pod"; exit 0; }
left=0 errs=0 locked=0
while read -r p; do
  id=$(jq -r .id <<<"$p"); name=$(jq -r .name <<<"$p")
  rec="" hd="" err=0
  for d in "$REC_ROOT/$name" "$DEV_ROOT/$name"; do
    [ -f "$d/pod-create.json" ] && [ "$(jq -r '.id // empty' "$d/pod-create.json" 2>/dev/null)" = "$id" ] && { rec=$d; break; }
  done
  [ -n "$rec" ] && hd=$(jq -r '.hard_deadline // empty' "$rec/pod-state.json" 2>/dev/null || true)
  if [ -n "$hd" ] && hde=$(epoch "$hd" 2>/dev/null); then src="pod-state.json hard_deadline"
  elif hde=$(epoch "$(jq -r '.createdAt // empty' <<<"$p")" 2>/dev/null) && [ -n "$(jq -r '.createdAt // empty' <<<"$p")" ]; then
    hde=$(( hde + HARD_S )); src="creation + 26 h"
  else err=1; hde=""; src="unreadable"; fi
  age=null; due=0; why=""
  if [ "$err" = 1 ]; then errs=$((errs + 1)); why="deadline unreadable: not deleted"
  elif [ "$(date +%s)" -ge "$hde" ]; then due=1; why="past its hard deadline ($src)"; fi
  if [ "$due" = 0 ]; then
    note --arg id "$id" --arg n "$name" --arg h "${hde:+$(date -u -d "@$hde" +%FT%TZ)}" --arg r "${rec#"$here/"}" --arg w "$why" '{at:$at, pod:$id, name:$n, hard_deadline:$h, receipts:$r, due:false, note:$w}'
    continue
  fi
  action="" rc=""
  # S1 (review round 3): the same per-name lock as run.sh and fetch.sh. Held by another process: skip this pod this pass.
  lockf="$RPC_STATE/$(printf '%s' "$name" | tr -c 'a-z0-9-' '_').lock"
  exec 8>"$lockf"
  if ! flock -n 8; then exec 8>&-; locked=$((locked + 1))
    note --arg id "$id" --arg n "$name" --arg w "$why" '{at:$at, pod:$id, name:$n, due:true, why:$w, action:"skipped: lock held by another run.sh, fetch.sh or reap.sh"}'
    echo "$name ($id): due ($why); lock held, skipped this pass"; continue; fi
  exec 8>&-                                   # released for fetch.sh, which takes the same lock itself
  if [ "$DRY" = 1 ]; then action="dry-run: would collect and delete"
  else
    if [ -n "$rec" ] && [ ! -f "$rec/cost.json" ]; then
      set +e; "$here/fetch.sh" "$name" </dev/null > "$REAP_DIR/fetch-$name-$(date -u +%Y%m%dT%H%M%SZ).out" 2>&1; rc=$?; set -e
      action="fetch.sh exit $rc"
    fi
    exec 8>"$lockf"
    if ! flock -w 60 8; then action="${action:+$action; }direct DELETE skipped: lock held"
    elif list_pkm 2>/dev/null | jq -se --arg i "$id" 'any(.[]; .id == $i)' >/dev/null; then
      session_action "$REAP_DIR" DELETE "$id" "reaper: $why"; [ -n "$rec" ] && session_action "$rec" DELETE "$id" "reaper: $why"
      c=$(api_code DELETE "$API/pods/$id" "$REAP_DIR/delete-$id-$(date -u +%Y%m%dT%H%M%SZ).response")
      action="${action:+$action; }direct DELETE HTTP $c"
    fi
    exec 8>&-
  fi
  gone=false
  for i in $(seq 1 12); do
    [ "$DRY" = 1 ] && break
    if l=$(list_pkm 2>/dev/null); then jq -se --arg i "$id" 'any(.[]; .id == $i)' <<<"$l" >/dev/null || { gone=true; break; }; fi
    sleep "$POLL_S"
  done
  [ "$gone" = true ] || [ "$DRY" = 1 ] || left=$((left + 1))
  note --arg id "$id" --arg n "$name" --arg h "$(date -u -d "@$hde" +%FT%TZ)" --arg r "${rec#"$here/"}" --arg w "$why" --arg act "$action" --argjson g "$gone" \
    '{at:$at, pod:$id, name:$n, hard_deadline:$h, receipts:$r, due:true, why:$w, action:$act, gone:$g}'
  echo "$name ($id): due ($why); $action; gone $gone"
done <<<"$pods"
[ "$left" = 0 ] || { echo "$left due pkm-x- pod(s) still listed: tell the owner" >&2; exit 7; }
[ "$locked" = 0 ] || echo "$locked due pkm-x- pod(s) skipped: lock held (next pass)" >&2
[ "$errs" = 0 ] || { echo "$errs pkm-x- pod(s) with an unreadable deadline (not deleted): tell the owner" >&2; exit 1; }
remind "runpod_capped/reaper/"
