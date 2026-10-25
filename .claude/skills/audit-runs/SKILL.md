---
name: audit-runs
description: Re-audit finished runbook and ecbench sessions with a nonce-sampled replay and digest checks, and the VALIDATION.json directories of the validation gate. Use for the daily audit, after a suspicious result, or when a row's evidence is in doubt. Reports; never edits sessions or results.
---

# audit-runs

Finished sessions are evidence and are never edited (AGENTS.md, ecbench
section). This skill re-checks a nonce-chosen sample of them so that a session
cannot be sure it will go unlooked-at.

## What it checks

1. **Sampling.** `scripts/audit_runs.py` draws a nonce at run time (or takes
   `--nonce` to reproduce a run), ranks every candidate by `sha256(nonce:kind:dir)`
   and audits the first `--sample N`. Candidates: every `ecbench.session/v1`
   directory with status `complete` and a `records.jsonl`, and every directory
   with a `VALIDATION.json` (fixtures excluded).
2. **Digest checks.** `session.json`'s `plan_sha256`, `host_sha256` and
   `records_sha256` against the bytes on disk. The four scratch sessions pinned
   in `docs/ecbench/INCOMPLETE_SESSIONS.tsv` are checked against their pins.
   `spec_sha256` is over the canonical spec and is left to `ecbench verify`.
3. **Replay.** With `--ecbench BIN --replay K`: `ecbench verify --dir D --replay K
   --exit-code` re-derives K runs. Counts and answers only on a shared runner.
4. **Validation directories.** `VALIDATION.json` is re-checked in full
   (docs/VALIDATION_GATE.md): pinned hashes, the validator's headline
   recompute, the matched rho reference and the 0 / 0.1 / 1 GAE band.

## Run it

    cargo build --release --bin ecbench
    python3 scripts/audit_runs.py --sample 8 --ecbench ./target/release/ecbench \
        --replay 4 --out audit-runs.json

Exit status 1 and a `FAIL <dir>: <reason>` line per failure. Quote the nonce in
any report so the sample can be redrawn.

## Where it runs

- **CI:** `.github/workflows/audit-runs-daily.yml`, daily at 05:41 UTC.
- **Scheduled-task alternative** (when CI cannot build `ecbench` or has no
  budget): a scheduled task in a Claude Code session on a runner that has the
  binary, firing daily with the prompt "Run `python3 scripts/audit_runs.py
  --sample 8 --ecbench ./target/release/ecbench --replay 4 --out audit-runs.json`
  from the repository root; report the nonce and every FAIL line; do not edit
  any session." Create it with the `create_trigger` tool (cron `41 5 * * *`,
  `create_new_session_on_fire`); a fresh session per firing keeps one day's
  context out of the next.

## Rules

- Report failures as found; never repair a session, a certificate or a row.
- A failed audit of a row's evidence is a reason to mark the row failed in the
  next sweep verdict, not to delete it.
