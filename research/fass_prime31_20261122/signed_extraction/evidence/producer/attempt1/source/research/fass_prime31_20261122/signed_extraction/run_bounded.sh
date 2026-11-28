#!/usr/bin/env bash
# Thin custody/build/run wrapper. All numerical work is performed by the
# supplied native executable. This script never overwrites a call receipt.
set -euo pipefail
set -o noclobber

if (( $# < 7 )); then
  printf '%s\n' 'usage: run_bounded.sh NEW_RECEIPT_DIR ABSOLUTE_INPUT_MANIFEST SECONDS MEMORY_KIB STDOUT_NAME -- COMMAND [ARG...]' >&2
  exit 64
fi
receipt_dir=$1
input_manifest=$2
seconds=$3
memory_kib=$4
stdout_name=$5
shift 5
[[ $1 == -- ]] || exit 64
shift
[[ $seconds =~ ^[1-9][0-9]*$ && $memory_kib =~ ^[1-9][0-9]*$ ]] || exit 64
[[ $input_manifest == /* && -f $input_manifest ]] || exit 64
[[ $stdout_name != */* && -n $stdout_name ]] || exit 64

mkdir -m 755 "$receipt_dir"
cp -- "$input_manifest" "$receipt_dir/pre-call-inputs.sha256"
sha256sum "$0" > "$receipt_dir/wrapper.sha256"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$receipt_dir/started-at.txt"
pwd -P > "$receipt_dir/cwd.txt"
printf '%q ' "$@" > "$receipt_dir/command.sh"
printf '\n' >> "$receipt_dir/command.sh"
printf 'timeout_seconds=%s\nvirtual_memory_kib=%s\nkill_grace_seconds=10\nthreads=1\n' \
  "$seconds" "$memory_kib" > "$receipt_dir/limits.txt"
uname -a > "$receipt_dir/host.txt"
sha256sum --check "$receipt_dir/pre-call-inputs.sha256" > "$receipt_dir/pre-call-check.txt"

set +e
(
  ulimit -v "$memory_kib"
  export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 RUST_TEST_THREADS=1 CARGO_BUILD_JOBS=1
  exec timeout --signal=TERM --kill-after=10s "${seconds}s" "$@"
) > "$receipt_dir/$stdout_name" 2> "$receipt_dir/stderr.log"
call_status=$?
set -e

printf '%s\n' "$call_status" > "$receipt_dir/exit-status.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$receipt_dir/finished-at.txt"
set +e
post_manifest_status=0
while IFS= read -r input_line; do
  input_path=${input_line#*  }
  sha256sum -- "$input_path" || post_manifest_status=$?
done < "$receipt_dir/pre-call-inputs.sha256" > "$receipt_dir/post-call-inputs.sha256" 2> "$receipt_dir/post-call-manifest.stderr"
sha256sum --check "$receipt_dir/pre-call-inputs.sha256" > "$receipt_dir/post-call-check.txt" 2> "$receipt_dir/post-call-check.stderr"
custody_status=$?
set -e
if (( post_manifest_status != 0 )); then
  custody_status=$post_manifest_status
fi
printf '%s\n' "$custody_status" > "$receipt_dir/custody-status.txt"
sha256sum "$receipt_dir/$stdout_name" "$receipt_dir/stderr.log" > "$receipt_dir/output.sha256"
wc -c "$receipt_dir/$stdout_name" "$receipt_dir/stderr.log" > "$receipt_dir/output-bytes.txt"
printf 'call_exit=%s custody_exit=%s receipt=%s\n' "$call_status" "$custody_status" "$receipt_dir"
if (( custody_status != 0 )); then
  exit 65
fi
exit "$call_status"
