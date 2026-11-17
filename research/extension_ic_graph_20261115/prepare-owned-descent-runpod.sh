#!/usr/bin/env bash
# Package published native code/binaries and the allowlisted supplied input.
set -euo pipefail
umask 077
[[ $# == 4 ]] || { echo 'usage: PREPARER NEW_STAGE INPUT_JSON ALLOCATION_JSON OLD42_RAW_DIR' >&2; exit 2; }
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo=$(/usr/bin/git -C "$here" rev-parse --show-toplevel)
cd "$repo"
[[ -z $(/usr/bin/git status --porcelain --untracked-files=normal) ]] || { echo 'Publish clean component source first.' >&2; exit 2; }
branch=$(/usr/bin/git symbolic-ref --short HEAD)
[[ $(/usr/bin/git rev-parse HEAD) == $(/usr/bin/git rev-parse "refs/remotes/origin/$branch") ]] || { echo 'Published source HEAD required.' >&2; exit 2; }
sha256sum -c "$here/validation/owned-descent-source.sha256" >/dev/null
sha256sum -c "$here/validation/owned-descent-build.sha256" >/dev/null
[[ $(sha256sum "$2" | cut -d ' ' -f1) == $(jq -er .input_sha256 "$here/anchored-descent-plan.json") ]] || exit 2
jq -e '.compute_type=="CPU" and .vcpuCount==4 and .memoryInGb==8 and .costPerHr>0 and .costPerHr<=0.40 and .lifetime_seconds==5400' "$3" >/dev/null
mkdir "$1"
stage=$(cd "$1" && pwd)
mkdir "$stage/bundle"
bundle=$stage/bundle
cp /tmp/ic-owned43-native/worker "$bundle/driver"
cp /tmp/ic-anchored42-native/checker "$bundle/analyzer"
cp /tmp/ic-owned43-native/repetition-check "$bundle/repeat-check"
mkdir "$bundle/prior42"
for mode in original-nagao full-only anchored; do cp "$4/$mode.raw.json" "$bundle/prior42/$mode.raw.json"; done
cp "$2" "$bundle/input.json"
cp "$3" "$bundle/allocation.json"
cp "$here/run-owned-descent-task.sh" "$bundle/run-task.sh"
cp "$here/anchored-descent-plan.json" "$bundle/"
cp .resume/RESUME.md "$bundle/RESUME-before.md"
/usr/bin/git rev-parse HEAD > "$bundle/source-revision.txt"
/home/agent/.cargo/bin/rustc -Vv > "$bundle/build-rustc.txt"
awk '{print $2}' "$here/validation/owned-descent-source.sha256" > "$stage/source-paths.txt"
while IFS= read -r path; do /usr/bin/git ls-files --error-unmatch -- "$path" >/dev/null || exit 2; done < "$stage/source-paths.txt"
tar -T "$stage/source-paths.txt" -cf "$bundle/source-checkpoint.tar"
(cd "$bundle" && sha256sum driver analyzer repeat-check prior42/*.raw.json input.json allocation.json run-task.sh anchored-descent-plan.json RESUME-before.md source-revision.txt build-rustc.txt source-checkpoint.tar) > "$bundle/bundle.sha256"
tar -czf "$stage/bundle.tar.gz" -C "$bundle" .
sha256sum "$stage/bundle.tar.gz" > "$stage/bundle-tar.sha256"
printf 'Immutable credential-free bundle: %s\n' "$stage"
