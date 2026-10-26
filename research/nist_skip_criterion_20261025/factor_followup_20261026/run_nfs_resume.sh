#!/bin/sh
set -eu
entry_dir=${NIST_FACTOR_BASE:-$(dirname -- "$0")}
here=$(CDPATH= cd -- "$entry_dir" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
: "${NIST_FACTOR_OUT:?set a fresh output under this follow-up runs directory}"
: "${NIST_NFS_PREVIOUS:=$here/runs/yafu-nfs-009}"
: "${YAFU_BIN:=/tmp/nist-yafu-source/yafu}"
: "${GGNFS_DIR:=/tmp/nist-yafu-source/factor/lasieve5_64/bin/avx512/}"
: "${PARI_PREFIX:=/workspace/tools/sage_skeleton_env}"
checkpoint() {
    if [ -n "${NIST_CHECKPOINT_WRAPPER:-}" ]; then
        "$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" "$@"
    else
        "$repo/scripts/checkpoint.sh" "$@"
    fi
}
case "$NIST_FACTOR_OUT" in "$here"/runs/*) ;; *) exit 1;; esac
case "$NIST_NFS_PREVIOUS" in "$here"/runs/*) ;; *) exit 1;; esac
[ ! -e "$NIST_FACTOR_OUT" ] || { echo 'output exists' >&2; exit 1; }
[ -f "$NIST_NFS_PREVIOUS/run.status" ] || { echo 'prior run not closed' >&2; exit 1; }
[ -s "$NIST_NFS_PREVIOUS/nfs.dat.0.p" ] || { echo 'no prior polynomial candidates' >&2; exit 1; }
cmp "$NIST_NFS_PREVIOUS/input.txt" "$here/runs/refine-001/p521-1136-plus.residuals.txt"
checkpoint gate
mkdir -p "$NIST_FACTOR_OUT/source-snapshot"
cp "$0" "$here/FACTORING_PROTOCOL.md" "$NIST_FACTOR_OUT/source-snapshot/"
cp "$here/check_polynomials.cpp" "$here/../native/common.hpp" "$NIST_FACTOR_OUT/source-snapshot/"
cp "$NIST_NFS_PREVIOUS/input.txt" "$NIST_FACTOR_OUT/input.txt"
cp "$NIST_NFS_PREVIOUS/nfs.dat.0.p" "$NIST_FACTOR_OUT/prior-thread-polynomials.p"
value=$(cat "$NIST_FACTOR_OUT/input.txt")
case "$value" in ''|*[!0-9]*) exit 1;; esac
build_dir=$(mktemp -d "${TMPDIR:-/tmp}/nist-nfs-resume.XXXXXX")
set +e
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror \
    -I"$PARI_PREFIX/include" -I"$here/../native" "$here/check_polynomials.cpp" \
    -L"$PARI_PREFIX/lib" -Wl,-rpath,"$PARI_PREFIX/lib" -lpari -lcrypto \
    -o "$build_dir/check_polynomials" > "$NIST_FACTOR_OUT/build.stdout" 2> "$NIST_FACTOR_OUT/build.stderr"
build_status=$?
set -e
printf '%s\n' "$build_status" > "$NIST_FACTOR_OUT/build.status"
[ "$build_status" -eq 0 ] || { checkpoint push; exit "$build_status"; }
set +e
"$build_dir/check_polynomials" "$NIST_FACTOR_OUT/input.txt" \
    "$NIST_FACTOR_OUT/prior-thread-polynomials.p" "$NIST_FACTOR_OUT/nfs.dat.p" \
    "$NIST_FACTOR_OUT/polynomial-admission.json" \
    > "$NIST_FACTOR_OUT/polynomial-admission.stdout" 2> "$NIST_FACTOR_OUT/polynomial-admission.stderr"
admission_status=$?
set -e
printf '%s\n' "$admission_status" > "$NIST_FACTOR_OUT/polynomial-admission.status"
sha256sum "$YAFU_BIN" "$NIST_FACTOR_OUT/input.txt" \
    "$NIST_FACTOR_OUT/prior-thread-polynomials.p" "$build_dir/check_polynomials" "$0" \
    > "$NIST_FACTOR_OUT/native-input.sha256"
printf 'threads=2\nseed=20261029\npoly_custom_start_stop=1001,1002\npoly_watchdog_seconds=900\nresume_watchdog_seconds=10800\n' > "$NIST_FACTOR_OUT/parameters.txt"
checkpoint push
[ "$admission_status" -eq 0 ] || { echo 'polynomial input admission failed' >&2; exit 65; }
cd "$NIST_FACTOR_OUT"
set +e
LD_LIBRARY_PATH="$PARI_PREFIX/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
timeout --kill-after=60 900 "$YAFU_BIN" 'nfs(@)' -R -np 1001,1002 \
    -threads 2 -seed 20261029 -ggnfs_dir "$GGNFS_DIR" \
    -logfile poly-factor.log -session poly-session.log \
    < input.txt > poly.stdout 2> poly.stderr
poly_status=$?
set -e
printf '%s\n' "$poly_status" > poly.status
checkpoint push
[ "$poly_status" -eq 0 ] && [ -s nfs.job ] || {
    echo 'native polynomial finalization did not complete; no sieving launched' >&2
    exit 65
}
set +e
LD_LIBRARY_PATH="$PARI_PREFIX/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
timeout --kill-after=60 10800 "$YAFU_BIN" 'nfs(@)' -R \
    -threads 2 -seed 20261029 -ggnfs_dir "$GGNFS_DIR" \
    -logfile factor.log -session session.log \
    < input.txt > run.stdout 2> run.stderr
status=$?
set -e
printf '%s\n' "$status" > run.status
checkpoint push
printf 'Native NFS continuation retained; status %s requires separate factor proof.\n' "$status"
