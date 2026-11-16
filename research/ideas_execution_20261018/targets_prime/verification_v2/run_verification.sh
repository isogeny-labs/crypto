#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
study=research/ideas_execution_20261018/targets_prime
amendment="$study/verification_v2"
run_dir="$amendment/runs/run-20261018-verifier-v2"
ordered="$study/runs/run-20261018-native-v1"
strict="$study/runs/run-20261018-strict4-v1"
ordered_hash=3d5790d8a7a0218e27d6d710c2cf77f47849f7614134f2f9423012e8fc081243
strict_hash=1106adee5ec933190f4d598c9d901f435bc3ee106967dcd1c8de57dcc97ad451
if [ -e "$run_dir" ]; then printf 'run directory exists\n' >&2; exit 2; fi
mkdir -p "$run_dir"
sha256sum "$amendment/protocol.json" "$amendment/verify_v2.cpp" "$amendment/controls.cpp" "$amendment/vendor/json.hpp" "$amendment/verify_v2" "$amendment/controls" "$ordered/results.jsonl" "$ordered/certificates.jsonl" "$strict/results.jsonl" "$strict/certificates.jsonl" "$study/protocol.json" "$study/strict4-protocol.json" "$study/diagnostic.cpp" "$study/verify.cpp" > "$run_dir/frozen-inputs.sha256"
date -u +%Y-%m-%dT%H:%M:%SZ > "$run_dir/started-utc.txt"
printf 'case\texpected_exit\tactual_exit\tcontrol_pass\n' > "$run_dir/attempts.tsv"
trial() {
  local name="$1" expected="$2"; shift 2
  printf '%q ' "$@" > "$run_dir/$name.command.txt"; printf '\n' >> "$run_dir/$name.command.txt"
  set +e
  (ulimit -v 524288; ulimit -t 30; "$@") > "$run_dir/$name.stdout" 2> "$run_dir/$name.stderr"
  local actual=$?
  set -e
  printf '%s\t%s\t%s\t%s\n' "$name" "$expected" "$actual" "$([ "$actual" -eq "$expected" ] && printf true || printf false)" >> "$run_dir/attempts.tsv"
  if [ "$actual" -ne "$expected" ]; then printf 'unexpected control result: %s\n' "$name" >&2; exit 1; fi
}
"$amendment/controls" "$run_dir/controls" > "$run_dir/controls.stdout" 2> "$run_dir/controls.stderr"
sha256sum "$run_dir"/controls/*/results.jsonl "$run_dir"/controls/*/certificates.jsonl > "$run_dir/adversarial-inputs.sha256"
trial ordered 0 "$amendment/verify_v2" ordered "$ordered" "$ordered_hash"
trial strict 0 "$amendment/verify_v2" strict "$strict" "$strict_hash" "$ordered" "$ordered_hash"
for name in wrong-target-id missing-one missing-all duplicate-certificate extra-target altered-on-curve-target false-strict-claim duplicate-json-key; do
  control_hash=$(sha256sum "$run_dir/controls/$name/results.jsonl" | cut -d' ' -f1)
  trial "$name" 1 "$amendment/verify_v2" strict "$run_dir/controls/$name" "$control_hash" "$ordered" "$ordered_hash"
done
trial wrong-result-digest 1 "$amendment/verify_v2" strict "$strict" 0000000000000000000000000000000000000000000000000000000000000000 "$ordered" "$ordered_hash"
date -u +%Y-%m-%dT%H:%M:%SZ > "$run_dir/finished-utc.txt"
