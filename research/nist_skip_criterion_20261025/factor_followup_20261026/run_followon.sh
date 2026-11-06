#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
: "${NIST_FACTOR_OUT:?set a fresh output under this follow-up runs directory}"
: "${MSIEVE_BIN:=/tmp/nist-msieve-source/msieve}"
: "${GMP_ECM:=/workspace/tools/sage_skeleton_env/bin/ecm}"
: "${PARI_PREFIX:=/workspace/tools/sage_skeleton_env}"
case "$NIST_FACTOR_OUT" in "$here"/runs/*) ;; *) exit 1;; esac
[ ! -e "$NIST_FACTOR_OUT" ] || { echo 'output exists' >&2; exit 1; }
"$repo/scripts/checkpoint.sh" gate
mkdir -p "$NIST_FACTOR_OUT"
build_dir=$(mktemp -d "${TMPDIR:-/tmp}/nist-factor-followon.XXXXXX")
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror \
  -I"$PARI_PREFIX/include" -I"$here/../native" "$here/prepare_factor.cpp" \
  -L"$PARI_PREFIX/lib" -Wl,-rpath,"$PARI_PREFIX/lib" -lpari -lcrypto \
  -o "$build_dir/prepare_factor"
"$build_dir/prepare_factor" "$here/../inputs/historical.tsv" 13 plus \
  "$here/hints/curve-13-plus.json" "$NIST_FACTOR_OUT/p239-2491-plus"
"$build_dir/prepare_factor" "$here/../inputs/historical.tsv" 20 plus \
  "$here/partial_hints/curve-20-plus.json" "$NIST_FACTOR_OUT/p521-1136-plus"
sha256sum "$MSIEVE_BIN" "$GMP_ECM" "$build_dir/prepare_factor" > "$NIST_FACTOR_OUT/native-binaries.sha256"
printf 'p239: Msieve SIQS, one thread, 1800-second watchdog\np521: GMP-ECM, B1=3000000, 256 curves, 1800-second watchdog\n' > "$NIST_FACTOR_OUT/parameters.txt"
"$repo/scripts/checkpoint.sh" push
p239_value=$(cat "$NIST_FACTOR_OUT/p239-2491-plus.residuals.txt")
case "$p239_value" in ''|*[!0-9]*) exit 1;; esac
timeout --kill-after=10 1800 "$MSIEVE_BIN" -v -t 1 \
  -s "$NIST_FACTOR_OUT/p239-2491-plus.msieve.dat" \
  -l "$NIST_FACTOR_OUT/p239-2491-plus.msieve.log" "$p239_value" \
  > "$NIST_FACTOR_OUT/p239-2491-plus.msieve.stdout" 2> "$NIST_FACTOR_OUT/p239-2491-plus.msieve.stderr" &
qs_pid=$!
timeout --kill-after=10 1800 "$GMP_ECM" -c 256 \
  -savea "$NIST_FACTOR_OUT/p521-1136-plus.ecm-resume.txt" 3000000 \
  < "$NIST_FACTOR_OUT/p521-1136-plus.residuals.txt" \
  > "$NIST_FACTOR_OUT/p521-1136-plus.ecm.stdout" 2> "$NIST_FACTOR_OUT/p521-1136-plus.ecm.stderr" &
ecm_pid=$!
printf 'msieve_timeout_pid=%s\necm_timeout_pid=%s\n' "$qs_pid" "$ecm_pid" > "$NIST_FACTOR_OUT/pids.txt"
set +e
wait "$qs_pid"; qs_status=$?
printf '%s\n' "$qs_status" > "$NIST_FACTOR_OUT/p239-2491-plus.msieve.status"
wait "$ecm_pid"; ecm_status=$?
printf '%s\n' "$ecm_status" > "$NIST_FACTOR_OUT/p521-1136-plus.ecm.status"
set -e
"$repo/scripts/checkpoint.sh" push
printf 'Follow-on records retained; verify all reported factors independently.\n'
