#!/usr/bin/env bash
# Client for runpod/server.py (Amendment R3 draft). Auth to rest.runpod.io is injected by the session proxy.
# Usage:
#   rp.sh create NAME VCPU          -> creates a reserved (interruptible=false) CPU pod running server.py; prints pod JSON
#   rp.sh pod ID                    -> pod record (costPerHr, vcpuCount, memoryInGb, desiredStatus)
#   rp.sh upload ID DIR             -> PUT every allowed file in DIR
#   rp.sh start ID STAGES_JSON      -> POST /start with stages + the frozen SHA-256s from ../image/image.json
#   rp.sh status ID | fetch ID OUT.tar.gz | delete ID
# Token: $RUNNER_TOKEN (generated per pod by `create`, kept in $TOKDIR/<pod>.token, never committed).
set -euo pipefail
API=https://rest.runpod.io/v1; H=(-H "Authorization: Bearer proxy-injected" -H "Content-Type: application/json")
here=$(cd "$(dirname "$0")" && pwd); TOKDIR=${TOKDIR:?scratch dir for pod tokens}
url() { echo "https://$1-8000.proxy.runpod.net"; }
tok() { cat "$TOKDIR/$1.token"; }
case "$1" in
  create)
    name=$2 vcpu=$3; t=$(python3 -c 'import secrets;print(secrets.token_hex(24))')
    boot="import base64;exec(base64.b64decode('$(base64 -w0 "$here/server.py")'))"
    body=$(jq -n --arg n "$name" --argjson v "$vcpu" --arg t "$t" --arg b "$boot" --arg h "$(sha256sum "$here/server.py" | cut -c1-64)" '{name:$n,computeType:"CPU",
      cpuFlavorIds:["cpu5m","cpu3m"],cpuFlavorPriority:"custom",vcpuCount:$v,interruptible:false,
      imageName:"python:3.12-slim@sha256:5e2bdb1b0d542d5039cffb8181c65930093949194a65c365dfb1b0fe13a4fa5b",containerDiskInGb:20,ports:["8000/http"],env:{RUNNER_TOKEN:$t,RUNNER_SHA256:$h,RESULTS_DIR:"/workspace/results"},volumeInGb:20,volumeMountPath:"/workspace",
      dockerEntrypoint:["python3","-u","-c"],dockerStartCmd:[$b]}')
    out=$(curl -sS -X POST "$API/pods" "${H[@]}" -d "$body"); id=$(jq -r '.id // empty' <<<"$out")
    [ -n "$id" ] || { echo "$out" >&2; exit 1; }
    umask 077; echo "$t" > "$TOKDIR/$id.token"; jq '{id,name,costPerHr,vcpuCount,memoryInGb,interruptible,machine,imageName}' <<<"$out" | tee "$TOKDIR/$id.create.json" ;;
  pod) curl -sS "$API/pods/$2" "${H[@]}" | jq '{id,name,desiredStatus,costPerHr,vcpuCount,memoryInGb,interruptible,lastStartedAt,machine}' ;;
  upload) # 2 MB chunks (Amendment 16): first chunk via /upload (truncates), the rest via /append
    for f in pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check pfd_grid_zeros entry_r2.sh run.sh build-info.json; do
      [ -f "$3/$f" ] || continue; tmp=$(mktemp -d); split -b 2M -d -a 4 "$3/$f" "$tmp/part."; ep=upload
      for c in "$tmp"/part.*; do curl -sS -f -m 300 -X PUT -H "X-Token: $(tok $2)" --data-binary "@$c" "$(url $2)/$ep/$f" > "$tmp/last"; ep=append; done
      cat "$tmp/last"; echo; rm -rf "$tmp"; done ;;
  start) # Kummer jobs. SHA_FILE (main's Amendment 17): a JSON {name: sha256} of the reproduction-gated rebuild; default: the
      # frozen image's. Guard fields as start-iso (branch amendments, preregistration 22-29)
      shas=$(if [ -n "${SHA_FILE:-}" ]; then jq . "$SHA_FILE"; else jq '.study_binary_sha256 | del(.["entry.sh"])' "$here/../image/image.json"; fi)
      cph=$(curl -sS "$API/pods/$2" "${H[@]}" | jq -r .costPerHr)
      jq -n --argjson s "$3" --argjson h "$shas" --argjson sec "${SECONDS_LIMIT:?}" --argjson cph "$cph" \
        --argjson mx "${MAX_USD:-40}" --argjson mf "${MEMORY_FLOOR_GIB:-56}" \
        '{stages:$s,sha256:$h,rayon:12,seconds:$sec,cost_per_hr:$cph,max_usd:$mx,memory_floor_gib:$mf}' |
      curl -sS -X POST -H "X-Token: $(tok $2)" -H "Content-Type: application/json" --data-binary @- "$(url $2)/start"; echo ;;
  upload-iso) # rp.sh upload-iso ID BINDIR COMMIT : the R4 binaries from BINDIR, and repo.tar = git archive COMMIT of iso/BUNDLE_PATHS
      repo=$(cd "$here/../../.." && pwd); tar=$(mktemp)
      git -C "$repo" archive -o "$tar" "$4" $(cat "$here/../iso/BUNDLE_PATHS")
      for f in pkm_tower_pilot_r4 prime_fall_degree_tools hky_fp_tower_check; do
        curl -sS -f -X PUT -H "X-Token: $(tok $2)" --data-binary "@$3/$f" "$(url $2)/upload/$f"; echo; done
      curl -sS -f -X PUT -H "X-Token: $(tok $2)" --data-binary "@$tar" "$(url $2)/upload/repo.tar"; echo
      echo "repo.tar $(sha256sum "$tar" | cut -c1-64) commit $4" | tee "$TOKDIR/$2.repo.tar.sha256"; rm -f "$tar" ;;
  start-iso) # rp.sh start-iso ID STAGES_JSON : pins from iso/PINS.json, repo.tar hash from upload-iso
      shas=$(jq --arg t "$(cut -d' ' -f2 "$TOKDIR/$2.repo.tar.sha256")" '.sha256 + {"repo.tar":$t}' "$here/../iso/PINS.json")
      for opt in pkm_tower_pilot rows.tar; do   # Amendment 20 optional uploads, from $ISO_OPT_DIR when set
        if [ -n "${ISO_OPT_DIR:-}" ] && [ -f "$ISO_OPT_DIR/$opt" ]; then
          curl -sS -f -X PUT -H "X-Token: $(tok $2)" --data-binary "@$ISO_OPT_DIR/$opt" "$(url $2)/upload/$opt" >/dev/null
          shas=$(jq --arg n "$opt" --arg h "$(sha256sum "$ISO_OPT_DIR/$opt" | cut -c1-64)" '. + {($n):$h}' <<<"$shas")
        fi
      done
      commit=$(cut -d' ' -f4 "$TOKDIR/$2.repo.tar.sha256"); repo=$(cd "$here/../../.." && pwd)
      pins=$(git -C "$repo" show "$commit:research/pkm_falldegree_unblock_20261107/iso/BUNDLE.sha256" |
        jq -R 'split("  ") | {(.[1]): .[0]}' | jq -s add)
      cph=$(curl -sS "$API/pods/$2" "${H[@]}" | jq -r .costPerHr)   # the pod's live price (E3.2)
      jq -n --argjson s "$3" --argjson h "$shas" --argjson b "$pins" --arg c "$commit" --argjson sec "${SECONDS_LIMIT:?}" \
        --argjson cph "$cph" --argjson mx "${MAX_USD:-40}" --argjson mf "${MEMORY_FLOOR_GIB:-56}" \
        '{entry:"iso",stages:$s,sha256:$h,bundle_sha256:$b,commit:$c,rayon:12,seconds:$sec,cost_per_hr:$cph,max_usd:$mx,memory_floor_gib:$mf}' |
      curl -sS -X POST -H "X-Token: $(tok $2)" -H "Content-Type: application/json" --data-binary @- "$(url $2)/start"; echo ;;
  status) curl -sS -H "X-Token: $(tok $2)" "$(url $2)/status" ;;
  fetch) # rp.sh fetch ID OUT.tar.gz : pod manifest first, then the tarball, then every file checked against the manifest (E3.5)
      curl -sS -f -H "X-Token: $(tok $2)" -o "$3.manifest.json" "$(url $2)/manifest"
      curl -sS -f -H "X-Token: $(tok $2)" -o "$3" "$(url $2)/results.tar.gz"
      d=$(mktemp -d); tar -xzf "$3" -C "$d"
      jq -r '.files | to_entries[] | "\(.value)  \(.key)"' "$3.manifest.json" | (cd "$d/results" && sha256sum -c --quiet -) \
        && echo "manifest check: all files match" || echo "manifest check: MISMATCH (files written after the manifest are expected while running)"
      rm -rf "$d"; sha256sum "$3" ;;
  delete) curl -sS -X DELETE "$API/pods/$2" "${H[@]}"; echo "deleted $2" ;;
  *) echo "unknown command" >&2; exit 2 ;;
esac
