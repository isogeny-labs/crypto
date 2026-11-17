#!/usr/bin/env bash
set -u
root=/workspace/crypto-native-smallroot/experiments/joint-smallroot-native/spec/checks/TASK-20261026-1525ac
phase="$root/cache-advice-1"
managed_docker() {
  env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS -u DOCKER_TLS_VERIFY -u DOCKER_CERT_PATH docker --host=unix:///var/run/docker.sock "$@"
}
node - "$phase" <<'JS' || exit 65
const fs=require('fs');const p=process.argv[2];const l=JSON.parse(fs.readFileSync('/workspace/smallroot-session/native-guard-successor-current-lease.json'));if(!l.live||new Date(l.expires_at)<=new Date()||Date.now()-new Date(l.observed_at)>60000)throw Error('fresh live T4 lease required');fs.copyFileSync('/workspace/smallroot-session/native-guard-successor-current-lease.json',p+'/lease-before-invocation.json');if(Date.now()>=Date.parse('2026-10-27T02:03:52Z'))throw Error('goal epoch1 expired');
JS
args=(--name jsr-1525ac-cache1 --label task=TASK-20261026-1525ac --label phase=cache-advice-1 --user 1000:1000 --cap-drop ALL --security-opt no-new-privileges --read-only --network none --memory 16g --memory-swap 16g --cpus 2 --pids-limit 64)
args+=(--mount "type=bind,src=$phase/records,dst=$root")
args+=(--mount "type=bind,src=$root/cache-eligibility,dst=$root/cache-eligibility,readonly")
args+=(--mount type=bind,src=/workspace/native-smallroot-guard-followup-b85397,dst=/workspace/native-smallroot-guard-followup-b85397,readonly)
args+=(--mount type=bind,src=/workspace/skeleton_sage-runtime/lib,dst=/workspace/skeleton_sage-runtime/lib,readonly)
for part in "$root"/cache-eligibility/eligible-*.tsv; do
  while IFS=$'\t' read -r pathname metadata; do
    [ -n "$pathname" ] || continue
    args+=(--mount "type=bind,src=$pathname,dst=$pathname,readonly")
  done < "$part"
done
args+=(--env LD_LIBRARY_PATH=/workspace/skeleton_sage-runtime/lib)
image=debian:trixie-slim@sha256:62bd423aeee912639ca8ee681c2b762cb3e25137e0a92b20be455898a9f034ad
managed_docker create "${args[@]}" "$image" /workspace/native-smallroot-guard-followup-b85397/cache-preparation --advise "$root/cache-eligibility/eligible-parts.txt" "$root/cache-advice-1" > "$phase/create.stdout" 2> "$phase/create.stderr"
result=$?
printf '%s\n' "$result" > "$phase/create.exit"
[ "$result" -eq 0 ] || exit 66
container=$(cat "$phase/create.stdout")
format='{"Id":{{json .Id}},"Image":{{json .Image}},"Path":{{json .Path}},"Args":{{json .Args}},"User":{{json .Config.User}},"State":{{json .State}},"HostConfig":{"Privileged":{{json .HostConfig.Privileged}},"ReadonlyRootfs":{{json .HostConfig.ReadonlyRootfs}},"NetworkMode":{{json .HostConfig.NetworkMode}},"CgroupnsMode":{{json .HostConfig.CgroupnsMode}},"Memory":{{json .HostConfig.Memory}},"MemorySwap":{{json .HostConfig.MemorySwap}},"NanoCpus":{{json .HostConfig.NanoCpus}},"PidsLimit":{{json .HostConfig.PidsLimit}},"CapAdd":{{json .HostConfig.CapAdd}},"CapDrop":{{json .HostConfig.CapDrop}},"SecurityOpt":{{json .HostConfig.SecurityOpt}}}}'
managed_docker inspect --format "$format" "$container" > "$phase/inspect-before.json" 2> "$phase/inspect-before.stderr"
managed_docker inspect --format '{{range .Mounts}}{{json .}}{{println}}{{end}}' "$container" > /workspace/native-smallroot-guard-followup-b85397/cache-advice-1-mounts.jsonl 2> "$phase/inspect-mounts.stderr"
node - "$phase" <<'JS'
const fs=require('fs');const rows=fs.readFileSync('/workspace/native-smallroot-guard-followup-b85397/cache-advice-1-mounts.jsonl','utf8').trim().split('\n').filter(Boolean);for(let i=0;i<rows.length;i+=40)fs.writeFileSync(process.argv[2]+'/actual-mounts-'+Math.floor(i/40)+'.jsonl',rows.slice(i,i+40).join('\n')+'\n');
JS
managed_docker start "$container" > "$phase/start.stdout" 2> "$phase/start.stderr"
result=$?
printf '%s\n' "$result" > "$phase/start.exit"
[ "$result" -eq 0 ] || exit 67
managed_docker wait "$container" > "$phase/container-exit" 2> "$phase/wait.stderr"
printf '%s\n' "$?" > "$phase/wait-cli.exit"
managed_docker logs "$container" > "$phase/container.stdout" 2> "$phase/container.stderr"
managed_docker inspect --format "$format" "$container" > "$phase/inspect-after.json" 2> "$phase/inspect-after.stderr"
managed_docker rm "$container" > "$phase/remove.stdout" 2> "$phase/remove.stderr"
printf '%s\n' "$?" > "$phase/remove.exit"
cat "$phase/container.stdout" "$phase/container.stderr"
