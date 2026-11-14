#!/usr/bin/env bash
# fetch.sh NAME [--abort]  -- collect one run started by run.sh, check the transfer by SHA-256, delete the pod
# (draft Amendment 18, AMENDMENT_R2_H_runpod_capped.md). Shell only.
# While the runner is still running and the pod deadline (run.sh start + 90,000 s) has not passed, it records one status
# and exits 41 without touching the pod. Once the runner is done, or the deadline has passed, or --abort is given, it:
#   fetches /results.tar.gz twice, extracts both, requires byte-identical per-file SHA-256 manifests, keeps one tree in
#   RPC_RESULTS (default ../results/rp-NAME) with local-sha256.txt, deletes the pod, and confirms by listing that no pod
#   named NAME remains. The admission checks (binary hashes, host and end lines) are printed here and decided by the
#   verify card, not by this script.
# Exit: 0 fetched and deleted; 41 still running; 2 usage; 6 transfer check failed (pod kept for a retry);
#       7 pod still listed after delete.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); . "$here/common.sh"
[ $# -ge 1 ] || { echo "usage: fetch.sh NAME [--abort]" >&2; exit 2; }
NAME=$1 MODE=${2:-}
st="$RPC_STATE/$NAME.json"; [ -f "$st" ] || { echo "no state $st" >&2; exit 2; }
id=$(jq -r .id "$st"); created=$(jq -r .created "$st"); deadline=$(jq -r '.deadline // empty' "$st")
REC=${RPC_RECEIPTS:-$here/receipts/$NAME}; OUT=${RPC_RESULTS:-$study/results/rp-$NAME}; mkdir -p "$REC"
log() { echo "$(now) $*" | tee -a "$REC/fetch.log"; }
stamp=$(date -u +%Y%m%dT%H%M%SZ)

"$RP" status "$id" > "$REC/status-$stamp.json" 2>&1 || true
state=$(jq -r '.state // "unreachable"' "$REC/status-$stamp.json" 2>/dev/null || echo unreachable)
log "pod $id state $state $(jq -c '{exit,started,ended}' "$REC/status-$stamp.json" 2>/dev/null || true)"
reason=done
if [ "$state" != done ]; then
  if [ "$MODE" = --abort ]; then reason=abort
  elif [ -n "$deadline" ] && [ "$(date +%s)" -ge "$(epoch "$deadline")" ]; then reason=deadline
  elif [ "$state" = running ]; then log "still running (deadline $deadline)"; exit 41
  else log "runner state $state; rerun with --abort to collect what exists and delete the pod"; exit 41; fi
fi
log "collecting: reason $reason"

if [ "$state" != unreachable ]; then
  w="$RPC_STATE/$NAME.fetch-$stamp"; rm -rf "$w"; mkdir -p "$w/a" "$w/b"
  for x in a b; do
    "$RP" fetch "$id" "$w/$x.tar.gz" | tee -a "$REC/fetch.log"
    gzip -t "$w/$x.tar.gz"; tar -xzf "$w/$x.tar.gz" -C "$w/$x"
    (cd "$w/$x/results" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -d '\n' sha256sum) > "$w/$x.sha256"
  done
  # The runner tars results/ on each request; the tree is final once the runner is done, so both copies must agree.
  # A run collected while still running (abort, deadline) can differ in its growing log: that is reported, not hidden.
  if ! cmp -s "$w/a.sha256" "$w/b.sha256"; then
    diff "$w/a.sha256" "$w/b.sha256" | tee -a "$REC/fetch.log" || true
    [ "$reason" = done ] && { log "TRANSFER CHECK FAILED: the two fetches differ; pod $id kept, run fetch.sh again"; exit 6; }
    log "the two fetches differ while the runner was not done (reason $reason); keeping the second"
    rm -rf "$w/a"; mv "$w/b" "$w/a"; cp "$w/b.sha256" "$w/a.sha256"
  fi
  rm -rf "$OUT"; mkdir -p "$(dirname "$OUT")"; mv "$w/a/results" "$OUT"
  (cd "$OUT" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -d '\n' sha256sum) > "$REC/local-sha256.txt"
  cmp -s "$REC/local-sha256.txt" "$w/a.sha256" || { log "local tree differs from the fetched manifest"; exit 6; }
  sha256sum "$w/a.tar.gz" | sed "s#$w/##" > "$REC/tarball.sha256"
  log "results in $OUT: $(wc -l < "$REC/local-sha256.txt") files, manifest $REC/local-sha256.txt"
  # Printed for the verify card (not a decision here).
  {
    echo "--- binary.sha256"; cat "$OUT/binary.sha256" 2>/dev/null || echo MISSING
    echo "--- host.txt (first 7 lines)"; head -7 "$OUT/host.txt" 2>/dev/null || echo MISSING
    echo "--- progress.txt"; cat "$OUT/progress.txt" 2>/dev/null || echo MISSING
    echo "--- runner_state.json"; cat "$OUT/runner_state.json" 2>/dev/null || echo MISSING
  } | tee "$REC/admission-lines.txt"
fi

# Delete, then confirm by listing.
api "$API/pods/$id" | jq 'del(.env, .dockerStartCmd)' > "$REC/pod-record-final.json" 2>/dev/null || true
deleted=$(now); api -X DELETE "$API/pods/$id" -o "$REC/delete.response" -w '%{http_code}\n' > "$REC/delete.http" || true
log "delete pod $id: HTTP $(cat "$REC/delete.http") at $deleted"
gone=0
for i in $(seq 1 12); do
  n=$(api "$API/pods" | jq --arg i "$id" '[.[] | select(.id == $i)] | length')
  [ "$n" = 0 ] && { gone=1; break; }; sleep 10
done
list_pkm > "$REC/pods-after-delete.jsonl" || true
log "pkm-x-* pods listed after delete: $(wc -l < "$REC/pods-after-delete.jsonl") $(tr '\n' ' ' < "$REC/pods-after-delete.jsonl")"
price=$(jq -r '.costPerHr // empty' "$REC/pod-record.json" 2>/dev/null || true)
if [ -n "$price" ]; then
  s=$(( $(epoch "$deleted") - $(epoch "$created") ))
  cost=$(awk -v p="$price" -v s="$s" 'BEGIN{printf "%.4f", p * s / 3600}')
  jq -n --arg id "$id" --arg c "$created" --arg d "$deleted" --argjson s "$s" --argjson p "$price" --arg r "$reason" \
    --argjson usd "$cost" '{pod:$id,created:$c,deleted:$d,seconds:$s,costPerHr:$p,usd_derived:$usd,reason:$r}' > "$REC/cost.json"
  log "cost (derived: costPerHr x pod lifetime): US\$$cost over $s s at $price/h"
fi
[ "$gone" = 1 ] || { log "POD $id STILL LISTED after delete"; exit 7; }
log "pod $id deleted and no longer listed"
