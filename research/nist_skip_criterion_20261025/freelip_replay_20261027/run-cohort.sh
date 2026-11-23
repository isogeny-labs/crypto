#!/bin/sh
set -eu
if [ "${1:-}" = worker ]; then
    lip_run=$2
    for lip_version in 1.0 1.1; do
      for lip_method in probprime ordinary; do
        lip_i=0
        while [ "$lip_i" -lt 21 ]; do
          lip_case="$lip_run/v$lip_version-$lip_method/curve-$lip_i"
          set +e
          (ulimit -c 0 && ulimit -v 786432 && UBSAN_OPTIONS=halt_on_error=1 timeout --kill-after=5s 60s "$lip_run/bin/qemu-i386-static" -R 268435456 "$lip_run/bin/v$lip_version-replay" "$lip_method" < "$lip_case/input") > "$lip_case/stdout" 2> "$lip_case/stderr"
          lip_status=$?
          set -e
          printf '%s\n' "$lip_status" > "$lip_case/process.status"
          lip_i=$((lip_i+1))
        done
      done
    done
    exit 0
fi
: "${LIP_BASE:?study absolute directory}"
: "${LIP_PACKAGE:?prepared source-bound package}"
: "${LIP_OUT:?fresh study runs output}"
[ "${LIP_BACKEND:-}" = local-fallback-existing-authorization ] || exit 2
case "$LIP_OUT" in "$LIP_BASE"/runs/*) ;; *) exit 3 ;; esac
[ ! -e "$LIP_OUT" ] || exit 4
lip_repo=$(git -C "$LIP_BASE" rev-parse --show-toplevel)
checkpoint() { /tmp/nist-checkpoint-run "$lip_repo/scripts/checkpoint.sh" "$@"; }
checkpoint gate
cmp "$LIP_PACKAGE/package.sha256" "$LIP_BASE/package.sha256"
(cd "$LIP_PACKAGE" && sha256sum -c package.sha256) > /workspace/nist-lip-package-admission.txt
mkdir -p "$LIP_OUT/source-snapshot" "$LIP_OUT/results"
cp -R "$LIP_PACKAGE/bin" "$LIP_OUT/"
cp "$LIP_PACKAGE/inputs/cohort.tsv" "$LIP_OUT/"
cp "$LIP_BASE/PROTOCOL.md" "$LIP_BASE/protocol.json" "$LIP_BASE/PREPARED.json" "$LIP_BASE/replay.c" "$LIP_BASE/bridge.c" "$LIP_BASE/bridge.h" "$LIP_BASE/summarize.cpp" "$LIP_PACKAGE/package.sha256" "$0" "$LIP_OUT/source-snapshot/"
for lip_version in 1.0 1.1; do
  for lip_method in probprime ordinary; do
    lip_i=0
    while [ "$lip_i" -lt 21 ]; do
      lip_case="$LIP_OUT/v$lip_version-$lip_method/curve-$lip_i"
      mkdir -p "$lip_case"
      cp "$LIP_PACKAGE/inputs/curve-$lip_i/input" "$lip_case/"
      lip_i=$((lip_i+1))
    done
  done
done
(cd "$LIP_OUT" && find bin source-snapshot v1* -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > frozen.sha256)
checkpoint push
set +e
(ulimit -c 0 && ulimit -v 786432 && timeout --kill-after=10s 600s sh "$LIP_OUT/source-snapshot/$(basename "$0")" worker "$LIP_OUT") > "$LIP_OUT/worker.stdout" 2> "$LIP_OUT/worker.stderr"
lip_status=$?
set -e
printf '%s\n' "$lip_status" > "$LIP_OUT/worker.status"
(cd "$LIP_OUT" && sha256sum -c frozen.sha256) > "$LIP_OUT/frozen-check.stdout" 2> "$LIP_OUT/frozen-check.stderr"
(ulimit -c 0 && ulimit -v 786432 && timeout --kill-after=5s 60s "$LIP_OUT/bin/summarize" "$LIP_OUT" "$LIP_OUT/results") > "$LIP_OUT/summary.stdout" 2> "$LIP_OUT/summary.stderr"
checkpoint push
printf 'Full84-call fixed replay closed; inspect native outcomes and every failure.\n'
