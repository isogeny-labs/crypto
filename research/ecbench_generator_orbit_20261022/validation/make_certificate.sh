#!/usr/bin/env bash
# Assemble replay-certificate.json from the validator's own replay receipts
# (validation/replay-<session>.json, written by
#  ./target/release/ecbench verify --dir sessions/<s> --replay-all --exit-code --out validation/replay-<s>.json)
# and the independent scalar check (validation/scalar-check.json, verify_scalars.py).
set -euo pipefail
V="$(cd "$(dirname "$0")" && pwd)"
HERE="$(dirname "$V")"
ROOT="$(git -C "$HERE" rev-parse --show-toplevel)"
BIN="$ROOT/target/release/ecbench"
SESSIONS="a0-n13 a0-n19 a0-n23 a0-n31 a0-n37 a1-n17 a1-n29"
BIN_SHA=$(sha256sum "$BIN" | cut -c1-64)
HOST_ENV=$("$BIN" host --json | jq -r '.env_class_id')
runs='[]'
for s in $SESSIONS; do
  d="$HERE/sessions/$s"; rc="$V/replay-$s.json"
  run=$(jq -n --slurpfile r "$rc" --slurpfile sess "$d/session.json" --arg s "$s" \
      --arg rsha "$(sha256sum "$rc" | cut -c1-64)" \
      --arg recsha "$(sha256sum "$d/records.jsonl" | cut -c1-64)" \
      --argjson lines "$(wc -l < "$d/records.jsonl")" \
      --argjson recbins "$(jq -s '[.[].binary_sha256] | unique' "$d/records.jsonl")" \
      --slurpfile sc "$V/scalar-check.json" '
    ($r[0]) as $r | ($sess[0]) as $se | ($sc[0].sessions[] | select(.session == $s)) as $c
    | { session: $s, session_id: $r.session_id,
        session_id_matches: ($r.session_id == $se.session_id),
        records: $lines, records_sha256: $recsha,
        records_sha256_matches_session_json: ($recsha == $se.records_sha256),
        warmups_not_replayed: ($lines - ($r.replays | length)),
        replays: ($r.replays | length),
        reproduced: ([ $r.replays[] | select(.reproduced == true) ] | length),
        replay_details: ([ $r.replays[].detail ] | unique),
        audit_ok: $r.ok, audit_problems: $r.problems,
        audit_checks_all_true: ([ $r.checks[] ] | all),
        ecbench_verified_records: $r.verified_records,
        auditor_env_class_id: $r.auditor_env_class_id,
        auditor_hardware_class_id: $r.auditor_hardware_class_id,
        session_env_class_id: $se.env_class_id,
        independent_scalar_check: { scalars_verified: $c.scalars_verified, scalars_failed: $c.scalars_failed,
                                    modulus_hex: $c.modulus_hex, targets_on_curve: $c.targets_on_curve,
                                    targets_in_subgroup: $c.targets_in_subgroup },
        scalar_ok: ($r.verified_records == $lines and $c.ok and $c.scalars_verified == $lines),
        record_binary_sha256: $recbins,
        session_binary_sha256: $se.binary_sha256,
        session_git_commit: $se.git_commit, session_git_dirty: $se.git_dirty,
        receipt: "validation/replay-\($s).json", receipt_sha256: $rsha,
        log: "validation/replay-\($s).log" }')
  runs=$(jq -c --argjson x "$run" '. + [$x]' <<< "$runs")
done
jq -n --argjson runs "$runs" --arg bin "$BIN_SHA" --arg env "$HOST_ENV" '
  ($runs | map(.replays == .reproduced and .audit_ok and .audit_checks_all_true and (.audit_problems | length) == 0
               and .session_id_matches and .records_sha256_matches_session_json) | all) as $replay_ok
  | ($runs | map(.scalar_ok) | all) as $scalar_ok
  | ($runs | map(.record_binary_sha256 == [$bin] and .session_binary_sha256 == $bin) | all) as $bin_ok
  | { schema: "replay-certificate/v1",
      study: "research/ecbench_generator_orbit_20261022",
      snapshot: {branch: "claude/ic-generator-orbit", head: "45a11de2eb7aa01220e07c3ec513662bf8c193fa", tree_clean: true},
      status: (if $replay_ok and $scalar_ok and $bin_ok then "verified" else "failed" end),
      independent: true,
      independence: "Replayed by the validator, a separate agent from the measuring session, from the committed snapshot; scalars re-checked a second time with validator-written code (validation/verify_scalars.py, Python GF(2^n) arithmetic, no repository code). Same binary and same host class as the sessions: not a cross-host or cross-implementation replay.",
      scalars_verified: $scalar_ok,
      scalars_verified_count: ($runs | map(.independent_scalar_check.scalars_verified) | add),
      records: ($runs | map(.records) | add),
      replays: ($runs | map(.replays) | add),
      reproduced: ($runs | map(.reproduced) | add),
      warmups_not_replayed: ($runs | map(.warmups_not_replayed) | add),
      warmup_note: "ecbench verify --replay-all re-executes measured (non-warm-up) deterministic runs; warm-up records are covered by the audit integrity checks and both scalar checks, not by replay.",
      binary: { path: "target/release/ecbench", sha256: $bin,
                matches_every_record_and_session: $bin_ok,
                session_git_commit: "de57ca373f79560121b266dfc75a1ebb62864870", session_git_dirty: false,
                rebuilt: false,
                note: "No mismatch, so nothing was rebuilt. The binary embeds no git commit; its binding to source rests on session.json (git_commit de57ca373f, git_dirty false at run time) and on timing: built 2026-10-22T08:42:40Z, after d2b3a56c32 (08:37:49Z, the last commit touching src/), and de57ca373f changed only PROTOCOL.md, run.sh and cards/attempt-1." },
      host: { env_class_id: $env,
              auditor_hardware_class_id: ($runs | map(.auditor_hardware_class_id) | unique),
              session_env_class_ids: ($runs | map(.session_env_class_id) | unique),
              same_host_class_as_sessions: ($runs | map(.session_env_class_id == $env and .auditor_env_class_id == $env) | all),
              note: "Same host class as the sessions (env class \($env), the env_class_id of every session and record). The cross-host replay the gate asks for was not available in this container." },
      command: "./target/release/ecbench verify --dir research/ecbench_generator_orbit_20261022/sessions/<session> --replay-all --exit-code --out research/ecbench_generator_orbit_20261022/validation/replay-<session>.json (exit 0 for all seven)",
      runs: $runs }
' > "$V/replay-certificate.json"
jq -r '"status=\(.status) records=\(.records) replays=\(.replays) reproduced=\(.reproduced) scalars=\(.scalars_verified_count) bin_ok=\(.binary.matches_every_record_and_session)"' "$V/replay-certificate.json"
