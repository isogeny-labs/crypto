#!/usr/bin/env bash
# PROPOSAL ONLY. Root must review/authorize execution separately. Never run fixture on host.
set -euo pipefail
[[ $# == 3 ]] || { echo 'Usage: run-reviewed-container.sh sha256:IMAGE FIXTURE_SHA256 NEW_RECEIPT_DIR' >&2; exit 2; }
image_id=$1 expected_binary=$2 receipts=$3
[[ "$image_id" =~ ^sha256:[a-f0-9]{64}$ && "$expected_binary" =~ ^[a-f0-9]{64}$ && "$receipts" == /* && ! -e "$receipts" ]] || exit 2
mkdir -m700 "$receipts"
nonce=$(cat /proc/sys/kernel/random/uuid)
name="isolab-private-backstop-${nonce}"
label_key=isolab.private-backstop.owner
cid=''
cleanup_done=false
creation_attempted=false
cleanup_deadline=0
cli() {
  local cap=15 remaining
  if (( cleanup_deadline > 0 )); then
    remaining=$((cleanup_deadline-SECONDS))
    (( remaining > 0 )) || return 124
    (( remaining >= cap )) || cap=$remaining
  fi
  timeout -k 1s "${cap}s" sudo -n docker "$@"
}
identity_check() {
  cli inspect "$cid" > "$receipts/cleanup-identity.json" || return 1
  jq -e --arg id "$cid" --arg name "/$name" --arg image "$image_id" --arg key "$label_key" --arg owner "$nonce" \
    'length==1 and .[0].Id==$id and .[0].Name==$name and .[0].Image==$image and .[0].Config.Labels[$key]==$owner' \
    "$receipts/cleanup-identity.json" >/dev/null
}
cleanup_body() {
  local original=$1
  trap - EXIT INT TERM
  set +e
  cleanup_deadline=$((SECONDS+28))
  if [[ -z "$cid" && "$creation_attempted" == true ]]; then
    if cli inspect "$name" > "$receipts/ambiguous-create-inspect.json" 2> "$receipts/ambiguous-create-inspect.stderr" &&
       jq -e --arg name "/$name" --arg image "$image_id" --arg key "$label_key" --arg owner "$nonce" \
       'length==1 and .[0].Name==$name and .[0].Image==$image and .[0].Config.Labels[$key]==$owner and (.[0].Id|test("^[a-f0-9]{64}$"))' "$receipts/ambiguous-create-inspect.json" >/dev/null; then
      cid=$(jq -r '.[0].Id' "$receipts/ambiguous-create-inspect.json")
    else
      printf '%s\n' "Creation state unresolved; reconcile exact name $name and label $label_key=$nonce before retry." > "$receipts/CREATE-AMBIGUOUS"
      original=91
    fi
  fi
  if [[ -n "$cid" && "$cleanup_done" == false ]]; then
    if identity_check; then
      cli logs "$cid" > "$receipts/terminal.stdout" 2> "$receipts/terminal.stderr"
      cli inspect "$cid" > "$receipts/pre-cleanup.json"
      if jq -e '.[0].State.Running' "$receipts/pre-cleanup.json" >/dev/null; then
        cli kill "$cid" > "$receipts/cleanup-kill.log" 2>&1
        cli wait "$cid" > "$receipts/cleanup-wait.log" 2>&1
      fi
      cli inspect "$cid" > "$receipts/terminal-inspect.json"
      # Never remove a running or identity-mismatched container.
      if identity_check && jq -e '.[0].State.Running==false' "$receipts/cleanup-identity.json" >/dev/null; then
        cli rm "$cid" > "$receipts/cleanup-remove.log" 2>&1
        removal=$?
        printf '%s\n' "$removal" > "$receipts/cleanup-remove.exit"
        [[ "$removal" == 0 ]] || original=91
      else
        printf '%s\n' 'Owned container not confirmed stopped; retained for explicit reconciliation.' > "$receipts/CLEANUP-INCOMPLETE"
        original=91
      fi
    else
      printf '%s\n' 'Identity could not be verified; no kill/removal attempted.' > "$receipts/CLEANUP-INCOMPLETE"
      original=91
    fi
  fi
  printf '%s\n' "$original" > "$receipts/orchestration.exit"
  exit "$original"
}
cleanup() {
  local original=$? result
  trap - EXIT INT TERM
  set +e
  export cid creation_attempted cleanup_done name label_key nonce image_id receipts
  export -f cli identity_check cleanup_body
  # One outer guard also covers blocked cleanup filesystem/CLI operations.
  # A 29-second TERM plus 1-second KILL grace caps this owned cleanup subprocess.
  timeout -k 1s 29s bash -c 'cleanup_body "$1"' _ "$original"
  result=$?
  if [[ "$result" == 124 || "$result" == 137 ]]; then
    printf '%s\n' "Overall cleanup deadline reached; reconcile exact name $name and label $label_key=$nonce." > "$receipts/CLEANUP-INCOMPLETE"
    result=91
  fi
  printf '%s\n' "$result" > "$receipts/orchestration.exit"
  exit "$result"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
cli ps --no-trunc > "$receipts/containers-before.txt"
cli image inspect "$image_id" > "$receipts/image.json"
jq -e --arg image "$image_id" 'length==1 and .[0].Id==$image and .[0].Architecture=="amd64" and .[0].Os=="linux" and ((.[0].Config.OnBuild // [])|length)==0 and ((.[0].Config.Volumes // {})|length)==0 and .[0].Config.Entrypoint==["/fixture"] and (.[0].Config.Cmd==null or .[0].Config.Cmd==[]) and .[0].Config.User=="0:0"' "$receipts/image.json" >/dev/null
# create has no fixture execution; inspect it before start.
printf '%s\n' "$name" > "$receipts/container-name.txt"
printf '%s\n' "$label_key=$nonce" > "$receipts/container-owner-label.txt"
creation_attempted=true
cli create --name "$name" --label "$label_key=$nonce" --network none --ipc private \
 --read-only --cap-drop ALL --security-opt no-new-privileges --memory 64m --memory-swap 64m \
 --cpus 0.25 --pids-limit 16 --user 0:0 --env ISOLAB_PRIVATE_QUALIFICATION=reviewed-container-only/v1 \
 --restart no --stop-timeout 1 --log-driver json-file --log-opt max-size=1m --log-opt max-file=1 \
 "$image_id" > "$receipts/create.stdout" 2> "$receipts/create.stderr"
cid=$(cat "$receipts/create.stdout")
[[ "$cid" =~ ^[a-f0-9]{64}$ ]] || {
  # Exact raw output remains in create.stdout. Force cleanup through the
  # unique-name/owner-label reconciliation path instead of using malformed ID.
  cid=''
  echo 'Creation output ambiguous: inspect exact name/label before any retry.' > "$receipts/CREATE-AMBIGUOUS"
  exit 90
}
printf '%s\n' "$name" > "$receipts/container-name.txt"
identity_check
cp "$receipts/cleanup-identity.json" "$receipts/created-inspect.json"
jq -e '.[0] | .State.Status=="created" and .State.Pid==0 and .State.Running==false and .HostConfig.PidMode=="" and .HostConfig.IpcMode=="private" and .HostConfig.UTSMode=="" and .HostConfig.Privileged==false and .HostConfig.NetworkMode=="none" and .HostConfig.ReadonlyRootfs==true and .HostConfig.CapDrop==["ALL"] and ((.HostConfig.CapAdd // [])|length)==0 and ((.HostConfig.Devices // [])|length)==0 and ((.HostConfig.DeviceRequests // [])|length)==0 and ((.HostConfig.Binds // [])|length)==0 and ((.Mounts // [])|length)==0 and ((.HostConfig.VolumesFrom // [])|length)==0 and (.HostConfig.Init==null or .HostConfig.Init==false) and .HostConfig.RestartPolicy.Name=="no" and .HostConfig.Memory==67108864 and .HostConfig.MemorySwap==67108864 and .HostConfig.NanoCpus==250000000 and .HostConfig.PidsLimit==16 and (.HostConfig.SecurityOpt==["no-new-privileges"] or .HostConfig.SecurityOpt==["no-new-privileges:true"]) and .Config.User=="0:0" and .Config.Entrypoint==["/fixture"] and (.Config.Env|index("ISOLAB_PRIVATE_QUALIFICATION=reviewed-container-only/v1"))!=null' "$receipts/created-inspect.json" >/dev/null
# Read the image binary without executing it or mounting host paths.
cli cp "$cid:/fixture" "$receipts/fixture-readback"
test "$(sha256sum "$receipts/fixture-readback" | cut -d' ' -f1)" = "$expected_binary"
readelf -l "$receipts/fixture-readback" > "$receipts/fixture-program-headers.txt"
! rg -q 'INTERP' "$receipts/fixture-program-headers.txt"
cli start "$cid" > "$receipts/start.stdout" 2> "$receipts/start.stderr"
# Absolute external wall guard; fixture output never drives a host command.
set +e
timeout -k 2s 10s sudo -n docker wait "$cid" > "$receipts/wait.stdout" 2> "$receipts/wait.stderr"
wait_exit=$?
set -e
printf '%s\n' "$wait_exit" > "$receipts/wait-command.exit"
[[ "$wait_exit" == 0 ]] || exit 90
[[ "$(cat "$receipts/wait.stdout")" == 125 ]] || exit 89
cli inspect "$cid" > "$receipts/result-inspect.json"
cli logs "$cid" > "$receipts/result.stdout" 2> "$receipts/result.stderr"
jq -e '.[0].State.Running==false and .[0].State.ExitCode==125 and .[0].State.OOMKilled==false' "$receipts/result-inspect.json" >/dev/null
rg -q '^watchdog_installed main_thread_blocking$' "$receipts/result.stderr"
rg -q '^detached_child_reaped signal=Some\(9\) code=None elapsed_ns=[0-9]+$' "$receipts/result.stderr"
printf '%s\n' 'PASS: exact fixture, private container, detached child SIGKILL observed, PID1 exit125.' > "$receipts/mechanism-result.txt"
