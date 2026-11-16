#!/usr/bin/env bash
# Source-only preparation TASK-20261029-30f612. DO NOT run during preparation.
# Future separately admitted TASK-20261029-6f4f28: original C1 Q4 only.
set -u -o pipefail
owner_root=/workspace/crypto-native-smallroot
programme_root=/workspace/crypto-autoresearcher
continuation=coordination/goals/GOAL-PFDR-fd09e0/batches/BATCH-ee839d/continuations/DEC-20261029-03e304
prep_root="$owner_root/experiments/joint-smallroot-native/spec/checks/TASK-20261029-30f612"
record_root="$owner_root/experiments/joint-smallroot-native/spec/checks/TASK-20261029-6f4f28"
build_root=/workspace/native-smallroot-guard-build-6374dd
image_ref=debian:trixie-slim@sha256:62bd423aeee912639ca8ee681c2b762cb3e25137e0a92b20be455898a9f034ad
image_id=sha256:133aec7ab1411181567fef7fc3f4512123e1f9660090b49851de4b698b4ff772
admission="$programme_root/$continuation/admission/TASK-20261029-6f4f28/prelaunch-admission.json"
safe_proof=/workspace/smallroot-session/20261029-q4-current-lease.json
managed_docker() {
  env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS -u DOCKER_TLS_VERIFY -u DOCKER_CERT_PATH docker --host=unix:///var/run/docker.sock "$@"
}
refuse() { printf '%s\n' "$*" >&2; exit 69; }
[ "$#" -eq 1 ] && [ "$1" = Q4 ] || refuse 'requires exactly Q4; no other class, cache branch or option'
for selector in LD_PRELOAD LD_AUDIT LD_LIBRARY_PATH; do
  [ -z "${!selector:-}" ] || refuse 'unexpected dynamic-loader selector; no alternate runtime'
done
check_file() {
  local file=$1 expected=$2 actual
  [[ "$expected" =~ ^[0-9a-f]{64}$ ]] || return 1
  actual=$(sha256sum -- "$file") || return 1
  [ "${actual%% *}" = "$expected" ]
}
check_reference() {
  local key=$1 expected_path=$2 path digest
  path=$(jq -er ".$key.path" "$admission") || return 1
  digest=$(jq -er ".$key.sha256" "$admission") || return 1
  [ "$path" = "$expected_path" ] && check_file "$path" "$digest"
}
check_lease() {
  local observed expires current
  jq -e --slurpfile proof "$safe_proof" '
    .case_lease as $a | $proof[0] as $p |
    $a.safe_proof_path == "/workspace/smallroot-session/20261029-q4-current-lease.json" and
    $a.task_ref == "T-6" and $p.task_ref == $a.task_ref and
    $p.task_id == $a.task_id and $p.lease_id == $a.lease_id and
    $p.fencing_epoch == $a.fencing_epoch and $p.fencing_epoch >= 1 and
    $a.live == true and $a.expected_principal == true and
    $p.live == true and $p.expected_principal == true and
    $p.task_status == "claimed" and $p.released_at == null
  ' "$admission" >/dev/null || return 1
  observed=$(date -u -d "$(jq -er .observed_at "$safe_proof")" +%s) || return 1
  expires=$(date -u -d "$(jq -er .expires_at "$safe_proof")" +%s) || return 1
  current=$(date -u +%s)
  [ "$current" -ge "$observed" ] && [ "$((current-observed))" -lt 75 ] && [ "$expires" -gt "$current" ]
}
# These are existing frozen publication/source/ownership requirements. Their
# actual immutable bytes and commits are supplied by root after preparation.
jq -e '.verdict == "admitted_Q4" and .task_id == "TASK-20261029-6f4f28"' "$admission" >/dev/null || refuse 'actual separately published Q4 admission absent'
check_reference source_runtime_seal "$prep_root/runtime-source-seal.json" || refuse 'runtime/source seal binding mismatch'
check_reference source_binary_library_manifest "$prep_root/source-binary-library-manifest.json" || refuse 'source/binary/library binding mismatch'
check_reference expected_checks "$prep_root/expected-checks.json" || refuse 'fixed expectations binding mismatch'
check_reference prep_publication "$programme_root/$continuation/archives/TASK-20261029-6cad2d/publication-verification.json" || refuse 'prep publication binding mismatch'
check_reference claim_publication "$programme_root/$continuation/admission/TASK-20261029-6f4f28/claim-publication.json" || refuse 'case claim publication binding mismatch'
for key in prep_publication claim_publication; do
  publication_commit=$(jq -er ".$key.commit" "$admission") || refuse 'missing actual publication commit'
  [[ "$publication_commit" =~ ^[0-9a-f]{40}$ ]] || refuse 'invalid publication commit'
  publication_path=$(jq -er ".$key.path" "$admission") || refuse 'missing publication path'
  git -C "$programme_root" -c gc.auto=0 -c maintenance.auto=false cat-file -e "$publication_commit^{commit}" || refuse 'publication commit unavailable'
  git -C "$programme_root" -c gc.auto=0 -c maintenance.auto=false show "$publication_commit:${publication_path#"$programme_root/"}" | cmp -s -- "$publication_path" - || refuse 'publication bytes absent at supplied immutable commit'
done
check_lease || refuse 'fresh maintained exact case ownership absent'
check_file "$0" "$(jq -er .launcher.sha256 "$prep_root/runtime-source-seal.json")" || refuse 'launcher changed'
while IFS=$'\t' read -r path digest; do
  check_file "$path" "$digest" || refuse "sealed runtime/source mismatch: $path"
done < <(jq -r '.sources + .binaries + .host_libraries | .[] | [.path,.sha256] | @tsv' "$prep_root/source-binary-library-manifest.json")
actual_image=$(managed_docker image inspect --format '{{.Id}}' "$image_ref") || refuse 'pinned image unavailable'
[ "$actual_image" = "$image_id" ] || refuse 'pinned image identity mismatch'

[ ! -L "$record_root" ] || refuse 'case namespace may not be a symlink'
mkdir -p "$record_root" || refuse 'case namespace unavailable'
[ "$(stat -c %u "$record_root")" = "$(id -u)" ] || refuse 'case namespace is not task-owned'
case_dir="$record_root/Q4"
mkdir "$case_dir" || refuse 'fresh Q4 directory required; no second case or automatic retry'
mkdir "$case_dir/records" "$case_dir/records/outer" || exit 65
cp -- "$admission" "$case_dir/prelaunch-admission.json" || exit 65
cp -- "$safe_proof" "$case_dir/lease-before-create.json" || exit 65
date -u +%FT%T.%NZ > "$case_dir/started-at"
container_id=''
inspect_format='{"Id":{{json .Id}},"Image":{{json .Image}},"State":{"Status":{{json .State.Status}},"Running":{{json .State.Running}},"OOMKilled":{{json .State.OOMKilled}},"Pid":{{json .State.Pid}},"ExitCode":{{json .State.ExitCode}},"Error":{{json .State.Error}},"StartedAt":{{json .State.StartedAt}},"FinishedAt":{{json .State.FinishedAt}}},"HostConfig":{"Privileged":{{json .HostConfig.Privileged}},"CgroupnsMode":{{json .HostConfig.CgroupnsMode}},"NetworkMode":{{json .HostConfig.NetworkMode}},"ReadonlyRootfs":{{json .HostConfig.ReadonlyRootfs}},"Memory":{{json .HostConfig.Memory}},"MemorySwap":{{json .HostConfig.MemorySwap}},"NanoCpus":{{json .HostConfig.NanoCpus}},"PidsLimit":{{json .HostConfig.PidsLimit}},"SecurityOpt":{{json .HostConfig.SecurityOpt}}},"Mounts":{{json .Mounts}}}'
finish() {
  local status=$?
  trap - EXIT
  if [ -n "$container_id" ]; then
    managed_docker logs "$container_id" > "$case_dir/container.stdout" 2> "$case_dir/container.stderr"
    printf '%s\n' "$?" > "$case_dir/logs.exit"
    managed_docker inspect --format "$inspect_format" "$container_id" > "$case_dir/inspect-after.json" 2> "$case_dir/inspect-after.stderr"
    printf '%s\n' "$?" > "$case_dir/inspect-after.exit"
    # Exact newly owned container only; retains cleanup failures and partial runs.
    managed_docker rm -f "$container_id" > "$case_dir/remove.stdout" 2> "$case_dir/remove.stderr"
    printf '%s\n' "$?" > "$case_dir/remove.exit"
  fi
  printf '%s\n' "$status" > "$case_dir/launcher.exit"
  date -u +%FT%T.%NZ > "$case_dir/ended-at"
  exit "$status"
}
trap finish EXIT
trap 'printf "%s\n" interrupted > "$case_dir/external-censorship"; exit 130' INT
trap 'printf "%s\n" terminated > "$case_dir/external-censorship"; exit 143' TERM

capture_outer() {
  local outer_rel outer_path outer_index=0 field
  date -u +%FT%T.%NZ > "$case_dir/records/outer/sample-started-at"
  cat /proc/self/cgroup > "$case_dir/records/outer/process-cgroup" || return 1
  awk '$0 ~ / - cgroup2 / {print}' /proc/self/mountinfo > "$case_dir/records/outer/mountinfo" || return 1
  cat /proc/self/stat > "$case_dir/records/outer/launcher-proc-stat" || return 1
  awk '/^(Uid|Gid|Cap|Seccomp|NoNewPrivs)/' /proc/self/status > "$case_dir/records/outer/launcher-identity" || return 1
  outer_rel=$(awk -F: '$1=="0" {print $3}' /proc/self/cgroup)
  [[ "$outer_rel" = /* && "$outer_rel" != *..* ]] || return 1
  # Require the observed unified mount to map this namespace root directly.
  awk '$4=="/" && $5=="/sys/fs/cgroup" && / - cgroup2 / {found=1} END {exit !found}' /proc/self/mountinfo || return 1
  outer_path="/sys/fs/cgroup${outer_rel%/}"
  while :; do
    mkdir "$case_dir/records/outer/$outer_index" || return 1
    date -u +%FT%T.%NZ > "$case_dir/records/outer/$outer_index/sample-started-at"
    printf '%s\n' "$outer_path" > "$case_dir/records/outer/$outer_index/mapping"
    for field in memory.max memory.current memory.swap.max memory.swap.current memory.events memory.peak memory.stat cpu.max cpu.stat pids.max pids.current cgroup.events; do
      cat "$outer_path/$field" > "$case_dir/records/outer/$outer_index/$field" 2> "$case_dir/records/outer/$outer_index/$field.read-error"
      field_status=$?
      printf '%s\n' "$field_status" > "$case_dir/records/outer/$outer_index/$field.read-exit"
      case "$field" in memory.max|memory.current|memory.swap.max|memory.swap.current) [ "$field_status" -eq 0 ] || return 1;; esac
    done
    date -u +%FT%T.%NZ > "$case_dir/records/outer/$outer_index/sample-ended-at"
    [ "$outer_path" = /sys/fs/cgroup ] && break
    outer_path=$(dirname "$outer_path")
    outer_index=$((outer_index+1))
  done
  printf '%s\n' 'observed visible namespace only; hidden ancestors unobserved; worker8GiB excluded by original native H check' > "$case_dir/records/outer/visibility"
  cat /proc/meminfo > "$case_dir/records/outer/host-meminfo" || return 1
  date -u +%FT%T.%NZ > "$case_dir/records/outer/sample-ended-at"
}
container_name=jsr-6f4f28-q4
managed_docker create --name "$container_name" --label task=TASK-20261029-6f4f28 \
  --privileged --cgroupns=private --network=none --read-only \
  --memory=17179869184 --memory-swap=17179869184 --cpus=2 --pids-limit=64 \
  --mount "type=bind,src=$build_root/guard-qualification,dst=$build_root/guard-qualification,readonly" \
  --mount "type=bind,src=$build_root/guard-worker,dst=$build_root/guard-worker,readonly" \
  --mount "type=bind,src=/workspace/skeleton_sage-runtime/lib/libcrypto.so.3,dst=/workspace/skeleton_sage-runtime/lib/libcrypto.so.3,readonly" \
  --mount "type=bind,src=/workspace/skeleton_sage-runtime/lib/libstdc++.so.6.0.36,dst=/workspace/skeleton_sage-runtime/lib/libstdc++.so.6,readonly" \
  --mount "type=bind,src=/workspace/skeleton_sage-runtime/lib/libgcc_s.so.1,dst=/workspace/skeleton_sage-runtime/lib/libgcc_s.so.1,readonly" \
  --mount "type=bind,src=$case_dir/records,dst=/records" \
  "$image_ref" "$build_root/guard-qualification" Q4 "$build_root/guard-worker" /records/case \
  > "$case_dir/container.id" 2> "$case_dir/create.stderr"
create_result=$?
printf '%s\n' "$create_result" > "$case_dir/create.exit"
[ "$create_result" -eq 0 ] || exit 66
container_id=$(cat "$case_dir/container.id")
managed_docker inspect --format "$inspect_format" "$container_id" > "$case_dir/inspect-before.json" 2> "$case_dir/inspect-before.stderr"
inspect_result=$?
printf '%s\n' "$inspect_result" > "$case_dir/inspect-before.exit"
[ "$inspect_result" -eq 0 ] || exit 66
jq -e --arg image "$image_id" '.Image==$image and .HostConfig.Memory==17179869184 and .HostConfig.MemorySwap==17179869184 and .HostConfig.NanoCpus==2000000000 and .HostConfig.PidsLimit==64 and .HostConfig.Privileged==true and .HostConfig.CgroupnsMode=="private" and .HostConfig.NetworkMode=="none" and .HostConfig.ReadonlyRootfs==true and ([.Mounts[].Destination]|index("/var/run/docker.sock")|not)' "$case_dir/inspect-before.json" > "$case_dir/resource-inspect-check.stdout" 2> "$case_dir/resource-inspect-check.stderr" || exit 66
# Refresh all actually visible enclosing samples after create, immediately before
# start. Original C1 recomputes H including current parent/delegated constraints.
capture_outer || { printf '%s\n' infrastructure_not_started > "$case_dir/infrastructure-state"; exit 68; }
check_lease || { printf '%s\n' infrastructure_not_started > "$case_dir/infrastructure-state"; exit 68; }
cp -- "$safe_proof" "$case_dir/lease-before-start.json" || exit 68
date -u +%FT%T.%NZ > "$case_dir/start-requested-at"
managed_docker start "$container_id" > "$case_dir/start.stdout" 2> "$case_dir/start.stderr"
start_result=$?
printf '%s\n' "$start_result" > "$case_dir/start.exit"
[ "$start_result" -eq 0 ] || exit 67
managed_docker inspect --format "$inspect_format" "$container_id" > "$case_dir/inspect-running.json" 2> "$case_dir/inspect-running.stderr"
printf '%s\n' "$?" > "$case_dir/inspect-running.exit"
timeout 1950 env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS -u DOCKER_TLS_VERIFY -u DOCKER_CERT_PATH \
  docker --host=unix:///var/run/docker.sock wait "$container_id" > "$case_dir/wait.container-exit" 2> "$case_dir/wait.stderr"
wait_result=$?
printf '%s\n' "$wait_result" > "$case_dir/wait-cli.exit"
if [ "$wait_result" -eq 124 ]; then
  printf '%s\n' 'external1950s fail-safe; censored, never production-watchdog/OOM success' > "$case_dir/external-censorship"
  managed_docker kill "$container_id" > "$case_dir/owned-container-kill.stdout" 2> "$case_dir/owned-container-kill.stderr"
  printf '%s\n' "$?" > "$case_dir/owned-container-kill.exit"
fi
[ "$wait_result" -eq 0 ] || exit "$wait_result"
container_exit=$(cat "$case_dir/wait.container-exit")
[[ "$container_exit" =~ ^[0-9]+$ ]] && [ "$container_exit" -le 255 ] || exit 70
exit "$container_exit"
