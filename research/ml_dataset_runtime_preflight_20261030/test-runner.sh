#!/bin/bash
set -euo pipefail
[[ $# == 1 ]] || { echo 'usage: test-runner.sh NEW_OUTPUT_DIRECTORY' >&2; exit 2; }
out=$1
[[ "$out" == /* && ! -e "$out" && ! -L "$out" ]]
mkdir -m 700 "$out"
source_file=$(cd "$(dirname "$0")" && pwd)/runner.rs
rustc --edition=2021 -D warnings -O "$source_file" -o "$out/runner"
source_commit=c41bec150e16e4c4f9a00634002b633b821de262
config=$(cd "$(dirname "$0")" && pwd)/DESIGN.md
"$out/runner" /bin/true "$source_commit" smoke_success 5 "$out/success" -- worker --config "$config" --out "$out/success/attempt"
jq -e '.command_completed and .exit_code==0 and .binary_stable and .inputs_stable and .observed_commands==1 and (.observed_wall_nanoseconds|tonumber)>0' "$out/success/receipt.json" >/dev/null
set +e
"$out/runner" /bin/false "$source_commit" smoke_failure 5 "$out/failure" -- worker --config "$config" --out "$out/failure/attempt"
code=$?
set -e
[[ "$code" == 1 ]]
jq -e '(.command_completed|not) and .exit_code==1 and .binary_stable and .inputs_stable' "$out/failure/receipt.json" >/dev/null
cat > "$out/sleep.sh" <<'EOF'
#!/bin/sh
sleep 3
EOF
chmod 700 "$out/sleep.sh"
set +e
"$out/runner" "$out/sleep.sh" "$source_commit" smoke_timeout 1 "$out/timeout" -- worker --config "$config" --out "$out/timeout/attempt"
code=$?
set -e
[[ "$code" == 1 ]]
jq -e '.timed_out and .exit_code==124 and (.command_completed|not) and (.observed_wall_nanoseconds|tonumber)>=900000000' "$out/timeout/receipt.json" >/dev/null
set +e
"$out/runner" /bin/true "$source_commit" smoke_reject 5 "$out/reject" -- worker --config "$config" --out "$out/reject/attempt" --publish > "$out/reject.stdout" 2> "$out/reject.stderr"
code=$?
set -e
[[ "$code" == 2 && ! -e "$out/reject" ]]
rg -q 'publication and cloud budget options are forbidden' "$out/reject.stderr"
jq -n --arg source_sha "$(sha256sum "$source_file" | cut -d' ' -f1)" --arg runner_sha "$(sha256sum "$out/runner" | cut -d' ' -f1)" \
  --arg success_sha "$(sha256sum "$out/success/receipt.json" | cut -d' ' -f1)" \
  --arg failure_sha "$(sha256sum "$out/failure/receipt.json" | cut -d' ' -f1)" \
  --arg timeout_sha "$(sha256sum "$out/timeout/receipt.json" | cut -d' ' -f1)" \
  '{schema:"ml-dataset.native-runtime-preflight-runner-test/v1",source_sha256:$source_sha,
    runner_sha256:$runner_sha,success_receipt_sha256:$success_sha,
    failure_receipt_sha256:$failure_sha,timeout_receipt_sha256:$timeout_sha,
    tests:{success:true,failure:true,timeout:true,publication_rejection:true},
    real_solver_commands:0,verified_cloud_dataset_cells:0}' > "$out/test-result.json"
cat "$out/test-result.json"
