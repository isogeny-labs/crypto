#!/usr/bin/env bash
# TASK-20261026-6374dd: thin Docker orchestration; no scientific entrypoint.
set -u
owner_root=/workspace/crypto-native-smallroot
build_root=/workspace/native-smallroot-guard-build-6374dd
record_root="$owner_root/experiments/joint-smallroot-native/spec/checks/TASK-20261026-6374dd"
image_ref=debian:trixie-slim@sha256:62bd423aeee912639ca8ee681c2b762cb3e25137e0a92b20be455898a9f034ad
managed_docker() {
  env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS -u DOCKER_TLS_VERIFY -u DOCKER_CERT_PATH docker --host=unix:///var/run/docker.sock "$@"
}
case "${1:-}" in Q1|Q2|Q3|Q4|Q5-missing|Q5-readonly) guard_case=$1;; *) echo 'requires one frozen class/subcase' >&2; exit 64;; esac
case_dir="$record_root/$guard_case"
mkdir "$case_dir" || exit 65
mkdir "$case_dir/records" "$case_dir/records/outer" || exit 65
# Read-only capture of every enclosing cgroup actually visible from this runtime.
cat /proc/self/cgroup > "$case_dir/records/outer/process-cgroup"
awk '$0 ~ / - cgroup2 / {print}' /proc/self/mountinfo > "$case_dir/records/outer/mountinfo"
outer_rel=$(awk -F: '$1=="0" {print $3}' /proc/self/cgroup)
outer_path="/sys/fs/cgroup${outer_rel%/}"
outer_index=0
while :; do
  mkdir "$case_dir/records/outer/$outer_index" || exit 65
  printf '%s\n' "$outer_path" > "$case_dir/records/outer/$outer_index/mapping"
  for guard_field in memory.max memory.current memory.swap.max memory.swap.current memory.events memory.peak memory.stat cpu.max cpu.stat pids.max pids.current cgroup.events; do
    cat "$outer_path/$guard_field" > "$case_dir/records/outer/$outer_index/$guard_field" 2> "$case_dir/records/outer/$outer_index/$guard_field.read-error" || true
  done
  [ "$outer_path" = /sys/fs/cgroup ] && break
  outer_path=$(dirname "$outer_path")
  outer_index=$((outer_index+1))
done
printf '%s\n' 'observed visible namespace only; hidden ancestors unobserved' > "$case_dir/records/outer/visibility"
cat /proc/meminfo > "$case_dir/records/outer/host-meminfo"
container_name="jsr-6374dd-${guard_case,,}"
privilege_args=(--privileged)
[ "$guard_case" = Q5-readonly ] && privilege_args=()
managed_docker create --name "$container_name" --label task=TASK-20261026-6374dd \
  "${privilege_args[@]}" --cgroupns=private --network=none --read-only \
  --memory=17179869184 --memory-swap=17179869184 --cpus=2 --pids-limit=64 \
  --mount "type=bind,src=$build_root,dst=$build_root,readonly" \
  --mount "type=bind,src=/workspace/skeleton_sage-runtime/lib,dst=/workspace/skeleton_sage-runtime/lib,readonly" \
  --mount "type=bind,src=$case_dir/records,dst=/records" \
  "$image_ref" "$build_root/guard-qualification" "$guard_case" "$build_root/guard-worker" /records/case \
  > "$case_dir/container.id" 2> "$case_dir/create.stderr"
create_result=$?
printf '%s\n' "$create_result" > "$case_dir/create.exit"
[ "$create_result" -eq 0 ] || exit 66
container_id=$(cat "$case_dir/container.id")
inspect_format='{"Id":{{json .Id}},"Image":{{json .Image}},"State":{"Status":{{json .State.Status}},"Running":{{json .State.Running}},"OOMKilled":{{json .State.OOMKilled}},"Pid":{{json .State.Pid}},"ExitCode":{{json .State.ExitCode}},"Error":{{json .State.Error}},"StartedAt":{{json .State.StartedAt}},"FinishedAt":{{json .State.FinishedAt}}},"HostConfig":{"Privileged":{{json .HostConfig.Privileged}},"CgroupnsMode":{{json .HostConfig.CgroupnsMode}},"NetworkMode":{{json .HostConfig.NetworkMode}},"ReadonlyRootfs":{{json .HostConfig.ReadonlyRootfs}},"Memory":{{json .HostConfig.Memory}},"MemorySwap":{{json .HostConfig.MemorySwap}},"NanoCpus":{{json .HostConfig.NanoCpus}},"PidsLimit":{{json .HostConfig.PidsLimit}}},"Mounts":{{json .Mounts}}}'
managed_docker inspect --format "$inspect_format" "$container_id" > "$case_dir/inspect-before.json" 2> "$case_dir/inspect-before.stderr"
managed_docker start "$container_id" > "$case_dir/start.stdout" 2> "$case_dir/start.stderr"
start_result=$?
printf '%s\n' "$start_result" > "$case_dir/start.exit"
[ "$start_result" -eq 0 ] || exit 67
managed_docker inspect --format "$inspect_format" "$container_id" > "$case_dir/inspect-running.json" 2> "$case_dir/inspect-running.stderr"
# External protection is1950s; it never qualifies the production1800s watchdog.
timeout 1950 env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS -u DOCKER_TLS_VERIFY -u DOCKER_CERT_PATH \
  docker --host=unix:///var/run/docker.sock wait "$container_id" > "$case_dir/wait.container-exit" 2> "$case_dir/wait.stderr"
wait_result=$?
printf '%s\n' "$wait_result" > "$case_dir/wait-cli.exit"
if [ "$wait_result" -eq 124 ]; then
  printf '%s\n' 'external1950s fail-safe; censored/failed, never production-watchdog success' > "$case_dir/external-censorship"
  managed_docker kill "$container_id" > "$case_dir/owned-container-kill.stdout" 2> "$case_dir/owned-container-kill.stderr" || true
fi
managed_docker logs "$container_id" > "$case_dir/container.stdout" 2> "$case_dir/container.stderr"
managed_docker inspect --format "$inspect_format" "$container_id" > "$case_dir/inspect-after.json" 2> "$case_dir/inspect-after.stderr"
managed_docker rm "$container_id" > "$case_dir/remove.stdout" 2> "$case_dir/remove.stderr"
printf '%s\n' "$?" > "$case_dir/remove.exit"
exit 0
