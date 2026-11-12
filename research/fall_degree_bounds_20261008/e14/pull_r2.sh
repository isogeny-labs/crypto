#!/usr/bin/env bash
# E14 r2 (Amendment 20) controller: pull out.tgz from the pod's HTTP endpoint every 10 minutes,
# verify its sha256, keep every snapshot, extract the final one, then delete the pod.
# Needs RUNPOD_API_KEY and e14/pod-r2/launch.json. Run from the repository root; safe to restart.
set -uo pipefail
S=research/fall_degree_bounds_20261008
R=$S/e14/pod-r2; L=$R/launch.json
pod_id=$(jq -r .pod_id "$L"); tok=$(jq -r .out_token "$L")
base="https://${pod_id}-8080.proxy.runpod.net/$tok"
mkdir -p "$R/pulls" "$R/out"
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$R/pull.log"; }
while :; do
  stamp=$(date -u +%Y%m%dT%H%M%SZ)
  if curl -sSf -m 600 -o "$R/pulls/$stamp.tgz" "$base/out.tgz" 2>>"$R/pull.log"; then
    want=$(curl -sSf -m 60 "$base/out.tgz.sha256" 2>/dev/null | tr -d ' \n')
    have=$(sha256sum "$R/pulls/$stamp.tgz" | cut -d' ' -f1)
    if [ -n "$want" ] && [ "$want" != "$have" ]; then
      log "pull $stamp: sha256 mismatch (server $want, got $have); kept as .mismatch"; mv "$R/pulls/$stamp.tgz" "$R/pulls/$stamp.mismatch"
    else
      log "pull $stamp: ok $have"
      if tar xzf "$R/pulls/$stamp.tgz" -O out/final_status >/dev/null 2>&1; then
        rm -rf "$R/out"; mkdir -p "$R/out"; tar xzf "$R/pulls/$stamp.tgz" -C "$R/out" --strip-components=1
        # the pod's trace may contain the control key: redact before anything is committed
        grep -rl "${RUNPOD_API_KEY:?}" "$R/out" 2>/dev/null | while read -r f; do sed -i "s/${RUNPOD_API_KEY}/[REDACTED_RUNPOD_API_KEY]/g" "$f"; done
        log "final_status: $(cat "$R/out/final_status"); extracted to $R/out; deleting pod $pod_id"
        curl -sS -m 60 -X DELETE -H "Authorization: Bearer ${RUNPOD_API_KEY:?}" "https://rest.runpod.io/v1/pods/$pod_id" >>"$R/pull.log" 2>&1
        exit 0
      fi
    fi
  else
    log "pull $stamp: endpoint not reachable (pod booting, or gone)"
    status=$(curl -sS -m 30 -H "Authorization: Bearer ${RUNPOD_API_KEY:?}" "https://rest.runpod.io/v1/pods/$pod_id" | jq -r '.desiredStatus // "missing"')
    [ "$status" = RUNNING ] || { log "pod status $status; stopping controller"; exit 1; }
  fi
  sleep 600
done
