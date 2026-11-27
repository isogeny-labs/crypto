#!/usr/bin/env bash
# Thin command/provenance recorder. No arithmetic, result derivation or retry.
# Usage from the repository root:
# record-command.sh RECEIPT_DIR SOURCE_PATH_LIST BINARY_OR_DASH COMMAND [ARG...]
set -euo pipefail
rr5_record=$1
rr5_source_list=$2
rr5_binary=$3
shift 3
test "$#" -gt 0
test ! -e "$rr5_record"
mkdir -p "$rr5_record"
cp "$rr5_source_list" "$rr5_record/actual-source-paths.txt"
while IFS= read -r rr5_path; do
  test -f "$rr5_path"
  sha256sum "$rr5_path"
done < "$rr5_source_list" > "$rr5_record/pre-command-sources.sha256"
if [ "$rr5_binary" != - ]; then
  sha256sum "$rr5_binary" > "$rr5_record/pre-call-binary.sha256"
fi
printf '%q ' "$@" > "$rr5_record/command.txt"
printf '\n' >> "$rr5_record/command.txt"
git rev-parse HEAD > "$rr5_record/parent-commit.txt"
rustc -Vv > "$rr5_record/rustc.txt"
cargo -V > "$rr5_record/cargo.txt"
date -u +%FT%TZ > "$rr5_record/started.utc"
set +e
"$@" > "$rr5_record/stdout.txt" 2> "$rr5_record/stderr.txt"
rr5_exit=$?
set -e
printf '%s\n' "$rr5_exit" > "$rr5_record/exit-status.txt"
date -u +%FT%TZ > "$rr5_record/completed.utc"
set +e
sha256sum -c "$rr5_record/pre-command-sources.sha256" > "$rr5_record/post-command-source-check.txt" 2>&1
rr5_sources_exit=$?
rr5_binary_exit=0
if [ "$rr5_binary" != - ]; then
  sha256sum -c "$rr5_record/pre-call-binary.sha256" > "$rr5_record/post-call-binary-check.txt" 2>&1
  rr5_binary_exit=$?
fi
set -e
printf '%s\n' "$rr5_sources_exit" > "$rr5_record/source-check-exit.txt"
printf '%s\n' "$rr5_binary_exit" > "$rr5_record/binary-check-exit.txt"
printf 'receipt=%s command_exit=%s source_check=%s binary_check=%s\n' \
  "$rr5_record" "$rr5_exit" "$rr5_sources_exit" "$rr5_binary_exit"
if [ "$rr5_sources_exit" != 0 ] || [ "$rr5_binary_exit" != 0 ]; then
  exit 125
fi
exit "$rr5_exit"
