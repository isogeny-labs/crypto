#!/usr/bin/env bash
set -euo pipefail
work=/home/ec2-user/scratch/n83-allocation-profile-20261021
set +e
sudo -n env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS -u DOCKER_TLS_VERIFY -u DOCKER_CERT_PATH docker --host=unix:///var/run/docker.sock run --rm --pull=never --name "n83-profile-loader-$$" --network=none --memory=8589934592 --memory-swap=8589934592 --cpus=1 --pids-limit=64 --cap-drop=ALL --security-opt=no-new-privileges --read-only --user=0:0 --mount "type=bind,src=$work/bin/native-storage-baseline,dst=/worker,readonly" sha256:541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63 /worker > "$work/loader-smoke.stdout" 2> "$work/loader-smoke.stderr"
rc=$?
set -e
printf '%s\n' "$rc" > "$work/loader-smoke.worker.exit"
test "$rc" -eq 1
rg -q 'usage:' "$work/loader-smoke.stderr"
printf '%s\n' PASS_IMAGE_LINKING_AND_UNPRIVILEGED_EXEC_NO_QUERY > "$work/loader-smoke.status.txt"
