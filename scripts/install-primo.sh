#!/usr/bin/env bash
set -euo pipefail

PRIMO_URL=https://github.com/primo-solver/primo.git
PRIMO_REV=fcc274c23ff5b146cc43d80b1b0317faf91c7f08

script_dir=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd -P)
repo_dir=$(CDPATH='' cd -- "$script_dir/.." && pwd -P)
default_data_dir=${XDG_DATA_HOME:-${HOME}/.local/share}
install_prefix=${PRIMO_PREFIX:-${default_data_dir}/isogeny-labs/primo}
mkdir -p "$install_prefix"
install_prefix=$(CDPATH='' cd -- "$install_prefix" && pwd -P)
source_dir=${install_prefix}/src
bin_dir=${install_prefix}/bin
binary=${source_dir}/build/primo
link=${bin_dir}/primo

for tool in git make cmake c++ python3 m4; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "install-primo: required tool not found: $tool" >&2
        exit 1
    fi
done

if [[ -e "$source_dir" ]]; then
    if [[ ! -d "$source_dir/.git" ]]; then
        echo "install-primo: existing source path is not a Git checkout: $source_dir" >&2
        exit 1
    fi
    if [[ -n $(git -C "$source_dir" status --porcelain) ]]; then
        echo "install-primo: refusing to change dirty source checkout: $source_dir" >&2
        exit 1
    fi
    remote_url=$(git -C "$source_dir" remote get-url origin)
    if [[ "$remote_url" != "$PRIMO_URL" && "$remote_url" != "${PRIMO_URL%.git}" ]]; then
        echo "install-primo: existing checkout has unexpected origin: $remote_url" >&2
        exit 1
    fi
else
    git clone --filter=blob:none --no-checkout "$PRIMO_URL" "$source_dir"
fi

git -C "$source_dir" fetch --depth 1 origin "$PRIMO_REV"
git -C "$source_dir" checkout --detach "$PRIMO_REV"
if [[ $(git -C "$source_dir" rev-parse HEAD) != "$PRIMO_REV" ]]; then
    echo "install-primo: source revision verification failed" >&2
    exit 1
fi

make -C "$source_dir" build
if [[ ! -x "$binary" ]]; then
    echo "install-primo: build did not produce $binary" >&2
    exit 1
fi

mkdir -p "$bin_dir"
if [[ -e "$link" && ! -L "$link" ]]; then
    echo "install-primo: refusing to replace non-symlink: $link" >&2
    exit 1
fi
ln -sfn "$binary" "$link"

smoke_output=$("$link" --quiet "$repo_dir/integrations/primo/model-smoke.smt2")
if [[ "$smoke_output" != *$'sat\n'* || "$smoke_output" != *"define-fun x0 () Bool true"* || "$smoke_output" != *"define-fun x1 () Bool false"* ]]; then
    echo "install-primo: model smoke test failed" >&2
    printf '%s\n' "$smoke_output" >&2
    exit 1
fi

"$link" --version
echo "install-primo: installed $link at $PRIMO_REV"
