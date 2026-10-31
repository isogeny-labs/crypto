#!/usr/bin/env bash
set -u
root=/workspace/crypto-native-smallroot/experiments/joint-smallroot-native/spec/checks/TASK-20261026-1525ac
phase="$root/post-advice-headroom-1"
managed_docker() {
  env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS -u DOCKER_TLS_VERIFY -u DOCKER_CERT_PATH docker --host=unix:///var/run/docker.sock "$@"
}
node - "$phase" <<'JS' || exit 65
const fs=require('fs'),p=process.argv[2],l=JSON.parse(fs.readFileSync('/workspace/smallroot-session/native-guard-successor-current-lease.json'));if(!l.live||new Date(l.expires_at)<=new Date()||Date.now()-new Date(l.observed_at)>60000||Date.now()>=Date.parse('2026-10-27T02:03:52Z'))throw Error('fresh live T4/goal claim required');fs.copyFileSync('/workspace/smallroot-session/native-guard-successor-current-lease.json',p+'/lease-before-probe.json');
JS
managed_docker ps --format '{{.ID}} {{.Names}}' > "$phase/active-containers-before"
[ ! -s "$phase/active-containers-before" ] || exit 66
cat /proc/self/cgroup > "$phase/outer-process-cgroup"
awk '$0 ~ / - cgroup2 / {print}' /proc/self/mountinfo > "$phase/outer-mountinfo"
capture_outer() {
  capture=$1
  mkdir "$capture"
  cat /proc/meminfo > "$capture/host-meminfo"
  date -u +%Y-%m-%dT%H:%M:%SZ > "$capture/observed-at"
  rel=$(awk -F: '$1=="0" {print $3}' /proc/self/cgroup)
  current="/sys/fs/cgroup${rel%/}"
  index=0
  while :; do
    mkdir "$capture/$index"
    printf '%s\n' "$current" > "$capture/$index/mapping"
    for key in memory.max memory.current memory.swap.max memory.swap.current memory.events memory.peak memory.stat cpu.max cpu.stat pids.max pids.current cgroup.events; do
      cat "$current/$key" > "$capture/$index/$key" 2> "$capture/$index/$key.read-error" || true
    done
    [ "$current" = /sys/fs/cgroup ] && break
    current=$(dirname "$current")
    index=$((index+1))
  done
  printf '%s\n' 'visible namespace only; hidden ancestors unobserved; no complete global headroom claim' > "$capture/visibility"
}
capture_outer "$phase/outer-before-probe"
image=debian:trixie-slim@sha256:62bd423aeee912639ca8ee681c2b762cb3e25137e0a92b20be455898a9f034ad
managed_docker create --name jsr-1525ac-postadvice-readback --label task=TASK-20261026-1525ac --label phase=post-advice-headroom-1 --privileged --cgroupns private --read-only --network none --memory 16g --memory-swap 16g --cpus 2 --pids-limit 64 --mount "type=bind,src=$phase/records,dst=/records" --mount "type=bind,src=$phase/probe.sh,dst=/probe.sh,readonly" --mount type=bind,src=/workspace/native-smallroot-guard-build-6374dd,dst=/build,readonly --mount type=bind,src=/workspace/skeleton_sage-runtime/lib,dst=/libprefix,readonly "$image" /bin/sh /probe.sh > "$phase/create.stdout" 2> "$phase/create.stderr"
result=$?
printf '%s\n' "$result" > "$phase/create.exit"
[ "$result" -eq 0 ] || exit 67
container=$(cat "$phase/create.stdout")
format='{"Id":{{json .Id}},"Image":{{json .Image}},"Path":{{json .Path}},"Args":{{json .Args}},"User":{{json .Config.User}},"State":{{json .State}},"HostConfig":{"Privileged":{{json .HostConfig.Privileged}},"ReadonlyRootfs":{{json .HostConfig.ReadonlyRootfs}},"NetworkMode":{{json .HostConfig.NetworkMode}},"CgroupnsMode":{{json .HostConfig.CgroupnsMode}},"Memory":{{json .HostConfig.Memory}},"MemorySwap":{{json .HostConfig.MemorySwap}},"NanoCpus":{{json .HostConfig.NanoCpus}},"PidsLimit":{{json .HostConfig.PidsLimit}},"CapAdd":{{json .HostConfig.CapAdd}},"CapDrop":{{json .HostConfig.CapDrop}},"SecurityOpt":{{json .HostConfig.SecurityOpt}}},"Mounts":{{json .Mounts}}}'
managed_docker inspect --format "$format" "$container" > "$phase/inspect-before.json" 2> "$phase/inspect-before.stderr"
managed_docker start "$container" > "$phase/start.stdout" 2> "$phase/start.stderr"
result=$?
printf '%s\n' "$result" > "$phase/start.exit"
[ "$result" -eq 0 ] || exit 68
managed_docker wait "$container" > "$phase/container-exit" 2> "$phase/wait.stderr"
printf '%s\n' "$?" > "$phase/wait-cli.exit"
capture_outer "$phase/outer-after-probe"
managed_docker logs "$container" > "$phase/container.stdout" 2> "$phase/container.stderr"
managed_docker inspect --format "$format" "$container" > "$phase/inspect-after.json" 2> "$phase/inspect-after.stderr"
managed_docker rm "$container" > "$phase/remove.stdout" 2> "$phase/remove.stderr"
printf '%s\n' "$?" > "$phase/remove.exit"
cat "$phase/container-exit" "$phase/container.stderr"
