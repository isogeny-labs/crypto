#!/usr/bin/env bash
# E14 r3 (Amendment 31): admission check, then create one RunPod CPU pod for MODE a or b.
# Needs RUNPOD_API_KEY. Writes e14/pod-r3<mode>/launch.json. Run from the repository root.
#   MODE=a research/fall_degree_bounds_20261008/e14/launch_r3.sh             # E14a draw 3 (hky_par ladder)
#   MODE=b research/fall_degree_bounds_20261008/e14/launch_r3.sh             # E14b controls (pe)
#   DRY_RUN=1 MODE=a research/fall_degree_bounds_20261008/e14/launch_r3.sh   # admission only
set -euo pipefail
MODE=${MODE:?MODE=a or MODE=b}
S=research/fall_degree_bounds_20261008
R=$S/e14/pod-r3$MODE; mkdir -p "$R"
API=https://rest.runpod.io/v1
auth=(-H "Authorization: Bearer ${RUNPOD_API_KEY:?RUNPOD_API_KEY unset}")
SRC_COMMIT=a2be660914e8e485eefd45f2958eb6d18344dadf
PATHS_SHA256=97b6bbde009ee1e04884041280b1c5dcd9030024d95b940f7e4a12bd9bd28ecd
TREE_SHA256=da6ee5f3f65fa864d7eddb1da48a6bb7d5d286e7072f8607cc1356e480c466ff
case "$MODE" in
  a) CAP_USD=27; MAX_HOURS=12; FLAVORS='["cpu5m"]' ;;            # about 200 GiB peak: 503 GiB flavor only
  b) CAP_USD=17; MAX_HOURS=8;  FLAVORS='["cpu5m","cpu3m"]' ;;
  *) echo "MODE must be a or b"; exit 2 ;;
esac
# --- admission (Amendment 31): balance >= CAP_USD + MAX_HOURS x (USD/h of every running pod); no running r3 pod of this mode
bal=$(curl -sS -m 30 "${auth[@]}" -H 'Content-Type: application/json' \
  -d '{"query":"{ myself { clientBalance } }"}' https://api.runpod.io/graphql | jq -r '.data.myself.clientBalance')
pods=$(curl -sS -m 30 "${auth[@]}" "$API/pods")
other_rate=$(echo "$pods" | jq '[.[] | select(.desiredStatus=="RUNNING") | .costPerHr] | add // 0')
same=$(echo "$pods" | jq --arg p "fd-e14-r3$MODE" '[.[] | select(.desiredStatus=="RUNNING" and (.name|startswith($p)))] | length')
need=$(awk -v r="$other_rate" -v c="$CAP_USD" -v h="$MAX_HOURS" 'BEGIN{ printf "%.2f", c + h*r }')
admitted=$(awk -v b="$bal" -v n="$need" -v f="$same" 'BEGIN{ print (b+0 >= n+0 && f+0 == 0) ? "true" : "false" }')
script_sha=$(sha256sum "$S/e14/run_pod_r3.sh" | cut -d' ' -f1)
echo "mode $MODE; balance USD $bal; running pods USD $other_rate/h; required USD $need; same-mode pods running: $same; admitted: $admitted"
if [ "$admitted" != true ]; then
  jq -n --arg m "$MODE" --arg bal "$bal" --arg other "$other_rate" --arg need "$need" --arg when "$(date -u +%FT%TZ)" \
    '{mode:$m, admitted:false, balance_usd:$bal, running_pods_usd_per_h:$other, required_usd:$need, checked_at:$when}' | tee "$R/admission.json"
  exit 3
fi
[ "${DRY_RUN:-0}" = 1 ] && { echo "dry run: admitted, not launching"; exit 0; }
OUT_TOKEN=$(head -c 16 /dev/urandom | od -An -tx1 | tr -d ' \n')
SCRIPT_B64=$(base64 -w0 "$S/e14/run_pod_r3.sh")
payload=$(jq -n --arg name "fd-e14-r3$MODE-$(date -u +%Y%m%dT%H%MZ)" --arg b64 "$SCRIPT_B64" --arg key "$RUNPOD_API_KEY" \
  --arg c "$SRC_COMMIT" --arg p "$PATHS_SHA256" --arg t "$TREE_SHA256" --arg tok "$OUT_TOKEN" --arg mode "$MODE" \
  --arg cap "$CAP_USD" --arg mh "$MAX_HOURS" --arg ph "${POST_HOURS:-18}" --argjson fl "$FLAVORS" '{
  name: $name, imageName: "runpod/base:1.0.2-ubuntu2404", cloudType: "SECURE", computeType: "CPU",
  cpuFlavorIds: $fl, cpuFlavorPriority: "custom", vcpuCount: 32, containerDiskInGb: 60,
  interruptible: false, ports: ["8080/http"],
  env: { MODE: $mode, POST_HOURS: $ph, CAP_USD: $cap, MAX_HOURS: $mh, SCRIPT_B64: $b64, POD_CTL_KEY: $key, SRC_COMMIT: $c, PATHS_SHA256: $p,
         TREE_SHA256: $t, OUT_TOKEN: $tok, MAX_HOURS_FALLBACK: $mh, PATHS_COMMIT: "43438f5ab6eb6d1844b8d0465c933ad079099326" },
  dockerEntrypoint: ["bash","-c"],
  dockerStartCmd: ["mkdir -p /workspace/out /workspace/serve/$OUT_TOKEN /root && ln -sfn /workspace/out /workspace/serve/$OUT_TOKEN/out && echo \"$(date -u +%FT%TZ) start command running\" >>/workspace/out/run.log && (cd /workspace/serve && nohup python3 -m http.server 8080 --bind 0.0.0.0 >>/workspace/out/http0.log 2>&1 &) && echo \"$SCRIPT_B64\" | base64 -d >/root/run.sh && echo \"$(date -u +%FT%TZ) script decoded $(sha256sum /root/run.sh | cut -c1-16)\" >>/workspace/out/run.log && exec bash /root/run.sh"]
}')
resp=$(curl -sS -m 120 "${auth[@]}" -H 'Content-Type: application/json' -d "$payload" "$API/pods")
pod_id=$(echo "$resp" | jq -r '.id // empty')
[ -n "$pod_id" ] || { echo "pod creation failed: $(echo "$resp" | sed "s/$RUNPOD_API_KEY/REDACTED/g")"; exit 4; }
jq -n --arg m "$MODE" --arg id "$pod_id" --arg tok "$OUT_TOKEN" --arg sha "$script_sha" --arg bal "$bal" --arg other "$other_rate" \
  --arg when "$(date -u +%FT%TZ)" --argjson resp "$resp" \
  '{mode:$m, admitted:true, pod_id:$id, out_token:$tok, run_pod_r3_sha256:$sha, balance_usd:$bal, running_pods_usd_per_h:$other,
    launched_at:$when, amendment:32, pod:($resp|{id,name,costPerHr,vcpuCount,memoryInGb,volumeInGb,imageName,ports})}' | tee "$R/launch.json"
echo "pull with: MODE=$MODE $S/e14/pull_r3.sh"
