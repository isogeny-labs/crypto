#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
: "${NIST_FACTOR_OUT:?set a fresh absolute output directory outside frozen evidence}"
: "${PARI_PREFIX:=/workspace/tools/sage_skeleton_env}"
: "${GMP_ECM:=/workspace/tools/sage_skeleton_env/bin/ecm}"
: "${NIST_FACTOR_B1:=1000000}"
: "${NIST_FACTOR_CURVES:=64}"
: "${NIST_FACTOR_TIMEOUT:=900}"
case "$NIST_FACTOR_OUT" in /*) ;; *) echo 'output path must be absolute' >&2; exit 1;; esac
case "$NIST_FACTOR_OUT" in "$here"/runs/*) ;; *) echo 'output must be under this follow-up runs directory for checkpoint capture' >&2; exit 1;; esac
[ ! -e "$NIST_FACTOR_OUT" ] || { echo 'output already exists' >&2; exit 1; }
if git -C "$repo" check-ignore -q "$NIST_FACTOR_OUT/.checkpoint-probe"; then
  echo 'output would be ignored by the checkpoint collector' >&2
  exit 1
fi
"$repo/scripts/checkpoint.sh" gate
mkdir -p "$NIST_FACTOR_OUT"
build_dir=$(mktemp -d "${TMPDIR:-/tmp}/nist-factor-build.XXXXXX")
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror \
  -I"$PARI_PREFIX/include" -I"$here/../native" "$here/prepare_factor.cpp" \
  -L"$PARI_PREFIX/lib" -Wl,-rpath,"$PARI_PREFIX/lib" -lpari -lcrypto \
  -o "$build_dir/prepare_factor"
printf 'B1=%s\ncurves=%s\nper_target_timeout_seconds=%s\nworkers=2\n' \
  "$NIST_FACTOR_B1" "$NIST_FACTOR_CURVES" "$NIST_FACTOR_TIMEOUT" > "$NIST_FACTOR_OUT/parameters.txt"
sha256sum "$build_dir/prepare_factor" "$GMP_ECM" > "$NIST_FACTOR_OUT/native-binaries.sha256"
tail -n +2 "$here/targets.tsv" | while IFS="$(printf '\t')" read -r label row target; do
  prefix="$NIST_FACTOR_OUT/$label-$target"
  set +e
  "$build_dir/prepare_factor" "$here/../inputs/historical.tsv" "$row" "$target" \
    "$here/hints/curve-$row-$target.json" "$prefix" \
    > "$prefix.preflight.stdout" 2> "$prefix.preflight.stderr"
  preflight_status=$?
  set -e
  printf '%s\n' "$preflight_status" > "$prefix.preflight.status"
  if [ "$preflight_status" -ne 0 ]; then
    cat "$prefix.preflight.stderr" >&2
    "$repo/scripts/checkpoint.sh" push
    exit "$preflight_status"
  fi
  cat "$prefix.preflight.stdout"
done
"$repo/scripts/checkpoint.sh" push
export NIST_FACTOR_OUT GMP_ECM NIST_FACTOR_B1 NIST_FACTOR_CURVES NIST_FACTOR_TIMEOUT
tail -n +2 "$here/targets.tsv" | while IFS="$(printf '\t')" read -r label row target; do
  printf '%s\000' "$label-$target"
done | xargs -0 -n 1 -P 2 sh -c '
  prefix="$NIST_FACTOR_OUT/$1"
  if [ ! -s "$prefix.residuals.txt" ]; then
    printf "no composite residual; no ECM run\n" > "$prefix.ecm.status"
    exit 0
  fi
  set +e
  timeout "$NIST_FACTOR_TIMEOUT" "$GMP_ECM" -c "$NIST_FACTOR_CURVES" \
    -savea "$prefix.ecm-resume.txt" "$NIST_FACTOR_B1" \
    < "$prefix.residuals.txt" > "$prefix.ecm.stdout" 2> "$prefix.ecm.stderr"
  status=$?
  printf "%s\n" "$status" > "$prefix.ecm.status"
' worker
"$repo/scripts/checkpoint.sh" push
printf 'Batch records retained. No completed factorization is inferred from ECM text or exit status.\n'
