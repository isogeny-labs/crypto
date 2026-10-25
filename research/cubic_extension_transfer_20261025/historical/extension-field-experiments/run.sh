#!/bin/sh
set -eu
experiment_root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
if command -v sage >/dev/null 2>&1; then
    exec sage -python "$experiment_root/run_experiments.py" "$@"
fi
experiment_image='sagemath/sagemath@sha256:19995db6194f4a4bab18ce9a88556fd15b9ed5e916b4504fefe618a7796ddbdb'
exec docker run --rm --network none \
    --mount "type=bind,src=$experiment_root,dst=/work" --workdir /work \
    --entrypoint /bin/sh "$experiment_image" \
    -c 'exec sage -python /work/run_experiments.py "$@"' sh "$@"
