set +e
printf '%s' "$BOOTSTRAP_BASE64" | base64 -d > /tmp/ecbench-bootstrap.sh
mkdir -p /results
# Evidence copies only: no process restore or solver state checkpoint.
(
  while sleep 60; do
    tar -czf /tmp/ecbench-partial.tgz --exclude=./target --exclude=./ladder --exclude=./analyze -C /results .
    tar_rc=$?
    if [ "$tar_rc" -le 1 ]; then
      curl --fail --silent --show-error --retry 2 --max-time 120 -X PUT --upload-file /tmp/ecbench-partial.tgz "$SNAPSHOT_UPLOAD_URL" && printf 'ECBENCH_PARTIAL_UPLOADED\n'
    fi
  done
) &
snapshot_pid=$!
bash /tmp/ecbench-bootstrap.sh
validation_rc=$?
kill "$snapshot_pid" 2>/dev/null
wait "$snapshot_pid" 2>/dev/null
tar -czf /tmp/ecbench-results.tgz --exclude=./target --exclude=./ladder --exclude=./analyze -C /results .
archive_rc=$?
if [ "$archive_rc" -eq 0 ]; then
  curl --fail --silent --show-error --retry 2 --max-time 180 -X PUT --upload-file /tmp/ecbench-results.tgz "$ARTIFACT_UPLOAD_URL"
  archive_rc=$?
  printf 'ECBENCH_ARCHIVE_SHA256 '
  sha256sum /tmp/ecbench-results.tgz | cut -d ' ' -f1
fi
printf 'ECBENCH_DONE validation=%s archive=%s\n' "$validation_rc" "$archive_rc"
sleep infinity
