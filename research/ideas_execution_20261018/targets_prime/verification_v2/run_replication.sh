#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
study=research/ideas_execution_20261018/targets_prime
amendment="$study/verification_v2"
run_dir="$amendment/runs/run-20261018-fp2-replication-v2"
if [ -e "$run_dir" ]; then printf 'run directory exists\n' >&2; exit 2; fi
mkdir -p "$run_dir"
sha256sum "$amendment/replication-protocol.json" "$amendment/diagnostic_v2.cpp" "$amendment/compare_replication.cpp" "$amendment/verify_v2.cpp" "$amendment/run_replication.sh" "$amendment/diagnostic_v2" "$amendment/compare_replication" "$amendment/verify_v2" "$study/diagnostic.cpp" "$study/protocol.json" "$study/runs/run-20261018-native-v1/results.jsonl" > "$run_dir/frozen-inputs.sha256"
date -u +%Y-%m-%dT%H:%M:%SZ > "$run_dir/started-utc.txt"
printf 'case\texpected_exit\tactual_exit\n' > "$run_dir/attempts.tsv"
trial() {
  local name="$1"; shift
  printf '%q ' "$@" > "$run_dir/$name.command.txt"; printf '\n' >> "$run_dir/$name.command.txt"
  set +e
  (ulimit -v 524288; ulimit -t 300; "$@") > "$run_dir/$name.stdout" 2> "$run_dir/$name.stderr"
  local actual=$?
  set -e
  printf '%s\t0\t%s\n' "$name" "$actual" >> "$run_dir/attempts.tsv"
  if [ "$actual" -ne 0 ]; then printf 'replication stage failed: %s\n' "$name" >&2; exit 1; fi
}
trial native-controls "$amendment/diagnostic_v2" --test
trial fp2-panel "$amendment/diagnostic_v2" "$run_dir/data"
new_hash=$(sha256sum "$run_dir/data/results.jsonl" | cut -d' ' -f1)
printf '%s\n' "$new_hash" > "$run_dir/results.sha256"
trial complete-verification "$amendment/verify_v2" ordered "$run_dir/data" "$new_hash"
trial exact-cohort-comparison "$amendment/compare_replication" "$study/runs/run-20261018-native-v1/results.jsonl" "$run_dir/data/results.jsonl" "$run_dir/data/candidate-draws.jsonl" "$run_dir/cohort-comparison.json"
sha256sum "$run_dir"/data/* "$run_dir/cohort-comparison.json" > "$run_dir/output.sha256"
date -u +%Y-%m-%dT%H:%M:%SZ > "$run_dir/finished-utc.txt"
