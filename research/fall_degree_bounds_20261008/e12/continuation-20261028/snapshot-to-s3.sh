#!/usr/bin/env bash
# Preserve live E12 output bytes; this does not checkpoint solver memory.
set -euo pipefail
job=${1:?expected job ID required}
case "$job" in
  20261028-021829-fd-e12a-r6a-20261028-r3-0c1d7f|20261028-021036-fd-e12c-20261028-r3-914d46|20261028-031317-fd-e12b-spot-20261028-r3-24148b) ;;
  *) echo 'Unregistered job' >&2; exit 2 ;;
esac
work=${E12_WORK_DIR:-/work}
cache=${E12_SNAPSHOT_DIR:-/var/lib/ephemeral-e12-snapshots}
bucket=ephemeral-runs-104186811093-us-east-1
mkdir -p "$cache"
exec 9>"$cache/upload.lock"
flock -n 9 || exit 0
jq -e --arg job "$job" '.id == $job' "$work/job.json" >/dev/null
deadline=$(jq -er '.deadline' "$work/job.json")
[ "$(date +%s)" -lt "$deadline" ] || exit 0
trap 'rc=$?; if [ "$rc" -ne 0 ]; then printf "[e12-snapshot] upload failed, exit=%s\n" "$rc" >>"$work/job.log"; fi' EXIT
snapshot=$(date -u +%Y%m%dT%H%M%SZ)-$(cut -c1-8 /proc/sys/kernel/random/uuid)
prefix="jobs/$job/snapshots/$snapshot"
parts=(out)
for name in host.txt rusage.txt; do
  [ ! -f "$work/$name" ] || parts+=("$name")
done
tar_rc=0
tar -C "$work" -czf "$cache/output.tar.gz" "${parts[@]}" 2>"$cache/tar.log" || tar_rc=$?
# tar's live-file-change warning is recorded. Other failures are not published.
[ "$tar_rc" -le 1 ] || exit "$tar_rc"
tar -tzf "$cache/output.tar.gz" >"$cache/files.txt"
sha=$(sha256sum "$cache/output.tar.gz" | cut -d' ' -f1)
checksum=$(openssl dgst -sha256 -binary "$cache/output.tar.gz" | base64 -w0)
aws s3api put-object --region us-east-1 --bucket "$bucket" \
  --key "$prefix/output.tar.gz" --body "$cache/output.tar.gz" \
  --if-none-match '*' --checksum-algorithm SHA256 >"$cache/upload-receipt.json"
jq -e --arg checksum "$checksum" '.ChecksumSHA256 == $checksum' \
  "$cache/upload-receipt.json" >/dev/null
jq -n --arg job "$job" --arg snapshot "$snapshot" --arg bucket "$bucket" \
  --arg key "$prefix/output.tar.gz" --arg sha "$sha" \
  --argjson bytes "$(stat -c %s "$cache/output.tar.gz")" --argjson tar_rc "$tar_rc" \
  --rawfile files "$cache/files.txt" --rawfile warnings "$cache/tar.log" \
  '{schema:"e12_output_snapshot/v1",job:$job,snapshot:$snapshot,created_utc:(now|todate),
    bucket:$bucket,archive_key:$key,sha256:$sha,bytes:$bytes,
    s3_checksum_verified:true,tar_exit_code:$tar_rc,tar_warnings:$warnings,
    files:($files|split("\n")|map(select(length>0))),
    capture:"live files; not an atomic solver-memory checkpoint",scientific_result:null}' \
  >"$cache/snapshot.json"
# The immutable manifest is the commit marker; publish it after the archive.
aws s3api put-object --region us-east-1 --bucket "$bucket" \
  --key "$prefix/snapshot.json" --body "$cache/snapshot.json" \
  --if-none-match '*' --checksum-algorithm SHA256 >/dev/null
aws s3api put-object --region us-east-1 --bucket "$bucket" \
  --key "jobs/$job/snapshots/latest.json" --body "$cache/snapshot.json" \
  --checksum-algorithm SHA256 >/dev/null
printf '[e12-snapshot] saved s3://%s/%s sha256=%s\n' "$bucket" "$prefix/output.tar.gz" "$sha" >>"$work/job.log"
cat "$cache/snapshot.json"
