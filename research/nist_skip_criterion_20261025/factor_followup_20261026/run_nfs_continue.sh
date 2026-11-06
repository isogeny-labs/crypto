#!/bin/sh
set -eu
entry_dir=${NIST_FACTOR_BASE:-$(dirname -- "$0")}
here=$(CDPATH= cd -- "$entry_dir" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
: "${NIST_FACTOR_OUT:?set a fresh output under this follow-up runs directory}"
: "${NIST_NFS_PREVIOUS:=$here/runs/yafu-nfs-012}"
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
cmp "$NIST_NFS_PREVIOUS/input.txt" "$here/runs/refine-001/p521-1136-plus.residuals.txt"
cmp "$NIST_NFS_PREVIOUS/nfs.job" "$NIST_NFS_PREVIOUS/selected-job-original.job"
[ -s "$NIST_NFS_PREVIOUS/selected-job-admission.json" ] || { echo 'selected job not independently checked' >&2; exit 1; }
checkpoint gate
mkdir -p "$NIST_FACTOR_OUT/source-snapshot"
printf '%s\n' "$$" > "$NIST_FACTOR_OUT/launcher.pid"
cp "$0" "$here/FACTORING_PROTOCOL.md" "$NIST_FACTOR_OUT/source-snapshot/"
cp "$NIST_NFS_PREVIOUS/input.txt" "$NIST_NFS_PREVIOUS/selected-job-admission.json" "$NIST_FACTOR_OUT/"
for file in "$NIST_NFS_PREVIOUS"/nfs.*; do
    [ -f "$file" ] && [ ! -L "$file" ] || { echo 'nonregular progress file' >&2; exit 1; }
    cp "$file" "$NIST_FACTOR_OUT/"
done
sha256sum "$YAFU_BIN" "$0" "$NIST_FACTOR_OUT/input.txt" > "$NIST_FACTOR_OUT/native-input.sha256"
for file in "$NIST_FACTOR_OUT"/nfs.*; do
    sha256sum "$file" >> "$NIST_FACTOR_OUT/native-input.sha256"
done
printf 'threads=4\nseed=20261030\nminimum_relations_first_filter=1000000\nfilter_target_bump_percent=50\nwatchdog_seconds=14400\n' > "$NIST_FACTOR_OUT/parameters.txt"
checkpoint push
cd "$NIST_FACTOR_OUT"
set +e
LD_LIBRARY_PATH="$PARI_PREFIX/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
timeout --kill-after=60 14400 "$YAFU_BIN" 'nfs(@)' -R \
    -threads 4 -seed 20261030 -minrels 1000000 -filt_bump 50 \
    -ggnfs_dir "$GGNFS_DIR" -logfile factor.log -session session.log \
    < input.txt > run.stdout 2> run.stderr
status=$?
set -e
printf '%s\n' "$status" > run.status
checkpoint push
printf 'Four-thread NFS continuation retained; status %s still requires full prime/product proof.\n' "$status"
