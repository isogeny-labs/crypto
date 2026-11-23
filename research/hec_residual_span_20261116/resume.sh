#!/usr/bin/env bash
set -eu
cd "$(git rev-parse --show-toplevel)"
study=research/hec_residual_span_20261116
prior=research/hec_degree2_lp_20261116/EXIT.txt
if [ ! -f "$prior" ] || [ "$(cat "$prior")" != 0 ]; then
  echo 'Original unchanged quadratic gate has not completed successfully; extension remains unexecuted.' >&2
  exit 1
fi
for output in CONTROL.tsv CONTROL.stderr EXIT.txt; do
  if [ -e "$study/$output" ]; then echo "Refusing to overwrite $study/$output" >&2; exit 1; fi
done
shasum -a 256 -c "$study/FREEZE.sha256" > "${TMPDIR:-/tmp}/hec-residual-span-freeze-check.txt"
profile="${CHECKPOINT_AWS_PROFILE:-default}"
env -u CHECKPOINT_UPLOAD_TOKEN CHECKPOINT_TOKEN_FILE=/dev/null CHECKPOINT_AWS_PROFILE="$profile" scripts/checkpoint.sh push
env -u CHECKPOINT_UPLOAD_TOKEN CHECKPOINT_TOKEN_FILE=/dev/null CHECKPOINT_AWS_PROFILE="$profile" scripts/checkpoint.sh gate
set +e
/usr/bin/perl -e 'alarm 180; exec @ARGV' /Volumes/SSD990-2/hec-residual-span-target/release/residual-span-gate > "$study/CONTROL.tsv" 2> "$study/CONTROL.stderr"
status=$?
set -e
printf '%s\n' "$status" > "$study/EXIT.txt"
exit "$status"
