#!/usr/bin/env bash
set -eu
cd "$(git rev-parse --show-toplevel)"
study=research/hec_degree2_lp_20261116
for output in CONTROL.tsv PREFIX.tsv EXIT.txt; do
  if [ -e "$study/$output" ]; then
    echo "Refusing to overwrite existing scientific output: $study/$output" >&2
    exit 1
  fi
done
shasum -a 256 -c "$study/FREEZE.sha256" > "${TMPDIR:-/tmp}/hec-degree2-resume-freeze-check.txt"
# The default credentialed profile authenticates; the known bucket currently rejects access.
# An approved replacement destination must be documented before setting CHECKPOINT_BUCKET.
profile="${CHECKPOINT_AWS_PROFILE:-default}"
env -u CHECKPOINT_UPLOAD_TOKEN CHECKPOINT_TOKEN_FILE=/dev/null CHECKPOINT_AWS_PROFILE="$profile" scripts/checkpoint.sh push
env -u CHECKPOINT_UPLOAD_TOKEN CHECKPOINT_TOKEN_FILE=/dev/null CHECKPOINT_AWS_PROFILE="$profile" scripts/checkpoint.sh gate
set +e
/usr/bin/perl -e 'alarm 180; exec @ARGV' /Volumes/SSD990-2/hec-degree2-lp-target/release/degree2-lp-gate > "$study/CONTROL.tsv" 2> "$study/CONTROL.stderr"
status=$?
set -e
printf '%s\n' "$status" > "$study/EXIT.txt"
exit "$status"
