#!/usr/bin/env bash
set -euo pipefail
fix=/home/ec2-user/pencil-controller-review-20261029/runtime-permission-review-20261030
old=104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:4201400427fcdba18fe88cfda6cd7f46b12db15fcc371ee56166d0fca5f5caa3
new=$(jq -er .runtime_image_uri "$fix/runtime-image.json")
docker() { sudo -n env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS -u DOCKER_TLS_VERIFY -u DOCKER_CERT_PATH /usr/bin/docker --host=unix:///var/run/docker.sock "$@"; }
docker pull "$new"
for kind in old new; do
  image=$old; [[ $kind == old ]] || image=$new
  docker run --rm --network none --read-only --cap-drop ALL --security-opt no-new-privileges --user 0:0 --entrypoint /bin/sh "$image" -c 'find /opt/isolab/recovery -printf "%P %m %U %G %y\n" | sort' > "$fix/$kind-modes.txt"
  docker run --rm --network none --read-only --cap-drop ALL --security-opt no-new-privileges --user 0:0 --entrypoint /bin/sh "$image" -c 'find /opt/isolab/recovery -type f -exec sha256sum {} + | sort; sha256sum /usr/local/bin/ecbench /opt/isolab/build-info.json' > "$fix/$kind-bytes.sha256"
done
cmp "$fix/old-bytes.sha256" "$fix/new-bytes.sha256"
sed 's/^isolab-criu-progress 700 /isolab-criu-progress 755 /' "$fix/old-modes.txt" | cmp - "$fix/new-modes.txt"
diff -u "$fix/old-modes.txt" "$fix/new-modes.txt" > "$fix/mode-only.diff" || [[ $? == 1 ]]
mkdir "$fix/stopped-extraction"
container=$(docker create "$new" /bin/true)
trap 'docker rm "$container" >/dev/null' EXIT
docker inspect "$container" --format '{{json .State}}' > "$fix/stopped-container-state.json"
docker cp "$container:/usr/local/bin/ecbench" "$fix/stopped-extraction/ecbench"
docker cp "$container:/opt/isolab/recovery/isolab-criu-progress" "$fix/stopped-extraction/isolab-criu-progress"
docker cp "$container:/opt/isolab/recovery/isolab-eks-recovery" "$fix/stopped-extraction/isolab-eks-recovery"
sudo -n sha256sum "$fix/stopped-extraction/"* > "$fix/stopped-extracted.sha256"
set +e
docker run --rm --network none --read-only --cap-drop ALL --security-opt no-new-privileges --user 1000:1000 --entrypoint /opt/isolab/recovery/isolab-criu-progress "$new" --help > "$fix/corrected-uid1000-help.stdout.log" 2> "$fix/corrected-uid1000-help.stderr.log"
code=$?
set -e
printf '%s\n' "$code" > "$fix/corrected-uid1000-help.exit"
[[ $code == 1 ]]
rg -q '^isolab-criu-progress: Usage:' "$fix/corrected-uid1000-help.stderr.log"
date -u +%Y-%m-%dT%H:%M:%SZ > "$fix/verification-end.txt"
