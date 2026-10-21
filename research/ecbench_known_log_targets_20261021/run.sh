#!/usr/bin/env bash
# Thin orchestration only; ecbench performs measurement/replay and the Rust
# example recomputes recorded-value summaries. Never overwrite a session.
set -euo pipefail
crypto_study_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
crypto_repo_dir=$(cd -- "$crypto_study_dir/../.." && pwd)
crypto_binary=${CRYPTO_ECBENCH_BINARY:-"$crypto_repo_dir/target/release/ecbench"}
crypto_analysis=${CRYPTO_KNOWN_LOG_ANALYSIS:-"$crypto_repo_dir/target/release/examples/known_log_targets"}
crypto_batch=${CRYPTO_KNOWN_LOG_BATCH:-$(date -u +%Y%m%dT%H%M%SZ)}
crypto_cpus=${CRYPTO_KNOWN_LOG_CPUS:-none}
cd "$crypto_repo_dir"
mkdir -p "$crypto_study_dir/reports" "$crypto_study_dir/sessions"
if (($#)); then
  crypto_panels=("$@")
else
  crypto_panels=(smoke primary holdout generator-1 generator-2 generator-7 cofactor-v2)
fi
crypto_status=0
for crypto_panel in "${crypto_panels[@]}"; do
  case "$crypto_panel" in
    smoke|primary|holdout|generator-1|generator-2|generator-7|cofactor|cofactor-v2) ;;
    *) printf 'Unknown frozen panel: %s\n' "$crypto_panel" >&2; exit 2 ;;
  esac
  crypto_report="$crypto_study_dir/reports/run-$crypto_batch-$crypto_panel"
  crypto_session="$crypto_study_dir/sessions/$crypto_batch-$crypto_panel"
  mkdir "$crypto_report"
  {
    printf 'revision: '; git rev-parse HEAD
    printf 'binary: '; sha256sum "$crypto_binary"
    printf 'spec: '; sha256sum "$crypto_study_dir/specs/$crypto_panel.json"
    printf 'run: %q run --wait --spec %q --out %q --cpus %q\n' \
      "$crypto_binary" "$crypto_study_dir/specs/$crypto_panel.json" "$crypto_session" "$crypto_cpus"
    printf 'memory: inherited RLIMIT_AS=%s KiB; one thread per child\n' "$(ulimit -v)"
  } > "$crypto_report/COMMANDS.txt"
  if ! "$crypto_binary" run --wait --spec "$crypto_study_dir/specs/$crypto_panel.json" \
      --out "$crypto_session" --cpus "$crypto_cpus" \
      > "$crypto_report/run.stdout" 2> "$crypto_report/run.stderr"; then
    printf 'Run failed: %s; evidence retained\n' "$crypto_panel" >&2
    crypto_status=1
  fi
  if ! "$crypto_binary" verify --dir "$crypto_session" --replay-all --exit-code \
      --out "$crypto_report/audit.json" \
      > "$crypto_report/verify.stdout" 2> "$crypto_report/verify.stderr"; then
    printf 'Replay failed: %s; evidence retained\n' "$crypto_panel" >&2
    crypto_status=1
  fi
  if ! "$crypto_analysis" summarize "$crypto_session" "$crypto_report/summary.json" \
      > "$crypto_report/summary.stdout" 2> "$crypto_report/summary.stderr"; then
    printf 'Summary failed: %s; evidence retained\n' "$crypto_panel" >&2
    crypto_status=1
  fi
  for crypto_pair in 'abscissa known-log' 'rho-neg known-log' 'known-log known-log-aa' 'abscissa abscissa-aa'; do
    read -r crypto_a crypto_b <<< "$crypto_pair"
    if ! "$crypto_binary" compare --dir "$crypto_session" --a "$crypto_a" --b "$crypto_b" --json \
        > "$crypto_report/compare-$crypto_a-$crypto_b.json" \
        2> "$crypto_report/compare-$crypto_a-$crypto_b.stderr"; then
      crypto_status=1
    fi
  done
  "$crypto_binary" resources --dir "$crypto_session" --out "$crypto_report/resources.json" \
    > "$crypto_report/resources.stdout" 2> "$crypto_report/resources.stderr" || crypto_status=1
  "$crypto_binary" table --dir "$crypto_session" > "$crypto_report/table.txt" \
    2> "$crypto_report/table.stderr" || crypto_status=1
  if [[ -f "$crypto_report/audit.json" ]]; then
    (cd "$crypto_report" && sha256sum audit.json > audit.json.sha256)
  fi
  printf 'Panel recorded: %s\n' "$crypto_panel"
done
exit "$crypto_status"
