set +e
printf '%s' "$BOOTSTRAP_BASE64" | base64 -d > /tmp/ecbench-bootstrap.sh
bash /tmp/ecbench-bootstrap.sh
validation_rc=$?
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
