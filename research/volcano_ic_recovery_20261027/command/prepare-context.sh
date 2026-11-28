#!/usr/bin/env bash
# Export committed, explicitly enumerated public source; no build or launch.
set -euo pipefail
umask 077
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
root=$(git -C "$here" rev-parse --show-toplevel)
pilot=research/volcano_ic_recovery_20261027
[[ $# == 1 && $1 == /* && ! -e $1 ]] || { echo 'usage: prepare-context.sh NEW_ABSOLUTE_DIRECTORY_OUTSIDE_REPOSITORY' >&2; exit 64; }
output=$(realpath -m -- "$1")
[[ $output != "$root" && $output != "$root/"* ]] || { echo 'context must be outside repository' >&2; exit 64; }
cd "$root"
[[ $(sha256sum "$pilot/SOURCES.sha256" | cut -d' ' -f1) == ff0c1804287eb3f281bbe2908d6ba149f2b33013f4abd6ecfaa2ff8f96c8e162 ]] || { echo 'frozen manifest changed' >&2; exit 65; }
sha256sum -c "$pilot/SOURCES.sha256" >&2
mapfile -t sources < <(awk '{print $2}' "$pilot/SOURCES.sha256")
sources+=("$pilot/SOURCES.sha256" "$pilot/command/launch.sh" "$pilot/command/Dockerfile" "$pilot/command/prepare-context.sh")
git diff --quiet HEAD -- "${sources[@]}" || { echo 'image sources differ from committed revision' >&2; exit 65; }
for path in "${sources[@]}"; do git cat-file -e "HEAD:$path" || { echo 'image sources must be committed first' >&2; exit 65; }; done
mkdir -- "$output"
mkdir -- "$output/repo"
for path in "${sources[@]}"; do
  mkdir -p -- "$output/repo/$(dirname -- "$path")"
  git show "HEAD:$path" > "$output/repo/$path"
done
git show "HEAD:$pilot/command/launch.sh" > "$output/launch.sh"
git show "HEAD:$pilot/command/Dockerfile" > "$output/Dockerfile"
git rev-parse HEAD > "$output/SOURCE_REVISION"
chmod 755 "$output/launch.sh"
(cd "$output" && sha256sum launch.sh Dockerfile SOURCE_REVISION > context.sha256)
echo "Prepared public committed source only: $output"
echo 'No image was built or published. Actual build/launch requires publication, budget, checkpoint and recovery admission.'
