set -euo pipefail
mkdir -p "$ISOLAB_OUTPUT_DIR"
finish() {
  run_exit=$?
  trap - EXIT TERM INT
  set +e
  printf '{"exit_code":%s,"job_name":"%s","pod_uid":"%s","node_name":"%s"}\n' "$run_exit" "$ISOLAB_JOB_ID" "$POD_UID" "$NODE_NAME" > "$ISOLAB_OUTPUT_DIR/job-exit.json"
  (
    set -e
    cd "$ISOLAB_OUTPUT_DIR"
    shopt -s globstar nullglob
    for artifact in **/*; do
      if [[ -f "$artifact" && "$artifact" != SHA256SUMS ]]; then sha256sum "$artifact"; fi
    done
  ) > "$ISOLAB_OUTPUT_DIR/SHA256SUMS"
  hash_exit=$?
  if [[ "$hash_exit" != 0 ]]; then exit 90; fi
  if aws s3 cp "$ISOLAB_OUTPUT_DIR/" "$ISOLAB_RESULTS_URI" --recursive --only-show-errors; then
    printf 'Artifacts saved: %s\n' "$ISOLAB_RESULTS_URI"
  else
    exit 91
  fi
  exit "$run_exit"
}
trap finish EXIT
trap 'exit 143' TERM
trap 'exit 130' INT
cp /opt/isolab/build-info.json "$ISOLAB_OUTPUT_DIR/build-info.json"
cp /opt/isolab/specs/smoke-prime.json "$ISOLAB_OUTPUT_DIR/input-spec.json"
sha256sum /usr/local/bin/ecbench > "$ISOLAB_OUTPUT_DIR/binary.sha256"
aws sts get-caller-identity > "$ISOLAB_OUTPUT_DIR/runner-identity.json"
cat /proc/self/status > "$ISOLAB_OUTPUT_DIR/process-status.txt"
ecbench run --spec /opt/isolab/specs/smoke-prime.json --out "$ISOLAB_OUTPUT_DIR/session" --cpus inherit --allow-busy > "$ISOLAB_OUTPUT_DIR/run-summary.json" 2> "$ISOLAB_OUTPUT_DIR/run.stderr"
ecbench verify --dir "$ISOLAB_OUTPUT_DIR/session" --replay-all --exit-code --out "$ISOLAB_OUTPUT_DIR/audit.json" > "$ISOLAB_OUTPUT_DIR/audit.stdout" 2> "$ISOLAB_OUTPUT_DIR/audit.stderr"
cat "$ISOLAB_OUTPUT_DIR/audit.stderr"
