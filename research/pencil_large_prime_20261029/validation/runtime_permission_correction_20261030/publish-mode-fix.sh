#!/usr/bin/env bash
set -euo pipefail
fix=/home/ec2-user/pencil-controller-review-20261029/runtime-permission-review-20261030
repo=104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks
tag="recovery-access-$(sha256sum "$fix/context/Dockerfile" | cut -c1-40)"
printf '%s\n' "$tag" > "$fix/image-tag.txt"
docker() { sudo -n env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS -u DOCKER_TLS_VERIFY -u DOCKER_CERT_PATH /usr/bin/docker --host=unix:///var/run/docker.sock "$@"; }
aws ecr list-images --region us-east-2 --repository-name isolab/tasks --filter tagStatus=TAGGED --output json | jq --arg tag "$tag" '[.imageIds[]|select(.imageTag==$tag)]' > "$fix/prior-matching-tags.json"
[[ $(jq length "$fix/prior-matching-tags.json") == 0 ]]
date -u +%Y-%m-%dT%H:%M:%SZ > "$fix/publication-start.txt"
docker build --platform linux/amd64 --file "$fix/context/Dockerfile" --tag "$repo:$tag" "$fix/context"
docker push "$repo:$tag"
aws ecr describe-images --region us-east-2 --repository-name isolab/tasks --image-ids "imageTag=$tag" > "$fix/published-image-readback.json"
digest=$(jq -er '.imageDetails[0].imageDigest' "$fix/published-image-readback.json")
jq -n --arg image "$repo@$digest" '{runtime_image_uri:$image,previous_runtime_image_uri:"104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:4201400427fcdba18fe88cfda6cd7f46b12db15fcc371ee56166d0fca5f5caa3",base_image_uri:"104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:13f27597322fced9230635cedc5a4e228daf2fb951f77652edbbcab17e674c1b",runtime_archive_sha256:"72b227c00c3e3781b5b351aae133ffd2ece5c43313a4476b19c6d322bbffdaec",change:"Only progress executable mode0700 to0755"}' > "$fix/runtime-image.json"
date -u +%Y-%m-%dT%H:%M:%SZ > "$fix/publication-end.txt"
