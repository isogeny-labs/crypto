#!/usr/bin/env bash
set -euo pipefail
umask 077
[[ $# == 2 ]] || { echo 'usage: PREPARER NEW_STAGE ALLOCATION' >&2; exit 2; }
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo=$(/usr/bin/git -C "$here" rev-parse --show-toplevel)
cd "$repo"
[[ -z $(/usr/bin/git status --porcelain --untracked-files=normal) ]] || { echo 'Publish clean source first.' >&2; exit 2; }
branch=$(/usr/bin/git symbolic-ref --short HEAD)
[[ $(/usr/bin/git rev-parse HEAD) == $(/usr/bin/git rev-parse "refs/remotes/origin/$branch") ]] || exit 2
sha256sum -c "$here/validation/table47-source.sha256" >/dev/null
sha256sum -c "$here/validation/table47-build.sha256" >/dev/null
jq -e '.compute_type=="CPU" and .vcpuCount==4 and .memoryInGb==8 and .costPerHr>0 and .costPerHr<=0.40 and .lifetime_seconds==4500' "$2" >/dev/null
mkdir "$1"
stage=$(cd "$1" && pwd)
mkdir "$stage/bundle"
bundle=$stage/bundle
cp /tmp/ic-table47-native/worker "$bundle/driver"
cp /tmp/ic-table47-native/checker "$bundle/analyzer"
cp "$2" "$bundle/allocation.json"
cp "$here/RESUME_TABLE47.md" "$bundle/RESUME-before.md"
cp "$here/run-extension-table-task.sh" "$bundle/run-task.sh"
cp "$here/table-roots-plan.json" "$bundle/"
cp "$here/validation/table47-baseline-binding.json" "$bundle/baseline-binding.json"
/usr/bin/git rev-parse HEAD > "$bundle/source-revision.txt"
/home/agent/.cargo/bin/rustc -Vv > "$bundle/build-rustc.txt"
awk '{print $2}' "$here/validation/table47-source.sha256" > "$stage/source-paths.txt"
while IFS= read -r path; do /usr/bin/git ls-files --error-unmatch -- "$path" >/dev/null || exit 2; done < "$stage/source-paths.txt"
tar -T "$stage/source-paths.txt" -cf "$bundle/source-checkpoint.tar"
cp /workspace/ic-runpod-runs/runpod-extension-pipeline-20261116-41/returned/task/run/source-checkpoint.tar "$bundle/frozen-root-source.tar"
cp "$here/validation/table47-frozen-root-source.sha256" "$bundle/frozen-root-source.sha256"
(cd "$bundle" && sha256sum -c frozen-root-source.sha256) >/dev/null
(cd "$bundle" && sha256sum driver analyzer baseline-binding.json allocation.json run-task.sh table-roots-plan.json RESUME-before.md source-revision.txt build-rustc.txt source-checkpoint.tar frozen-root-source.tar frozen-root-source.sha256) > "$bundle/bundle.sha256"
tar -czf "$stage/bundle.tar.gz" -C "$bundle" .
sha256sum "$stage/bundle.tar.gz" > "$stage/bundle-tar.sha256"
printf 'Credential-free immutable bundle: %s\n' "$stage"
