#!/usr/bin/env bash
# External to the paid pod. Thin transport, accounting and cleanup only.
set -euo pipefail
test "$#" -eq 3 || exit 2
out=$1 auth=$2 upload=$3
pod=$(cat "$out/pod-id.txt")
nonce=$(cat "$out/result-namespace.private.txt")
deadline=$(cat "$out/original-deadline-epoch.txt")
[[ $pod =~ ^[a-z0-9]+$ && $nonce =~ ^[a-f0-9]+$ ]] || exit 2
endpoint="https://${pod}-8000.proxy.runpod.net/$nonce"
pod_deleted=0
delete_pod() {
  local status
  status=$(curl --config "$auth" --silent --show-error --max-time 20 -X DELETE "https://rest.runpod.io/v1/pods/$pod" --output "$out/delete.private.txt" --write-out '%{http_code}') || status=000
  jq -n --arg pod "$pod" --arg code "$status" --arg utc "$(date -u +%FT%TZ)" \
    '{schema:"trace-runpod-cleanup/v1",pod_id:$pod,http_status:$code,utc:$utc,deleted:($code=="200" or $code=="204" or $code=="404")}' > "$out/cleanup.json"
  if [[ $status == 200 || $status == 204 || $status == 404 ]]; then pod_deleted=1;return 0;fi
  return 1
}
cleanup() { trap - EXIT; if ((pod_deleted==0)); then delete_pod || true;fi; if [[ -n ${watchdog_pid:-} ]]; then kill "$watchdog_pid" 2>/dev/null || true; fi; }
trap cleanup EXIT
remaining=$((deadline-$(date +%s)))
if ((remaining<=0)); then exit 7; fi
(
  sleep "$remaining"
  curl --config "$auth" --silent --show-error --max-time 20 -X DELETE "https://rest.runpod.io/v1/pods/$pod" > "$out/watchdog-delete.private.txt"
) &
watchdog_pid=$!
completed=0
while (($(date +%s)<deadline)); do
  if curl --fail --silent --show-error --max-time 8 "$endpoint/finished.json" -o "$out/finished.next" 2>/dev/null && jq -e '.sha256 and .bytes and (.worker_exit|type=="number")' "$out/finished.next" >/dev/null; then
    mv "$out/finished.next" "$out/finished.json";completed=1;break
  fi
  if curl --fail --silent --show-error --max-time 8 "$endpoint/raw/status.json" -o "$out/status.next" 2>/dev/null; then
    if ! cmp -s "$out/status.next" "$out/status.json"; then cp "$out/status.next" "$out/status.json";jq -c . "$out/status.json";fi
  fi
  sleep 10
done
if ((completed==0)); then echo 'Original Runpod deadline reached; retaining available status and deleting pod.' >&2;exit 8;fi
curl --fail --silent --show-error --max-time 90 "$endpoint/results.tar.gz" -o "$out/results.tar.gz"
expected=$(jq -er '.sha256' "$out/finished.json")
printf '%s  %s\n' "$expected" "$out/results.tar.gz" | sha256sum -c -
delete_pod
kill "$watchdog_pid" 2>/dev/null || true
unset watchdog_pid
tar -tzf "$out/results.tar.gz" > "$out/archive-contents.txt"
form=()
while IFS= read -r -d '' field; do form+=(-F "$field");done < <(jq -j '.post.fields|to_entries[]|(.key+"="+.value+"\u0000")' "$upload")
url=$(jq -er '.post.url' "$upload")
upload_status=$(curl --silent --show-error --max-time 90 "${form[@]}" -F "file=@$out/results.tar.gz" "$url" --output "$out/s3-upload.private.txt" --write-out '%{http_code}')
[[ $upload_status == 200 || $upload_status == 201 || $upload_status == 204 ]] || { echo "S3 result custody upload returned HTTP $upload_status" >&2;exit 9; }
readback_url=$(jq -er '.readback_url' "$upload")
curl --fail --silent --show-error --max-time 90 "$readback_url" -o "$out/s3-readback.tar.gz"
printf '%s  %s\n' "$expected" "$out/s3-readback.tar.gz" | sha256sum -c -
jq --arg hash "$expected" --arg code "$upload_status" --argjson bytes "$(stat -c %s "$out/results.tar.gz")" \
  '{schema:"trace-runpod-custody/v1",bucket,key,sha256:$hash,bytes:$bytes,upload_http_status:$code,readback_hash_matches:true}' "$upload" > "$out/custody.json"
mkdir -p "$out/extracted"
tar -xzf "$out/results.tar.gz" -C "$out/extracted"
jq -c . "$out/deployment.json" "$out/finished.json" "$out/cleanup.json" "$out/custody.json"
