#!/usr/bin/env bash
# E14 r2 (Amendment 20): admission checks, then create the one RunPod CPU pod.
# Needs RUNPOD_API_KEY. Writes e14/pod-r2/launch.json. Run from the repository root.
#   research/fall_degree_bounds_20261008/e14/launch_r2.sh            # admission + launch
#   DRY_RUN=1 research/fall_degree_bounds_20261008/e14/launch_r2.sh  # admission only
set -euo pipefail
S=research/fall_degree_bounds_20261008
R=$S/e14/pod-r2; mkdir -p "$R"
API=https://rest.runpod.io/v1
auth=(-H "Authorization: Bearer ${RUNPOD_API_KEY:?RUNPOD_API_KEY unset}")
SRC_COMMIT=a2be660914e8e485eefd45f2958eb6d18344dadf
PATHS_SHA256=97b6bbde009ee1e04884041280b1c5dcd9030024d95b940f7e4a12bd9bd28ecd
TREE_SHA256=da6ee5f3f65fa864d7eddb1da48a6bb7d5d286e7072f8607cc1356e480c466ff
CAP_USD=100
# --- admission
bal=$(curl -sS -m 30 "${auth[@]}" -H 'Content-Type: application/json' \
  -d '{"query":"{ myself { clientBalance } }"}' https://api.runpod.io/graphql | jq -r '.data.myself.clientBalance')
pods=$(curl -sS -m 30 "${auth[@]}" "$API/pods")
other_rate=$(echo "$pods" | jq '[.[] | select(.desiredStatus=="RUNNING") | .costPerHr] | add // 0')
fd_pods=$(echo "$pods" | jq '[.[] | select(.desiredStatus=="RUNNING" and (.name|startswith("fd-e14")))] | length')
need=$(awk -v r="$other_rate" -v c="$CAP_USD" 'BEGIN{ printf "%.2f", c + 24*r }')
admitted=$(awk -v b="$bal" -v n="$need" -v f="$fd_pods" -v o="${OWNER_OVERRIDE:-0}" 'BEGIN{ print ((b+0 >= n+0 || o == 1) && f+0 == 0) ? "true" : "false" }')
override=$([ "${OWNER_OVERRIDE:-0}" = 1 ] && echo true || echo false)   # Amendment 21: bypasses only the balance condition
[ -s .resume/RESUME.md ] || { echo "fill .resume/RESUME.md first (repository rule)"; exit 2; }
script_sha=$(sha256sum "$S/e14/run_pod_r2.sh" | cut -d' ' -f1)
OUT_TOKEN=$(head -c 16 /dev/urandom | od -An -tx1 | tr -d ' \n')
echo "balance USD $bal; other running pods USD $other_rate/h; required USD $need; fd-e14 pods running: $fd_pods; admitted: $admitted"
if [ "$admitted" != true ]; then
  jq -n --arg bal "$bal" --arg other "$other_rate" --arg need "$need" --arg when "$(date -u +%FT%TZ)" \
    '{admitted:false, balance_usd:$bal, other_pods_usd_per_h:$other, required_usd:$need, checked_at:$when}' | tee "$R/launch.json"
  exit 3
fi
[ "${DRY_RUN:-0}" = 1 ] && { echo "dry run: admitted, not launching"; exit 0; }
# --- create the pod
SCRIPT_B64=$(base64 -w0 "$S/e14/run_pod_r2.sh")
payload=$(jq -n --arg name "fd-e14-r2c-$(date -u +%Y%m%dT%H%MZ)" --arg pub "${EXTRA_PUBKEY:-}" --arg b64 "$SCRIPT_B64" --arg key "$RUNPOD_API_KEY" \
  --arg c "$SRC_COMMIT" --arg p "$PATHS_SHA256" --arg t "$TREE_SHA256" --arg tok "$OUT_TOKEN" '{
  name: $name, imageName: "runpod/base:1.0.2-ubuntu2404", cloudType: "SECURE", computeType: "CPU",
  cpuFlavorIds: ["cpu3g","cpu5g"], cpuFlavorPriority: "custom", vcpuCount: 64, containerDiskInGb: 150,
  interruptible: false, ports: ["8080/http","22/tcp"],
  env: { SCRIPT_B64: $b64, POD_CTL_KEY: $key, SRC_COMMIT: $c, PATHS_SHA256: $p, TREE_SHA256: $t, OUT_TOKEN: $tok, MAX_HOURS_FALLBACK: "18", EXTRA_PUBKEY: $pub },
  dockerEntrypoint: ["bash","-c"],
  dockerStartCmd: ["mkdir -p /workspace /root && echo \"$SCRIPT_B64\" | base64 -d >/root/run.sh && exec bash /root/run.sh"]
}')
resp=$(curl -sS -m 120 "${auth[@]}" -H 'Content-Type: application/json' -d "$payload" "$API/pods")
pod_id=$(echo "$resp" | jq -r '.id // empty')
[ -n "$pod_id" ] || { echo "pod creation failed: $resp"; exit 4; }
jq -n --arg id "$pod_id" --arg tok "$OUT_TOKEN" --arg sha "$script_sha" --arg bal "$bal" --arg other "$other_rate" --argjson ov "$override" \
  --arg when "$(date -u +%FT%TZ)" --argjson resp "$resp" \
  '{admitted:true, owner_override:$ov, pod_id:$id, out_token:$tok, run_pod_r2_sha256:$sha, balance_usd:$bal, other_pods_usd_per_h:$other,
    launched_at:$when, amendment:23, pod:($resp|{id,name,costPerHr,vcpuCount,memoryInGb,imageName,ports})}' | tee "$R/launch.json"
echo "pull with: $S/e14/pull_r2.sh"
