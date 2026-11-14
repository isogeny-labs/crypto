#!/usr/bin/env bash
# E14 r3 (Amendment 31): one-shot pull. If the pod is stopped (it stops itself after finishing), start it,
# wait for its server, download out.tgz, check the sha256, extract with the key redacted, and delete the
# pod only once final_status is in the extracted output. Run from the repository root: MODE=a|b pull_r3.sh
set -uo pipefail
MODE=${MODE:?MODE=a or MODE=b}
R=research/fall_degree_bounds_20261008/e14/pod-r3$MODE; L=$R/launch.json
API=https://rest.runpod.io/v1; auth=(-H "Authorization: Bearer ${RUNPOD_API_KEY:?}")
pod_id=$(jq -r .pod_id "$L"); tok=$(jq -r .out_token "$L"); base="https://${pod_id}-8080.proxy.runpod.net/$tok"
mkdir -p "$R/pulls" "$R/out"; LOG="$R/pulls/pull-live.log"
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$LOG"; }
status=$(curl -sS -m 30 "${auth[@]}" "$API/pods/$pod_id" | jq -r '.desiredStatus // "missing"')
log "pod $pod_id status $status"
[ "$status" = missing ] && { log "pod missing: nothing to pull"; exit 3; }
if [ "$status" != RUNNING ]; then
  curl -sS -m 60 -X POST "${auth[@]}" "$API/pods/$pod_id/start" >/dev/null; log "start requested"
fi
for i in $(seq 1 40); do curl -sSf -m 20 -o /dev/null "$base/out.tgz.sha256" 2>/dev/null && break; sleep 15; done
stamp=$(date -u +%Y%m%dT%H%M%SZ)
curl -sSf -m 600 -o "$R/pulls/$stamp.tgz" "$base/out.tgz" || { log "download failed"; exit 4; }
want=$(curl -sSf -m 60 "$base/out.tgz.sha256" | tr -d ' \n'); got=$(sha256sum "$R/pulls/$stamp.tgz" | cut -d' ' -f1)
[ "$want" = "$got" ] || { log "pull $stamp: sha mismatch (want $want got $got)"; exit 5; }
rm -rf "$R/out" && mkdir -p "$R/out" && tar xzf "$R/pulls/$stamp.tgz" -C "$R/out" --strip-components=1
grep -rl "$RUNPOD_API_KEY" "$R/out" 2>/dev/null | while read -r f; do sed -i "s/$RUNPOD_API_KEY/REDACTED/g" "$f"; done
log "pull $stamp: ok $got"
if [ -s "$R/out/final_status" ]; then
  log "final_status: $(cat "$R/out/final_status")"
  curl -sS -m 60 -X DELETE "${auth[@]}" "$API/pods/$pod_id" >/dev/null && log "pod deleted"
fi
