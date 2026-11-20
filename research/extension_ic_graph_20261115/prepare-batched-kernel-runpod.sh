#!/usr/bin/env bash
set -euo pipefail
umask 077
[[ $# == 3 ]] || { echo 'usage: PREPARER NEW_STAGE INPUT ALLOCATION' >&2; exit 2; }
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo=$(/usr/bin/git -C "$here" rev-parse --show-toplevel)
cd "$repo"
[[ -z $(/usr/bin/git status --porcelain --untracked-files=normal) ]] || { echo 'Publish clean source first.' >&2; exit 2; }
branch=$(/usr/bin/git symbolic-ref --short HEAD)
[[ $(/usr/bin/git rev-parse HEAD) == $(/usr/bin/git rev-parse "refs/remotes/origin/$branch") ]] || exit 2
sha256sum -c "$here/validation/sparse45-source.sha256" >/dev/null
sha256sum -c "$here/validation/sparse45-build.sha256" >/dev/null
[[ $(sha256sum "$2" | cut -d ' ' -f1) == $(jq -er .input_sha256 "$here/batched-kernel-plan.json") ]] || exit 2
jq -e '.compute_type=="CPU" and .vcpuCount==4 and .memoryInGb==8 and .costPerHr>0 and .costPerHr<=0.40 and .lifetime_seconds==5400' "$3" >/dev/null
mkdir "$1"
stage=$(cd "$1" && pwd)
mkdir "$stage/bundle"
bundle=$stage/bundle
cp /tmp/ic-sparse44-native/worker "$bundle/driver"
cp /tmp/ic-sparse45-native/worker "$bundle/batch-driver"
cp /tmp/ic-sparse44-native/checker "$bundle/analyzer"
cp /tmp/ic-sparse45-native/equality-check "$bundle/equality-check"
cp "$2" "$bundle/input.json"
cp "$3" "$bundle/allocation.json"
cp "$here/run-batched-kernel-task.sh" "$bundle/run-task.sh"
cp "$here/batched-kernel-plan.json" "$bundle/"
/usr/bin/git rev-parse HEAD > "$bundle/source-revision.txt"
/home/agent/.cargo/bin/rustc -Vv > "$bundle/build-rustc.txt"
awk '{print $2}' "$here/validation/sparse45-source.sha256" > "$stage/source-paths.txt"
while IFS= read -r path; do /usr/bin/git ls-files --error-unmatch -- "$path" >/dev/null || exit 2; done < "$stage/source-paths.txt"
tar -T "$stage/source-paths.txt" -cf "$bundle/source-checkpoint.tar"
cp /workspace/ic-runpod-runs/runpod-extension-pipeline-20261116-41/returned/task/run/source-checkpoint.tar "$bundle/frozen-root-source.tar"
cp "$here/validation/sparse45-frozen-root-source.sha256" "$bundle/frozen-root-source.sha256"
(cd "$bundle" && sha256sum -c frozen-root-source.sha256) >/dev/null
(cd "$bundle" && sha256sum driver batch-driver analyzer equality-check input.json allocation.json run-task.sh batched-kernel-plan.json source-revision.txt build-rustc.txt source-checkpoint.tar frozen-root-source.tar frozen-root-source.sha256) > "$bundle/bundle.sha256"
tar -czf "$stage/bundle.tar.gz" -C "$bundle" .
sha256sum "$stage/bundle.tar.gz" > "$stage/bundle-tar.sha256"
printf 'Credential-free immutable bundle: %s\n' "$stage"
