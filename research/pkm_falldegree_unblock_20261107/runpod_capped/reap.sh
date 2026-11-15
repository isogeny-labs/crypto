#!/usr/bin/env bash
# reap.sh [--dry-run]  -- the backstop for the per-job ceiling (draft Amendment 18, section 2.4). Shell only. Idempotent.
# Lists the account's pods. A pod whose name starts with pkm-x- is due when it is older than 26 h since creation, or past
# its recorded run deadline (the runner's start in start.json + 90,000 s), or has receipts but no runner start 70 min
# after creation (a launcher that died before start). For a due pod with receipts it runs
# fetch.sh NAME, which at or after the run deadline collects what it can and deletes the pod even if the transfer fails;
# a due pod that is still listed afterwards (or has no receipts, or no pod token here) is deleted directly. Every pass
# appends one JSON line per pkm-x- pod to reaper/reap-YYYYMMDD.jsonl. It never touches a pod outside pkm-x-.
# The main session runs it as a scheduled check-in (section 2.4): the US$40 ceiling relies on the session or this.
# Exit: 0 no due pod remains listed; 1 the pod list could not be read; 7 a due pod is still listed.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); . "$here/common.sh"
DRY=0; [ "${1:-}" = --dry-run ] && DRY=1
rpc_init; trap rpc_cleanup EXIT
mkdir -p "$REAP_DIR"; rlog="$REAP_DIR/reap-$(date -u +%Y%m%d).jsonl"
note() { jq -nc --arg at "$(now)" "$@" >> "$rlog"; }
pods=$(list_pkm) || { note --arg e "pod list not readable" '{at:$at, error:$e}'; echo "pod list not readable" >&2; exit 1; }
[ -n "$pods" ] || { note '{at:$at, pkm_x_pods:0}'; echo "no pkm-x- pod"; exit 0; }
left=0
while read -r p; do
  id=$(jq -r .id <<<"$p"); name=$(jq -r .name <<<"$p")
  ce=$(epoch "$(jq -r .createdAt <<<"$p" | sed 's/ UTC$//')"); age=$(( $(date +%s) - ce ))
  rec="" s0="" dl=""
  for d in "$REC_ROOT/$name" "$DEV_ROOT/$name"; do
    [ -f "$d/pod-create.json" ] && [ "$(jq -r '.id // empty' "$d/pod-create.json")" = "$id" ] && { rec=$d; break; }
  done
  [ -n "$rec" ] && s0=$(jq -r 'select(.state == "running") | .started // empty' "$rec/start.json" 2>/dev/null || true)
  [ -n "$s0" ] && dl=$(( $(epoch "$s0") + DEADLINE_S ))
  due=0; why=""
  [ "$age" -ge "$HARD_S" ] && { due=1; why="older than 26 h"; }
  [ -n "$dl" ] && [ "$(date +%s)" -ge "$dl" ] && { due=1; why="${why:+$why; }past its run deadline"; }
  [ -n "$rec" ] && [ -z "$s0" ] && [ "$age" -gt $(( START_WITHIN_S + 900 )) ] && { due=1; why="${why:+$why; }no runner start within $(( (START_WITHIN_S + 900) / 60 )) min of creation"; }
  if [ "$due" = 0 ]; then
    note --arg id "$id" --arg n "$name" --argjson a "$age" --arg r "${rec#"$here/"}" '{at:$at, pod:$id, name:$n, age_s:$a, receipts:$r, due:false}'
    continue
  fi
  action="" rc=""
  if [ "$DRY" = 1 ]; then action="dry-run: would collect and delete"
  else
    if [ -n "$rec" ] && [ ! -f "$rec/cost.json" ] && [ -f "$RPC_STATE/$id.token" ]; then
      set +e; "$here/fetch.sh" "$name" </dev/null > "$REAP_DIR/fetch-$name-$(date -u +%Y%m%dT%H%M%SZ).out" 2>&1; rc=$?; set -e
      action="fetch.sh exit $rc"
    fi
    if list_pkm 2>/dev/null | jq -se --arg i "$id" 'any(.[]; .id == $i)' >/dev/null; then
      c=$(api_code DELETE "$API/pods/$id" "$REAP_DIR/delete-$id-$(date -u +%Y%m%dT%H%M%SZ).response")
      action="${action:+$action; }direct DELETE HTTP $c"
    fi
  fi
  gone=false
  for i in $(seq 1 12); do
    [ "$DRY" = 1 ] && break
    if l=$(list_pkm 2>/dev/null); then jq -se --arg i "$id" 'any(.[]; .id == $i)' <<<"$l" >/dev/null || { gone=true; break; }; fi
    sleep "$POLL_S"
  done
  [ "$gone" = true ] || [ "$DRY" = 1 ] || left=$((left + 1))
  note --arg id "$id" --arg n "$name" --argjson a "$age" --arg r "${rec#"$here/"}" --arg w "$why" --arg act "$action" --argjson g "$gone" \
    '{at:$at, pod:$id, name:$n, age_s:$a, receipts:$r, due:true, why:$w, action:$act, gone:$g}'
  echo "$name ($id): due ($why); $action; gone $gone"
done <<<"$pods"
[ "$left" = 0 ] || { echo "$left due pkm-x- pod(s) still listed: tell the owner" >&2; exit 7; }
