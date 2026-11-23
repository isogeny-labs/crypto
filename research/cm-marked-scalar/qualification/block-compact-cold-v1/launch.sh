#!/usr/bin/env bash
set -eu
cd /home/ec2-user/crypto-cm-marked-scalar-20261108
q=research/cm-marked-scalar/qualification/block-compact-cold-v1
[ -f "$q/publication.json" ]
[ ! -e "$q/build.exit" ]
(cd "$q" && sha256sum -c source.sha256)
for source in "$q"/source/*; do cmp "$source" "research/cm-marked-scalar/$(basename "$source")"; done
ulimit -v 2097152
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
set +e
timeout -s TERM -k 5s 120s make -C research/cm-marked-scalar block-compact-eval-binary > "$q/build.log" 2>&1
status=$?
set -e
printf '%s\n' "$status" > "$q/build.exit"
[ "$status" = 0 ] || exit "$status"
sha256sum /tmp/cm-marked-scalar-build/block-compact-eval > "$q/binaries.sha256"
set +e
timeout -s TERM -k 5s 120s /tmp/cm-marked-scalar-build/block-compact-eval --qualify research/cm-marked-scalar/fixtures-v2 research/cm-marked-scalar/residual-v1/plans.json > "$q/qualifier.json" 2> "$q/qualifier.stderr"
status=$?
set -e
printf '%s\n' "$status" > "$q/qualifier.exit"
exit "$status"
